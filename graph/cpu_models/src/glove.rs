use crate::*;
use express_measures::{dot_product_sequential_unchecked, ThreadFloat};
use graph::{Graph, NodeT, ThreadDataRaceAware};
use indicatif::ProgressIterator;
use num_traits::{AsPrimitive, Float};
use rayon::prelude::*;
use vec_rand::splitmix64;

impl<W> Node2Vec<W>
where
    W: WalkTransformer,
{
    pub(crate) fn fit_transform_glove<F: Float + ThreadFloat + 'static>(
        &self,
        graph: &Graph,
        embedding: &mut [&mut [F]],
    ) -> Result<(), String>
    where
        f32: AsPrimitive<F>,
        NodeT: AsPrimitive<F>,
    {
        let scale_factor = (self.get_embedding_size() as f32).sqrt().as_();
        let embedding_size = self.embedding_size;
        let mut walk_parameters = self.walk_parameters.clone();
        let mut random_state = splitmix64(self.walk_parameters.get_random_state() as u64);
        let mut learning_rate = self.learning_rate.as_();
        let cv = self.clipping_value.as_();

        // Update the random state
        random_state = splitmix64(random_state);

        // Wrapping the layers into shared structures.
        let shared_embedding = ThreadDataRaceAware::new(embedding);

        let pb = self.get_loading_bar();

        // We start to loop over the required amount of epochs.
        for _ in (0..self.get_number_of_epochs()).progress_with(pb) {
            // We update the random state used to generate the random walks
            // and the negative samples.
            random_state = splitmix64(random_state);
            walk_parameters = walk_parameters.set_random_state(Some(random_state as usize));

            // We start to compute the new gradients.
            graph
                .par_iter_cooccurence_matrix(&walk_parameters, self.window_size, None)?
                .for_each(|(src, dst, frequency)| unsafe {
                    let src_embedding = &mut (*shared_embedding.get())[0]
                        [(src as usize) * embedding_size..((src as usize) + 1) * embedding_size];
                    let dst_embedding = &mut (*shared_embedding.get())[1]
                        [(dst as usize) * embedding_size..((dst as usize) + 1) * embedding_size];

                    let dot: F = dot_product_sequential_unchecked(src_embedding, dst_embedding)
                        / scale_factor;

                    if dot > cv || dot < -cv {
                        return;
                    }

                    let prediction = F::one() / (F::one() + (-dot).exp());
                    let mut variation = prediction - frequency.as_();
                    let adaptative_learning_rate = F::one()
                        / (graph.get_unchecked_node_degree_from_node_id(src).as_()
                            * graph.get_unchecked_node_degree_from_node_id(dst).as_())
                        .sqrt();

                    variation *= adaptative_learning_rate;

                    let src_variation =
                        variation * get_node_prior(graph, src as NodeT, learning_rate);
                    let dst_variation =
                        variation * get_node_prior(graph, dst as NodeT, learning_rate);

                    src_embedding
                        .iter_mut()
                        .zip(dst_embedding.iter_mut())
                        .for_each(|(src_feature, dst_feature)| {
                            *src_feature -= *dst_feature * src_variation;
                            *dst_feature -= *src_feature * dst_variation;
                        });
                });

            learning_rate *= self.learning_rate_decay.as_()
        }
        Ok(())
    }
}
