use crate::*;
use core::sync::atomic::Ordering;
use graph::{Graph, NodeT};
use num_traits::Atomic;
use parallel_frontier::prelude::*;

#[derive(Clone, Debug)]
pub struct BasicSPINE {
    /// Baseline parameters
    baine: BasicALPINE,
    /// Maximum depth of the shortest path.
    maximum_depth: usize,
}

impl BasicSPINE {
    /// Return new instance of basic SPINE model.
    ///
    /// # Arguments
    /// * `embedding_size`: Option<usize> - Size of the embedding. By default 100.
    /// * `maximum_depth`: Option<usize> - Maximum depth of the shortest path.
    /// * `verbose`: Option<bool> - Whether to show a loading bar while computing the embedding.
    pub fn new(
        embedding_size: Option<usize>,
        maximum_depth: Option<usize>,
        verbose: Option<bool>,
    ) -> Result<Self, String> {
        Ok(Self {
            baine: BasicALPINE::new(embedding_size, verbose)?,
            maximum_depth: must_not_be_zero(maximum_depth, usize::max_value(), "Maximum depth")?,
        })
    }

    pub fn get_basic_inferred_node_embedding(&self) -> &BasicALPINE {
        &self.baine
    }
}

pub trait SPINEBased {
    fn get_basic_spine(&self) -> &BasicSPINE;

    fn get_maximum_depth(&self) -> usize {
        self.get_basic_spine().maximum_depth
    }
}

impl<M> LandmarkBasedFeature<{ LandmarkFeatureType::ShortestPaths }> for M
where
    M: SPINEBased,
{
    unsafe fn compute_unchecked_feature_from_bucket<Feature>(
        &self,
        graph: &Graph,
        bucket: Vec<NodeT>,
        mut features: &mut [Feature],
        _feature_number: usize,
    ) where
        Feature: IntegerFeatureType,
    {
        // We initialize the provided slice with the maximum distance.
        features.par_iter_mut().for_each(|distance| {
            *distance = Feature::max_value();
        });

        // We wrap the features object in an unsafe cell so
        // it may be shared among threads.
        let shared_features = Feature::from_mut_slice(features);
        let mut eccentricity: Feature = Feature::zero();

        // We iterate over the source node IDs and we assign
        // to each of them a distance of zero.
        bucket.par_iter().copied().for_each(|node_id| {
            shared_features[node_id as usize].store(Feature::zero(), Ordering::Relaxed);
        });

        let mut primary_frontier: Frontier<NodeT> = bucket.into();
        let mut temporary_frontier = Frontier::default();

        // Until the bucket is not empty we start to iterate.
        let max_depth = Feature::try_from(self.get_maximum_depth()).unwrap_or(Feature::max_value());
        while !primary_frontier.is_empty() {
            if eccentricity == max_depth {
                break;
            }
            eccentricity += Feature::one();

            // We compute the next bucket of nodes, i.e. the next step of the frontier.
            primary_frontier.par_iter().for_each(|&node_id| {
                graph
                    .iter_unchecked_neighbour_node_ids_from_source_node_id(node_id)
                    .for_each(|neighbour_node_id| {
                        if shared_features[neighbour_node_id as usize]
                            .compare_exchange(
                                Feature::max_value(),
                                eccentricity,
                                Ordering::SeqCst,
                                Ordering::SeqCst,
                            )
                            .is_ok()
                        {
                            // add the node to the nodes to explore
                            temporary_frontier.push(neighbour_node_id);
                        }
                    });
            });
            primary_frontier.clear();
            std::mem::swap(&mut primary_frontier, &mut temporary_frontier);
        }

        // We retrieve the reference to the features slice.
        features = Feature::get_mut_slice(shared_features);

        // We set all remaining MAX features to the computed exentricity.
        features.par_iter_mut().for_each(|distance| {
            if *distance == Feature::max_value() {
                *distance = eccentricity;
            }
        });
    }
}
