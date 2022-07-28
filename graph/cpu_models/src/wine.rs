use crate::AnchorFeatureTypes;
use crate::{
    must_not_be_zero, AnchorsBasedFeature, BasicAnchorsInferredNodeEmbedding, FeatureType,
};
use core::sync::atomic::Ordering;
use ensmallen_traits::prelude::*;
use graph::{Graph, NodeT, ThreadDataRaceAware};
use rayon::prelude::*;

#[derive(Clone, Debug)]
pub struct BasicWINE {
    /// Baseline parameters
    baine: BasicAnchorsInferredNodeEmbedding,
    /// Length of the random walk.
    walk_length: usize,
}

impl BasicWINE {
    /// Return new instance of basic WINE model.
    ///
    /// # Arguments
    /// * `embedding_size`: Option<usize> - Size of the embedding. By default 100.
    /// * `walk_length`: Option<usize> - Length of the random walk.
    /// * `verbose`: Option<bool> - Whether to show a loading bar while computing the embedding.
    pub fn new(
        embedding_size: Option<usize>,
        walk_length: Option<usize>,
        verbose: Option<bool>,
    ) -> Result<Self, String> {
        Ok(Self {
            baine: BasicAnchorsInferredNodeEmbedding::new(embedding_size, verbose)?,
            walk_length: must_not_be_zero(walk_length, usize::MAX, "Maximum depth")?,
        })
    }

    pub fn get_basic_inferred_node_embedding(&self) -> &BasicAnchorsInferredNodeEmbedding {
        &self.baine
    }
}

pub trait WINEBased {
    fn get_basic_Wine(&self) -> &BasicWINE;

    fn get_walk_length(&self) -> usize {
        self.get_basic_Wine().walk_length
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
        mut features: &mut [Feature],
    ) where
        Feature: FeatureType,
    {
        // We initialize the provided slice with the maximum distance.
        features.par_iter_mut().for_each(|distance| {
            *distance = Feature::ZERO;
        });

        // We wrap the features object in an unsafe cell so
        // it may be shared among threads.
        let shared_features = Feature::from_mut_slice(features);
        let mut random_walk_length: Feature = Feature::ZERO;

        // Until the bucket is not empty we start to iterate.
        let max_depth = Feature::try_from(self.get_walk_length()).unwrap_or(Feature::MAX);
        while !bucket.is_empty() && random_walk_length < max_depth {
            random_walk_length += Feature::ONE;
            // We compute the next bucket of nodes, i.e. the next step of the frontier.
            bucket = bucket
                .into_par_iter()
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

        // We retrieve the reference to the features slice.
        features = Feature::get_mut_slice(shared_features);
    }
}
