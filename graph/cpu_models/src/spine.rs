use crate::{
    must_not_be_zero, AnchorsBasedFeature, BasicAnchorsInferredNodeEmbedding, FeatureType,
};
use core::sync::atomic::Ordering;
use crate::AnchorFeatureTypes;
use ensmallen_traits::prelude::*;
use graph::{Graph, NodeT, ThreadDataRaceAware};
use rayon::prelude::*;

#[derive(Clone, Debug)]
pub struct BasicSPINE {
    /// Baseline parameters
    baine: BasicAnchorsInferredNodeEmbedding,
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
            baine: BasicAnchorsInferredNodeEmbedding::new(embedding_size, verbose)?,
            maximum_depth: must_not_be_zero(maximum_depth, usize::MAX, "Maximum depth")?,
        })
    }

    pub fn get_basic_inferred_node_embedding(&self) -> &BasicAnchorsInferredNodeEmbedding {
        &self.baine
    }
}

pub trait SPINEBased {
    fn get_basic_spine(&self) -> &BasicSPINE;

    fn get_maximum_depth(&self) -> usize {
        self.get_basic_spine().maximum_depth
    }
}

impl<M> AnchorsBasedFeature<{AnchorFeatureTypes::ShortestPaths}> for M
where
    M: SPINEBased,
{
    unsafe fn compute_unchecked_feature_from_bucket<Feature>(
        &self,
        graph: &Graph,
        mut bucket: Vec<NodeT>,
        mut features: &mut [Feature],
    ) where
        Feature: FeatureType,
    {
        // We initialize the provided slice with the maximum distance.
        features.par_iter_mut().for_each(|distance| {
            *distance = Feature::MAX;
        });

        // We wrap the features object in an unsafe cell so
        // it may be shared among threads.
        let shared_features = Feature::from_mut_slice(features);
        let mut eccentricity: Feature = Feature::ZERO;

        // We iterate over the source node IDs and we assign
        // to each of them a distance of zero.
        bucket.par_iter().copied().for_each(|node_id| {
            shared_features[node_id as usize].store(Feature::ZERO, Ordering::Relaxed);
        });

        // Until the bucket is not empty we start to iterate.
        let max_depth = Feature::try_from(self.get_maximum_depth()).unwrap_or(Feature::MAX);
        while !bucket.is_empty() {
            if eccentricity == max_depth {
                break;
            }
            eccentricity += Feature::ONE;

            // We compute the next bucket of nodes, i.e. the next step of the frontier.
            bucket = bucket
                .into_par_iter()
                .flat_map_iter(|node_id| {
                    graph
                        .iter_unchecked_neighbour_node_ids_from_source_node_id(node_id)
                        .filter_map(|neighbour_node_id| {
                            if shared_features[neighbour_node_id as usize].compare_exchange(
                                Feature::MAX,
                                eccentricity,
                                Ordering::Relaxed,
                                Ordering::Relaxed,
                            ).is_ok() {
                                // add the node to the nodes to explore
                                Some(neighbour_node_id)
                            } else {
                                None
                            }
                        })
                })
                .collect::<Vec<NodeT>>();
        }

        // We retrieve the reference to the features slice.
        features = Feature::get_mut_slice(shared_features);

        // We set all remaining MAX features to the computed exentricity.
        features.par_iter_mut().for_each(|distance| {
            if *distance == Feature::MAX {
                *distance = eccentricity;
            }
        });
    }
}
