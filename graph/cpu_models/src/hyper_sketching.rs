use std::cell::SyncUnsafeCell;

use graph::{Graph, NodeT};
use heterogeneous_graphlets::prelude::*;
use hyperloglog_rs::prelude::*;
use num_traits::Float;
use rayon::prelude::*;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use std::collections::HashMap;
use vec_rand::{splitmix64, xorshift};

#[derive(Clone, Deserialize, Serialize)]
/// Struct implementing Hyper Subgraph Sketching.
///
/// # Implementation details
///
/// # References
/// The original paper describing this approach for edge prediction
/// feature mining is "Graph Neural Networks for Link Prediction with Subgraph sketching"
///
pub struct HyperSketching<PRECISION: Precision<BITS>, const BITS: usize, const HOPS: usize> {
    /// Vector of HyperLogLog counters
    counters: Vec<HyperLogLogArray<PRECISION, BITS, HOPS>>,
    /// Whether to include the node types in the sketch
    include_node_types: bool,
    /// whether to include the edge types in the sketch
    include_edge_types: bool,
    /// whether to include the edge ids in the sketch
    include_edge_ids: bool,
    /// whether to include the node ids in the sketch
    include_node_ids: bool,
    /// whether to include self-loops.
    include_selfloops: bool,
    /// whether to include the typed graphlets in the sketch
    include_typed_graphlets: bool,
    /// Random state for random integers, if requested.
    random_state: u64,
    /// Number of random integers to add per node - by default 0.
    number_of_random_integers: usize,
    /// Normalize by symmetric Laplacian
    normalize_by_symmetric_laplacian: bool,
    /// Concatenate the normalized and non-normalized features
    concatenate_features: bool,
    /// The embedding data type.
    dtype: String,
}

