use crate::*;
use core::sync::atomic::Ordering;
use graph::{Graph, NodeT};
use num_traits::{AsPrimitive, Atomic};
use rayon::prelude::*;
use vec_rand::splitmix64;

#[derive(Clone, Debug)]
pub struct RUBICONE {
    /// Baseline parameters
    baine: BasicALPINE,
    /// Number of convolutions.
    number_of_convolutions: usize,
    /// Random state
    random_state: u64,
}

impl RUBICONE {
    /// Return new instance of RUBICONE model.
    ///
    /// # Arguments
    /// * `embedding_size`: Option<usize> - Size of the embedding. By default 100.
    /// * `number_of_convolutions`: Option<usize> - Number of convolutions.
    /// * `random_state`: Option<u64> - Random state to reproduce the embedding procedure.
    /// * `verbose`: Option<bool> - Whether to show a loading bar while computing the embedding.
    pub fn new(
        embedding_size: Option<usize>,
        number_of_convolutions: Option<usize>,
        random_state: Option<u64>,
        verbose: Option<bool>,
    ) -> Result<Self, String> {
        Ok(Self {
            baine: BasicALPINE::new(embedding_size, verbose)?,
            number_of_convolutions: must_not_be_zero(
                number_of_convolutions,
                2,
                "Number of convolutions",
            )?,
            random_state: random_state.unwrap_or(42),
        })
    }

    pub fn get_number_of_convolutions(&self) -> usize {
        self.number_of_convolutions
    }

    pub fn get_random_state(&self) -> u64 {
        self.random_state
    }
}

impl LandmarkBasedFeature<{ LandmarkFeatureType::Random }> for RUBICONE {
    unsafe fn compute_unchecked_feature_from_bucket<Feature>(
        &self,
        graph: &Graph,
        _bucket: Vec<NodeT>,
        features: &mut [Feature],
        feature_number: usize,
    ) where
        Feature: IntegerFeatureType,
        u64: AsPrimitive<Feature>,
    {
        let random_state = splitmix64(self.get_random_state())
            .wrapping_mul(self.get_random_state().wrapping_add(feature_number as u64));

        // We initialize the provided slice with the maximum distance.

        let maximum_value: u64 = Feature::MAX.as_();

        features
            .par_iter_mut()
            .enumerate()
            .for_each(|(i, distance)| {
                *distance = (splitmix64(
                    (random_state.wrapping_add(i as u64)).wrapping_mul(random_state + i as u64),
                ) % maximum_value)
                    .as_()
            });

        // We wrap the features object in an unsafe cell so
        // it may be shared among threads.
        let shared_features = Feature::from_mut_slice(features);

        let number_of_bits = (maximum_value as f32).log2().ceil() as usize;

        (0..self.get_number_of_convolutions()).for_each(|_| {
            graph.par_iter_node_ids().for_each(|src| {
                let mut new_src_feature = vec![0; number_of_bits];
                let half_number_of_neighbours = graph
                    .iter_unchecked_neighbour_node_ids_from_source_node_id(src)
                    .map(|dst| {
                        let mut dst_feature = shared_features[dst as usize].load(Ordering::Relaxed);
                        new_src_feature.iter_mut().for_each(|value| {
                            if dst_feature & Feature::ONE == Feature::ONE {
                                *value += 1;
                            }
                            dst_feature = dst_feature >> Feature::ONE;
                        });
                    })
                    .count()
                    / 2;
                shared_features[src as usize].store(
                    new_src_feature.into_iter().rev().fold(
                        Feature::ZERO,
                        |mut feature_being_built, feature_count| {
                            if feature_count > half_number_of_neighbours {
                                feature_being_built |= Feature::ONE;
                            }
                            feature_being_built << Feature::ONE
                        },
                    ),
                    Ordering::Relaxed,
                );
            });
        });
    }
}

impl EmbeddingSize for RUBICONE {
    fn get_embedding_size(&self, _graph: &graph::Graph) -> Result<usize, String> {
        Ok(self
            .get_basic_inferred_node_embedding()
            .get_embedding_size())
    }
}

impl EmptyLandmarkGenerator for RUBICONE {}

impl ALPINE<{ LandmarkType::Empty }, { LandmarkFeatureType::Random }> for RUBICONE {
    fn get_model_name(&self) -> String {
        "RUBICONE".to_string()
    }

    fn get_basic_inferred_node_embedding(&self) -> &crate::BasicALPINE {
        &self.baine
    }
}
