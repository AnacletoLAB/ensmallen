use crate::*;
use express_measures::{
    element_wise_subtraction, matrix_vector_dot_product_sequential_unchecked,
    normalize_vector_inplace, vector_norm, ThreadFloat,
};
use graph::{EdgeT, EdgeTypeT, Graph, NodeT, ThreadDataRaceAware};
use indicatif::ProgressIterator;
use num_traits::AsPrimitive;
use rayon::prelude::*;
use vec_rand::splitmix64;

#[derive(Clone, Debug)]
pub struct StructuredEmbedding {
    model: BasicSiameseModel,
}

impl From<BasicSiameseModel> for StructuredEmbedding {
    fn from(model: BasicSiameseModel) -> Self {
        Self { model }
    }
}

impl GraphEmbedder for StructuredEmbedding {
    fn get_model_name(&self) -> String {
        "Structured Embedding".into()
    }

    fn get_random_state(&self) -> u64 {
        self.model.get_random_state()
    }

    fn is_verbose(&self) -> bool {
        self.model.is_verbose()
    }

    fn get_dtype(&self) -> String {
        self.model.get_dtype()
    }

    fn get_number_of_steps(&self) -> usize {
        self.model.get_number_of_epochs()
    }

    fn requires_random_initialization(&self) -> bool {
        true
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
                self.model.get_embedding_size() * self.model.get_embedding_size(),
            )
                .into(),
            (
                graph.get_number_of_edge_types()? as usize,
                self.model.get_embedding_size() * self.model.get_embedding_size(),
            )
                .into(),
        ])
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
        let edge_matrix_size = embedding_size * embedding_size;
        let scale_factor = (embedding_size as f32).sqrt();
        let mut learning_rate = (self.model.get_learning_rate() / scale_factor).as_();
        let mut random_state = self.get_random_state();

        let shared_embedding = ThreadDataRaceAware::new(embedding);

        let pb = self.get_loading_bar();

        // We start to loop over the required amount of epochs.
        for _ in (0..self.get_number_of_steps()).progress_with(pb) {
            // We update the random state used to generate the random walks
            // and the negative samples.
            random_state = splitmix64(random_state);

            // We iterate over the graph edges.
            graph
                .par_iter_siamese_mini_batch_with_edge_types(
                    random_state,
                    graph.get_number_of_directed_edges() as usize,
                )
                .map(|(_, src, dst, not_src, not_dst, edge_type_id)| {
                    (
                        src as usize,
                        dst as usize,
                        not_src as usize,
                        not_dst as usize,
                        edge_type_id.unwrap() as usize,
                    )
                })
                .for_each(|(src, dst, not_src, not_dst, edge_type_id)| unsafe {
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
                    let src_edge_type_matrix = {
                        &mut (*shared_embedding.get())[1][(edge_type_id * edge_matrix_size)
                            ..((edge_type_id + 1) * edge_matrix_size)]
                    };
                    let dst_edge_type_matrix = {
                        &mut (*shared_embedding.get())[2][(edge_type_id * edge_matrix_size)
                            ..((edge_type_id + 1) * edge_matrix_size)]
                    };

                    normalize_vector_inplace(dst_embedding);
                    normalize_vector_inplace(not_dst_embedding);
                    normalize_vector_inplace(src_embedding);
                    normalize_vector_inplace(not_src_embedding);

                    let matrix_vector_dot_src: Vec<F> =
                        matrix_vector_dot_product_sequential_unchecked(
                            src_edge_type_matrix,
                            src_embedding,
                        );

                    let matrix_vector_dot_not_src: Vec<F> =
                        matrix_vector_dot_product_sequential_unchecked(
                            src_edge_type_matrix,
                            not_src_embedding,
                        );

                    let matrix_vector_dot_dst: Vec<F> =
                        matrix_vector_dot_product_sequential_unchecked(
                            dst_edge_type_matrix,
                            dst_embedding,
                        );

                    let matrix_vector_dot_not_dst: Vec<F> =
                        matrix_vector_dot_product_sequential_unchecked(
                            dst_edge_type_matrix,
                            not_dst_embedding,
                        );

                    let src_sub_dst: Vec<F> =
                        element_wise_subtraction(&matrix_vector_dot_src, &matrix_vector_dot_dst);
                    let src_sub_dst_norm: F = vector_norm(&src_sub_dst);
                    let not_src_sub_dst: Vec<F> = element_wise_subtraction(
                        &matrix_vector_dot_not_src,
                        &matrix_vector_dot_not_dst,
                    );
                    let not_src_sub_dst_norm: F = vector_norm(&not_src_sub_dst);

                    // If the delta is lower than zero, there is no need to continue
                    // further, as the gradient will be zero.
                    if not_src_sub_dst_norm - src_sub_dst_norm > self.model.relu_bias.as_() {
                        return;
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

                    let edge_type_prior =
                        get_edge_type_prior(graph, edge_type_id as EdgeTypeT, learning_rate);

                    let src_sub_dst_squared_norm = src_sub_dst_norm.powf(F::one() + F::one());
                    let not_src_sub_dst_squared_norm =
                        not_src_sub_dst_norm.powf(F::one() + F::one());

                    src_sub_dst
                        .into_iter()
                        .zip(not_src_sub_dst.into_iter())
                        .zip(
                            src_edge_type_matrix
                                .chunks_mut(embedding_size)
                                .zip(dst_edge_type_matrix.chunks_mut(embedding_size)),
                        )
                        .enumerate()
                        .for_each(
                            |(
                                row_number,
                                (
                                    (true_distance, false_distance),
                                    (src_edge_type_row, dst_edge_type_row),
                                ),
                            )| {
                                let normalized_true_distance =
                                    true_distance / src_sub_dst_squared_norm;
                                let normalized_false_distance =
                                    false_distance / not_src_sub_dst_squared_norm;

                                let (
                                    src_total_gradient,
                                    dst_total_gradient,
                                    not_src_total_gradient,
                                    not_dst_total_gradient,
                                ) = src_edge_type_row
                                    .iter_mut()
                                    .zip(dst_edge_type_row.iter_mut())
                                    .zip(src_embedding.iter().zip(dst_embedding.iter()).zip(
                                        not_src_embedding.iter().zip(not_dst_embedding.iter()),
                                    ))
                                    .map(
                                        |(
                                            (src_edge_type_feature, dst_edge_type_feature),
                                            (
                                                (src_feature, dst_feature),
                                                (not_src_feature, not_dst_feature),
                                            ),
                                        )| {
                                            let to_return = (
                                                *src_edge_type_feature * normalized_true_distance,
                                                *dst_edge_type_feature * normalized_true_distance,
                                                *src_edge_type_feature * normalized_false_distance,
                                                *dst_edge_type_feature * normalized_false_distance,
                                            );
                                            *src_edge_type_feature -= (normalized_true_distance
                                                * *src_feature
                                                - normalized_false_distance * *not_src_feature)
                                                * edge_type_prior;
                                            *dst_edge_type_feature += (normalized_true_distance
                                                * *dst_feature
                                                - normalized_false_distance * *not_dst_feature)
                                                * edge_type_prior;
                                            to_return
                                        },
                                    )
                                    .reduce(
                                        |(
                                            src_total_gradient,
                                            dst_total_gradient,
                                            not_src_total_gradient,
                                            not_dst_total_gradient,
                                        ),
                                         (
                                            src_partial_gradient,
                                            dst_partial_gradient,
                                            not_src_partial_gradient,
                                            not_dst_partial_gradient,
                                        )| {
                                            (
                                                src_total_gradient + src_partial_gradient,
                                                dst_total_gradient + dst_partial_gradient,
                                                not_src_total_gradient + not_src_partial_gradient,
                                                not_dst_total_gradient + not_dst_partial_gradient,
                                            )
                                        },
                                    )
                                    .unwrap();

                                src_embedding[row_number] -= src_total_gradient * node_priors[0];
                                dst_embedding[row_number] += dst_total_gradient * node_priors[1];
                                not_src_embedding[row_number] +=
                                    not_src_total_gradient * node_priors[2];
                                not_dst_embedding[row_number] -=
                                    not_dst_total_gradient * node_priors[3];
                            },
                        );
                });

            learning_rate *= (self.model.get_learning_rate_decay()).as_();
        }
        Ok(())
    }
}
