use crate::*;
use core::ops::{Add, AddAssign, Div};
use graph::{Graph, ThreadDataRaceAware};
use indicatif::ProgressBar;
use indicatif::ProgressIterator;
use indicatif::ProgressStyle;
use num_traits::{Coerced, One, Zero};
use rayon::prelude::*;
use vec_rand::splitmix64;

#[derive(Clone, Debug)]
pub struct RandomGraphConvolutionEmbedding {
    /// Number of convolutions
    number_of_convolutions: usize,
    /// Dimensionality of the embedding.
    embedding_size: usize,
    /// Random state to reproduce the model training.
    random_state: u64,
    /// Whether to show loading bars.
    verbose: bool,
}

impl Default for RandomGraphConvolutionEmbedding {
    fn default() -> Self {
        Self::new(None, None, None, None).unwrap()
    }
}

impl RandomGraphConvolutionEmbedding {
    /// Return new instance of Binary First-order LINE.
    ///
    pub fn new(
        number_of_convolutions: Option<usize>,
        embedding_size: Option<usize>,
        random_state: Option<u64>,
        verbose: Option<bool>,
    ) -> Result<Self, String> {
        Ok(Self {
            number_of_convolutions: must_not_be_zero(
                number_of_convolutions,
                10,
                "number of convolutions",
            )?,
            embedding_size: must_not_be_zero(embedding_size, 100, "embedding size")?,
            random_state: random_state.unwrap_or(42),
            verbose: verbose.unwrap_or(true),
        })
    }

    pub fn is_verbose(&self) -> bool {
        self.verbose
    }

    pub fn get_embedding_size(&self) -> usize {
        self.embedding_size
    }

    pub fn get_random_state(&self) -> u64 {
        self.random_state
    }

    pub fn fit_transform<Feature>(
        &self,
        graph: &Graph,
        embedding: &mut [Feature],
    ) -> Result<(), String>
    where
        Feature: Add<Feature> + AddAssign + Copy + Zero + One + Sync + Send + Div<Output = Feature>,
        u32: Coerced<Feature>,
    {
        let shared_node_embedding = ThreadDataRaceAware::new(embedding);
        let progress_bar = if self.is_verbose() {
            let pb = ProgressBar::new(self.number_of_convolutions as u64);
            pb.set_style(ProgressStyle::default_bar().template(concat!(
                "Binary First-order LINE {spinner:.green} [{elapsed_precise}] ",
                "[{bar:40.cyan/blue}] ({pos}/{len}, ETA {eta})"
            )));
            pb
        } else {
            ProgressBar::hidden()
        };

        // We start to loop over the required amount of epochs.
        (0..self.number_of_convolutions)
            .progress_with(progress_bar)
            .for_each(|_| {
                // We iterate over the graph edges.
                graph.par_iter_node_ids().for_each(|src| unsafe {
                    let mut vector = vec![Feature::zero(); self.embedding_size];
                    let mut node_degree = Feature::one();
                    graph
                        .iter_unchecked_neighbour_node_ids_from_source_node_id(src)
                        .for_each(|dst| {
                            node_degree += Feature::one();
                            vector
                                .iter_mut()
                                .zip(
                                    (*shared_node_embedding.get())[((dst as usize)
                                        * self.get_embedding_size())
                                        ..((dst as usize + 1) * self.get_embedding_size())]
                                        .iter()
                                        .copied(),
                                )
                                .for_each(|(a, b)| {
                                    *a += b;
                                });
                        });
                    (&mut (*shared_node_embedding.get())[((src as usize)
                        * self.get_embedding_size())
                        ..((src as usize + 1) * self.get_embedding_size())])
                        .iter_mut()
                        .zip(vector.into_iter())
                        .for_each(|(a, b)| {
                            *a = (*a + b) / node_degree;
                        });
                });
            });
        Ok(())
    }
}
