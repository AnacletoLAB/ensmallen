//! A graph representation optimized for executing random walks on huge graphs.
use std::{intrinsics::unlikely, sync::atomic::AtomicU8};

use super::*;
use bitvec::prelude::*;
use elias_fano_rust::*;
use rayon::prelude::*;

/// A graph representation optimized for executing random walks on huge graphs.
#[derive(Clone, Debug)]
pub struct Graph {
    /// The main datastructure where all the edges are saved
    /// in the endoced form ((src << self.node_bits) | dst) this allows us to do almost every
    /// operation in O(1) without decompressing the data.
    pub(crate) edges: EliasFano,
    /// Optional vector of the weights of every edge.
    /// `weights[10]` return the weight of the edge with edge_id 10
    pub(crate) weights: Option<Vec<WeightT>>,
    /// Vocabulary that save the mappings from string to index of every node type
    pub(crate) node_types: Option<NodeTypeVocabulary>,
    // This is the next attribute that will be embedded inside of edges once
    // the first refactoring is done
    /// Vocabulary that save the mappings from string to index of every edge type
    pub(crate) edge_types: Option<EdgeTypeVocabulary>,
    /// Vocabulary that save the mappings from string to index of every node
    pub(crate) nodes: Vocabulary<NodeT>,

    /// How many bits are needed to save a node.
    pub(crate) node_bits: u8,
    /// The mask used to extract the dst value form an encoded edge.
    /// This is saved for speed sake. It's equivalent to (1 << self.node_bits) - 1;
    pub(crate) node_bit_mask: u64,

    /// if the graph is directed or undirected
    pub(crate) directed: bool,
    /// Graph name
    pub(crate) name: String,

    // /////////////////////////////////////////////////////////////////////////
    // Elias-Fano Caching related attributes
    // /////////////////////////////////////////////////////////////////////////
    /// Vector of destinations to execute fast walks if required.
    pub(crate) destinations: Option<Vec<NodeT>>,
    /// Vector of sources to execute fast link prediction sequences if required.
    pub(crate) sources: Option<Vec<NodeT>>,
    /// Vector of cumulative_node_degrees to execute fast walks if required.
    pub(crate) cumulative_node_degrees: Option<Vec<EdgeT>>,
    /// Option of Elias-Fano of unique sources.
    /// When it is None it means that ALL nodes are sources.
    pub(crate) unique_sources: Option<EliasFano>,
    /// Option of bitvec containing connected nodes.
    /// When it is None it means that ALL nodes are connected, i.e. not singleton or singletons with selfloops.
    pub(crate) connected_nodes: Option<BitVec<Lsb0, u8>>,
    /// Number of connected nodes in the graph.
    pub(crate) connected_nodes_number: NodeT,

    // /////////////////////////////////////////////////////////////////////////
    pub(crate) cache: ClonableUnsafeCell<PropertyCache>,
}

use std::string::ToString;
impl ToString for Graph {
    fn to_string(&self) -> String {
        self.textual_report()
    }
}

/// # Graph utility methods
impl Graph {
    /// Return new instance of a Graph object.
    ///
    /// # Arguments
    /// * `directed`: bool - Whether to build the graph as directed or undirected.
    /// * `nodes`: Vocabulary<NodeT> - The nodes vocabulary.
    /// * `node_types`: Option<NodeTypeVocabulary> - The optional node types vocabulary.
    /// * `edges`: EliasFano - The Elias-Fano data structure containing the adjacency metric.
    /// * `edge_types`: Option<EdgeTypeVocabulary> - The optional edge types vocabulary.
    /// * `weights`: Option<Vec<WeightT>> - The optional edge weights vector.
    /// * `may_have_singletons`: bool - Whether the graph may contain singletons.
    /// * `may_have_singleton_with_selfloops`: bool - Whether the graph may contain singleton with selfloops.
    /// * `name`: S - The name of the graph.
    pub(crate) fn new<S: Into<String>>(
        directed: bool,
        nodes: Vocabulary<NodeT>,
        node_types: Option<NodeTypeVocabulary>,
        edges: EliasFano,
        edge_types: Option<EdgeTypeVocabulary>,
        weights: Option<Vec<WeightT>>,
        may_have_singletons: bool,
        may_have_singleton_with_selfloops: bool,
        name: S,
    ) -> Graph {
        let nodes_number = nodes.len();
        let node_bits = get_node_bits(nodes_number as NodeT);
        let node_bit_mask = (1 << node_bits) - 1;
        let mut graph = Graph {
            directed,
            edges,
            node_bits,
            node_bit_mask,
            weights,
            node_types,
            edge_types,
            nodes,
            sources: None,
            destinations: None,
            cumulative_node_degrees: None,
            name: name.into(),
            cache: ClonableUnsafeCell::default(),
            unique_sources: None,
            connected_nodes: None,
            connected_nodes_number: nodes_number as NodeT,
        };
        if may_have_singletons || may_have_singleton_with_selfloops {
            let connected_nodes =
                graph.get_connected_nodes(may_have_singletons, may_have_singleton_with_selfloops);
            let connected_nodes_number = connected_nodes.count_ones() as NodeT;
            // If there are less connected nodes than the number of nodes
            // in the graph, it means that there must be some singleton or singleton with selfloops.
            if connected_nodes_number < graph.get_nodes_number() {
                graph.connected_nodes = Some(connected_nodes);
                graph.connected_nodes_number = connected_nodes_number;
                graph.unique_sources = Some(graph.get_unique_sources());
            }
        }
        graph
    }

