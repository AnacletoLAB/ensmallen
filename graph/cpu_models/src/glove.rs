use crate::*;
use express_measures::{cosine_similarity_sequential_unchecked, ThreadFloat};
use graph::{Graph, ThreadDataRaceAware};
use num::Zero;
use num_traits::Coerced;
use rayon::prelude::*;
use vec_rand::splitmix64;

impl<W> Node2Vec<W>
where
    W: WalkTransformer,
{
    pub(crate) fn fit_transform_glove<F: Coerced<f32> + ThreadFloat>(
        &self,
        graph: &Graph,
        embedding: &mut [&mut [F]],
    ) -> Result<(), String> {
        let embedding_size = self.embedding_size;
        let mut walk_parameters = self.walk_parameters.clone();
        let mut random_state = splitmix64(self.walk_parameters.get_random_state() as u64);
        let mut learning_rate = self.learning_rate;

        // Update the random state
        random_state = splitmix64(random_state);

        // Wrapping the layers into shared structures.
        let shared_embedding = ThreadDataRaceAware::new(embedding);

        let pb = self.get_loading_bar();

        // We start to loop over the required amount of epochs.
        for _ in 0..self.get_number_of_epochs() {
            // We update the random state used to generate the random walks
            // and the negative samples.
            random_state = splitmix64(random_state);
            walk_parameters = walk_parameters.set_random_state(Some(random_state as usize));

            // We start to compute the new gradients.
            let total_variation = graph
                .par_iter_log_normalized_cooccurence_matrix(
                    &walk_parameters,
                    self.window_size,
                    None,
                )?
                .map(|(src, dst, freq)| unsafe {
                    let src_embedding = &mut (*shared_embedding.get())[0]
                        [(src as usize) * embedding_size..((src as usize) + 1) * embedding_size];
                    let dst_embedding = &mut (*shared_embedding.get())[1]
                        [(dst as usize) * embedding_size..(dst as usize + 1) * embedding_size];

                    let (similarity, src_norm, dst_norm): (f32, f32, f32) =
                        cosine_similarity_sequential_unchecked(src_embedding, dst_embedding);

                    let variation: f32 = freq.powf(self.alpha) * (similarity - freq.ln());

                    let node_priors = get_node_priors(graph, &[src, dst], learning_rate);

                    let src_variation = F::coerce_from(variation * node_priors[0]);
                    let dst_variation = F::coerce_from(variation * node_priors[1]);

                    src_embedding
                        .iter_mut()
                        .zip(dst_embedding.iter_mut())
                        .for_each(|(src_feature, dst_feature)| {
                            *src_feature /= F::coerce_from(src_norm);
                            *dst_feature /= F::coerce_from(dst_norm);
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
            pb.set_message(format!("variation {:.4}", total_variation));
            learning_rate *= self.learning_rate_decay;
        }
        Ok(())
    }
}