impl<PRECISION: Precision<BITS> + DeserializeOwned, const BITS: usize, const HOPS: usize>
    HyperSketching<PRECISION, BITS, HOPS>
{
    /// Creates a new HyperSketching model.
    ///
    /// # Arguments
    /// * `include_node_types`: Option<bool> - Whether to include the node types in the sketch. By default, false.
    /// * `include_edge_types`: Option<bool> - Whether to include the edge types in the sketch. By default, false.
    /// * `include_edge_ids`: Option<bool> - Whether to include the edge ids in the sketch. By default, false.
    /// * `include_node_ids`: Option<bool> - Whether to include the node ids in the sketch. By default, true.
    /// * `include_selfloops`: Option<bool> - Whether to include self-loops. By default, true.
    /// * `include_typed_graphlets`: Option<bool> - Whether to include the typed graphlets in the sketch. By default, false.
    /// * `random_state`: Option<u64> - Random state for random integers, if requested. By default, 42.
    /// * `number_of_random_integers`: Option<usize> - Number of random integers to add per node - by default 0.
    /// * `normalize_by_symmetric_laplacian`: Option<bool> - Whether to normalize the Sketching cardinalities by the symmetric Laplacian. By default, false.
    /// * `concatenate_features`: Option<bool> - Whether to concatenate the normalized and non-normalized features. By default, false.
    /// * `dtype`: Option<String> - The data type to be employed, by default f32.
    ///
    /// # Raises
    /// * The feature concatenation only makes sense if the normalization is enabled.
    /// * If none of the include parameters is set to true.
    /// * If the edge ids are requested, but only two HOPs is used, as the edge ids would surely be completely distinct for all edges.
    /// * The data type is not supported. Supported data types are f16, f32 and f64.
    pub fn new(
        include_node_types: Option<bool>,
        include_edge_types: Option<bool>,
        include_edge_ids: Option<bool>,
        include_node_ids: Option<bool>,
        include_selfloops: Option<bool>,
        include_typed_graphlets: Option<bool>,
        random_state: Option<u64>,
        number_of_random_integers: Option<usize>,
        normalize_by_symmetric_laplacian: Option<bool>,
        concatenate_features: Option<bool>,
        dtype: Option<String>,
    ) -> Result<Self, String> {
        if concatenate_features.unwrap_or(false)
            && !normalize_by_symmetric_laplacian.unwrap_or(false)
        {
            return Err(
                "The feature concatenation only makes sense if the normalization is enabled."
                    .to_string(),
            );
        }

        if !include_node_types.unwrap_or(false)
            && !include_edge_types.unwrap_or(false)
            && !include_edge_ids.unwrap_or(false)
            && !include_node_ids.unwrap_or(true)
            && !include_selfloops.unwrap_or(true)
            && number_of_random_integers.unwrap_or(0) == 0
            && !include_typed_graphlets.unwrap_or(false)
        {
            return Err("At least one of the include parameters must be set to true.".to_string());
        }

        if include_edge_ids.unwrap_or(false) && HOPS == 2 {
            return Err(concat!(
                "You requested to include the edge ids in the sketch, ",
                "but also built this model so that only one hop is used. ",
                "This means that the edge ids would surely be completely distinct for all nodes ",
                "as with a single hop there would be no overlap between the edges. ",
            )
            .to_string());
        }

        if !["f16", "f32", "f64"].contains(&dtype.as_ref().unwrap_or(&"f32".to_string()).as_str()) {
            return Err(format!(
                concat!(
                    "The data type `{}` is not supported. ",
                    "Supported data types are f16, f32 and f64."
                ),
                dtype.as_ref().unwrap_or(&"f32".to_string())
            ));
        }

        Ok(Self {
            counters: Vec::new(),
            include_node_types: include_node_types.unwrap_or(false),
            include_edge_types: include_edge_types.unwrap_or(false),
            include_edge_ids: include_edge_ids.unwrap_or(false),
            include_node_ids: include_node_ids.unwrap_or(true),
            include_selfloops: include_selfloops.unwrap_or(true),
            include_typed_graphlets: include_typed_graphlets.unwrap_or(false),
            random_state: random_state.unwrap_or(42),
            number_of_random_integers: number_of_random_integers.unwrap_or(0),
            normalize_by_symmetric_laplacian: normalize_by_symmetric_laplacian.unwrap_or(false),
            concatenate_features: concatenate_features.unwrap_or(false),
            dtype: dtype.unwrap_or("f32".to_string()),
        })
    }

    /// Returns whether the model has been trained.
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
    ///
    /// # Raises
    /// * If the provided graph does not have node types but the model has been initialized with `include_node_types` set to true.
    /// * If the provided graph does not have edge types but the model has been initialized with `include_edge_types` set to true.
    pub fn fit(&mut self, graph: &Graph) -> Result<(), String> {
        // Check that the graph has node types if the model is initialized with `include_node_types` set to true
        if self.include_node_types && !graph.has_node_types() {
            return Err(
                "The provided graph does not have node types but the model has been initialized with `include_node_types` set to true.".to_string(),
            );
        }

        // Check that the graph has edge types if the model is initialized with `include_edge_types` set to true
        if self.include_edge_types && !graph.has_edge_types() {
            return Err(
                "The provided graph does not have edge types but the model has been initialized with `include_edge_types` set to true.".to_string(),
            );
        }

        let random_state = splitmix64(self.random_state);

        // We add an offset to the node ids if they are requested.
        let node_id_offset = if self.include_node_ids {
            graph.get_number_of_nodes() as usize
        } else {
            0
        };

        // We add an offset to the edge ids if they are requested.
        let edge_id_offset = node_id_offset
            + if self.include_edge_ids {
                graph.get_number_of_edges() as usize
            } else {
                0
            };

        // We add an offset to the node types so that there won't be any collisions
        // with the node ids or edge type ids.
        let node_type_offset = edge_id_offset
            + if self.include_node_types {
                graph.get_number_of_node_types()? as usize
            } else {
                0
            };

        // We add an offset to the edge types so that there won't be any collisions
        // with the node ids or node type ids.
        let edge_type_offset = node_type_offset
            + if self.include_edge_types {
                graph.get_number_of_edge_types()? as usize
            } else {
                0
            };

        let mut counters = vec![
            HyperLogLogArray::<PRECISION, BITS, HOPS>::new();
            graph.get_number_of_nodes() as usize
        ];

        // Create HyperLogLog counters for all nodes in the graph
        counters
            .par_iter_mut()
            .enumerate()
            .for_each(|(node_id, counters)| {
                // If the self-loops are requested, we add the node id itself to the counter.
                // It may happen that the node id ALSO has actual self-loop, but as the counter
                // counts the unique appereaances, it will not be a problem.
                if self.include_selfloops {
                    counters[0].insert(node_id);
                }
                // If the node neighbours are requested, we add the node neighbour node ids.
                if self.include_node_ids {
                    counters[0] |= unsafe {
                        graph
                            .iter_unchecked_neighbour_node_ids_from_source_node_id(node_id as NodeT)
                    }
                    .collect::<HyperLogLog<PRECISION, BITS>>();
                }
                if self.include_edge_ids {
                    counters[0] |= unsafe {
                        graph.iter_unchecked_edge_ids_from_source_node_id(node_id as NodeT)
                    }
                    .map(|edge_id| edge_id as usize + node_id_offset)
                    .collect::<HyperLogLog<PRECISION, BITS>>();
                }
                if self.include_node_types {
                    counters[0] |= unsafe {
                        graph
                            .iter_unchecked_neighbour_node_ids_from_source_node_id(node_id as NodeT)
                    }
                    .flat_map(|dst| {
                        unsafe { graph.get_unchecked_node_type_ids_from_node_id(dst) }
                            .unwrap_or(&[])
                    })
                    .map(|&node_type_id| node_type_id as usize + edge_id_offset)
                    .collect::<HyperLogLog<PRECISION, BITS>>();
                }
                if self.include_edge_types {
                    counters[0] |= unsafe {
                        graph.iter_unchecked_edge_type_id_from_source_node_id(node_id as NodeT)
                    }
                    .filter_map(|edge_type_id| edge_type_id)
                    .map(|edge_type_id| edge_type_id as usize + node_id_offset)
                    .collect::<HyperLogLog<PRECISION, BITS>>();
                }
                if self.include_typed_graphlets {
                    counters[0] |= unsafe {
                        graph
                            .iter_unchecked_neighbour_node_ids_from_source_node_id(node_id as NodeT)
                    }
                    .flat_map(|dst| {
                        let graphlets: HashMap<u16, u32> =
                            graph.get_heterogeneous_graphlet(node_id, dst as usize);
                        graphlets.into_keys()
                    })
                    .map(|node_type_id| node_type_id as usize + edge_type_offset)
                    .collect::<HyperLogLog<PRECISION, BITS>>();
                }
                if self.number_of_random_integers {
                    let mut random_state =
                        splitmix64(random_state.wrapping_mul(node_id as u64 + 1));
                    counters[0] |= (0..self.number_of_random_integers)
                        .map(|&random_integer| {
                            random_state = xorshift(random_state);
                            random_state
                        })
                        .collect::<HyperLogLog<PRECISION, BITS>>();
                }
            });

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
    pub unsafe fn get_subgraph_sketch_from_node_ids_unchecked<F: Primitive<f32>>(
        &self,
        src: usize,
        dst: usize,
    ) -> ([[F; HOPS]; HOPS], [F; HOPS], [F; HOPS]) {
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
    pub fn get_subgraph_sketch_from_node_ids<F: Primitive<f32>>(
        &self,
        src: usize,
        dst: usize,
    ) -> Result<([[F; HOPS]; HOPS], [F; HOPS], [F; HOPS]), String> {
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
        PRECISION::EXPONENT
    }

    /// Return the number of bits used for the HyperLogLog counters.
    pub fn get_bits(&self) -> usize {
        BITS
    }

    /// Returns the dtype.
    pub fn get_dtype(&self) -> &str {
        &self.dtype
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
    pub fn get_sketching_for_all_edges<I, F: Primitive<f32> + Float>(
        &self,
        overlaps: &mut [F],
        src_differences: &mut [F],
        dst_differences: &mut [F],
        graph: &Graph,
        edge_iterator: I,
    ) -> Result<(), String>
    where
        I: IndexedParallelIterator<Item = (NodeT, NodeT)>,
    {
        // Check that the model has been trained
        self.must_be_trained()?;
        let factor = if self.concatenate_features { 2 } else { 1 };

        // Check that the provided slices have the expected size
        if overlaps.len() != edge_iterator.len() as usize * factor * HOPS * HOPS {
            return Err(format!(
                concat!(
                    "The provided `overlaps` slice has a length of `{}` ",
                    "but it should have a length of `{}`."
                ),
                overlaps.len(),
                edge_iterator.len() as usize * factor * HOPS * HOPS
            ));
        }

        if src_differences.len() != edge_iterator.len() as usize * factor * HOPS {
            return Err(format!(
                concat!(
                    "The provided `src_differences` slice has a length of `{}` ",
                    "but it should have a length of `{}`."
                ),
                src_differences.len(),
                edge_iterator.len() as usize * factor * HOPS
            ));
        }

        if dst_differences.len() != edge_iterator.len() as usize * factor * HOPS {
            return Err(format!(
                concat!(
                    "The provided `dst_differences` slice has a length of `{}` ",
                    "but it should have a length of `{}`."
                ),
                dst_differences.len(),
                edge_iterator.len() as usize * factor * HOPS
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

        let offset = if self.concatenate_features { 1 } else { 0 };

        edge_iterator
            .zip(overlaps.par_chunks_exact_mut(HOPS * HOPS * factor))
            .zip(src_differences.par_chunks_exact_mut(HOPS * factor))
            .zip(dst_differences.par_chunks_exact_mut(HOPS * factor))
            .map(
                |((((src, dst), overlaps), src_differences), dst_differences)| unsafe {
                    // If the source or destination node is not in the graph, we return an error:
                    if src as usize >= self.counters.len() || dst as usize >= self.counters.len() {
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

                    let (
                        mut sketch_overlaps,
                        mut sketch_src_differences,
                        mut sketch_dst_differences,
                    ) = self
                        .get_subgraph_sketch_from_node_ids_unchecked(src as usize, dst as usize);

                    // Copy the estimated overlaps
                    std::ptr::copy_nonoverlapping(
                        sketch_overlaps.as_ptr() as *const F,
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

                    if self.normalize_by_symmetric_laplacian {
                        let src_degree: F = F::reverse(
                            1.0 + graph
                                .get_unchecked_selfloop_excluded_node_degree_from_node_id(src)
                                as f32,
                        );
                        let dst_degree: F = F::reverse(
                            1.0 + graph
                                .get_unchecked_selfloop_excluded_node_degree_from_node_id(dst)
                                as f32,
                        );

                        let degree_sqrt_recip = (src_degree * dst_degree).sqrt().recip();

                        // Normalize the estimated overlaps
                        sketch_overlaps.iter_mut().for_each(|overlap| {
                            overlap.iter_mut().for_each(|overlap| {
                                *overlap *= degree_sqrt_recip;
                            });
                        });
                        sketch_src_differences
                            .iter_mut()
                            .for_each(|src_difference| {
                                *src_difference *= degree_sqrt_recip;
                            });
                        sketch_dst_differences
                            .iter_mut()
                            .for_each(|dst_difference| {
                                *dst_difference *= degree_sqrt_recip;
                            });

                        // Copy the estimated overlaps
                        std::ptr::copy_nonoverlapping(
                            sketch_overlaps.as_ptr() as *const F,
                            overlaps[offset * HOPS * HOPS..].as_mut_ptr(),
                            HOPS * HOPS,
                        );

                        // Copy the estimated src differences
                        std::ptr::copy_nonoverlapping(
                            sketch_src_differences.as_ptr(),
                            src_differences[offset * HOPS..].as_mut_ptr(),
                            HOPS,
                        );

                        // Copy the estimated dst differences
                        std::ptr::copy_nonoverlapping(
                            sketch_dst_differences.as_ptr(),
                            dst_differences[offset * HOPS..].as_mut_ptr(),
                            HOPS,
                        );
                    }

                    Ok(())
                },
            )
            .collect::<Result<(), String>>()?;

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
        serde_json::from_reader(std::fs::File::open(path).map_err(move |e| e.to_string())?)
            .map_err(move |e| e.to_string())
    }

    pub fn loads(json: &str) -> Result<Self, String> {
        serde_json::from_str(json).map_err(|e| e.to_string())
    }
}
