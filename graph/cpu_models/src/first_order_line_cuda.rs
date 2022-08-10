use crate::{BasicEmbeddingModel, GraphEmbedder, MatrixShape};
use core::intrinsics::unlikely;
use graph::Graph;
use num_traits::Zero;
use rayon::prelude::*;
use vec_rand::splitmix64;
use vec_rand::xorshift::xorshift;

#[derive(Clone, Debug)]
pub struct FirstOrderLINECUDA {
    model: BasicEmbeddingModel,
}

impl From<BasicEmbeddingModel> for FirstOrderLINECUDA {
    fn from(model: BasicEmbeddingModel) -> Self {
        Self { model }
    }
}

impl Default for FirstOrderLINECUDA {
    fn default() -> Self {
        BasicEmbeddingModel::new(
            Some(100),
            Some(1000),
            Some(0.05),
            Some(0.9),
            Some(false),
            Some(true),
            Some(42),
            Some(true),
        )
        .unwrap()
        .into()
    }
}

impl FirstOrderLINECUDA {
    pub fn get_embedding_size(&self) -> usize {
        self.model.get_embedding_size()
    }
}

impl GraphEmbedder for FirstOrderLINECUDA {
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
        let mut random_state = self.get_random_state();
        let mut learning_rate = self.model.learning_rate;
        let pb = self.get_loading_bar();
        let number_of_nodes = graph.get_number_of_nodes() as usize;
        let number_of_directed_edges = graph.get_number_of_directed_edges() as usize;
        let embedding_size = self.model.get_embedding_size();
        let comulative_node_degrees = graph.get_cumulative_node_degrees();
        let destinations = graph.get_directed_destination_node_ids();
        let embedding = &mut embedding[0];

        let get_node_degree = |node_id: usize| {
            let comulative_degree = comulative_node_degrees[node_id];
            // let previous_comulative_degree =
            //     ((node_id == 0) as u64).wrapping_sub(1) & node_degrees[node_id - 1];
            let previous_comulative_degree = if node_id == 0 {
                0
            } else {
                comulative_node_degrees[node_id - 1]
            };
            let degree = comulative_degree - previous_comulative_degree;
            (previous_comulative_degree, degree)
        };

        // We start to loop over the required amount of epochs.
        for _ in 0..self.model.epochs {
            // We update the random state used to generate the random walks
            // and the negative samples.
            random_state = splitmix64(random_state);
            // We iterate over the graph edges.
            let total_variation = (0..graph.get_number_of_directed_edges())
                .map(|edge_number| {
                    let mut random_state =
                        xorshift((edge_number as u64 + random_state).wrapping_mul(random_state));
                    let src = random_state as usize % number_of_nodes;

                    let (previous_comulative_degree, src_degree) = get_node_degree(src);
                    if unlikely(src_degree == 0) {
                        return 0.0;
                    }
                    random_state = splitmix64(random_state);
                    let true_dst = destinations[(previous_comulative_degree
                        + (xorshift(random_state) % src_degree))
                        as usize] as usize;

                    random_state = splitmix64(random_state);
                    let false_dst = destinations
                        [xorshift(random_state) as usize % number_of_directed_edges]
                        as usize;

                    if unlikely(true_dst == false_dst) {
                        return 0.0;
                    };

                    let (true_dot, false_dot, src_squared, true_dst_squared, false_dst_squared) =
                        embedding[(embedding_size * src)..(embedding_size * (src + 1))]
                            .iter()
                            .zip(
                                embedding[(embedding_size * true_dst)
                                    ..(embedding_size * (true_dst + 1))]
                                    .iter()
                                    .zip(
                                        embedding[(embedding_size * false_dst)
                                            ..(embedding_size * (false_dst + 1))]
                                            .iter(),
                                    ),
                            )
                            .map(|(&src_value, (&true_dst_value, &false_dst_value))| {
                                (
                                    src_value * true_dst_value,
                                    src_value * false_dst_value,
                                    src_value * src_value,
                                    true_dst_value * true_dst_value,
                                    false_dst_value * false_dst_value,
                                )
                            })
                            .reduce(|a, b| (a.0 + b.0, a.1 + b.1, a.2 + b.2, a.3 + b.3, a.4 + b.4))
                            .unwrap();

                    let src_norm = src_squared.sqrt() + f32::EPSILON;
                    let true_dst_norm = true_dst_squared.sqrt() + f32::EPSILON;
                    let false_dst_norm = false_dst_squared.sqrt() + f32::EPSILON;
                    let true_cosine_similarity =
                        true_dot / (src_norm * true_dst_norm + f32::EPSILON);
                    let false_cosine_similarity =
                        false_dot / (src_norm * false_dst_norm + f32::EPSILON);

                    let true_variation = 1.0 / (1.0 + (-true_cosine_similarity).exp2()) - 1.0;
                    let false_variation = 1.0 / (1.0 + (-false_cosine_similarity).exp2());

                    let (_, true_dst_degree) = get_node_degree(true_dst);
                    let (_, false_dst_degree) = get_node_degree(false_dst);

                    let src_prior =
                        learning_rate * (number_of_nodes as f32 / (src_degree as f32 + 1.0));
                    let src_true_variation = true_variation * src_prior;
                    let src_false_variation = false_variation * src_prior;
                    let true_dst_variation = true_variation
                        * (number_of_nodes as f32 / (true_dst_degree as f32 + 1.0))
                        * learning_rate;
                    let false_dst_variation = false_variation
                        * (number_of_nodes as f32 / (false_dst_degree as f32 + 1.0))
                        * learning_rate;

                    (0..embedding_size)
                        .zip((0..embedding_size).zip(0..embedding_size))
                        .for_each(|(i, (j, k))| {
                            embedding[src * embedding_size + i] /= src_norm;
                            embedding[true_dst * embedding_size + j] /= true_dst_norm;
                            embedding[false_dst * embedding_size + k] /= false_dst_norm;
                            let src_value = embedding[src * embedding_size + i];
                            let true_dst_value = embedding[true_dst * embedding_size + j];
                            let false_dst_value = embedding[false_dst * embedding_size + k];
                            embedding[src * embedding_size + i] -= src_true_variation
                                * true_dst_value
                                + src_false_variation * false_dst_value;
                            embedding[true_dst * embedding_size + j] -=
                                true_dst_variation * src_value;
                            embedding[false_dst * embedding_size + k] -=
                                false_dst_variation * src_value;
                        });
                    true_variation.abs() + false_variation.abs()
                })
                .sum::<f32>();

            if total_variation.is_zero() {
                break;
            }

            pb.inc(1);
            pb.set_message(format!(", variation: {:.4}", total_variation));
            learning_rate *= self.model.get_learning_rate_decay();
        }
        Ok(())
    }
}
