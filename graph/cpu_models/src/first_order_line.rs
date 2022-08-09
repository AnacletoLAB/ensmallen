use crate::{get_node_priors, BasicEmbeddingModel, GraphEmbedder, MatrixShape};
use express_measures::cosine_similarity_sequential_unchecked;
use graph::{Graph, NodeT, ThreadDataRaceAware};
use num_traits::Zero;
use rayon::prelude::*;
use vec_rand::splitmix64;

#[derive(Clone, Debug)]
pub struct FirstOrderLINE {
    model: BasicEmbeddingModel,
}

impl From<BasicEmbeddingModel> for FirstOrderLINE {
    fn from(model: BasicEmbeddingModel) -> Self {
        Self { model }
    }
}

impl GraphEmbedder for FirstOrderLINE {
    fn get_model_name(&self) -> String {
        "First-order LINE".to_string()
    }

    fn get_number_of_epochs(&self) -> usize {
        self.model.epochs
    }

    fn is_verbose(&self) -> bool {
        self.model.verbose
    }

    fn get_random_state(&self) -> u64 {
        self.model.random_state
    }

    fn get_embedding_shapes(&self, graph: &Graph) -> Result<Vec<MatrixShape>, String> {
        Ok(vec![(
            graph.get_number_of_nodes() as usize,
            self.model.get_embedding_size(),
        )
            .into()])
    }

    fn _fit_transform(&self, graph: &Graph, embedding: &mut [&mut [f32]]) -> Result<(), String> {
        let shared_node_embedding = ThreadDataRaceAware::new(&mut embedding[0]);
        let mut random_state = self.get_random_state();
        let mut learning_rate = self.model.learning_rate;
        let pb = self.get_loading_bar();

        // We start to loop over the required amount of epochs.
        for _ in 0..self.model.epochs {
            // We update the random state used to generate the random walks
            // and the negative samples.
            random_state = splitmix64(random_state);
            // We iterate over the graph edges.
            let total_variation = graph
                .par_iter_edge_prediction_mini_batch(
                    random_state,
                    graph.get_number_of_directed_edges() as usize,
                    false,
                    Some(0.5),
                    Some(self.model.get_avoid_false_negatives()),
                    None,
                    Some(self.model.can_use_scale_free_distribution()),
                    None,
                    None,
                )?
                .map(|(src, dst, label)| {
                    let src = src as usize;
                    let dst = dst as usize;
                    let src_embedding = unsafe {
                        &mut (*shared_node_embedding.get())[(src * self.model.embedding_size)
                            ..((src + 1) * self.model.embedding_size)]
                    };
                    let dst_embedding = unsafe {
                        &mut (*shared_node_embedding.get())[(dst * self.model.embedding_size)
                            ..((dst + 1) * self.model.embedding_size)]
                    };

                    let (similarity, src_norm, dst_norm): (f32, f32, f32) = unsafe {
                        cosine_similarity_sequential_unchecked(src_embedding, dst_embedding)
                    };

                    let prediction = 1.0 / (1.0 + (-similarity).exp());
                    let variation = if label { prediction - 1.0 } else { prediction };
                    let node_priors =
                        get_node_priors(graph, &[src as NodeT, dst as NodeT], learning_rate);

                    let src_variation = variation * node_priors[0];
                    let dst_variation = variation * node_priors[1];

                    src_embedding
                        .iter_mut()
                        .zip(dst_embedding.iter_mut())
                        .for_each(|(src_feature, dst_feature)| {
                            *src_feature /= src_norm;
                            *dst_feature /= dst_norm;
                            *src_feature -= *dst_feature * src_variation;
                            *dst_feature -= *src_feature * dst_variation;
                        });

                    variation.abs()
                })
                .sum::<f32>();

            if total_variation.is_zero() {
                break;
            }

            pb.inc(1);
            pb.set_message(format!(", variation: {:.4}", total_variation));
            learning_rate *= self.model.learning_rate_decay;
        }
        Ok(())
    }
}
