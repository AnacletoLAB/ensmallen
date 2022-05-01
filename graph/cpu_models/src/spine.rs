use funty::Integral;
use graph::{EdgeT, Graph, NodeT, ThreadDataRaceAware};
use indicatif::{ProgressBar, ProgressIterator, ProgressStyle};
use rayon::iter::IntoParallelIterator;
use rayon::iter::IntoParallelRefMutIterator;
use rayon::iter::ParallelIterator;
use rayon::prelude::ParallelSliceMut;
use rayon::prelude::*;

pub trait DistanceType: Send + Sync + Integral {}

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
    fn get_anchor_nodes_buckets(&self, graph: &Graph) -> Result<Vec<Vec<NodeT>>, String> {
        let number_of_edge_per_bucket: EdgeT =
            ((graph.get_number_of_directed_edges() as f32 / 2 as f32 / self.embedding_size as f32)
                .ceil() as EdgeT)
                .max(1);

        let mut node_ids: Vec<NodeT> = graph.get_node_ids();
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
            current_bucket_size += graph.get_unchecked_node_degree_from_node_id(node_id) as EdgeT;
            current_bucket.push(node_id);
        });

        if buckets.len() < self.embedding_size {
            return Err(format!(
                concat!(
                    "It was not possible to create buckets for the requested number of features ({embedding_size}) ",
                    "but only for {actual_embedding_size} features.",
                    "Please reduce the requested embedding size to a value equal to or smaller ",
                    "than the number of features that can be created in this graph instance."
                ),
                embedding_size=self.embedding_size,
                actual_embedding_size=buckets.len()
            ));
        }

        Ok(buckets)
    }

    pub unsafe fn compute_unchecked_feature_from_bucket<Distance>(
        &self,
        graph: &Graph,
        mut bucket: Vec<NodeT>,
        mut distances: &mut [Distance],
    ) where
        Distance: DistanceType,
    {
        // We initialize the provided slice with the maximum distance.
        distances.par_iter_mut().for_each(|distance| {
            *distance = Distance::MAX;
        });

        // We wrap the distances object in an unsafe cell so
        // it may be shared among threads.
        let shared_distances = ThreadDataRaceAware::new(distances);
        let mut eccentricity: Distance = Distance::ZERO;

        // We iterate over the source node IDs and we assign
        // to each of them a distance of zero.
        bucket.par_iter().copied().for_each(|node_id| {
            *(*shared_distances.get()).get_unchecked_mut(node_id as usize) = Distance::ZERO;
        });

        // Until the bucket is not empty we start to iterate.
        while !bucket.is_empty() {
            eccentricity += Distance::ONE;

            // We compute the next bucket of nodes, i.e. the next step of the frontier.
            bucket = bucket
                .into_par_iter()
                .flat_map_iter(|node_id| {
                    graph
                        .iter_unchecked_neighbour_node_ids_from_source_node_id(node_id)
                        .filter_map(|neighbour_node_id| {
                            let distance = (*shared_distances.get())
                                .get_unchecked_mut(neighbour_node_id as usize);
                            if *distance == Distance::MAX {
                                // Set it's distance
                                *distance = eccentricity;
                                // add the node to the nodes to explore
                                Some(neighbour_node_id)
                            } else {
                                None
                            }
                        })
                })
                .collect::<Vec<NodeT>>();
        }

        // We retrieve the reference to the distances slice.
        distances = shared_distances.into_inner();

        // We set all remaining MAX distances to the computed exentricity.
        distances.par_iter_mut().for_each(|distance| {
            if *distance == Distance::MAX {
                *distance = eccentricity;
            }
        });
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
        Distance: DistanceType,
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

        // Depending whether verbosity was requested by the user
        // we create or not a visible progress bar to show the progress
        // in the computation of the features.
        let features_progress_bar = if verbose {
            let pb = ProgressBar::new(self.embedding_size as u64);
            pb.set_style(ProgressStyle::default_bar().template(
                "SPINE features {spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] ({pos}/{len}, ETA {eta})",
            ));
            pb
        } else {
            ProgressBar::hidden()
        };

        // We start to compute the features
        embedding
            .chunks_mut(self.embedding_size)
            .zip(self.get_anchor_nodes_buckets(graph)?)
            .progress_with(features_progress_bar)
            .for_each(|(empty_feature, bucket)| unsafe {
                self.compute_unchecked_feature_from_bucket(graph, bucket, empty_feature);
            });

        Ok(())
    }
}
