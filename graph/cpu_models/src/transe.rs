use crate::*;
use express_measures::{vector_norm, ThreadFloat};
use graph::{EdgeT, EdgeTypeT, Graph, NodeT, ThreadDataRaceAware};
use indicatif::ProgressIterator;
use num_traits::AsPrimitive;
use rayon::prelude::*;
use vec_rand::splitmix64;

#[derive(Clone, Debug)]
pub struct TransE {
    model: BasicSiameseModel,
}

impl From<BasicSiameseModel> for TransE {
    fn from(model: BasicSiameseModel) -> Self {
        Self { model }
    }
}

impl GraphEmbedder for TransE {
    fn get_model_name(&self) -> String {
        "TransE".to_string()
    }

    fn get_number_of_steps(&self) -> usize {
        self.model.get_number_of_epochs()
    }

    fn requires_random_initialization(&self) -> bool {
        true
    }

    fn is_verbose(&self) -> bool {
        self.model.is_verbose()
    }

    fn get_dtype(&self) -> String {
        self.model.get_dtype()
    }

    fn get_embedding_shapes(&self, graph: &Graph) -> Result<Vec<MatrixShape>, String> {
        Ok(vec![
            (
                graph.get_number_of_nodes() as usize,
                self.model.model.embedding_size,
            )
                .into(),
            (
                graph.get_number_of_edge_types()? as usize,
                self.model.model.embedding_size,
            )
                .into(),
        ])
    }

    fn get_random_state(&self) -> u64 {
        self.model.model.random_state
    }

    fn _fit_transform<F: ThreadFloat + 'static>(
        &self,
        graph: &Graph,
        embedding: &mut [&mut [F]],
    ) -> Result<(), String>
    where
        f32: AsPrimitive<F>,
        NodeT: AsPrimitive<F>,
        EdgeT: AsPrimitive<F>,
    {
        let embedding_size = self.model.get_embedding_size();
        let scale_factor = (embedding_size as f32).sqrt();
        let mut learning_rate = (self.model.get_learning_rate() / scale_factor).as_();
        let mut random_state = self.get_random_state();

        let shared_embedding = ThreadDataRaceAware::new(embedding);

        let pb = self.get_loading_bar();

        // We start to loop over the required amount of epochs.
        (0..self.get_number_of_steps())
            .progress_with(pb)
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
                    .for_each(|(_, src, dst, not_src, not_dst, edge_type)| {
                        let src = src as usize;
                        let dst = dst as usize;
                        let not_src = not_src as usize;
                        let not_dst = not_dst as usize;
                        let edge_type = edge_type.unwrap() as usize;
                        let src_embedding = unsafe {
                            &mut (*shared_embedding.get())[0]
                                [(src * embedding_size)..((src + 1) * embedding_size)]
                        };
                        let not_src_embedding = unsafe {
                            &mut (*shared_embedding.get())[0]
                                [(not_src * embedding_size)..((not_src + 1) * embedding_size)]
                        };
                        let dst_embedding = unsafe {
                            &mut (*shared_embedding.get())[0]
                                [(dst * embedding_size)..((dst + 1) * embedding_size)]
                        };
                        let not_dst_embedding = unsafe {
                            &mut (*shared_embedding.get())[0]
                                [(not_dst * embedding_size)..((not_dst + 1) * embedding_size)]
                        };
                        let edge_type_embedding = unsafe {
                            &mut (*shared_embedding.get())[1]
                                [(edge_type * embedding_size)..((edge_type + 1) * embedding_size)]
                        };

                        let (dst_norm, not_dst_norm, src_norm, not_src_norm) = (
                            vector_norm(dst_embedding),
                            vector_norm(not_dst_embedding),
                            vector_norm(src_embedding),
                            vector_norm(not_src_embedding),
                        );
                        let node_priors: Vec<F> = get_node_priors(
                            graph,
                            &[
                                src as NodeT,
                                dst as NodeT,
                                not_src as NodeT,
                                not_dst as NodeT,
                            ],
                            learning_rate,
                        );

                        let edge_type_prior =
                            get_edge_type_prior(graph, edge_type as EdgeTypeT, learning_rate);

                        src_embedding
                            .iter_mut()
                            .zip(not_src_embedding.iter_mut())
                            .zip(dst_embedding.iter_mut().zip(not_dst_embedding.iter_mut()))
                            .zip(edge_type_embedding.iter_mut())
                            .for_each(
                                |(
                                    (
                                        (src_feature, not_src_feature),
                                        (dst_feature, not_dst_feature),
                                    ),
                                    edge_type_feature,
                                )| {
                                    *src_feature /= src_norm;
                                    *not_src_feature /= not_src_norm;
                                    *dst_feature /= dst_norm;
                                    *not_dst_feature /= not_dst_norm;

                                    let positive_distance =
                                        *src_feature + *edge_type_feature - *dst_feature;
                                    let negative_distance =
                                        *not_src_feature + *edge_type_feature - *not_dst_feature;
                                    let loss = positive_distance.powf(F::one() + F::one())
                                        - negative_distance.powf(F::one() + F::one());

                                    if loss > -self.model.relu_bias.as_() {
                                        *src_feature -= positive_distance * node_priors[0];
                                        *dst_feature += positive_distance * node_priors[1];
                                        *not_src_feature += negative_distance * node_priors[2];
                                        *not_dst_feature -= negative_distance * node_priors[3];
                                        *edge_type_feature -= (positive_distance
                                            - negative_distance)
                                            * edge_type_prior;
                                    }
                                },
                            );
                    });
                learning_rate *= self.model.get_learning_rate_decay().as_();
            });
        Ok(())
    }
}
