use super::*;
use graph::{EdgeT, Graph, NodeT, ThreadDataRaceAware, WalksParameters};
use indicatif::{ProgressBar, ProgressIterator, ProgressStyle};
use num::traits::{One, Unsigned, Zero};
use rayon::iter::IndexedParallelIterator;
use rayon::iter::IntoParallelIterator;
use rayon::iter::IntoParallelRefMutIterator;
use rayon::iter::ParallelIterator;
use rayon::prelude::ParallelSlice;
use rayon::prelude::ParallelSliceMut;
use rayon::prelude::*;
use std::convert::TryFrom;

#[derive(Clone, Debug)]
pub struct SPINE {
    embedding_size: usize,
}

impl SPINE {
    /// Return new instance of SPINE model.
    ///
    /// # Arguments
    /// `embedding_size`: Option<usize> - Size of the embedding. By default 100.
    pub fn new(embedding_size: Option<usize>) -> Result<Self, String> {
        // Handle the values of the default parameters.
        let embedding_size = embedding_size.unwrap_or(100);

        // Validate that the provided parameters are within
        // reasonable bounds.
        if embedding_size == 0 {
            return Err(concat!("The embedding size cannot be equal to zero.").to_string());
        }

        Ok(Self { embedding_size })
    }

    /// Returns the used embedding size.
    pub fn get_embedding_size(&self) -> usize {
        self.embedding_size
    }

    /// Return vector of vectors of anchor node IDs.
    fn get_anchor_node_ids(&self, graph: &Graph) -> Result<Vec<Vec<NodeT>>, String> {
        let number_of_edge_per_bucket: EdgeT =
            ((graph.get_number_of_directed_edges() as f32 / 2 as f32 / self.embedding_size as f32)
                .ceil() as EdgeT)
                .max(1);

        let mut node_ids: Vec<NodeT> = self.get_node_ids();
        node_ids.par_sort_unstable_by(|&a, &b| unsafe {
            graph
                .get_unchecked_node_degree_from_node_id(b)
                .partial_cmp(&graph.get_unchecked_node_degree_from_node_id(a))
                .unwrap()
        });
        // Allocate the node scores
        let mut current_bucket_size = 0;
        let mut buckets: Vec<Vec<NodeT>> = Vec::new();
        let mut current_bucket: Vec<NodeT> = Vec::new();
        node_ids.into_iter().for_each(|node_id| unsafe {
            if current_bucket_size > number_of_edge_per_bucket {
                current_bucket_size = 0;
                buckets.push(current_bucket.clone());
                current_bucket = Vec::new();
            }
            if buckets.len() == self.embedding_size {
                return;
            }
            current_bucket_size += self.get_unchecked_node_degree_from_node_id(node_id) as EdgeT;
            current_bucket.push(node_id);
        });

        Ok(buckets)
    }

    pub unsafe fn get_distances_from_bucket<Distance>(
        &self,
        mut bucket: Vec<NodeT>,
        distances: &mut [Distance],
    ) where
        Distance: Send + Sync + IsInteger + TryFrom<usize> + Zero + One,
    {
        // We initialize the provided slice with the maximum distance.
        distances.par_iter_mut().for_each(|distance| {
            *distance = Distance::MAX;
        });

        let shared_distances = ThreadDataRaceAware::new(distances);
        let zero = Distance::zero();
        let one = Distance::one();
        let mut eccentricity: Distance = zero;

        bucket.par_iter().copied().for_each(|node_id| {
            *(*thread_shared_distances.get()).get_unchecked_mut(node_id) = zero;
        });

        while !frontier.is_empty() {
            eccentricity += one;
            if maximal_depth.map_or(false, |maximal_depth| maximal_depth > eccentricity) {
                break;
            }

            frontier = frontier
                .into_par_iter()
                .flat_map_iter(|node_id| {
                    // TODO!: The following line can be improved when the par iter is made
                    // generally available also for the elias-fano graphs.

                    self.iter_unchecked_neighbour_node_ids_from_source_node_id(node_id)
                })
                .filter_map(|neighbour_node_id| {
                    if (*thread_shared_distances.value.get())[neighbour_node_id as usize]
                        == node_not_present
                    {
                        // Set it's distance
                        (*thread_shared_distances.value.get())[neighbour_node_id as usize] =
                            eccentricity;
                        // add the node to the nodes to explore
                        Some(neighbour_node_id)
                    } else {
                        None
                    }
                })
                .collect::<Vec<NodeT>>();
        }
        eccentricity = eccentricity.saturating_sub(T::try_from(1).ok().unwrap());
        (distances, eccentricity, most_distant_node)
    }

    /// Computes in the provided slice of embedding the SPINE node embedding.
    ///
    /// # Arguments
    /// `graph`: &Graph - The graph to embed
    /// `embedding`: &mut [f32] - The memory area where to write the embedding.
    /// `verbose`: Option<bool> - Whether to show the loading bar, by default true.
    pub fn fit_transform<Distance>(
        &self,
        graph: &Graph,
        embedding: &mut [Distance],
        verbose: Option<bool>,
    ) -> Result<(), String>
    where
        Distance: TryFrom<u32> + Into<u32> + Send + Sync + IsInteger + TryFrom<usize>,
    {
        let verbose = verbose.unwrap_or(true);

        let expected_embedding_len = self.embedding_size * graph.get_nodes_number() as usize;

        if embedding.len() != expected_embedding_len {
            return Err(format!(
                "The given memory allocation for the embeddings is {} long but we expect {}.",
                embedding.len(),
                expected_embedding_len
            ));
        }

        // Check that the graph has edges.
        graph.must_have_edges()?;
    }
}
