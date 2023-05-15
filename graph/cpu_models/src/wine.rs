use crate::*;
use core::sync::atomic::Ordering;
use graph::{Graph, NodeT};
use num_traits::Atomic;
use parallel_frontier::prelude::*;

#[derive(Clone, Debug)]
pub struct BasicWINE {
    /// Baseline parameters
    baine: BasicALPINE,
    /// Length of the random walk.
    window_size: usize,
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
    /// * `window_size`: Option<usize> - Length of the random walk. By default 2, to capture exclusively the immediate context.
    /// * `verbose`: Option<bool> - Whether to show a loading bar while computing the embedding.
    pub fn new(
        embedding_size: Option<usize>,
        window_size: Option<usize>,
        verbose: Option<bool>,
    ) -> Result<Self, String> {
        Ok(Self {
            baine: BasicALPINE::new(embedding_size, verbose)?,
            window_size: must_not_be_zero(window_size, 2, "Random walk length")?,
            random_state: 42,
            max_neighbours: 1000,
        })
    }

    pub fn get_basic_inferred_node_embedding(&self) -> &BasicALPINE {
        &self.baine
    }
}

pub trait WINEBased {
    fn get_basic_wine(&self) -> &BasicWINE;

    fn get_window_size(&self) -> usize {
        self.get_basic_wine().window_size
    }

    fn get_random_state(&self) -> u64 {
        self.get_basic_wine().random_state
    }

    fn get_max_neighbours(&self) -> usize {
        self.get_basic_wine().max_neighbours
    }
}

impl<M> LandmarkBasedFeature<{ LandmarkFeatureType::Windows }> for M
where
    M: WINEBased,
{
    unsafe fn compute_unchecked_feature_from_bucket<Feature>(
        &self,
        graph: &Graph,
        bucket: Vec<NodeT>,
        features: &mut [Feature],
        _feature_number: usize,
    ) where
        Feature: IntegerFeatureType,
    {
        // We initialize the provided slice with the maximum distance.
        features.par_iter_mut().for_each(|distance| {
            *distance = Feature::zero();
        });
        // We wrap the features object in an unsafe cell so
        // it may be shared among threads.
        let shared_features = &Feature::from_mut_slice(features);
        // Initialize to 1 the count of the nodes in the buckets
        bucket.par_iter().for_each(|node_id| {
            shared_features[*node_id as usize].store(Feature::one(), Ordering::Relaxed);
        });

        if self.get_window_size() == 1 {
            bucket.into_par_iter().for_each(|node_id| {
                graph
                    .iter_unchecked_neighbour_node_ids_from_source_node_id(node_id)
                    .for_each(|neighbour_node_id| {
                        shared_features[neighbour_node_id as usize]
                            .fetch_saturating_add(Feature::one(), Ordering::Relaxed);
                    });
            });
        } else if self.get_window_size() == 2 {
            let frontier = Frontier::default();

            bucket.into_par_iter().for_each(|node_id| {
                graph
                    .iter_unchecked_neighbour_node_ids_from_source_node_id(node_id)
                    .for_each(|neighbour_node_id| {
                        if shared_features[neighbour_node_id as usize]
                            .fetch_saturating_add(Feature::one(), Ordering::Relaxed)
                            == Feature::zero()
                        {
                            frontier.push(neighbour_node_id)
                        }
                    });
            });

            let variation: Frontier<Feature> = frontier
                .par_iter_vectors()
                .map(|vector| {
                    vector
                        .iter()
                        .map(|&node_id| shared_features[node_id as usize].load(Ordering::Relaxed))
                        .collect::<Vec<Feature>>()
                })
                .collect::<Vec<Vec<Feature>>>()
                .try_into()
                .unwrap();

            frontier
                .par_iter()
                .zip(variation.par_iter())
                .for_each(|(&node_id, &count)| {
                    graph
                        .iter_unchecked_neighbour_node_ids_from_source_node_id(node_id)
                        .for_each(|neighbour_node_id| {
                            shared_features[neighbour_node_id as usize]
                                .fetch_saturating_add(count, Ordering::Relaxed);
                        });
                });
        } else {
            let mut first_counter = vec![Feature::zero(); graph.get_number_of_nodes() as usize];
            let mut second_counter = vec![Feature::zero(); graph.get_number_of_nodes() as usize];
            {
                let shared_first_counter = Feature::from_mut_slice(&mut first_counter);
                // Initialize to 1 the count of the nodes in the buckets
                bucket.par_iter().for_each(|node_id| {
                    shared_first_counter[*node_id as usize].store(Feature::one(), Ordering::Relaxed);
                });
            }

            // Until the bucket is not empty we start to iterate.
            let mut primary_frontier: Frontier<NodeT> = bucket.into();
            let mut temporary_frontier = Frontier::default();

            for _ in 0..self.get_window_size() {
                if primary_frontier.is_empty() {
                    break;
                }

                let shared_first_counter = Feature::from_mut_slice(&mut first_counter);
                let shared_second_counter = Feature::from_mut_slice(&mut second_counter);

                primary_frontier.par_iter().for_each(|&node_id| {
                    let count = shared_first_counter[node_id as usize].load(Ordering::Relaxed);
                    graph
                        .iter_unchecked_neighbour_node_ids_from_source_node_id(node_id)
                        .for_each(|neighbour_node_id| {
                            if shared_second_counter[neighbour_node_id as usize]
                                .fetch_saturating_add(count, Ordering::Relaxed)
                                == Feature::zero()
                            {
                                temporary_frontier.push(neighbour_node_id)
                            }
                            shared_features[neighbour_node_id as usize]
                                .fetch_saturating_add(count, Ordering::Relaxed);
                        });
                });
                primary_frontier.clear();
                std::mem::swap(&mut first_counter, &mut second_counter);
                second_counter.par_iter_mut().for_each(|count| {
                    *count = Feature::zero();
                });
                std::mem::swap(&mut primary_frontier, &mut temporary_frontier);
            }
        }
    }
}
