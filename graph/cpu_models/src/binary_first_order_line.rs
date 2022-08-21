use crate::*;
use graph::{Graph, ThreadDataRaceAware};
use indicatif::ProgressBar;
use indicatif::ProgressIterator;
use indicatif::ProgressStyle;
use rayon::prelude::*;
use vec_rand::splitmix64;

#[derive(Clone, Debug)]
pub struct BinaryFirstOrderLINE {
    /// Dimensionality of the embedding.
    /// This value should always be a factor of `64`
    /// as we use a word size of 64 bits.
    number_of_words: usize,
    /// Number of training epochs.
    number_of_epochs: usize,
    /// Random state to reproduce the model training.
    random_state: u64,
    /// Whether to sample the edges using scale free distribution.
    use_scale_free_distribution: bool,
    /// Whether to avoid false negatives during the sampling. Do note
    /// that avoiding false negatives will be slower.
    avoid_false_negatives: bool,
    /// Whether to show loading bars.
    verbose: bool,
}

impl Default for BinaryFirstOrderLINE {
    fn default() -> Self {
        Self::new(None, None, None, None, None, None).unwrap()
    }
}

impl BinaryFirstOrderLINE {
    /// Return new instance of Binary First-order LINE.
    ///
    /// # Arguments
    /// * `number_of_words`: Option<usize> - Size of the words in the embedding.
    /// * `number_of_epochs`: Option<usize> - The number of epochs to run the model for, by default 10.
    /// * `avoid_false_negatives`: Option<bool> - Whether to avoid sampling false negatives. This may cause a slower training.
    /// * `use_scale_free_distribution`: Option<bool> - Whether to sample using scale free distribution. By default, true.
    /// * `random_state`: Option<u64> - The random state to use to reproduce the training.
    /// * `verbose`: Option<bool> - Whether to show loading bar.
    pub fn new(
        number_of_words: Option<usize>,
        number_of_epochs: Option<usize>,
        avoid_false_negatives: Option<bool>,
        use_scale_free_distribution: Option<bool>,
        random_state: Option<u64>,
        verbose: Option<bool>,
    ) -> Result<Self, String> {
        Ok(Self {
            number_of_words: must_not_be_zero(number_of_words, 50, "number of words")?,
            number_of_epochs: must_not_be_zero(number_of_epochs, 3000, "epochs")?,
            avoid_false_negatives: avoid_false_negatives.unwrap_or(false),
            use_scale_free_distribution: use_scale_free_distribution.unwrap_or(true),
            random_state: random_state.unwrap_or(42),
            verbose: verbose.unwrap_or(true),
        })
    }

    pub fn is_verbose(&self) -> bool {
        self.verbose
    }

    pub fn get_number_of_words(&self) -> usize {
        self.number_of_words
    }

    pub fn get_random_state(&self) -> u64 {
        self.random_state
    }

    pub fn fit_transform(&self, graph: &Graph, embedding: &mut [u64]) -> Result<(), String> {
        let mut random_state = self.get_random_state();
        let shared_node_embedding = ThreadDataRaceAware::new(embedding);
        let progress_bar = if self.is_verbose() {
            let pb = ProgressBar::new(self.number_of_epochs as u64);
            pb.set_style(ProgressStyle::default_bar().template(concat!(
                "Binary First-order LINE {spinner:.green} [{elapsed_precise}] ",
                "[{bar:40.cyan/blue}] ({pos}/{len}, ETA {eta})"
            )));
            pb
        } else {
            ProgressBar::hidden()
        };

        // We start to loop over the required amount of epochs.
        (0..self.number_of_epochs)
            .progress_with(progress_bar)
            .for_each(|_| {
                // We update the random state used to generate the random walks
                // and the negative samples.
                random_state = splitmix64(random_state);
                // We iterate over the graph edges.
                graph
                    .par_iter_edge_prediction_mini_batch(
                        random_state,
                        graph.get_number_of_directed_edges() as usize,
                        false,
                        Some(0.5),
                        Some(self.avoid_false_negatives),
                        None,
                        Some(self.use_scale_free_distribution),
                        None,
                        None,
                    )
                    .unwrap()
                    .map(|(src, dst, label)| (src as usize, dst as usize, label))
                    .for_each(|(src, dst, label)| {
                        let random_state = splitmix64(
                            random_state
                                .wrapping_mul(src as u64)
                                .wrapping_mul(dst as u64),
                        );
                        let src_embedding = unsafe {
                            &mut (*shared_node_embedding.get())[(src * self.get_number_of_words())
                                ..((src + 1) * self.get_number_of_words())]
                        };
                        let dst_embedding = unsafe {
                            &mut (*shared_node_embedding.get())[(dst * self.get_number_of_words())
                                ..((dst + 1) * self.get_number_of_words())]
                        };

                        // let similarity: f32 = unsafe {
                        //     cosine_similarity_sequential_from_bits_unchecked(src_embedding, dst_embedding).0
                        // };

                        // let prediction = 1.0 / (1.0 + (-similarity).exp());

                        if label {
                            // In the case these are two nodes that are
                            // connected in the graph, we need to switch `step_size * prediction`
                            // bits to one that are NOT shared between the two vectors.
                            src_embedding
                                .iter_mut()
                                .zip(dst_embedding.iter_mut())
                                .for_each(|(src_word, dst_word)| {
                                    let src_word_backup = *src_word;
                                    let dst_word_backup = *dst_word;
                                    *src_word |= dst_word_backup;
                                    *dst_word |= src_word_backup;
                                });
                        } else {
                            // In the case these are two nodes that are NOT
                            // connected in the graph, we need to switch `step_size * prediction`
                            // bits to zero that are shared between the two vectors.
                            src_embedding
                                .iter_mut()
                                .zip(dst_embedding.iter_mut())
                                .for_each(|(src_word, dst_word)| {
                                    let src_word_backup = *src_word;
                                    let dst_word_backup = *dst_word;
                                    let intersection = src_word_backup & dst_word_backup;
                                    *src_word &= !intersection;
                                    *dst_word &= !intersection;
                                });
                        };
                    });
            });
        Ok(())
    }
}
