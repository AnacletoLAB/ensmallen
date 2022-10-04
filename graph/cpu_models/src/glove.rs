use crate::*;
use express_measures::{dot_product_sequential_unchecked, ThreadFloat};
use graph::{Graph, NodeT, ThreadDataRaceAware};
use indicatif::ProgressIterator;
use num_traits::Coerced;
use rayon::prelude::*;
use vec_rand::splitmix64;

impl<W> Node2Vec<W>
where
    W: WalkTransformer,
{
    pub(crate) fn fit_transform_glove<F: ThreadFloat>(
        &self,
        graph: &Graph,
        embedding: &mut [&mut [F]],
    ) -> Result<(), String>
    where
        NodeT: Coerced<F>,
    {
        let embedding_size = self.embedding_size;
        let mut walk_parameters = self.walk_parameters.clone();
        let mut random_state = splitmix64(self.walk_parameters.get_random_state() as u64);
        let mut learning_rate = F::coerce_from(self.learning_rate);
        let alpha = F::coerce_from(self.alpha);

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
                .par_iter_log_normalized_cooccurence_matrix(
                    &walk_parameters,
                    self.window_size,
                    None,
                )?
                .map(|(src, dst, freq)| (src as usize, dst as usize, F::coerce_from(freq)))
                .for_each(|(src, dst, freq)| unsafe {
                    let src_embedding = &mut (*shared_embedding.get())[0]
                        [src * embedding_size..(src + 1) * embedding_size];
                    let dst_embedding = &mut (*shared_embedding.get())[1]
                        [dst * embedding_size..(dst + 1) * embedding_size];

                    let similarity = dot_product_sequential_unchecked(src_embedding, dst_embedding);

                    let variation: F = freq.powf(alpha) * (similarity - freq.ln());

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

            learning_rate *= F::coerce_from(self.learning_rate_decay);
        }
        Ok(())
    }
}
