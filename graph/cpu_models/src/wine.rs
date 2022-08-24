use crate::AnchorFeatureTypes;
use crate::{
    must_not_be_zero, AnchorsBasedFeature, BasicAnchorsInferredNodeEmbedding, IntegerFeatureType,
};
use core::sync::atomic::Ordering;
use ensmallen_traits::prelude::*;
use graph::{Graph, NodeT};
use indicatif::{ParallelProgressIterator, ProgressBar, ProgressStyle};
use rayon::prelude::*;
use vec_rand::splitmix64;

#[derive(Clone, Debug)]
pub struct BasicWINE {
    /// Baseline parameters
    baine: BasicAnchorsInferredNodeEmbedding,
    /// Length of the random walk.
    walk_length: usize,
    /// Random state to use for the neighbours sampling.
    random_state: u64,
    /// Maximum number of neighbours to sample.
    max_neighbours: usize,
}

impl BasicWINE {
    /// Return new instance of basic WINE model.
    ///
    /// # Arguments
    /// * `embedding_size`: Option<usize> - Size of the embedding. By default 100.
    /// * `walk_length`: Option<usize> - Length of the random walk. By default 2, to capture exclusively the immediate context.
    /// * `verbose`: Option<bool> - Whether to show a loading bar while computing the embedding.
    pub fn new(
        embedding_size: Option<usize>,
        walk_length: Option<usize>,
        verbose: Option<bool>,
    ) -> Result<Self, String> {
        Ok(Self {
            baine: BasicAnchorsInferredNodeEmbedding::new(embedding_size, verbose)?,
            walk_length: must_not_be_zero(walk_length, 2, "Random walk length")?,
            random_state: 42,
            max_neighbours: 1000,
        })
    }

    pub fn get_basic_inferred_node_embedding(&self) -> &BasicAnchorsInferredNodeEmbedding {
        &self.baine
    }

    pub fn is_verbose(&self) -> bool {
        self.baine.is_verbose()
    }
}

pub trait WINEBased {
    fn get_basic_wine(&self) -> &BasicWINE;

    fn get_walk_length(&self) -> usize {
        self.get_basic_wine().walk_length
    }

    fn get_random_state(&self) -> u64 {
        self.get_basic_wine().random_state
    }

    fn get_max_neighbours(&self) -> usize {
        self.get_basic_wine().max_neighbours
    }
}

impl<M> AnchorsBasedFeature<{ AnchorFeatureTypes::Walks }> for M
where
    M: WINEBased,
{
    unsafe fn compute_unchecked_feature_from_bucket<Feature>(
        &self,
        graph: &Graph,
        mut bucket: Vec<NodeT>,
        features: &mut [Feature],
    ) where
        Feature: IntegerFeatureType,
    {
        // We initialize the provided slice with the maximum distance.
        features.par_iter_mut().for_each(|distance| {
            *distance = Feature::ZERO;
        });

        // We wrap the features object in an unsafe cell so
        // it may be shared among threads.
        let shared_features = &Feature::from_mut_slice(features);
        // let max_neighbours = self.get_max_neighbours();
        let mut random_walk_length: Feature = Feature::ZERO;
        let mut random_state = splitmix64(self.get_random_state());

        // Depending whether verbosity was requested by the user
        // we create or not a visible progress bar to show the progress
        // in the computation of the features.
        let depth_progress = if self.get_basic_wine().is_verbose() {
            let pb = ProgressBar::new(self.get_walk_length() as u64);
            pb.set_style(ProgressStyle::default_bar().template(concat!(
                "depth {spinner:.green} [{elapsed_precise}] ",
                "[{bar:40.cyan/blue}] ({pos}/{len}, ETA {eta})"
            )));
            pb
        } else {
            ProgressBar::hidden()
        };

        // Until the bucket is not empty we start to iterate.
        let max_depth = Feature::try_from(self.get_walk_length()).unwrap_or(Feature::MAX);
        while !bucket.is_empty() {
            random_walk_length += Feature::ONE;
            random_state = splitmix64(random_state);

            let frontier_progress = if self.get_basic_wine().is_verbose() {
                let pb = ProgressBar::new(bucket.len() as u64);
                pb.set_style(ProgressStyle::default_bar().template(concat!(
                    "frontier {spinner:.green} [{elapsed_precise}] ",
                    "[{bar:40.cyan/blue}] ({pos}/{len}, ETA {eta})"
                )));
                pb
            } else {
                ProgressBar::hidden()
            };

            if random_walk_length < max_depth {
                // We compute the next bucket of nodes, i.e. the next step of the frontier.
                if random_walk_length == Feature::ONE {
                    bucket = bucket
                        .into_par_iter()
                        .progress_with(frontier_progress)
                        .flat_map_iter(|node_id| {
                            graph
                                .iter_unchecked_neighbour_node_ids_from_source_node_id(node_id)
                                .filter_map(|neighbour_node_id| {
                                    let previous_count = shared_features
                                        [neighbour_node_id as usize]
                                        .fetch_add(Feature::ONE, Ordering::Relaxed);
                                    if previous_count == Feature::ZERO {
                                        Some(neighbour_node_id)
                                    } else {
                                        None
                                    }
                                })
                        })
                        .collect::<Vec<NodeT>>();
                } else if random_walk_length == (Feature::ONE + Feature::ONE) {
                    bucket = bucket
                        .into_par_iter()
                        .progress_with(frontier_progress)
                        .flat_map_iter(|node_id| {
                            let number_of_visits =
                                shared_features[node_id as usize].load(Ordering::Relaxed);
                            graph
                                .iter_unchecked_neighbour_node_ids_from_source_node_id(node_id)
                                .map(move |neighbour_node_id| {
                                    shared_features[neighbour_node_id as usize]
                                        .fetch_add(number_of_visits, Ordering::Relaxed);
                                    neighbour_node_id
                                })
                        })
                        .collect::<Vec<NodeT>>();
                } else {
                    bucket = bucket
                        .into_par_iter()
                        .progress_with(frontier_progress)
                        .flat_map_iter(|node_id| {
                            graph
                                .iter_unchecked_neighbour_node_ids_from_source_node_id(node_id)
                                .map(|neighbour_node_id| {
                                    shared_features[neighbour_node_id as usize]
                                        .fetch_add(Feature::ONE, Ordering::Relaxed);
                                    neighbour_node_id
                                })
                        })
                        .collect::<Vec<NodeT>>();
                }
                depth_progress.inc(1);
            } else {
                // We compute the next bucket of nodes, i.e. the next step of the frontier.
                if random_walk_length == (Feature::ONE + Feature::ONE) {
                    bucket
                        .into_par_iter()
                        .progress_with(frontier_progress)
                        .for_each(|node_id| {
                            let number_of_visits =
                                shared_features[node_id as usize].load(Ordering::Relaxed);
                            graph
                                .iter_unchecked_neighbour_node_ids_from_source_node_id(node_id)
                                .for_each(move |neighbour_node_id| {
                                    shared_features[neighbour_node_id as usize]
                                        .fetch_add(number_of_visits, Ordering::Relaxed);
                                });
                        });
                } else {
                    bucket
                        .into_par_iter()
                        .progress_with(frontier_progress)
                        .for_each(|node_id| {
                            graph
                                .iter_unchecked_neighbour_node_ids_from_source_node_id(node_id)
                                .for_each(|neighbour_node_id| {
                                    shared_features[neighbour_node_id as usize]
                                        .fetch_add(Feature::ONE, Ordering::Relaxed);
                                });
                        });
                }
                depth_progress.inc(1);
                return;
            }
        }
    }
}
