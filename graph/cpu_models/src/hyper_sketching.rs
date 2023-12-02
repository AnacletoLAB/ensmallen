use std::cell::SyncUnsafeCell;

use core::hash::Hash;
use core::mem::MaybeUninit;
use graph::{Graph, NodeT};
use heterogeneous_graphlets::prelude::*;
use hyperloglog_rs::prelude::*;
use num_traits::Float;
use rayon::prelude::*;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use vec_rand::{splitmix64, xorshift};

struct Offsets {
    node_id_offset: usize,
    edge_id_offset: usize,
    node_type_offset: usize,
    edge_type_offset: usize,
}

// Method to allocate an array of HashSets using maybe uninitialized memory,
// so to circumvent the fact that HashSet does not implement Copy.
fn allocate_array_of_hashsets<const N: usize, T>() -> [HashSet<T>; N] {
    unsafe {
        let mut array: [HashSet<T>; N] = MaybeUninit::uninit().assume_init();
        for i in 0..N {
            // We replace the previosly initialized value with an hashset
            // and we forget the previous value.
            std::mem::forget(std::mem::replace(&mut array[i], HashSet::new()));
        }
        array
    }
}

trait MutableSetLike<T>: Sized {
    fn array<const HOPS: usize>() -> [Self; HOPS];
    fn insert(&mut self, value: T) -> bool;
}

impl<T: Hash + Eq> MutableSetLike<T> for HashSet<T> {
    fn array<const HOPS: usize>() -> [Self; HOPS] {
        allocate_array_of_hashsets()
    }

    fn insert(&mut self, value: T) -> bool {
        HashSet::insert(self, value)
    }
}

impl<T: Hash + Eq, P, const BITS: usize> MutableSetLike<T> for HyperLogLog<P, BITS>
where
    P: Precision + WordType<BITS>,
{
    fn array<const HOPS: usize>() -> [Self; HOPS] {
        [HyperLogLog::default(); HOPS]
    }

    fn insert(&mut self, value: T) -> bool {
        let contained_before = HyperLogLog::may_contain(self, &value);
        HyperLogLog::insert(self, value);
        !contained_before
    }
}

#[derive(Clone, Deserialize, Serialize)]
/// Struct implementing Hyper Subgraph Sketching.
///
/// # Implementation details
///
/// # References
/// The original paper describing this approach for edge prediction
/// feature mining is "Graph Neural Networks for Link Prediction with Subgraph sketching"
///
pub struct HyperSketching<
    PRECISION: Precision + WordType<BITS>,
    const BITS: usize,
    const HOPS: usize,
