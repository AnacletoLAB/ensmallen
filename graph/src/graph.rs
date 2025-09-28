use std::sync::atomic::AtomicU8;

use super::*;
use bitvec::prelude::*;
use csr::CSR;
use elias_fano_rust::EliasFano;
use rayon::prelude::*;
use std::sync::Arc;

/// This is the main struct in Ensmallen, it allows to load and manipulate Graphs efficently.
/// You are not supposed to directly instantiate this struct but instead you should use the
/// static method `from_csv`, which allows to load the graph from an edge-list.
///
/// To get information about a loaded graph, you can call the `textual_report` method which
/// generates an human-readable HTML report.
///
/// By default we use EliasFano to store the Adjacency Matrix, this allows to save memory but
/// is slower than a CSR. For this reason you can use the `enable` method to enable optimizzations
/// which speeds up the operations at the cost of more memory usage. You can check the memory usage
/// in bytes using `get_total_memory_used` and you can get a detailed memory report of each data-structure
/// inside Graph using `memory_stats`.
///
/// You can pre-compute the memory needed (in bits) to store the adjacency matrix of a Graph with $|E|$ edges and $|V|$ nodes:
///  $$2 |E| + |E| \\left\\lceil \\log_2 \\frac{|V|^2}{|E|} \\right\\rceil$$
///
/// Most Graph properties are automatically cached to speed up.
#[derive(Clone, Debug)]
pub struct Graph {
    /// The main datastructure where all the edges are saved
    pub(crate) edges: Arc<CSR>,
    /// Optional vector of the weights of every edge.
    /// `weights[10]` return the weight of the edge with edge_id 10
    pub(crate) weights: Arc<Option<Vec<WeightT>>>,
    /// Vocabulary that save the mappings from string to index of every node type
    pub(crate) node_types: Arc<Option<NodeTypeVocabulary>>,
    // This is the next attribute that will be embedded inside of edges once
    // the first refactoring is done
    /// Vocabulary that save the mappings from string to index of every edge type
    pub(crate) edge_types: Arc<Option<EdgeTypeVocabulary>>,
    /// Vocabulary that save the mappings from string to index of every node
    pub(crate) nodes: Arc<Vocabulary<NodeT>>,

    /// if the graph is directed or undirected
    pub(crate) directed: bool,
    /// Graph name
    pub(crate) name: Arc<String>,

    // /////////////////////////////////////////////////////////////////////////
    // Caching related attributes
    // /////////////////////////////////////////////////////////////////////////
    /// Option of Elias-Fano of unique sources.
    /// When it is None it means that ALL nodes are sources.
    pub(crate) unique_sources: Arc<Option<EliasFano>>,

    /// Option of bitvec containing connected nodes.
    /// When it is None it means that ALL nodes are connected, i.e. not singleton or singletons with selfloops.
    pub(crate) connected_nodes: Arc<Option<BitVec<u8, Lsb0>>>,

    /// Number of connected nodes in the graph.
    pub(crate) connected_number_of_nodes: NodeT,

    // /////////////////////////////////////////////////////////////////////////
    // Kernels Caching related attributes
    // /////////////////////////////////////////////////////////////////////////
    pub(crate) reciprocal_sqrt_degrees: Arc<Option<Vec<WeightT>>>,