    /// Returns Elias-Fano data structure with the source nodes.
    fn get_unique_sources(&self) -> EliasFano {
        let unique_source_nodes_number = self.get_nodes_number()
            - self.get_singleton_nodes_number()
            - self.get_trap_nodes_number();
        let mut elias_fano_unique_sources = EliasFano::new(
            self.get_nodes_number() as u64,
            unique_source_nodes_number as usize,
        )
        .unwrap();
        self.iter_node_ids()
            .filter(|&node_id| unsafe { self.get_unchecked_node_degree_from_node_id(node_id) > 0 })
            .for_each(|node_id| {
                elias_fano_unique_sources.unchecked_push(node_id as u64);
            });
        elias_fano_unique_sources
    }

    /// Return bitvector containing nodes as true when they have incoming nodes.
    ///
    /// # Arguments
    /// * `may_have_singletons`: bool - Whether the graph has singleton.
    /// * `may_have_singleton_with_selfloops`: bool - Whether the graph has singleton with selfloops.
    fn get_connected_nodes(
        &self,
        may_have_singletons: bool,
        may_have_singleton_with_selfloops: bool,
    ) -> BitVec<Lsb0, u8> {
        let connected_nodes = if may_have_singletons && self.is_directed() {
            let mut connected_nodes = bitvec![Lsb0, AtomicU8; 0; self.get_nodes_number() as usize];
            let thread_shared_connected_nodes = ThreadDataRaceAware {
                value: std::cell::UnsafeCell::new(&mut connected_nodes),
            };
            // If the graph may contain singletons, we need to iterate on all
            // the nodes neighbours in order to find if whether a node is a singleton or
            // if it is a trap node.
            // Compute in parallel the bitvector of all the nodes that have incoming edges
            self.par_iter_node_ids().for_each(|node_id| unsafe {
                let connected_nodes = thread_shared_connected_nodes.value.get();
                let mut is_connected = false;
                self.iter_unchecked_neighbour_node_ids_from_source_node_id(node_id)
                    .filter(|&dst_id| node_id != dst_id)
                    .for_each(|dst_id| {
                        is_connected = true;
                        *(*connected_nodes).get_unchecked_mut(dst_id as usize) = true;
                    });
                if is_connected {
                    *(*connected_nodes).get_unchecked_mut(node_id as usize) = true;
                }
            });
            connected_nodes
        } else {
            let mut connected_nodes = bitvec![Lsb0, AtomicU8; 1; self.get_nodes_number() as usize];
            let thread_shared_connected_nodes = ThreadDataRaceAware {
                value: std::cell::UnsafeCell::new(&mut connected_nodes),
            };
            self.par_iter_node_degrees()
                .enumerate()
                .for_each(|(node_id, node_degree)| unsafe {
                    // If this node is a singleton we mark it as disconnected.
                    // We use the unlikely directive to tell the compiler that this
                    // should be a rare occurrence: in a well formed graph
                    // there should be only a small amount of singletons.
                    // The same applies also to singletons with selfloops.
                    if unlikely(
                        may_have_singletons && node_degree == 0
                            || may_have_singleton_with_selfloops
                                && node_degree > 0
                                && self
                                    .iter_unchecked_neighbour_node_ids_from_source_node_id(
                                        node_id as NodeT,
                                    )
                                    .all(|dst_id| node_id as NodeT == dst_id),
                    ) {
                        let connected_nodes = thread_shared_connected_nodes.value.get();
                        *(*connected_nodes).get_unchecked_mut(node_id) = false;
                    }
                });
            connected_nodes
        };
        unsafe { std::mem::transmute::<BitVec<Lsb0, AtomicU8>, BitVec<Lsb0, u8>>(connected_nodes) }
    }

