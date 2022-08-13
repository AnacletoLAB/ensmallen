use crate::{get_node_prior, BasicEmbeddingModel, GraphEmbedder, MatrixShape};
use express_measures::{normalize_vector_inplace, ThreadFloat};
use graph::{Graph, ThreadDataRaceAware};
use indicatif::ProgressIterator;
use num_traits::Coerced;
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

    fn get_dtype(&self) -> String {
        self.model.get_dtype()
    }

    fn get_random_state(&self) -> u64 {
        self.model.random_state
    }

    fn get_embedding_shapes(&self, graph: &Graph) -> Result<Vec<MatrixShape>, String> {
        Ok(vec![(
            graph.get_number_of_nodes() as usize,
            self.model.embedding_size,
        )
            .into()])
    }

    fn _fit_transform<F: Coerced<f32> + ThreadFloat>(
        &self,
        graph: &Graph,
        embedding: &mut [&mut [F]],
    ) -> Result<(), String> {
        let embedding = ThreadDataRaceAware::new(&mut embedding[0]);
        let mut random_state = self.get_random_state();
        let mut learning_rate = self.model.get_learning_rate();
        let embedding_size = self.model.get_embedding_size();
        let pb = self.get_loading_bar();

        // We start to loop over the required amount of epochs.
        for _ in (0..self.model.epochs).progress_with(pb) {
            // We update the random state used to generate the random walks
            // and the negative samples.
            random_state = splitmix64(random_state);
            // We iterate over the graph edges.
            graph.par_iter_node_ids().for_each(|src| unsafe {
                // We get a copy of the source embedding so we can exploit better locality.
                let mut src_embedding = (*embedding.get())
                    [((src as usize) * embedding_size)..((src as usize + 1) * embedding_size)]
                    .iter()
                    .map(|value| value.coerce_into())
                    .collect::<Vec<f32>>();
                // We allocate a vector for the gradient of the source node.
                let mut source_variation = vec![0.0; embedding_size];
                normalize_vector_inplace(&mut src_embedding);

                let total_number_of_neighbours = graph
                    .iter_unchecked_neighbour_node_ids_from_source_node_id(src)
                    .map(|dst| {
                        (
                            dst,
                            graph.get_random_outbounds_scale_free_node(
                                random_state
                                    .wrapping_mul(src as u64 + 1)
                                    .wrapping_mul(1 + dst as u64),
                            ),
                        )
                    })
                    .map(|(dst, not_dst)| {
                        let dst_embedding = &mut (*embedding.get())[((dst as usize)
                            * embedding_size)
                            ..((dst as usize + 1) * embedding_size)];
                        let not_dst_embedding = &mut (*embedding.get())[((not_dst as usize)
                            * embedding_size)
                            ..((not_dst as usize + 1) * embedding_size)];
                        let mut total_dot_products = 0.0;
                        let mut total_not_dot_products = 0.0;
                        let mut total_squared_dst = 0.0;
                        let mut total_squared_not_dst = 0.0;
                        src_embedding
                            .iter()
                            .zip(dst_embedding.iter().zip(not_dst_embedding.iter()))
                            .map(|(src_feature, (&dst_feature, &not_dst_feature))| {
                                (
                                    src_feature,
                                    dst_feature.coerce_into(),
                                    not_dst_feature.coerce_into(),
                                )
                            })
                            .for_each(|(&src_feature, dst_feature, not_dst_feature)| {
                                total_dot_products += src_feature * dst_feature;
                                total_not_dot_products += src_feature * not_dst_feature;
                                total_squared_dst += dst_feature * dst_feature;
                                total_squared_not_dst += not_dst_feature * not_dst_feature;
                            });

                        let dst_norm = total_squared_dst.sqrt() + f32::EPSILON;
                        let not_dst_norm = total_squared_not_dst.sqrt() + f32::EPSILON;

                        let true_similarity = total_dot_products / dst_norm;
                        let false_similarity = total_not_dot_products / not_dst_norm;

                        let dst_norm = F::coerce_from(dst_norm);
                        let not_dst_norm = F::coerce_from(not_dst_norm);

                        let true_variation = 1.0 / (1.0 + (-true_similarity).exp()) - 1.0;
                        let false_variation = 1.0 / (1.0 + (-false_similarity).exp());

                        let dst_prior = get_node_prior(graph, dst, learning_rate);
                        let not_dst_prior = get_node_prior(graph, not_dst, learning_rate);

                        let dst_variation = true_variation * dst_prior;
                        let not_dst_variation = true_variation * not_dst_prior;

                        source_variation
                            .iter_mut()
                            .zip(src_embedding.iter())
                            .zip(dst_embedding.iter_mut().zip(not_dst_embedding.iter_mut()))
                            .for_each(
                                |(
                                    (src_variation, &src_feature),
                                    (dst_feature, not_dst_feature),
                                )| {
                                    *dst_feature /= dst_norm;
                                    *not_dst_feature /= not_dst_norm;
                                    *src_variation += dst_feature.coerce_into() * true_variation
                                        + not_dst_feature.coerce_into() * false_variation;
                                    *dst_feature -= F::coerce_from(src_feature * dst_variation);
                                    *not_dst_feature -=
                                        F::coerce_from(src_feature * not_dst_variation);
                                },
                            );
                    })
                    .count();

                if total_number_of_neighbours == 0 {
                    return;
                }

                let total_number_of_neighbours = total_number_of_neighbours as f32;

                (&mut (*embedding.get())
                    [((src as usize) * embedding_size)..((src as usize + 1) * embedding_size)])
                    .iter_mut()
                    .zip(src_embedding.into_iter())
                    .zip(source_variation.into_iter())
                    .for_each(|((src_feature, normalized_src_feature), variation)| {
                        *src_feature = F::coerce_from(
                            normalized_src_feature - variation / total_number_of_neighbours,
                        );
                    });
            });

            learning_rate *= self.model.get_learning_rate_decay();
        }
        Ok(())
    }
}