> {
    /// Vector of HyperLogLog counters
    counters: Vec<HyperLogLogArray<PRECISION, BITS, HOPS>>,
    /// Whether to use the unbiased version for the algorithm.
    unbiased: bool,
    /// Whether to use the exact version for the algorithm.
    exact: bool,
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

impl<
        PRECISION: Precision + WordType<BITS> + DeserializeOwned,
        const BITS: usize,
        const HOPS: usize,
    > HyperSketching<PRECISION, BITS, HOPS>
{
    /// Creates a new HyperSketching model.
    ///
    /// # Arguments
    /// * `unbiased`: Option<bool> - Whether to use the unbiased version for the algorithm. By default, false.
    /// * `exact`: Option<bool> - Whether to use the exact version for the algorithm. By default, false.
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
        unbiased: Option<bool>,
        exact: Option<bool>,
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

        let unbiased = unbiased.unwrap_or(false);
        let exact = exact.unwrap_or(false);

        // At this time, we do not support the exact or unbiased version of the algorithm
        // that uses the node types, edge types, edge ids or graphlets. The node ids MUST
        // be included, as otherwise the algorithm would not make sense.

        if (unbiased || exact)
            && (include_node_types.unwrap_or(false)
                || include_edge_types.unwrap_or(false)
                || include_edge_ids.unwrap_or(false)
                || include_typed_graphlets.unwrap_or(false))
        {
            return Err(concat!(
                "At this time, we do not support the exact or unbiased version of the algorithm ",
                "that uses the node types, edge types, edge ids or graphlets. ",
                "The node ids MUST be included, as otherwise the algorithm would not make sense."
            )
            .to_string());
        }

        Ok(Self {
            counters: Vec::new(),
            unbiased,
            exact,
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
        if !(self.unbiased || self.exact) && self.counters.is_empty() {
            return Err(concat!(
                "This model has not been trained yet. ",
                "You should call the `.fit` method first."
            )
            .to_string());
        }
        Ok(())
    }

    fn get_offsets(&self, support: &Graph) -> Result<Offsets, String> {
        // We add an offset to the node ids if they are requested.
        let node_id_offset = if self.include_node_ids {
            support.get_number_of_nodes() as usize
        } else {
            0
        };

        // We add an offset to the edge ids if they are requested.
        let edge_id_offset = node_id_offset
            + if self.include_edge_ids {
                support.get_number_of_edges() as usize
            } else {
                0
            };

        // We add an offset to the node types so that there won't be any collisions
        // with the node ids or edge type ids.
        let node_type_offset = edge_id_offset
            + if self.include_node_types {
                support.get_number_of_node_types()? as usize
            } else {
                0
            };

        // We add an offset to the edge types so that there won't be any collisions
        // with the node ids or node type ids.
        let edge_type_offset = node_type_offset
            + if self.include_edge_types {
                support.get_number_of_edge_types()? as usize
            } else {
                0
            };

        Ok(Offsets {
            node_id_offset,
            edge_id_offset,
            node_type_offset,
            edge_type_offset,
        })
    }

    /// Fit the HyperBall model to the provided support.
    ///
    /// # Arguments
    /// * `support`: &Graph - The graph whose edges are to be learned.
    ///
    /// # Raises
    /// * If the provided graph does not have node types but the model has been initialized with `include_node_types` set to true.
    /// * If the provided graph does not have edge types but the model has been initialized with `include_edge_types` set to true.
    pub fn fit(&mut self, support: &Graph) -> Result<(), String> {
        // The unbiased and exact versions of the algorithm do not require training
        // as they are necessarily computed on the fly.
        if self.unbiased || self.exact {
            return Ok(());
        }

        // Check that the graph has node types if the model is initialized with `include_node_types` set to true
        if self.include_node_types && !support.has_node_types() {
            return Err(
                "The provided graph does not have node types but the model has been initialized with `include_node_types` set to true.".to_string(),
            );
        }

        // Check that the graph has edge types if the model is initialized with `include_edge_types` set to true
        if self.include_edge_types && !support.has_edge_types() {
            return Err(
                "The provided graph does not have edge types but the model has been initialized with `include_edge_types` set to true.".to_string(),
            );
        }

        let random_state = splitmix64(self.random_state);

        let offsets = self.get_offsets(support)?;

        let mut counters = vec![
            HyperLogLogArray::<PRECISION, BITS, HOPS>::new();
            support.get_number_of_nodes() as usize
        ];

        // Create HyperLogLog counters for all nodes in the graph
        counters.par_iter_mut().enumerate().for_each(
            |(node_id, counters): (usize, &mut HyperLogLogArray<PRECISION, BITS, HOPS>)| {
                // If the self-loops are requested, we add the node id itself to the counter.
                // It may happen that the node id ALSO has actual self-loop, but as the counter
                // counts the unique appereaances, it will not be a problem.
                if self.include_selfloops {
                    // The conversion to NodeT is essential, as the hash of the node id
                    // as usize and as NodeT is different.
                    counters[0].insert(node_id as NodeT);
                }
                // If the node neighbours are requested, we add the node neighbour node ids.
                if self.include_node_ids {
                    counters[0] |= unsafe {
                        support
                            .iter_unchecked_neighbour_node_ids_from_source_node_id(node_id as NodeT)
                    }
                    .collect::<HyperLogLog<PRECISION, BITS>>();
                }
                if self.include_edge_ids {
                    counters[0] |= unsafe {
                        support.iter_unchecked_edge_ids_from_source_node_id(node_id as NodeT)
                    }
                    .map(|edge_id| edge_id as usize + offsets.node_id_offset)
                    .collect::<HyperLogLog<PRECISION, BITS>>();
                }
                if self.include_node_types {
                    counters[0] |= unsafe {
                        support
                            .iter_unchecked_neighbour_node_ids_from_source_node_id(node_id as NodeT)
                    }
                    .flat_map(|dst| {
                        unsafe { support.get_unchecked_node_type_ids_from_node_id(dst) }
                            .unwrap_or(&[])
                    })
                    .map(|&node_type_id| node_type_id as usize + offsets.edge_id_offset)
                    .collect::<HyperLogLog<PRECISION, BITS>>();
                }
                if self.include_edge_types {
                    counters[0] |= unsafe {
                        support.iter_unchecked_edge_type_id_from_source_node_id(node_id as NodeT)
                    }
                    .filter_map(|edge_type_id| edge_type_id)
                    .map(|edge_type_id| edge_type_id as usize + offsets.node_type_offset)
                    .collect::<HyperLogLog<PRECISION, BITS>>();
                }
                if self.include_typed_graphlets {
                    counters[0] |= unsafe {
                        support
                            .iter_unchecked_neighbour_node_ids_from_source_node_id(node_id as NodeT)
                    }
                    .flat_map(|dst| {
                        let graphlets: HashMap<u16, u32> =
                            support.get_heterogeneous_graphlet(node_id, dst as usize);
                        graphlets.into_keys()
                    })
                    .map(|node_type_id| node_type_id as usize + offsets.edge_type_offset)
                    .collect::<HyperLogLog<PRECISION, BITS>>();
                }
                if self.number_of_random_integers > 0 {
                    let mut random_state =
                        splitmix64(random_state.wrapping_mul(node_id as u64 + 1));
                    counters[0] |= (0..self.number_of_random_integers)
                        .map(|_| {
                            random_state = xorshift(random_state);
                            random_state
                        })
                        .collect::<HyperLogLog<PRECISION, BITS>>();
                }
            },
        );

        let shared_counters = SyncUnsafeCell::new(&mut counters);

        // Iterate over all hops and update the counters accordingly
        (1..HOPS).for_each(|k| unsafe {
            // Iterate over all nodes
            (*shared_counters.get())
                .par_iter_mut()
                .enumerate()
                .for_each(
                    |(node_id, counters): (usize, &mut HyperLogLogArray<PRECISION, BITS, HOPS>)| {
                        // Iterate over all neighbors of the current node
                        counters[k] = support
                            .iter_unchecked_neighbour_node_ids_from_source_node_id(node_id as NodeT)
                            .map(|dst| &(*shared_counters.get())[dst as usize][k - 1])
                            .union()
                            | counters[k - 1];
                    },
                );
        });

        self.counters = counters;

        Ok(())
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
    /// than the expected number of nodes in the support.
    ///
    pub unsafe fn get_subgraph_sketch_from_node_ids_unchecked<F: Primitive<f32>>(
        &self,
        src: usize,
        dst: usize,
    ) -> ([[F; HOPS]; HOPS], [F; HOPS], [F; HOPS]) {
        self.counters[src].overlap_and_differences_cardinality_matrices(&self.counters[dst])
    }

    /// Returns the unbiased exact subgraph sketch associates with the two provided nodes.
    ///
    /// # Arguments
    /// * `src`: NodeT - The source node.
    /// * `dst`: NodeT - The destination node.
    /// * `support`: &Graph - The support graph from which to extract the topology.
    ///
    fn get_unbiased_edge_sketching_from_edge_node_ids<
        F: Primitive<f32>,
        S: HyperSpheresSketch<F> + MutableSetLike<NodeT> + Clone,
    >(
        &self,
        src: NodeT,
        dst: NodeT,
        support: &Graph,
    ) -> ([[F; HOPS]; HOPS], [F; HOPS], [F; HOPS]) {
        let mut src_array: [S; HOPS] = S::array::<HOPS>();
        let mut dst_array: [S; HOPS] = S::array::<HOPS>();

        for (root, array) in [(src, &mut src_array), (dst, &mut dst_array)] {
            if self.include_selfloops {
                array[0].insert(root);
            }

            let mut frontier: Vec<NodeT> = vec![root];

            // Then, we populate the hypersphere of neighbours up to the given number of hops.
            for i in 0..HOPS {
                if i > 0 {
                    array[i] = array[i - 1].clone();
                }

                let mut temporary_frontier = Vec::new();

                for node in frontier.drain(..) {
                    for neighbour in unsafe {
                        support.iter_unchecked_neighbour_node_ids_from_source_node_id(node)
                    } {
                        if self.unbiased
                            && ((node == src && neighbour == dst)
                                || (node == dst && neighbour == src && !support.is_directed()))
                        {
                            continue;
                        }

                        if !array[i].insert(neighbour) {
                            temporary_frontier.push(neighbour);
                        }
                    }
                }
                frontier = temporary_frontier;
            }
        }

        // Now, we can compute the overlap matrix.
        S::overlap_and_differences_cardinality_matrices(&src_array, &dst_array)
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
    /// * If the provided nodes are not lower than the expected number of nodes in the support.
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
    /// * `features`: &mut [f32] - Area where to write the estimated overlaps, which is expected to be a flat array.
    /// * `support`: &Graph - The support graph from which to extract the topology.
    /// * `edge_iterator`: I - The iterator over the edges for which to compute the Sketching.
    ///
    /// # Raises
    /// * If the model has not been trained yet.
    /// * If one of the provided slices does not have the expected size.
    /// * If the provided graph has a different number of nodes than the model.
    ///
    /// # Safety
    /// The source and destination nodes are not checked to be lower than the expected number of nodes in the graph
    /// because it would slow down the computation too much without significant benefits. Please do this check BEFORE
    /// calling this method.
    ///
    pub unsafe fn get_sketching_for_all_edges_unchecked<I, F: Primitive<f32> + Float>(
        &self,
        features: &mut [F],
        support: &Graph,
        edge_iterator: I,
    ) -> Result<(), String>
    where
        I: IndexedParallelIterator<Item = (NodeT, NodeT)>,
    {
        // Check that the model has been trained
        self.must_be_trained()?;
        let factor = if self.concatenate_features { 2 } else { 1 };

        // Check that the provided slices have the expected size
        if features.len() != edge_iterator.len() as usize * factor * (HOPS * HOPS + HOPS + HOPS) {
            return Err(format!(
                concat!(
                    "The provided `features` slice has a length of `{}` ",
                    "but it should have a length of `{}`."
                ),
                features.len(),
                edge_iterator.len() as usize * factor * (HOPS * HOPS + HOPS + HOPS)
            ));
        }

        // Check that the graph has the same number of nodes as the model
        if self.get_normalize_by_symmetric_laplacian()
            && support.get_number_of_nodes() as usize != self.counters.len()
        {
            return Err(format!(
                concat!(
                    "The provided graph has `{}` nodes ",
                    "but the model has been trained on a graph with `{}` nodes."
                ),
                support.get_number_of_nodes(),
                self.counters.len()
            ));
        }

        edge_iterator
            .zip(features.par_chunks_exact_mut((HOPS * HOPS + HOPS + HOPS) * factor))
            .for_each(|((src, dst), edge_feature)| unsafe {
                let (sketch_overlaps, sketch_src_differences, sketch_dst_differences) =
                    if self.unbiased {
                        if self.exact {
                            self.get_unbiased_edge_sketching_from_edge_node_ids::<F, HashSet<NodeT>>(src, dst, support)
                        } else {
                            self.get_unbiased_edge_sketching_from_edge_node_ids::<F, HyperLogLog<PRECISION, BITS>>(src, dst, support)
                        }
                    } else {
                        self.get_subgraph_sketch_from_node_ids_unchecked(src as usize, dst as usize)
                    };

                // Copy the estimated overlaps
                std::ptr::copy_nonoverlapping(
                    sketch_overlaps.as_ptr() as *const F,
                    edge_feature[..HOPS * HOPS].as_mut_ptr(),
                    HOPS * HOPS,
                );

                // Copy the estimated src differences
                std::ptr::copy_nonoverlapping(
                    sketch_src_differences.as_ptr(),
                    edge_feature[HOPS * HOPS..HOPS * HOPS + HOPS].as_mut_ptr(),
                    HOPS,
                );

                // Copy the estimated dst differences
                std::ptr::copy_nonoverlapping(
                    sketch_dst_differences.as_ptr(),
                    edge_feature[HOPS * HOPS + HOPS..].as_mut_ptr(),
                    HOPS,
                );

                if self.concatenate_features {
                    let (left, right) = edge_feature.split_at_mut(HOPS * HOPS + HOPS + HOPS);

                    // Copy the estimated overlaps
                    std::ptr::copy_nonoverlapping(
                        left.as_ptr(),
                        right.as_mut_ptr(),
                        HOPS * HOPS + HOPS + HOPS,
                    );
                }

                if self.normalize_by_symmetric_laplacian {
                    let src_degree: F = F::reverse(
                        1.0 + support.get_unchecked_selfloop_excluded_node_degree_from_node_id(src)
                            as f32,
                    );
                    let dst_degree: F = F::reverse(
                        1.0 + support.get_unchecked_selfloop_excluded_node_degree_from_node_id(dst)
                            as f32,
                    );

                    let degree_sqrt_recip = (src_degree * dst_degree).sqrt().recip();

                    // We iterate over the last HOPS^2 + 2*HOPS elements of the edge feature
                    // and we normalize them by the symmetric Laplacian
                    let offset = if self.concatenate_features {
                        HOPS * HOPS + HOPS + HOPS
                    } else {
                        0
                    };

                    edge_feature[offset..]
                        .iter_mut()
                        .for_each(|feature| *feature *= degree_sqrt_recip);
                }
            });

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
