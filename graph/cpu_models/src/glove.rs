use crate::*;
use express_measures::dot_product_sequential_unchecked;
use graph::{Graph, ThreadDataRaceAware};
use num::Zero;
use rayon::prelude::*;
use vec_rand::splitmix64;

impl<W> Node2Vec<W>
where
    W: WalkTransformer,
{
    pub(crate) fn fit_transform_glove(
        &self,
        graph: &Graph,
        embedding: &mut [&mut [f32]],
    ) -> Result<(), String> {
        let embedding_size = self.embedding_size;
        let mut walk_parameters = self.walk_parameters.clone();
        let mut random_state = splitmix64(self.walk_parameters.get_random_state() as u64);
        let mut learning_rate = self.learning_rate;
        // This is used to scale the dot product to avoid getting NaN due to
        // exp(dot) being inf and the sigmoid becomes Nan
        // we multiply by context size so we have a faster division when computing
        // the dotproduct of the mean contexted mebedding
        let scale_factor = (embedding_size as f32).sqrt();

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

                    let dot = dot_product_sequential_unchecked(src_embedding, dst_embedding)
                        / scale_factor;

                    if dot > self.clipping_value || dot < -self.clipping_value {
                        return 0.0;
                    }

                    let variation: f32 =
                        learning_rate * 2.0 * freq.powf(self.alpha) * (dot - freq.ln());

                    src_embedding
                        .iter_mut()
                        .zip(dst_embedding.iter_mut())
                        .for_each(|(src_feature, dst_feature)| {
                            *src_feature -= *dst_feature * variation;
                            *dst_feature -= *src_feature * variation;
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
