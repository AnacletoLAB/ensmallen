use std::cell::SyncUnsafeCell;

use graph::{Graph, NodeT};
use hyperloglog_rs::prelude::*;
use rayon::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize)]
/// Struct implementing Hyper Subgraph Sketching.
///
/// # Implementation details
///
/// # References
/// The original paper describing this approach for edge prediction
/// feature mining is "Graph Neural Networks for Link Prediction with Subgraph sketching"
///
pub struct HyperSketching<const PRECISION: usize, const BITS: usize, const HOPS: usize>
where
    [(); ceil(1 << PRECISION, 32 / BITS)]:,
{
    /// Vector of HyperLogLog counters
    counters: Vec<HyperLogLogArray<PRECISION, BITS, HOPS>>,
    /// Normalize by symmetric Laplacian
    normalize_by_symmetric_laplacian: bool,
    /// Concatenate the normalized and non-normalized features
    concatenate_features: bool,
}

impl<const PRECISION: usize, const BITS: usize, const HOPS: usize>
    HyperSketching<PRECISION, BITS, HOPS>
where
    [(); ceil(1 << PRECISION, 32 / BITS)]:,
{
    /// Creates a new HyperSketching model.
    ///
    /// # Arguments
    /// * `normalize_by_symmetric_laplacian`: Option<bool> - Whether to normalize the Sketching cardinalities by the symmetric Laplacian. By default, false.
    /// * `concatenate_features`: Option<bool> - Whether to concatenate the normalized and non-normalized features. By default, false.
    ///
    /// # Raises
    /// * The feature concatenation only makes sense if the normalization is enabled.
    pub fn new(
        normalize_by_symmetric_laplacian: Option<bool>,
        concatenate_features: Option<bool>,
    ) -> Result<Self, String> {
        if concatenate_features.unwrap_or(false)
            && !normalize_by_symmetric_laplacian.unwrap_or(false)
        {
            return Err(
                "The feature concatenation only makes sense if the normalization is enabled."
                    .to_string(),
            );
        }

        Ok(Self {
            counters: Vec::new(),
            normalize_by_symmetric_laplacian: normalize_by_symmetric_laplacian.unwrap_or(false),
            concatenate_features: concatenate_features.unwrap_or(false),
        })
    }

    fn must_be_trained(&self) -> Result<(), String> {
        if self.counters.is_empty() {
            return Err(concat!(
                "This model has not been trained yet. ",
                "You should call the `.fit` method first."
            )
            .to_string());
        }
        Ok(())
    }

    /// Fit the HyperBall model to the provided graph.
    ///
    /// # Arguments
    /// * `graph`: &Graph - The graph whose edges are to be learned.
    pub fn fit(&mut self, graph: &Graph) -> Result<(), String> {
        // Create HyperLogLog counters for all nodes in the graph
        let mut counters = graph
            .par_iter_node_ids()
            .map(|node_id| {
                let mut counters = HyperLogLogArray::<PRECISION, BITS, HOPS>::new();
                unsafe {
                    counters[0] = graph
                        .iter_unchecked_neighbour_node_ids_from_source_node_id(node_id)
                        .collect();
                }
                counters
            })
            .collect::<Vec<_>>();

        let shared_counters = SyncUnsafeCell::new(&mut counters);

        // Iterate over all hops and update the counters accordingly
        (1..HOPS).for_each(|k| unsafe {
            // Iterate over all nodes
            (*shared_counters.get())
                .par_iter_mut()
                .enumerate()
                .for_each(|(node_id, counters)| {
                    // Iterate over all neighbors of the current node
                    counters[k] = graph
                        .iter_unchecked_neighbour_node_ids_from_source_node_id(node_id as NodeT)
                        .map(|dst| &(*shared_counters.get())[dst as usize][k - 1])
                        .union()
                        | counters[k - 1];
                });
        });

        self.counters = counters;

        Ok(())
    }

    /// Returns the estimated exclusive overlap cardinality between two nodes.
    ///
    /// # Arguments
    /// * `src`: NodeT - The source node.
    /// * `dst`: NodeT - The destination node.
    ///
    /// # Safety
    /// This method is unsafe because it does not check that the provided nodes are lower
    /// than the expected number of nodes in the graph.
    pub unsafe fn get_overlap_cardinalities_from_node_ids_unchecked(
        &self,
        src: usize,
        dst: usize,
    ) -> [[f32; HOPS]; HOPS] {
        self.counters[src].estimate_overlap_cardinalities(&self.counters[dst])
    }

    /// Returns the estimated exclusive overlap cardinality between two nodes.
    ///
    /// # Arguments
    /// * `src`: NodeT - The source node.
    /// * `dst`: NodeT - The destination node.
    ///
    /// # Raises
    /// * If the model has not been trained yet.
    /// * If the provided nodes are not lower than the expected number of nodes in the graph.
    pub fn get_overlap_cardinalities_from_node_ids(
        &self,
        src: usize,
        dst: usize,
    ) -> Result<[[f32; HOPS]; HOPS], String> {
        // Check that the model has been trained
        self.must_be_trained()?;

        // We check whether the two provided nodes are lower
        // than the expected number of nodes in the graph
        if src >= self.counters.len() || dst >= self.counters.len() {
            return Err(format!(
                concat!(
                    "The provided nodes {} and {} are not lower than the ",
                    "expected number of nodes in the graph `{}`."
                ),
                src,
                dst,
                self.counters.len()
            ));
        }

        Ok(unsafe { self.get_overlap_cardinalities_from_node_ids_unchecked(src, dst) })
    }

    /// Returns the estimated exclusive differences cardinality between two nodes.
    ///
    /// # Arguments
    /// * `src`: NodeT - The source node.
    /// * `dst`: NodeT - The destination node.
    ///
    /// # Safety
    /// This method is unsafe because it does not check that the provided nodes are lower
    /// than the expected number of nodes in the graph.
    pub unsafe fn get_difference_cardinalities_from_node_ids_unchecked(
        &self,
        src: usize,
        dst: usize,
    ) -> [f32; HOPS] {
        self.counters[src].estimated_difference_cardinality_vector(&self.counters[dst][HOPS - 1])
    }

    /// Returns the estimated exclusive differences cardinality between two nodes.
    ///
    /// # Arguments
    /// * `src`: NodeT - The source node.
    /// * `dst`: NodeT - The destination node.
    ///
    /// # Raises
    /// * If the model has not been trained yet.
    /// * If the provided nodes are not lower than the expected number of nodes in the graph.
    ///
    pub fn get_difference_cardinalities_from_node_ids(
        &self,
        src: usize,
        dst: usize,
    ) -> Result<[f32; HOPS], String> {
        // Check that the model has been trained
        self.must_be_trained()?;

        // We check whether the two provided nodes are lower
        // than the expected number of nodes in the graph
        if src >= self.counters.len() || dst >= self.counters.len() {
            return Err(format!(
                concat!(
                    "The provided nodes {} and {} are not lower than the ",
                    "expected number of nodes in the graph `{}`."
                ),
                src,
                dst,
                self.counters.len()
            ));
        }

        Ok(unsafe { self.get_difference_cardinalities_from_node_ids_unchecked(src, dst) })
    }

    /// Returns the subgraph sketch associates with the two provided nodes.
    ///
    /// # Arguments
    /// * `src`: NodeT - The source node.
    /// * `dst`: NodeT - The destination node.
    ///
    /// # Returns
    /// The subgraph sketch is composed of the following features:
    /// * The estimated exclusive overlap cardinality between the two nodes, which is a matrix of shape `(HOPS, HOPS)`.
    /// * The estimated exclusive differences cardinality between the src vector and the last element of the dst vector, which is a vector of shape `(HOPS,)`.
    /// * The estimated exclusive differences cardinality between the dst vector and the last element of the src vector, which is a vector of shape `(HOPS,)`.
    ///
    /// # Safety
    /// This method is unsafe because it does not check that the provided nodes are lower
    /// than the expected number of nodes in the graph.
    ///
    pub unsafe fn get_subgraph_sketch_from_node_ids_unchecked(
        &self,
        src: usize,
        dst: usize,
    ) -> ([[f32; HOPS]; HOPS], [f32; HOPS], [f32; HOPS]) {
        self.counters[src]
            .estimated_overlap_and_differences_cardinality_matrices(&self.counters[dst])
    }

    /// Returns the subgraph sketch associates with the two provided nodes.
    ///
    /// # Arguments
    /// * `src`: NodeT - The source node.
    /// * `dst`: NodeT - The destination node.
    ///
    /// # Returns
    /// The subgraph sketch is composed of the following features:
    /// * The estimated exclusive overlap cardinality between the two nodes, which is a matrix of shape `(HOPS, HOPS)`.
    /// * The estimated exclusive differences cardinality between the src vector and the last element of the dst vector, which is a vector of shape `(HOPS,)`.
    /// * The estimated exclusive differences cardinality between the dst vector and the last element of the src vector, which is a vector of shape `(HOPS,)`.
    ///
    /// # Raises
    /// * If the model has not been trained yet.
    /// * If the provided nodes are not lower than the expected number of nodes in the graph.
    ///
    pub fn get_subgraph_sketch_from_node_ids(
        &self,
        src: usize,
        dst: usize,
    ) -> Result<([[f32; HOPS]; HOPS], [f32; HOPS], [f32; HOPS]), String> {
        // Check that the model has been trained
        self.must_be_trained()?;

        // We check whether the two provided nodes are lower
        // than the expected number of nodes in the graph
        if src >= self.counters.len() || dst >= self.counters.len() {
            return Err(format!(
                concat!(
                    "The provided nodes {} and {} are not lower than the ",
                    "expected number of nodes in the graph `{}`."
                ),
                src,
                dst,
                self.counters.len()
            ));
        }

        Ok(unsafe { self.get_subgraph_sketch_from_node_ids_unchecked(src, dst) })
    }

    /// Returns whether the features will be normalized using the symmetric Laplacian.
    pub fn get_normalize_by_symmetric_laplacian(&self) -> bool {
        self.normalize_by_symmetric_laplacian
    }

    /// Returns whether the features will be concatenated.
    pub fn get_concatenate_features(&self) -> bool {
        self.concatenate_features
    }

    /// Return the number of hops.
    pub fn get_number_of_hops(&self) -> usize {
        HOPS
    }

    /// Return the precision used for the HyperLogLog counters.
    pub fn get_precision(&self) -> usize {
        PRECISION
    }

    /// Return the number of bits used for the HyperLogLog counters.
    pub fn get_bits(&self) -> usize {
        BITS
    }

    /// Returns the estimated Sketching for all edges.
    ///
    /// # Arguments
    /// * `overlaps`: &mut [f32] - Area where to write the estimated overlaps, which is expected to be a flattened version of the 3d matrix with shape `(number_of_edges, HOPS, HOPS)`.
    /// * `src_differences`: &mut [f32] - Area where to write the estimated src differences, which is expected to be a flattened version of the 2d matrix with shape `(number_of_edges, HOPS)`.
    /// * `dst_differences`: &mut [f32] - Area where to write the estimated dst differences, which is expected to be a flattened version of the 2d matrix with shape `(number_of_edges, HOPS)`.
    /// * `graph`: &Graph - The graph whose edges are to be learned.
    ///
    /// # Raises
    /// * If the model has not been trained yet.
    /// * If one of the provided slices does not have the expected size.
    /// * If the provided graph has a different number of nodes than the model.
    ///
    pub fn get_sketching_for_all_edges(
        &self,
        overlaps: &mut [f32],
        src_differences: &mut [f32],
        dst_differences: &mut [f32],
        graph: &Graph,
    ) -> Result<(), String> {
        // Check that the model has been trained
        self.must_be_trained()?;

        // Check that the provided slices have the expected size
        if overlaps.len() != graph.get_number_of_edges() as usize * HOPS * HOPS {
            return Err(format!(
                concat!(
                    "The provided `overlaps` slice has a length of `{}` ",
                    "but it should have a length of `{}`."
                ),
                overlaps.len(),
                graph.get_number_of_edges() as usize * HOPS * HOPS
            ));
        }

        if src_differences.len() != graph.get_number_of_edges() as usize * HOPS {
            return Err(format!(
                concat!(
                    "The provided `src_differences` slice has a length of `{}` ",
                    "but it should have a length of `{}`."
                ),
                src_differences.len(),
                graph.get_number_of_edges() as usize * HOPS
            ));
        }

        if dst_differences.len() != graph.get_number_of_edges() as usize * HOPS {
            return Err(format!(
                concat!(
                    "The provided `dst_differences` slice has a length of `{}` ",
                    "but it should have a length of `{}`."
                ),
                dst_differences.len(),
                graph.get_number_of_edges() as usize * HOPS
            ));
        }

        // Check that the graph has the same number of nodes as the model
        if graph.get_number_of_nodes() as usize != self.counters.len() {
            return Err(format!(
                concat!(
                    "The provided graph has `{}` nodes ",
                    "but the model has been trained on a graph with `{}` nodes."
                ),
                graph.get_number_of_nodes(),
                self.counters.len()
            ));
        }

        // Iterate over all edges in the graph and compute the sketches
        // We zip the three slices together to have better cache locality
        // and we use copy non overlapping, which is an instruction to the
        // compiler to not assume that the slices are overlapping.

        graph
            .par_iter_directed_edge_node_ids()
            .zip(overlaps.par_chunks_exact_mut(
                HOPS * HOPS * (if self.concatenate_features { 2 } else { 1 }),
            ))
            .zip(
                src_differences
                    .par_chunks_exact_mut(HOPS * (if self.concatenate_features { 2 } else { 1 })),
            )
            .zip(
                dst_differences
                    .par_chunks_exact_mut(HOPS * (if self.concatenate_features { 2 } else { 1 })),
            )
            .for_each(
                |((((_, src, dst), overlaps), src_differences), dst_differences)| unsafe {
                    let (sketch_overlaps, sketch_src_differences, sketch_dst_differences) = self
                        .get_subgraph_sketch_from_node_ids_unchecked(src as usize, dst as usize);

                    // Copy the estimated overlaps
                    std::ptr::copy_nonoverlapping(
                        sketch_overlaps.as_ptr() as *const f32,
                        overlaps.as_mut_ptr(),
                        HOPS * HOPS,
                    );

                    // Copy the estimated src differences
                    std::ptr::copy_nonoverlapping(
                        sketch_src_differences.as_ptr(),
                        src_differences.as_mut_ptr(),
                        HOPS,
                    );

                    // Copy the estimated dst differences
                    std::ptr::copy_nonoverlapping(
                        sketch_dst_differences.as_ptr(),
                        dst_differences.as_mut_ptr(),
                        HOPS,
                    );
                },
            );

        // Normalize the features by the symmetric Laplacian, which is defined as
        // dividing all values in all edges by the square root of the product of the
        // degrees of the two nodes.

        if self.normalize_by_symmetric_laplacian {
            graph
                .par_iter_directed_edge_node_ids()
                .zip(overlaps.par_chunks_exact_mut(
                    HOPS * HOPS * (if self.concatenate_features { 2 } else { 1 }),
                ))
                .zip(
                    src_differences.par_chunks_exact_mut(
                        HOPS * (if self.concatenate_features { 2 } else { 1 }),
                    ),
                )
                .zip(
                    dst_differences.par_chunks_exact_mut(
                        HOPS * (if self.concatenate_features { 2 } else { 1 }),
                    ),
                )
                .for_each(
                    |((((_, src, dst), overlaps), src_differences), dst_differences)| unsafe {
                        let src_degree = graph.get_unchecked_node_degree_from_node_id(src) as f32;
                        let dst_degree = graph.get_unchecked_node_degree_from_node_id(dst) as f32;

                        let degree_sqrt_recip = (src_degree * dst_degree).sqrt().recip();

                        // Normalize the estimated overlaps
                        for i in 0..HOPS {
                            for j in 0..HOPS {
                                overlaps[HOPS * HOPS + i * HOPS + j] =
                                    overlaps[i * HOPS + j] * degree_sqrt_recip;
                            }
                            src_differences[HOPS + i] = src_differences[i] * degree_sqrt_recip;
                            dst_differences[HOPS + i] = dst_differences[i] * degree_sqrt_recip;
                        }
                    },
                );
        }

        Ok(())
    }

    pub fn dump(&self, path: &str) -> Result<(), String> {
        serde_json::to_writer(
            std::fs::File::create(path).map_err(|e| e.to_string())?,
            self,
        )
        .map_err(|e| e.to_string())?;
        Ok(())
    }

    pub fn dumps(&self) -> Result<String, String> {
        serde_json::to_string(self).map_err(|e| e.to_string())
    }

    pub fn load(path: &str) -> Result<Self, String> {
        serde_json::from_reader(std::fs::File::open(path).map_err(|e| e.to_string())?)
            .map_err(|e| e.to_string())
    }

    pub fn loads(json: &str) -> Result<Self, String> {
        serde_json::from_str(json).map_err(|e| e.to_string())
    }
}
