use crate::*;
use express_measures::dot_product_sequential_unchecked;
use graph::{EdgeTypeT, Graph, NodeT, ThreadDataRaceAware};
use indicatif::ProgressIterator;
use indicatif::{ProgressBar, ProgressStyle};
use rayon::prelude::*;
use vec_rand::splitmix64;

#[derive(Clone, Debug)]
pub struct TransH {
    embedding_size: usize,
    relu_bias: f32,
    random_state: u64,
}

impl TransH {
    /// Return new instance of TransH model.
    ///
    /// # Arguments
    /// `embedding_size`: Option<usize> - Size of the embedding.
    /// `relu_bias`: Option<f32> - The bias to apply to the relu. By default, 1.0.
    /// `random_state`: Option<u64> - The random state to use to reproduce the training.
    pub fn new(
        embedding_size: Option<usize>,
        relu_bias: Option<f32>,
        random_state: Option<u64>,
    ) -> Result<Self, String> {
        // Handle the values of the default parameters.
        let relu_bias = relu_bias.unwrap_or(1.0);
        let random_state = random_state.unwrap_or(42);

        // Validate that the provided parameters are within
        // reasonable bounds.
        let embedding_size = must_not_be_zero(embedding_size, 100, "embedding size")?;

        Ok(Self {
            embedding_size,
            relu_bias,
            random_state,
        })
    }

    /// Returns the used embedding size.
    pub fn get_embedding_size(&self) -> usize {
        self.embedding_size
    }

