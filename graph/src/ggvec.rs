use super::*;
use atomic_float::AtomicF32;
use indicatif::ProgressIterator;
use itertools::Itertools;
use rayon::iter::IndexedParallelIterator;
use rayon::iter::IntoParallelIterator;
use rayon::iter::IntoParallelRefIterator;
use rayon::iter::ParallelIterator;
use std::sync::atomic::Ordering;
use vec_rand::{random_f32, sample_uniform};

impl Graph {
    #[manual_binding]
    /// Returns embedding obtained using the GGVCE algorithm.
    ///
    /// # Parameters
    /// * `embedding`: &mut [f32] - The mutable unrolled embedding to edit inplace.
    /// * `embedding_size`: Option<usize> - The embedding size. By default, 100.
    /// * `epochs`: Option<usize> - Maximal number of epochs to train for. By default, 500.
    /// * `negative_ratio`: Option<f32> - Negative sampling ratio. Setting this higher will do more negative sampling. This is slower, but can lead to higher quality embeddings.
    /// * `exponent`: Option<f32> - Weighing exponent in loss function. Having this lower reduces effect of large edge weights.
    /// * `tollerance`: Option<f64> - Optimization early stopping criterion. Stops average loss < tollerance for tol_samples epochs. By default, sets as a function of learning_rate
    /// * `patience`: Option<usize> - Optimization early stopping criterion. This is the number of epochs to sample for loss stability. Once loss is stable over this number of epochs we stop early. By defauly 75.
    /// * `negative_decay`: Option<f32> - Decay on negative ratio. If >0 then negative ratio will decay by (1-negative_decay) ** epoch. You should usually leave this to 0.
    /// * `learning_rate`: Option<f32> - Optimization learning rate.
    /// * `max_loss`: Option<f32> - Loss value ceiling for numerical stability.
    /// * `random_state`: Option<u64> - Random rate for reproducible embeddings.
    /// * `verbose`: Option<bool> - Whether to show the loading bar. By default, true.
    ///
    /// # References
    /// Please refer to the original implementation that can be found
    /// in the [CSRGraph repository](https://github.com/VHRanger/CSRGraph/blob/master/csrgraph/ggvec.py)
    ///
    /// # Raises
    /// * If the number of epochs is not a strictly positive integer.
    /// * If the negative ratio is not between zero and one.
    /// * If the negative decay is not between zero and one.
    /// * If the learning rate is not between zero and one.
    /// * If the graph does not have weights.
    pub fn compute_ggvec_embedding(
        &self,
        embedding: &mut [f32],
        embedding_size: Option<usize>,
        epochs: Option<usize>,
        negative_ratio: Option<f32>,
        exponent: Option<f32>,
        tollerance: Option<f64>,
        patience: Option<usize>,
        negative_decay: Option<f32>,
        learning_rate: Option<f32>,
        max_loss: Option<f32>,
        random_state: Option<u64>,
        verbose: Option<bool>,
    ) -> Result<()> {
        let embedding_size = embedding_size.unwrap_or(100);
        let epochs = epochs.unwrap_or(500);
        let negative_ratio = negative_ratio.unwrap_or(0.15);
        let negative_decay = negative_decay.unwrap_or(0.0);
        let learning_rate = learning_rate.unwrap_or(0.05);
        let exponent = exponent.unwrap_or(0.5);
        let patience = patience.unwrap_or(75);
        let max_loss = max_loss.unwrap_or(30.0);
        let tollerance = tollerance.unwrap_or(learning_rate.max(0.1) as f64 / 2.0);
        let mut random_state = random_state.unwrap_or(42);
        random_state = splitmix64(random_state);
        let verbose = verbose.unwrap_or(true);
        let number_of_directed_edges = self.get_number_of_directed_edges();
        let nodes_number = self.get_nodes_number();
        let scale_factor = (embedding_size as f32).sqrt();

        for (variable_name, value) in vec![
            ("embedding size", embedding_size),
            ("epochs", epochs),
            ("patience", patience),
        ] {
            if value == 0 {
                return Err(format!(
                    "The {} is expected to be a strictly positive integer.",
                    variable_name
                ));
            }
        }

        for (variable_name, value) in vec![
            ("negative ratio", negative_ratio),
            ("negative decay", negative_decay),
            ("learning rate", learning_rate),
        ] {
            if value < 0.0 || value > 1.0 {
                return Err(format!(
                    "The {} is expected to be between zero and one, but you have provided `{}`.",
                    variable_name, value
                ));
            }
        }

        if !self.has_nodes() {
            return Err("The current graph does not have any node.".to_string());
        }

        if !self.has_edge_weights() {
            return Err("The current graph does not have edge weights.".to_string());
        }

        let expected_embedding_len = embedding_size * self.get_nodes_number() as usize;

        if embedding.len() != expected_embedding_len {
            return Err(format!(
                "The given memory allocation for the embeddings is {} long but we expect {}.",
                embedding.len(),
                expected_embedding_len
            ));
        }

        let embedding = unsafe { core::mem::transmute::<&mut [f32], &mut [AtomicF32]>(embedding) };
        let bias = unsafe {
            core::mem::transmute::<Vec<f32>, Vec<AtomicF32>>(vec![
                0.0;
                self.get_nodes_number() as usize
            ])
        };

        embedding.par_iter().enumerate().for_each(|(i, e)| {
            e.store(random_f32(random_state + i as u64) - 0.5, Ordering::SeqCst)
        });

        let pb = get_loading_bar(verbose, "Training GGVEC model", epochs);

        let predict_edge_score = |src: usize, dst: usize| -> f32 {
            (embedding[(src * embedding_size)..((src + 1) * embedding_size)]
                .iter()
                .zip(embedding[(dst * embedding_size)..((dst + 1) * embedding_size)].iter())
                .map(|(v1, v2)| v1.load(Ordering::SeqCst) * v2.load(Ordering::SeqCst))
                .sum::<f32>()
                + bias[src].load(Ordering::SeqCst)
                + bias[dst].load(Ordering::SeqCst))
                / scale_factor
        };

        let clip_loss = |score: f32| -> f32 {
            if score < -max_loss {
                -max_loss
            } else if score > max_loss {
                max_loss
            } else {
                score
            }
        };

        let update_embedding_and_bias = |src: usize, dst: usize, loss: f32| {
            embedding[(src * embedding_size)..((src + 1) * embedding_size)]
                .iter()
                .zip(embedding[(dst * embedding_size)..((dst + 1) * embedding_size)].iter())
                .for_each(|(v1, v2)| {
                    let destination_value = v2.load(Ordering::SeqCst);
                    let source_value = v1
                        .fetch_update(Ordering::SeqCst, Ordering::SeqCst, |source_value| {
                            let new_value = source_value - destination_value * loss * learning_rate;
                            Some(if new_value < -1.0 {
                                -1.0
                            } else if new_value > 1.0 {
                                1.0
                            } else {
                                new_value
                            })
                        })
                        .unwrap();
                    let _ = v2
                        .fetch_update(Ordering::SeqCst, Ordering::SeqCst, |destination_value| {
                            let new_value = destination_value - source_value * loss * learning_rate;
                            Some(if new_value < -1.0 {
                                -1.0
                            } else if new_value > 1.0 {
                                1.0
                            } else {
                                new_value
                            })
                        })
                        .unwrap();
                });
            bias[src].fetch_sub(learning_rate * loss, Ordering::SeqCst);
            bias[dst].fetch_sub(learning_rate * loss, Ordering::SeqCst);
        };

        let mut losses = vec![f64::INFINITY; patience];

        for epoch in (0..epochs).progress_with(pb) {
            random_state = splitmix64(random_state);

            // We compute the number of negative edges to be sampled during this epoch,
            // according to the negative decay parameter (if it is greater than zero).
            let number_of_negative_edges_to_sample = (number_of_directed_edges as f32
                * negative_ratio
                * ((1.0 - negative_decay).powf(epoch as f32)))
            .floor() as EdgeT;

            // Execute the relaxation pass
            // Start to iterate on the required amount of negative edges
            (0..number_of_negative_edges_to_sample)
                .into_par_iter()
                .for_each(|seed| {
                    let random_edge_id =
                        sample_uniform(number_of_directed_edges, splitmix64(random_state + seed))
                            as EdgeT;
                    let src = ((random_edge_id & 0xffffffff) as u32 % nodes_number) as usize;
                    let dst = ((random_edge_id >> 32) as u32 % nodes_number) as usize;
                    update_embedding_and_bias(src, dst, clip_loss(predict_edge_score(src, dst)));
                });

            // Execute the contraction pass
            let loss = self
                .par_iter_directed_edge_node_ids()
                .zip(self.par_iter_edge_weights().unwrap())
                .map(|((_, src, dst), edge_weight)| {
                    let src = src as usize;
                    let dst = dst as usize;
                    let loss = clip_loss(
                        (predict_edge_score(src, dst) - edge_weight)
                            .powf(exponent)
                            .abs(),
                    );
                    update_embedding_and_bias(src, dst, loss);
                    loss.abs() as f64
                })
                .sum::<f64>()
                / (number_of_directed_edges as f64);

            let (min_latest, max_latest) = losses.iter().minmax().into_option().unwrap().to_owned();
            let min_latest = min_latest.to_owned();
            let max_latest = max_latest.to_owned();
            let improvement = (max_latest - min_latest).abs() / max_latest;
            if epoch > patience && improvement < tollerance {
                if loss < max_latest {
                    break;
                }
                return Err(format!(
                    concat!(
                        "Could not learn: loss {loss} = max loss {max_latest} ",
                        "This is often due to too large learning rates, which ",
                        "currently is set to {learning_rate}."
                    ),
                    loss = loss,
                    max_latest = max_latest,
                    learning_rate = learning_rate
                ));
            } else if !loss.is_finite() {
                return Err(format!(
                    concat!(
                        "The loss is currently non-finite. ",
                        "This is often due to too large learning rates, which ",
                        "currently is set to {learning_rate}."
                    ),
                    learning_rate = learning_rate
                ));
            } else {
                losses.rotate_right(1);
                losses[patience - 1] = loss;
            }
        }
        Ok(())
    }
}
