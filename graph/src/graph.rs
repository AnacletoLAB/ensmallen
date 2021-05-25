//! A graph representation optimized for executing random walks on huge graphs.
use super::*;
use bitvec::prelude::*;
use elias_fano_rust::EliasFano;
use rayon::prelude::*;
use roaring::RoaringBitmap;
use std::collections::HashMap;

/// A graph representation optimized for executing random walks on huge graphs.
///
/// This class should be initialized using the two constructors:
/// `graph::Graph::new_directed` or `graph::Graph::new_undirected`
///
/// # Example
/// Load the graph Cora:
/// ```rust
/// use graph::*;
///
/// // Create the edge file reader
/// let edges_reader = EdgeFileReader::new("tests/data/cora/edges.tsv").unwrap()
///     .set_separator(Some("\t")).unwrap()
///     .set_verbose(Some(false))
///     .set_sources_column(Some("subject")).unwrap()
///     .set_destinations_column(Some("object")).unwrap()
///     .set_default_weight(Some(1.0))
///     .set_edge_types_column(Some("edge_type")).unwrap();
///
/// // Create the node file reader
/// let nodes_reader = Some(
///     NodeFileReader::new("tests/data/cora/nodes.tsv").unwrap()
///         .set_separator(Some("\t")).unwrap()
///         .set_nodes_column(Some("id")).unwrap()
///         .set_verbose(Some(false))
///         .set_node_types_column(Some("node_type")).unwrap(),
/// );
///
/// // Load the graph
/// let mut cora = Graph::from_unsorted_csv(
///     edges_reader,
///     nodes_reader,
///     false,          // if the graph is Directed
///     false,          // if the edge list is Directed
///     "Cora".to_string()
///    ).unwrap();
///
/// // Enable Speed-ups but it uses more memory.
/// cora.enable(true, true, true, None).unwrap();
/// ```
#[derive(Clone, Debug)]
pub struct Graph {
    /// The main datastructure where all the edges are saved
    /// in the endoced form ((src << self.node_bits) | dst) this allows us to do almost every
    /// operation in O(1) without decompressing the data.
    pub(crate) edges: EliasFano,
    /// How many bits are needed to save a node.
    pub(crate) node_bits: u8,
    /// The mask used to extract the dst value form an encoded edge.
    /// This is saved for speed sake. It's equivalent to (1 << self.node_bits) - 1;
    pub(crate) node_bit_mask: u64,

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

    // //////////////////////////////////////////////////////////////////////////
    // Cached properties
    // //////////////////////////////////////////////////////////////////////////
    /// if the graph is directed or undirected
    pub(crate) directed: bool,
    /// Number of nodes that have at least a self-loop.
    /// This means that if a nodes has multiples self-loops they will be count as one.
    pub(crate) unique_selfloop_number: NodeT,
    /// Number of self-loop edges. This counts multiple times eventual multi-graph self-loops.
    pub(crate) selfloop_number: EdgeT,
    /// Number of nodes that have at least an edge inbound or outbound.
    pub(crate) connected_nodes_number: NodeT,
    /// Number of singleton nodes that have a self-loop
    pub(crate) singleton_nodes_with_selfloops_number: NodeT,
    /// How many unique edges the graph has (excluding the multi-graph ones)
    pub(crate) unique_edges_number: EdgeT,
    /// Minimum outbound node degree.
    pub(crate) min_node_degree: NodeT,
    /// Maximum outbound node degree.
    pub(crate) max_node_degree: NodeT,
    /// Minimum edge weight. Is None if weights are not defined.
    pub(crate) min_edge_weight: Option<WeightT>,
    /// Maximum edge weight. Is None if weights are not defined.
    pub(crate) max_edge_weight: Option<WeightT>,
    /// Graph name
    pub(crate) name: String,
    pub(crate) connected_nodes: Option<BitVec<Lsb0, u8>>,
    pub(crate) singleton_nodes_with_selfloops: Option<RoaringBitmap>,
    pub(crate) unique_sources: Option<EliasFano>,

    /// Cache of the textual report. This is needed because in some of the bindings
    /// (such as whitin jupyter) the textual report is called multiple times like\
    /// every time the IDE tries to auto-complete.
    /// This cache must be invalidated everytime the graph is modified.
    pub(crate) cached_report: ClonableRwLock<Option<String>>,

    // /////////////////////////////////////////////////////////////////////////
    // Elias-Fano Caching related attributes
    // /////////////////////////////////////////////////////////////////////////
    /// Vector of destinations to execute fast walks if required.
    pub(crate) destinations: Option<Vec<NodeT>>,
    /// Vector of sources to execute fast link prediction sequences if required.
    pub(crate) sources: Option<Vec<NodeT>>,
    /// Vector of cumulative_node_degrees to execute fast walks if required.
    pub(crate) cumulative_node_degrees: Option<Vec<EdgeT>>,
    // Hashmap of cached destinations to execute faster walks if required.
    pub(crate) cached_destinations: Option<HashMap<NodeT, Vec<NodeT>>>,
}

/// # Graph utility methods
impl Graph {
    pub(crate) fn new<S: Into<String>>(
        directed: bool,
        unique_selfloop_number: NodeT,
        selfloop_number: EdgeT,
        connected_nodes_number: NodeT,
        singleton_nodes_with_selfloops_number: NodeT,
        unique_edges_number: EdgeT,
        edges: EliasFano,
        unique_sources: Option<EliasFano>,
        nodes: Vocabulary<NodeT>,
        node_bit_mask: EdgeT,
        node_bits: u8,
        edge_types: Option<EdgeTypeVocabulary>,
        name: S,
        weights: Option<Vec<WeightT>>,
        min_edge_weight: Option<WeightT>,
        max_edge_weight: Option<WeightT>,
        node_types: Option<NodeTypeVocabulary>,
        connected_nodes: Option<BitVec<Lsb0, u8>>,
        singleton_nodes_with_selfloops: Option<RoaringBitmap>,
        min_node_degree: NodeT,
        max_node_degree: NodeT,
    ) -> Graph {
        Graph {
            directed,
            unique_selfloop_number,
            selfloop_number,
            connected_nodes_number,
            singleton_nodes_with_selfloops_number,
            unique_edges_number,
            edges,
            unique_sources,
            node_bit_mask,
            node_bits,
            weights,
            min_edge_weight,
            max_edge_weight,
            min_node_degree,
            max_node_degree,
            node_types: node_types.map(|nts| nts.set_numeric_ids(false)),
            edge_types: edge_types.map(|ets| ets.set_numeric_ids(false)),
            nodes: nodes.set_numeric_ids(false),
            sources: None,
            destinations: None,
            cumulative_node_degrees: None,
            cached_destinations: None,
            name: name.into(),
            connected_nodes,
            singleton_nodes_with_selfloops,
            cached_report: ClonableRwLock::new(None),
        }
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
    ///     42,
    ///     0.8,
    ///     false,
    ///     None,
    ///     None,
    ///     false
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
    pub fn overlaps(&self, other: &Graph) -> Result<bool, String> {
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
    ///     42,
    ///     0.8,
    ///     false,
    ///     None,
    ///     None,
    ///     false
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
    pub fn contains(&self, other: &Graph) -> Result<bool, String> {
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
