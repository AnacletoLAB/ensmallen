use graph::{DijkstraQueue, EdgeT, Graph, NodeT};
use indicatif::ParallelProgressIterator;
use indicatif::{ProgressBar, ProgressStyle};
use rayon::iter::IntoParallelRefMutIterator;
use rayon::iter::ParallelIterator;
use rayon::prelude::ParallelSliceMut;
use rayon::prelude::*;

#[derive(Clone, Debug)]
pub struct WeightedSPINE {
    embedding_size: usize,
    use_edge_weights_as_probabilities: bool,
}

impl WeightedSPINE {
    /// Return new instance of SPINE model.
    ///
    /// # Arguments
    /// `embedding_size`: Option<usize> - Size of the embedding. By default 100.
    /// `use_edge_weights_as_probabilities`: Option<bool> - Whether to treat the weights as probabilities.
    pub fn new(
        embedding_size: Option<usize>,
        use_edge_weights_as_probabilities: Option<bool>,
    ) -> Result<Self, String> {
        // Handle the values of the default parameters.
        let embedding_size = embedding_size.unwrap_or(100);
        let use_edge_weights_as_probabilities = use_edge_weights_as_probabilities.unwrap_or(false);

        // Validate that the provided parameters are within
        // reasonable bounds.
        if embedding_size == 0 {
            return Err(concat!("The embedding size cannot be equal to zero.").to_string());
        }

        Ok(Self {
            embedding_size,
            use_edge_weights_as_probabilities,
        })
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

    pub unsafe fn compute_unchecked_feature_from_bucket(
        &self,
        graph: &Graph,
        bucket: Vec<NodeT>,
        distances: &mut [f32],
    ) {
        // We initialize the provided slice with the maximum distance.
        distances.par_iter_mut().for_each(|distance| {
            *distance = f32::MAX;
        });

        let mut nodes_to_explore: DijkstraQueue<f32> = DijkstraQueue::with_capacity_from_roots(
            graph.get_number_of_nodes() as usize,
            bucket,
            distances,
        );
        let mut eccentricity: f32 = 0.0;

        while let Some(closest_node_id) = nodes_to_explore.pop() {
            // Update the distances metrics
            let closest_node_id_distance = nodes_to_explore[closest_node_id];
            if closest_node_id_distance > eccentricity {
                eccentricity = closest_node_id_distance;
            }

            graph
                .iter_unchecked_neighbour_node_ids_from_source_node_id(closest_node_id as NodeT)
                .zip(
                    graph.iter_unchecked_edge_weights_from_source_node_id(closest_node_id as NodeT),
                )
                .for_each(|(neighbour_node_id, weight)| {
                    let new_neighbour_distance = nodes_to_explore[closest_node_id]
                        + if self.use_edge_weights_as_probabilities {
                            -weight.ln()
                        } else {
                            weight
                        };
                    if new_neighbour_distance < nodes_to_explore[neighbour_node_id as usize] {
                        nodes_to_explore.push(neighbour_node_id as usize, new_neighbour_distance);
                    }
                });
        }

        // If the edge weights are to be treated as probabilities
        // we need to adjust the distances back using the exponentiation.
        if self.use_edge_weights_as_probabilities {
            eccentricity = (-eccentricity).exp();
            distances.par_iter_mut().for_each(|distance| {
                if *distance == f32::MAX {
                    *distance = eccentricity;
                } else {
                    *distance = (-*distance).exp();
                }
            });
        } else {
            distances.par_iter_mut().for_each(|distance| {
                if *distance == f32::MAX {
                    *distance = eccentricity;
                }
            });
        }
    }

    /// Computes in the provided slice of embedding the SPINE node embedding.
    ///
    /// # Arguments
    /// `graph`: &Graph - The graph to embed
    /// `embedding`: &mut [f32] - The memory area where to write the embedding.
    /// `verbose`: Option<bool> - Whether to show the loading bar, by default true.
    ///
    /// # Raises
    /// * If the graph does not have weights.
    /// * If the provided embedding is not of the right shape.
    /// * If the edge weights are requested to be treated as probabilit
    pub fn fit_transform(
        &self,
        graph: &Graph,
        embedding: &mut [f32],
        verbose: Option<bool>,
    ) -> Result<(), String> {
        if !graph.has_edge_weights() {
            return Err(concat!(
                "The provided graph does not have weights, which are necessary ",
                "to execute the weighted SPINE embedding. If you graph does not have ",
                "weights, consider using the normal SPINE embedding, which is surely ",
                "much more efficient in terms of memory and time requirements for this ",
                "use case."
            )
            .to_string());
        }
        if graph.has_constant_edge_weights().unwrap() {
            return Err(concat!(
                "The provided graph does have weights, but they are constant. ",
                "Distances weighted by a constant weights are equal to unweighted ",
                "distances, and therefore you would be better off by using the normal ",
                "SPINE embedding, which is surely much more efficient in terms of ",
                "memory and time requirements for this use case."
            )
            .to_string());
        }
        if graph.has_negative_edge_weights().unwrap() {
            return Err(concat!(
                "The provided graph has negative edge weights, which are not currently ",
                "handled by this weighted SPINE implementation as it uses Dijkstra to ",
                "compute the weighted distances. There exists graph algorithms that are ",
                "able to compute distances including negative edge weights: if you need ",
                "to execute such a use case, do consider to open a pull request on the ",
                "Ensmallen GitHub repository."
            )
            .to_string());
        }

        if self.use_edge_weights_as_probabilities
            && !graph.has_edge_weights_representing_probabilities().unwrap()
        {
            return Err(concat!(
                "It has been requested to handle the edge weights for the weighted ",
                "shortest paths as if they were probabilities, but the values are ",
                "not normalized between zero and one. Possibly, you wanted to execute ",
                "either the `graph.normalize_edge_weights_inplace()`, ",
                "`graph.normalize_edge_weights()`, `graph.divide_edge_weights_inplace(value)` ",
                " or `graph.divide_edge_weights(value)` methods.",
            )
            .to_string());
        }

        let verbose = verbose.unwrap_or(true);

        let expected_embedding_len = self.embedding_size * graph.get_number_of_nodes() as usize;

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
            ).unwrap());
            pb
        } else {
            ProgressBar::hidden()
        };

        // We start to compute the features
        embedding
            .par_chunks_mut(graph.get_number_of_nodes() as usize)
            .zip(self.get_anchor_nodes_buckets(graph)?)
            .progress_with(features_progress_bar)
            .for_each(|(empty_feature, bucket)| unsafe {
                self.compute_unchecked_feature_from_bucket(graph, bucket, empty_feature);
            });

        Ok(())
    }
}
