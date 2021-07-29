//! A graph representation optimized for executing random walks on huge graphs.
use shared::*;
use bitvec::prelude::*;
use elias_fano_rust::*;
use crate::vocabularies::*;
use crate::cache::*;

/// A graph representation optimized for executing random walks on huge graphs.
#[derive(Clone, Debug)]
pub struct Graph {
    /// The main datastructure where all the edges are saved
    /// in the endoced form ((src << self.node_bits) | dst) this allows us to do almost every
    /// operation in O(1) without decompressing the data.
    pub edges: EliasFano,
    /// Optional vector of the weights of every edge.
    /// `weights[10]` return the weight of the edge with edge_id 10
    pub weights: Option<Vec<WeightT>>,
    /// Vocabulary that save the mappings from string to index of every node type
    pub node_types: Option<NodeTypeVocabulary>,
    // This is the next attribute that will be embedded inside of edges once
    // the first refactoring is done
    /// Vocabulary that save the mappings from string to index of every edge type
    pub edge_types: Option<EdgeTypeVocabulary>,
    /// Vocabulary that save the mappings from string to index of every node
    pub nodes: Vocabulary<NodeT>,

    /// How many bits are needed to save a node.
    pub node_bits: u8,
    /// The mask used to extract the dst value form an encoded edge.
    /// This is saved for speed sake. It's equivalent to (1 << self.node_bits) - 1;
    pub node_bit_mask: u64,

    /// if the graph is directed or undirected
    pub directed: bool,
    /// Graph name
    pub name: String,

    // /////////////////////////////////////////////////////////////////////////
    // Elias-Fano Caching related attributes
    // /////////////////////////////////////////////////////////////////////////
    /// Vector of destinations to execute fast walks if required.
    pub destinations: Option<Vec<NodeT>>,
    /// Vector of sources to execute fast link prediction sequences if required.
    pub sources: Option<Vec<NodeT>>,
    /// Vector of cumulative_node_degrees to execute fast walks if required.
    pub cumulative_node_degrees: Option<Vec<EdgeT>>,
    /// Option of Elias-Fano of unique sources.
    /// When it is None it means that ALL nodes are sources.
    pub unique_sources: Option<EliasFano>,
    /// Option of bitvec containing connected nodes.
    /// When it is None it means that ALL nodes are connected, i.e. not singleton or singletons with selfloops.
    pub connected_nodes: Option<BitVec<Lsb0, u8>>,
    /// Number of connected nodes in the graph.
    pub connected_nodes_number: NodeT,

    // /////////////////////////////////////////////////////////////////////////
    pub cache: ClonableUnsafeCell<PropertyCache>,
}