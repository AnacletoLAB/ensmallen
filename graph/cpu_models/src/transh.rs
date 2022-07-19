use crate::*;
use express_measures::{dot_product_sequential_unchecked, normalize_vector_inplace};
use graph::{EdgeTypeT, Graph, NodeT, ThreadDataRaceAware};
use num_traits::Zero;
use rayon::prelude::*;
use vec_rand::splitmix64;

#[derive(Clone, Debug)]
pub struct TransH {
    model: BasicSiameseModel,
}

impl From<BasicSiameseModel> for TransH {
    fn from(model: BasicSiameseModel) -> Self {
        Self { model }
    }
}

impl GraphEmbedder for TransH {
    fn get_model_name(&self) -> String {
        "TransH".into()
    }

    fn get_random_state(&self) -> u64 {
        self.model.get_random_state()
    }

    fn is_verbose(&self) -> bool {
        self.model.is_verbose()
    }

    fn get_number_of_epochs(&self) -> usize {
        self.model.get_number_of_epochs()
    }

    fn get_embedding_shapes(&self, graph: &Graph) -> Result<Vec<MatrixShape>, String> {
        Ok(vec![
            (
                graph.get_number_of_nodes() as usize,
                self.model.get_embedding_size(),
            )
                .into(),
            (
                graph.get_number_of_edge_types()? as usize,
                self.model.get_embedding_size(),
            )
                .into(),
            (
                graph.get_number_of_edge_types()? as usize,
                self.model.get_embedding_size(),
            )
                .into(),
        ])
    }

    fn _fit_transform(&self, graph: &Graph, embedding: &mut [&mut [f32]]) -> Result<(), String> {
        let embedding_size = self.model.get_embedding_size();
        let scale_factor = (embedding_size as f32).sqrt();
        let mut learning_rate = self.model.get_learning_rate() / scale_factor;
        let mut random_state = self.get_random_state();

        let shared_embedding = ThreadDataRaceAware::new(embedding);

        let pb = self.get_loading_bar();

        let compute_mini_batch_step = |src: usize,
                                       not_src: usize,
                                       dst: usize,
                                       not_dst: usize,
                                       edge_type: usize,
                                       learning_rate: f32| unsafe {
            let src_embedding = {
                &mut (*shared_embedding.get())[0]
                    [(src * embedding_size)..((src + 1) * embedding_size)]
            };
            let not_src_embedding = {
                &mut (*shared_embedding.get())[0]
                    [(not_src * embedding_size)..((not_src + 1) * embedding_size)]
            };
            let dst_embedding = {
                &mut (*shared_embedding.get())[0]
                    [(dst * embedding_size)..((dst + 1) * embedding_size)]
            };
            let not_dst_embedding = {
                &mut (*shared_embedding.get())[0]
                    [(not_dst * embedding_size)..((not_dst + 1) * embedding_size)]
            };
            let multiplicative_edge_type_embedding = {
                &mut (*shared_embedding.get())[1]
                    [(edge_type * embedding_size)..((edge_type + 1) * embedding_size)]
            };
            let bias_edge_type_embedding = {
                &mut (*shared_embedding.get())[2]
                    [(edge_type * embedding_size)..((edge_type + 1) * embedding_size)]
            };

            let (dst_norm, not_dst_norm, src_norm, not_src_norm, multiplicative_norm, bias_norm) = (
                normalize_vector_inplace(dst_embedding),
                normalize_vector_inplace(not_dst_embedding),
                normalize_vector_inplace(src_embedding),
                normalize_vector_inplace(not_src_embedding),
                normalize_vector_inplace(multiplicative_edge_type_embedding),
                normalize_vector_inplace(bias_edge_type_embedding),
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
            if false_triple_distance_norm - true_triple_distance_norm > self.model.relu_bias {
                return 0.0;
            }

            let node_priors = get_node_priors(
                graph,
                &[
                    src as NodeT,
                    dst as NodeT,
                    not_src as NodeT,
                    not_dst as NodeT,
                ],
                learning_rate,
            );
            let edge_type_prior = get_edge_type_prior(graph, edge_type as EdgeTypeT, learning_rate);

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
                            normalized_true_distance_feature * learning_rate / node_priors[0];
                        *dst_feature +=
                            normalized_true_distance_feature * learning_rate / node_priors[1];
                        *not_src_feature +=
                            normalized_false_distance_feature * learning_rate / node_priors[2];
                        *not_dst_feature -=
                            normalized_false_distance_feature * learning_rate / node_priors[3];
                        normalized_delta
                    },
                )
                .sum::<f32>()
        };

        // We start to loop over the required amount of epochs.
        for _ in 0..self.get_number_of_epochs() {
            // We update the random state used to generate the random walks
            // and the negative samples.
            random_state = splitmix64(random_state);

            // We iterate over the graph edges.
            let total_variation = graph
                .par_iter_siamese_mini_batch_with_edge_types(
                    random_state,
                    graph.get_number_of_directed_edges() as usize,
                )
                .map(|(_, src, dst, not_src, not_dst, edge_type_id)| {
                    compute_mini_batch_step(
                        src as usize,
                        not_src as usize,
                        dst as usize,
                        not_dst as usize,
                        edge_type_id.unwrap() as usize,
                        learning_rate,
                    )
                })
                .sum::<f32>();

            if total_variation.is_zero() {
                break;
            }

            learning_rate *= self.model.get_learning_rate_decay();
            pb.inc(1);
            pb.set_message(format!(", variation: {:.4}", total_variation));
        }
        Ok(())
    }
}