    /// Return whether given graph has any edge overlapping with current graph.
    ///
    /// # Arguments
    ///
    /// * `other`: &Graph - The graph to check against.
    ///
    /// # Example
    /// You can whether two graphs are overlapping as follows:
    /// ```rust
    /// # let ppi = graph::test_utilities::load_ppi(true, true, false, false, false, false);
    /// # let cora = graph::test_utilities::load_cora();
    /// assert!(ppi.overlaps(&ppi).unwrap());
    /// assert!(cora.overlaps(&cora).unwrap());
    /// assert!(!ppi.overlaps(&cora).unwrap());
    /// assert!(!cora.overlaps(&ppi).unwrap());
    /// let (train, test) = ppi.random_holdout(
    ///     0.8,
    ///     Some(42),
    ///     Some(false),
    ///     None,
    ///     None,
    ///     None,
    /// ).unwrap();
    /// assert!(ppi.overlaps(&train).unwrap());
    /// assert!(ppi.overlaps(&test).unwrap());
    /// assert!(train.overlaps(&ppi).unwrap());
    /// assert!(test.overlaps(&ppi).unwrap());
    /// assert!(!train.overlaps(&test).unwrap());
    /// assert!(!test.overlaps(&train).unwrap());
    /// ```
    ///
    /// # Raises
    /// * If a graph is directed and the other is undirected.
    /// * If one of the two graphs has edge weights and the other does not.
    /// * If one of the two graphs has node types and the other does not.
    /// * If one of the two graphs has edge types and the other does not.
    pub fn overlaps(&self, other: &Graph) -> Result<bool> {
        Ok(match self.is_compatible(other)? {
            true => other
                .par_iter_edge_node_ids_and_edge_type_id(other.directed)
                .any(|(_, src, dst, et)| {
                    self.has_edge_from_node_ids_and_edge_type_id(src, dst, et)
                }),
            false => other
                .par_iter_edge_node_names_and_edge_type_name(other.directed)
                .any(|(_, _, src_name, _, dst_name, _, edge_type_name)| {
                    self.has_edge_from_node_names_and_edge_type_name(
                        &src_name,
                        &dst_name,
                        edge_type_name.as_deref(),
                    )
                }),
        })
    }

    /// Return true if given graph edges are all contained within current graph.
    ///
    /// # Arguments
    ///
    /// * `other`: &Graph - The graph to check against.
    ///
    /// # Example
    /// You can whether two graphs contain one another as follows:
    /// ```rust
    /// # let graph = graph::test_utilities::load_ppi(true, true, false, false, false, false);
    /// let (train, test) = graph.random_holdout(
    ///     0.8,
    ///     Some(42),
    ///     Some(false),
    ///     None,
    ///     None,
    ///     None,
    /// ).unwrap();
    /// assert!(graph.contains(&train).unwrap());
    /// assert!(graph.contains(&test).unwrap());
    /// assert!(!train.contains(&graph).unwrap());
    /// assert!(!test.contains(&graph).unwrap());
    /// assert!(!train.contains(&test).unwrap());
    /// assert!(!test.contains(&train).unwrap());
    /// ```
    ///
    /// # Raises
    /// * If a graph is directed and the other is undirected.
    /// * If one of the two graphs has edge weights and the other does not.
    /// * If one of the two graphs has node types and the other does not.
    /// * If one of the two graphs has edge types and the other does not.
    pub fn contains(&self, other: &Graph) -> Result<bool> {
        Ok(match self.is_compatible(other)? {
            true => other
                .par_iter_edge_node_ids_and_edge_type_id(other.directed)
                .all(|(_, src, dst, et)| {
                    self.has_edge_from_node_ids_and_edge_type_id(src, dst, et)
                }),
            false => other
                .par_iter_edge_node_names_and_edge_type_name(other.directed)
                .all(|(_, _, src_name, _, dst_name, _, edge_type_name)| {
                    self.has_edge_from_node_names_and_edge_type_name(
                        &src_name,
                        &dst_name,
                        edge_type_name.as_deref(),
                    )
                }),
        })
    }
}