    // /////////////////////////////////////////////////////////////////////////
    pub(crate) cache: Arc<ClonableUnsafeCell<PropertyCache>>,
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
        nodes: Arc<Vocabulary<NodeT>>,
        node_types: Arc<Option<NodeTypeVocabulary>>,
        edges: Arc<CSR>,
        edge_types: Arc<Option<EdgeTypeVocabulary>>,
        weights: Arc<Option<Vec<WeightT>>>,
        may_have_singletons: bool,
        may_have_singleton_with_selfloops: bool,
        name: S,
    ) -> Graph {
        let number_of_nodes = nodes.len();
        let mut graph = Graph {
            directed,
            edges: edges,
            weights: weights,
            node_types: node_types,
            edge_types: edge_types,
            nodes: nodes,
            name: Arc::new(name.into()),
            cache: Arc::new(ClonableUnsafeCell::default()),
            unique_sources: Arc::new(None),
            connected_nodes: Arc::new(None),
            connected_number_of_nodes: number_of_nodes as NodeT,
            reciprocal_sqrt_degrees: Arc::new(None),
        };
        if may_have_singletons || may_have_singleton_with_selfloops {
            let connected_nodes =
                graph.get_connected_nodes(may_have_singletons, may_have_singleton_with_selfloops);
            let connected_number_of_nodes = connected_nodes.count_ones() as NodeT;
            // If there are less connected nodes than the number of nodes
            // in the graph, it means that there must be some singleton or singleton with selfloops.
            if connected_number_of_nodes < graph.get_number_of_nodes() {
                graph.connected_nodes = Arc::new(Some(connected_nodes));
                graph.connected_number_of_nodes = connected_number_of_nodes;
                graph.unique_sources = Arc::new(Some(graph.get_unique_sources()));
            }
        }
        graph
    }

    /// Returns Elias-Fano data structure with the source nodes.
    fn get_unique_sources(&self) -> EliasFano {
        let unique_source_number_of_nodes = self.get_number_of_nodes()
            - self.get_number_of_singleton_nodes()
            - self.get_number_of_trap_nodes();
        let mut elias_fano_unique_sources = EliasFano::new(
            self.get_number_of_nodes() as u64,
            unique_source_number_of_nodes as usize,
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
    ) -> BitVec<u8, Lsb0> {
        let connected_nodes =
            if (may_have_singletons || may_have_singleton_with_selfloops) && self.is_directed() {
                let mut connected_nodes =
                    bitvec![AtomicU8, Lsb0; 0; self.get_number_of_nodes() as usize];
                let thread_shared_connected_nodes = ThreadDataRaceAware::new(&mut connected_nodes);
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
                let mut connected_nodes =
                    bitvec![AtomicU8, Lsb0; 1; self.get_number_of_nodes() as usize];
                let thread_shared_connected_nodes = ThreadDataRaceAware::new(&mut connected_nodes);
                self.par_iter_node_degrees().enumerate().for_each(
                    |(node_id, node_degree)| unsafe {
                        // If this node is a singleton we mark it as disconnected.
                        // We use the unlikely directive to tell the compiler that this
                        // should be a rare occurrence: in a well formed graph
                        // there should be only a small amount of singletons.
                        // The same applies also to singletons with selfloops.
                        if may_have_singletons && node_degree == 0
                            || may_have_singleton_with_selfloops
                                && node_degree > 0
                                && self
                                    .iter_unchecked_neighbour_node_ids_from_source_node_id(
                                        node_id as NodeT,
                                    )
                                    .all(|dst_id| node_id as NodeT == dst_id)
                        {
                            let connected_nodes = thread_shared_connected_nodes.value.get();
                            *(*connected_nodes).get_unchecked_mut(node_id) = false;
                        }
                    },
                );
                connected_nodes
            };
        unsafe { std::mem::transmute::<BitVec<AtomicU8, Lsb0>, BitVec<u8, Lsb0>>(connected_nodes) }
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
        Ok(if other.has_edge_types() && self.has_edge_types() {
            self.par_iter_directed_edge_node_names_and_edge_type_name()
                .any(|(_, _, src_name, _, dst_name, _, edge_type_name)| {
                    other.has_edge_from_node_names_and_edge_type_name(
                        &src_name,
                        &dst_name,
                        edge_type_name.as_deref(),
                    )
                })
        } else {
            self.par_iter_directed_edges()
                .any(|(_, _, src_name, _, dst_name)| {
                    other.has_edge_from_node_names(&src_name, &dst_name)
                })
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
            true => {
                if other.has_edge_types() {
                    other
                        .par_iter_edge_node_ids_and_edge_type_id(other.directed)
                        .all(|(_, src, dst, et)| {
                            self.has_edge_from_node_ids_and_edge_type_id(src, dst, et)
                        })
                } else {
                    other
                        .par_iter_edge_node_ids(other.directed)
                        .all(|(_, src, dst)| self.has_edge_from_node_ids(src, dst))
                }
            }
            false => {
                if other.has_edge_types() {
                    other
                        .par_iter_edge_node_names_and_edge_type_name(other.directed)
                        .all(|(_, _, src_name, _, dst_name, _, edge_type_name)| {
                            self.has_edge_from_node_names_and_edge_type_name(
                                &src_name,
                                &dst_name,
                                edge_type_name.as_deref(),
                            )
                        })
                } else {
                    other
                        .par_iter_edges(other.directed)
                        .all(|(_, _, src_name, _, dst_name)| {
                            self.has_edge_from_node_names(&src_name, &dst_name)
                        })
                }
            }
        })
    }
}