    /// Computes in the provided slice of embedding the TransH node and edge type embedding.
    ///
    /// # Implementative details
    /// This implementation is NOT thread safe, that is, different threads may try
    /// to overwrite each others memory. This version is faster than the memory safe
    /// version and requires less memory. In most use cases, you would prefer to use
    /// this version over the memory safe version.
    ///
    /// # Arguments
    /// `graph`: &Graph - The graph to embed
    /// `node_embedding`: &mut [f32] - The memory area where to write the node embedding.
    /// `multiplicative_edge_type_embedding`: &mut [f32] - The optional memory area where to write the multiplicative edge type embedding.
    /// `bias_edge_type_embedding`: &mut [f32] - The optional memory area where to write the bias edge type embedding.
    /// `epochs`: Option<usize> - The number of epochs to run the model for, by default 10.
    /// `learning_rate`: Option<f32> - The learning rate to update the gradient, by default 0.01.
    /// `learning_rate_decay`: Option<f32> - Factor to reduce the learning rate for at each epoch. By default 0.9.
    /// `verbose`: Option<bool> - Whether to show the loading bar, by default true.
    ///
    /// # Raises
    /// * If graph does not have node types and node types should be used.
    /// * If graph contains unknown node types and node types should be used.
    /// * If graph does not have edge types and edge types should be used.
    /// * If graph contains unknown edge types and edge types should be used.
    pub fn fit_transform(
        &self,
        graph: &Graph,
        node_embedding: &mut [f32],
        multiplicative_edge_type_embedding: &mut [f32],
        bias_edge_type_embedding: &mut [f32],
        epochs: Option<usize>,
        learning_rate: Option<f32>,
        learning_rate_decay: Option<f32>,
        verbose: Option<bool>,
    ) -> Result<(), String> {
        let epochs = epochs.unwrap_or(10);
        let verbose = verbose.unwrap_or(true);
        let scale_factor = (self.embedding_size as f32).sqrt();
        let mut learning_rate = learning_rate.unwrap_or(0.001) / scale_factor;
        let learning_rate_decay = learning_rate_decay.unwrap_or(0.9);
        let mut random_state = splitmix64(self.random_state);

        if !graph.has_edge_types() {
            return Err(concat!(
                "The edge types should be used, but the provided ",
                "graph does not contain edge types."
            )
            .to_string());
        }

        if graph.has_unknown_edge_types().unwrap() {
            return Err(concat!(
                "The edge types should be used, but the provided ",
                "graph contains unknown edge types and it is not ",
                "well-defined how to use them."
            )
            .to_string());
        }

        if graph.has_homogeneous_edge_types().unwrap() {
            return Err(concat!(
                "The edge types should be used, but the provided ",
                "graph contains exclusively a single edge type ",
                "making using edge types useless."
            )
            .to_string());
        }

        let edge_types_number = graph.get_number_of_edge_types().unwrap() as usize;
        let expected_edge_embedding_size = self.embedding_size * edge_types_number;

        if multiplicative_edge_type_embedding.len() != expected_edge_embedding_size {
            return Err(format!(
                "The given memory allocation for the multiplicative edge type embeddings is {} long but we expect {}.",
                multiplicative_edge_type_embedding.len(),
                expected_edge_embedding_size
            ));
        }

        if bias_edge_type_embedding.len() != expected_edge_embedding_size {
            return Err(format!(
                "The given memory allocation for the bias edge type embeddings is {} long but we expect {}.",
                bias_edge_type_embedding.len(),
                expected_edge_embedding_size
            ));
        }

        if !graph.has_nodes() {
            return Err("The provided graph does not have any node.".to_string());
        }

        let number_of_directed_edges = graph.get_number_of_directed_edges();
        let nodes_number = graph.get_number_of_nodes();
        let expected_node_embedding_size = self.embedding_size * nodes_number as usize;

        if node_embedding.len() != expected_node_embedding_size {
            return Err(format!(
                "The given memory allocation for the embeddings is {} long but we expect {}.",
                node_embedding.len(),
                expected_node_embedding_size
            ));
        }

        // Populate the embedding layers with random uniform value
        populate_vectors(
            &mut [
                node_embedding,
                multiplicative_edge_type_embedding,
                bias_edge_type_embedding,
            ],
            random_state,
            scale_factor,
        );

        let shared_node_embedding = ThreadDataRaceAware::new(node_embedding);
        let shared_multiplicative_edge_type_embedding =
            ThreadDataRaceAware::new(multiplicative_edge_type_embedding);
        let shared_bias_edge_type_embedding = ThreadDataRaceAware::new(bias_edge_type_embedding);

        // Depending whether verbosity was requested by the user
        // we create or not a visible progress bar to show the progress
        // in the training epochs.
        let epochs_progress_bar = if verbose {
            let pb = ProgressBar::new(epochs as u64);
            pb.set_style(ProgressStyle::default_bar().template(
                "TransH {msg} {spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] ({pos}/{len}, ETA {eta})",
            ));
            pb
        } else {
            ProgressBar::hidden()
        };

        let compute_mini_batch_step = |src: usize,
                                       not_src: usize,
                                       dst: usize,
                                       not_dst: usize,
                                       edge_type: usize,
                                       learning_rate: f32| unsafe {
            let src_embedding = {
                &mut (*shared_node_embedding.get())
                    [(src * self.embedding_size)..((src + 1) * self.embedding_size)]
            };
            let not_src_embedding = {
                &mut (*shared_node_embedding.get())
                    [(not_src * self.embedding_size)..((not_src + 1) * self.embedding_size)]
            };
            let dst_embedding = {
                &mut (*shared_node_embedding.get())
                    [(dst * self.embedding_size)..((dst + 1) * self.embedding_size)]
            };
            let not_dst_embedding = {
                &mut (*shared_node_embedding.get())
                    [(not_dst * self.embedding_size)..((not_dst + 1) * self.embedding_size)]
            };
            let multiplicative_edge_type_embedding = {
                &mut (*shared_multiplicative_edge_type_embedding.get())
                    [(edge_type * self.embedding_size)..((edge_type + 1) * self.embedding_size)]
            };
            let bias_edge_type_embedding = {
                &mut (*shared_bias_edge_type_embedding.get())
                    [(edge_type * self.embedding_size)..((edge_type + 1) * self.embedding_size)]
            };

            let (dst_norm, not_dst_norm, src_norm, not_src_norm, multiplicative_norm, bias_norm) = (
                norm(dst_embedding),
                norm(not_dst_embedding),
                norm(src_embedding),
                norm(not_src_embedding),
                norm(multiplicative_edge_type_embedding),
                norm(bias_edge_type_embedding),
            );

            src_embedding.iter_mut().for_each(|src_feature| {
                *src_feature /= src_norm;
            });
            dst_embedding.iter_mut().for_each(|dst_feature| {
                *dst_feature /= dst_norm;
            });
            not_src_embedding.iter_mut().for_each(|not_src_feature| {
                *not_src_feature /= not_src_norm;
            });
            not_dst_embedding.iter_mut().for_each(|not_dst_feature| {
                *not_dst_feature /= not_dst_norm;
            });
            multiplicative_edge_type_embedding
                .iter_mut()
                .for_each(|mult_feature| {
                    *mult_feature /= multiplicative_norm;
                });
            bias_edge_type_embedding
                .iter_mut()
                .for_each(|bias_feature| {
                    *bias_feature /= bias_norm;
                });

            let mult_dot_bias =
                dot_product_sequential_unchecked(src_embedding, multiplicative_edge_type_embedding);

            let src_dot_mult =
                dot_product_sequential_unchecked(src_embedding, multiplicative_edge_type_embedding);

            let not_src_dot_mult = dot_product_sequential_unchecked(
                not_src_embedding,
                multiplicative_edge_type_embedding,
            );

            let dst_dot_mult =
                dot_product_sequential_unchecked(dst_embedding, multiplicative_edge_type_embedding);

            let not_dst_dot_mult = dot_product_sequential_unchecked(
                not_dst_embedding,
                multiplicative_edge_type_embedding,
            );

            let true_dot_delta = dst_dot_mult - src_dot_mult;
            let false_dot_delta = not_dst_dot_mult - not_src_dot_mult;

            let mut true_triple_distance_squared_sum: f32 = 0.0;
            let true_triple_feature_wise_distance_vector = src_embedding
                .iter()
                .zip(dst_embedding.iter())
                .zip(
                    multiplicative_edge_type_embedding
                        .iter()
                        .zip(bias_edge_type_embedding.iter()),
                )
                .map(
                    |((src_feature, dst_feature), (mult_feature, bias_feature))| {
                        let distance = src_feature - dst_feature
                            + bias_feature
                            + mult_feature * true_dot_delta;
                        true_triple_distance_squared_sum += distance.powf(2.0);
                        distance
                    },
                )
                .collect::<Vec<f32>>();

            let true_triple_distance_norm = true_triple_distance_squared_sum.sqrt();

            let mut false_triple_distance_squared_sum: f32 = 0.0;
            let false_triple_feature_wise_distance_vector = not_src_embedding
                .iter()
                .zip(not_dst_embedding.iter())
                .zip(
                    multiplicative_edge_type_embedding
                        .iter()
                        .zip(bias_edge_type_embedding.iter()),
                )
                .map(
                    |((not_src_feature, not_dst_feature), (mult_feature, bias_feature))| {
                        let distance = not_src_feature - not_dst_feature
                            + bias_feature
                            + mult_feature * false_dot_delta;
                        false_triple_distance_squared_sum += distance.powf(2.0);
                        distance
                    },
                )
                .collect::<Vec<f32>>();

            let false_triple_distance_norm = false_triple_distance_squared_sum.sqrt();

            // If the delta is lower than zero, there is no need to continue
            // further, as the gradient will be zero.
            if false_triple_distance_norm - true_triple_distance_norm > self.relu_bias {
                return 0.0;
            }

            let src_prior = compute_prior(
                graph.get_unchecked_node_degree_from_node_id(src as NodeT) as f32,
                nodes_number as f32,
            );
            let dst_prior = compute_prior(
                graph.get_unchecked_node_degree_from_node_id(dst as NodeT) as f32,
                nodes_number as f32,
            );
            let not_src_prior = compute_prior(
                graph.get_unchecked_node_degree_from_node_id(not_src as NodeT) as f32,
                nodes_number as f32,
            );
            let not_dst_prior = compute_prior(
                graph.get_unchecked_node_degree_from_node_id(not_dst as NodeT) as f32,
                nodes_number as f32,
            );
            let edge_type_prior = compute_prior(
                {
                    graph.get_unchecked_edge_count_from_edge_type_id(Some(edge_type as EdgeTypeT))
                        as f32
                },
                number_of_directed_edges as f32,
            );

            let mult_dot_bias_squared = mult_dot_bias.powf(2.0);

            true_triple_feature_wise_distance_vector
                .into_iter()
                .zip(false_triple_feature_wise_distance_vector.into_iter())
                .zip(
                    src_embedding
                        .iter_mut()
                        .zip(dst_embedding.iter_mut())
                        .zip(
                            not_src_embedding
                                .iter_mut()
                                .zip(not_dst_embedding.iter_mut()),
                        )
                        .zip(
                            bias_edge_type_embedding
                                .iter_mut()
                                .zip(multiplicative_edge_type_embedding.iter_mut()),
                        ),
                )
                .map(
                    |(
                        (true_distance_feature, false_distance_feature),
                        (
                            ((src_feature, dst_feature), (not_src_feature, not_dst_feature)),
                            (bias_feature, mult_feature),
                        ),
                    )| {
                        let normalized_true_distance_feature =
                            true_distance_feature / true_triple_distance_norm;
                        let normalized_false_distance_feature =
                            false_distance_feature / false_triple_distance_norm;
                        let normalized_delta =
                            normalized_true_distance_feature - normalized_false_distance_feature;

                        *mult_feature -= (normalized_true_distance_feature
                            * (*mult_feature * (*dst_feature - *src_feature) + true_dot_delta)
                            - normalized_false_distance_feature
                                * (*mult_feature * (*not_dst_feature - *not_src_feature)
                                    + false_dot_delta)
                            + 2.0 * mult_dot_bias * *mult_feature)
                            / edge_type_prior
                            * learning_rate;

                        *bias_feature -= (normalized_delta + mult_dot_bias_squared * *bias_feature
                            - 2.0 * mult_dot_bias)
                            / edge_type_prior
                            * learning_rate;
                        *src_feature -=
                            normalized_true_distance_feature * learning_rate / src_prior;
                        *dst_feature +=
                            normalized_true_distance_feature * learning_rate / dst_prior;
                        *not_src_feature +=
                            normalized_false_distance_feature * learning_rate / not_src_prior;
                        *not_dst_feature -=
                            normalized_false_distance_feature * learning_rate / not_dst_prior;
                        normalized_delta
                    },
                )
                .sum::<f32>()
        };

        // We start to loop over the required amount of epochs.
        (0..epochs)
            .progress_with(epochs_progress_bar)
            .for_each(|_| {
                // We update the random state used to generate the random walks
                // and the negative samples.
                random_state = splitmix64(random_state);

                // We iterate over the graph edges.
                graph
                    .par_iter_siamese_mini_batch_with_edge_types(
                        random_state,
                        graph.get_number_of_directed_edges() as usize,
                    )
                    .for_each(|(_, src, dst, not_src, not_dst, edge_type_id)| {
                        compute_mini_batch_step(
                            src as usize,
                            not_src as usize,
                            dst as usize,
                            not_dst as usize,
                            edge_type_id.unwrap() as usize,
                            learning_rate,
                        );
                    });

                learning_rate *= learning_rate_decay;
            });
        Ok(())
    }
}
