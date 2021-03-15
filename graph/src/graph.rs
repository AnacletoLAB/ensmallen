//! A graph representation optimized for executing random walks on huge graphs.
use super::*;
use counter::Counter;
use elias_fano_rust::EliasFano;
use rayon::prelude::*;
use std::collections::HashMap;

/// A graph representation optimized for executing random walks on huge graphs.
///
/// This class should be initialized using the two constructors:
/// `graph::Graph::new_directed` or `graph::Graph::new_undirected`
///
/// # Examples
///
#[derive(Clone, Debug)]
pub struct Graph {
    // properties
    /// if the graph is directed or undirected
    pub(crate) directed: bool,
    /// Number of nodes that have at least a self-loop.
    /// This means that if a nodes has multiples self-loops they will be count as one.
    pub(crate) unique_self_loop_number: NodeT,
    /// Number of self-loop edges. This counts multiple times eventual multi-graph self-loops.
    pub(crate) self_loop_number: EdgeT,
    /// Number of nodes that have at least an edge inbound or outbound.
    pub(crate) not_singleton_nodes_number: NodeT,
    /// Number of singleton nodes that have a self-loop
    pub(crate) singleton_nodes_with_self_loops_number: NodeT,
    /// How many unique edges the graph has (excluding the multi-graph ones)
    pub(crate) unique_edges_number: EdgeT,
    /// Vector of destinations to execute fast walks if required.
    pub(crate) destinations: Option<Vec<NodeT>>,
    /// Vector of sources to execute fast link prediction sequences if required.
    pub(crate) sources: Option<Vec<NodeT>>,
    /// Vector of outbounds to execute fast walks if required.
    pub(crate) outbounds: Option<Vec<EdgeT>>,
    // Hashmap of cached destinations to execute faster walks if required.
    pub(crate) cached_destinations: Option<HashMap<NodeT, Vec<NodeT>>>,
    /// Graph name
    pub(crate) name: String,
    // Graph embedding for fast edge embedding operations.
    pub(crate) embedding: Option<Vec<Vec<f64>>>,

    /// The main datastructure where all the edges are saved
    /// in the endoced form ((src << self.node_bits) | dst) this allows us to do almost every
    /// operation in O(1) without decompressing the data.
    pub(crate) edges: EliasFano,
    /// How many bits are needed to save a node.
    pub(crate) node_bits: u8,
    /// The mask used to extract the dst value form an encoded edge.
    /// This is saved for speed sake. It's equivalent to (1 << self.node_bits) - 1;
    pub(crate) node_bit_mask: u64,
    /// Vocabulary that save the mappings from string to index of every node
    pub(crate) nodes: Vocabulary<NodeT>,
    pub(crate) unique_sources: EliasFano,

    /// Optional vector of the weights of every edge.
    /// `weights[10]` return the weight of the edge with edge_id 10
    pub(crate) weights: Option<Vec<WeightT>>,
    /// Vocabulary that save the mappings from string to index of every node type
    pub(crate) node_types: Option<NodeTypeVocabulary>,
    // This is the next attribute that will be embedded inside of edges once
    // the first refactoring is done
    /// Vocabulary that save the mappings from string to index of every edge type
    pub(crate) edge_types: Option<EdgeTypeVocabulary>,
}

/// # Graph utility methods
impl Graph {
    pub fn new<S: Into<String>>(
        directed: bool,
        unique_self_loop_number: NodeT,
        self_loop_number: EdgeT,
        not_singleton_nodes_number: NodeT,
        singleton_nodes_with_self_loops_number: NodeT,
        unique_edges_number: EdgeT,
        edges: EliasFano,
        unique_sources: EliasFano,
        nodes: Vocabulary<NodeT>,
        node_bit_mask: EdgeT,
        node_bits: u8,
        edge_types: Option<EdgeTypeVocabulary>,
        name: S,
        weights: Option<Vec<WeightT>>,
        node_types: Option<NodeTypeVocabulary>,
    ) -> Graph {
        Graph {
            directed,
            unique_self_loop_number,
            self_loop_number,
            not_singleton_nodes_number,
            singleton_nodes_with_self_loops_number,
            unique_edges_number,
            edges,
            unique_sources,
            node_bit_mask,
            node_bits,
            weights,
            node_types: node_types.map(|nts| nts.set_numeric_ids(false)),
            edge_types: edge_types.map(|ets| ets.set_numeric_ids(false)),
            nodes: nodes.set_numeric_ids(false),
            sources: None,
            embedding: None,
            destinations: None,
            outbounds: None,
            cached_destinations: None,
            name: name.into(),
        }
    }

    pub fn get_unchecked_edge_id(
        &self,
        src: NodeT,
        dst: NodeT,
        edge_type: Option<EdgeTypeT>,
    ) -> EdgeT {
        if let Some(ets) = &self.edge_types {
            return self
                .get_unchecked_edge_ids_range(src, dst)
                // The vectors of the edge types can only have one element.
                .find(|edge_id| ets.ids[*edge_id as usize] == edge_type)
                .unwrap();
        }
        self.get_unchecked_edge_id_from_tuple(src, dst)
    }

    pub fn get_edge_id(
        &self,
        src: NodeT,
        dst: NodeT,
        edge_type: Option<EdgeTypeT>,
    ) -> Option<EdgeT> {
        if let Some(ets) = &self.edge_types {
            return self.get_edge_ids(src, dst).and_then(|mut edge_ids| {
                edge_ids.find(|edge_id| ets.ids[*edge_id as usize] == edge_type)
            });
        }
        self.get_edge_id_from_tuple(src, dst)
    }

    pub fn get_edge_id_string(
        &self,
        src_name: &str,
        dst_name: &str,
        edge_type_name: Option<&String>,
    ) -> Option<EdgeT> {
        if let Some(src) = self.nodes.get(src_name) {
            if let Some(dst) = self.nodes.get(dst_name) {
                let edge_type_id = edge_type_name.and_then(|etn| match &self.edge_types {
                    Some(ets) => ets.get(etn).copied(),
                    None => None,
                });
                if edge_type_id.is_none() && edge_type_name.is_some() {
                    return None;
                }
                return self.get_edge_id(*src, *dst, edge_type_id);
            }
        }
        None
    }

    /// Returns edge type counts.
    ///
    /// # Arguments
    ///
    /// None
    ///
    /// # Examples
    /// ```rust
    /// # let graph = graph::test_utilities::load_ppi(true, true, true, true, false, false).unwrap();
    /// for (edge_type_id, count) in graph.get_edge_type_counts().unwrap().iter() {
    ///     println!("edge type id {}: count: {}", edge_type_id, count);
    /// }
    /// ```
    pub fn get_edge_type_counts(&self) -> Result<Counter<EdgeTypeT, usize>, String> {
        if let Some(et) = &self.edge_types {
            Ok(Counter::init(
                et.ids.iter().filter_map(|edge_type| *edge_type),
            ))
        } else {
            Err(String::from(
                "Edge types are not defined for current graph instance.",
            ))
        }
    }

    /// Returns edge type counts hashmap.
    ///
    /// # Arguments
    ///
    /// None
    ///
    /// # Examples
    /// ```rust
    /// # let graph = graph::test_utilities::load_ppi(true, true, true, true, false, false).unwrap();
    /// for (edge_type_id, count) in graph.get_edge_type_counts().unwrap().iter() {
    ///     println!("edge type id {}: count: {}", edge_type_id, count);
    /// }
    /// ```
    pub fn get_edge_type_counts_hashmap(&self) -> Result<HashMap<EdgeTypeT, usize>, String> {
        Ok(self.get_edge_type_counts()?.into_map())
    }

    /// Return translated edge types from string to internal edge ID.
    ///
    /// # Arguments
    ///
    /// * `edge_types`: Vec<String> - Vector of edge types to be converted.
    pub fn translate_edge_types(&self, edge_types: Vec<String>) -> Result<Vec<EdgeTypeT>, String> {
        match &self.edge_types {
            None => Err(String::from("Current graph does not have edge types.")),
            Some(ets) => {
                Ok(edge_types
                .iter()
                .map(|edge_type| match ets.get(edge_type) {
                    None => Err(format!(
                        "The edge type {} does not exist in current graph. The available edge types are {}.",
                        edge_type,
                        ets.keys().join(", ")
                    )),
                    Some(et) => Ok(*et),
                })
                .collect::<Result<Vec<EdgeTypeT>, String>>()?)
            }
        }
    }

    /// Return translated node types from string to internal node ID.
    ///
    /// # Arguments
    ///
    /// * `node_types`: Vec<String> - Vector of node types to be converted.
    pub fn translate_node_types(&self, node_types: Vec<String>) -> Result<Vec<NodeTypeT>, String> {
        match &self.node_types {
            None => Err(String::from("Current graph does not have node types.")),
            Some(nts) => {
                Ok(node_types
                .iter()
                .map(|node_type| match nts.get(node_type) {
                    None => Err(format!(
                        "The node type {} does not exist in current graph. The available node types are {}.",
                        node_type,
                        nts.keys().join(", ")
                    )),
                    Some(et) => Ok(*et),
                })
                .collect::<Result<Vec<NodeTypeT>, String>>()?)
            }
        }
    }

    /// Returns node type counts.
    ///
    /// # Arguments
    ///
    /// None
    ///
    /// ```rust
    /// # let graph = graph::test_utilities::load_ppi(true, true, true, true, false, false).unwrap();
    /// for (node_type_id, count) in graph.get_node_type_counts().unwrap().iter() {
    ///     println!("node type id {}: count: {}", node_type_id, count);
    /// }
    /// ```
    pub fn get_node_type_counts(&self) -> Result<Counter<NodeTypeT, usize>, String> {
        if let Some(nt) = &self.node_types {
            Ok(Counter::init(
                nt.ids
                    .iter()
                    .filter_map(|node_type| node_type.clone())
                    .flatten(),
            ))
        } else {
            Err(String::from(
                "Node types are not defined for current graph instance.",
            ))
        }
    }

    /// Returns node type counts hashmap.
    ///
    /// # Arguments
    ///
    /// None
    ///
    /// ```rust
    /// # let graph = graph::test_utilities::load_ppi(true, true, true, true, false, false).unwrap();
    /// for (node_type_id, count) in graph.get_node_type_counts().unwrap().iter() {
    ///     println!("node type id {}: count: {}", node_type_id, count);
    /// }
    /// ```
    pub fn get_node_type_counts_hashmap(&self) -> Result<HashMap<EdgeTypeT, usize>, String> {
        Ok(self.get_node_type_counts()?.into_map())
    }

    /// Returns boolean representing if edge passing between given nodes exists.
    ///
    /// # Arguments
    ///
    /// * src: NodeT - The source node of the edge.
    /// * dst: NodeT - The destination node of the edge.
    /// * edge_type: Option<EdgeTypeT> - The (optional) edge type.
    ///
    pub fn has_edge(&self, src: NodeT, dst: NodeT, edge_type: Option<EdgeTypeT>) -> bool {
        self.get_edge_id(src, dst, edge_type).is_some()
    }

    /// Returns boolean representing if edge passing between given nodes exists.
    ///
    /// # Arguments
    ///
    /// * src: String - The source node name of the edge.
    /// * dst: String - The destination node name of the edge.
    /// * edge_type: Option<String> - The (optional) edge type name.
    ///
    pub fn has_edge_string(
        &self,
        src_name: &str,
        dst_name: &str,
        edge_type_name: Option<&String>,
    ) -> bool {
        self.get_edge_id_string(src_name, dst_name, edge_type_name)
            .is_some()
    }

    /// Returns boolean representing if node with given name exists in current graph.
    ///
    /// # Arguments
    ///
    /// * node_name: String - The node name.
    ///
    pub fn has_node_string(&self, node_name: &str, node_type_name: Option<Vec<String>>) -> bool {
        match self.get_node_id(node_name) {
            Err(_) => false,
            Ok(node_id) => {
                let our_node_types = self.get_node_type_string(node_id);
                match (our_node_types, node_type_name) {
                    (Some(mut our_nts), Some(mut other_nts)) => {
                        our_nts.sort();
                        other_nts.sort();
                        our_nts == other_nts
                    }
                    (None, None) => true,
                    _ => false,
                }
            }
        }
    }

    /// Return true if given graph has any edge overlapping with current graph.
    ///
    /// # Arguments
    ///
    /// * other: Graph - The graph to check against.
    ///
    pub fn overlaps(&self, other: &Graph) -> Result<bool, String> {
        Ok(match self.is_compatible(other)? {
            true => other
                .get_edges_par_triples(other.directed)
                .any(|(_, src, dst, et)| self.has_edge(src, dst, et)),
            false => other
                .get_edges_par_string_triples(other.directed)
                .any(|(_, src, dst, et)| self.has_edge_string(&src, &dst, et.as_ref())),
        })
    }

    /// Return true if given graph edges are all contained within current graph.
    ///
    /// # Arguments
    ///
    /// * other: Graph - The graph to check against.
    ///
    pub fn contains(&self, other: &Graph) -> Result<bool, String> {
        Ok(match self.is_compatible(other)? {
            true => other
                .get_edges_par_triples(other.directed)
                .all(|(_, src, dst, et)| self.has_edge(src, dst, et)),
            false => other
                .get_edges_par_string_triples(other.directed)
                .all(|(_, src, dst, et)| self.has_edge_string(&src, &dst, et.as_ref())),
        })
    }

    /// Return range of outbound edges IDs for given Node.
    ///
    /// # Arguments
    ///
    /// * src: NodeT - Source node.
    /// * dst: NodeT - Destination node.
    ///
    pub(crate) fn get_unchecked_edge_types_min_max_edge_ids(
        &self,
        src: NodeT,
        dst: NodeT,
    ) -> (EdgeT, EdgeT) {
        (
            self.get_unchecked_edge_id_from_tuple(src, dst),
            self.get_unchecked_edge_id_from_tuple(src, dst + 1),
        )
    }

    /// Return range of outbound edges IDs for given Node.
    ///
    /// # Arguments
    ///
    /// * src: NodeT - Source node.
    /// * dst: NodeT - Destination node.
    ///
    pub(crate) fn get_unchecked_edge_types_number_from_tuple(
        &self,
        src: NodeT,
        dst: NodeT,
    ) -> EdgeT {
        let (min_edge_id, max_edge_id) = self.get_unchecked_edge_types_min_max_edge_ids(src, dst);
        max_edge_id - min_edge_id
    }

    /// Return range of outbound edges IDs for given Node.
    ///
    /// # Arguments
    ///
    /// * src: NodeT - Source node.
    /// * dst: NodeT - Destination node.
    ///
    pub(crate) fn get_edge_types_min_max_edge_ids(
        &self,
        src: NodeT,
        dst: NodeT,
    ) -> Option<(EdgeT, EdgeT)> {
        if let Some(min_edge) = self.get_edge_id_from_tuple(src, dst) {
            let max_edge = self.get_unchecked_edge_id_from_tuple(src, dst + 1);
            return Some((min_edge as EdgeT, max_edge as EdgeT));
        }
        None
    }

    /// Return range of outbound edges IDs for given Node.
    ///
    /// # Arguments
    ///
    /// * src: NodeT - Node for which we need to compute the outbounds range.
    ///
    pub(crate) fn get_destinations_min_max_edge_ids(&self, src: NodeT) -> (EdgeT, EdgeT) {
        match &self.outbounds {
            Some(outbounds) => {
                let min_edge_id = if src == 0 {
                    0
                } else {
                    outbounds[src as usize - 1]
                };
                (min_edge_id, outbounds[src as usize])
            }
            None => {
                let min_edge_id: EdgeT = self.get_unchecked_edge_id_from_tuple(src, 0);
                (
                    min_edge_id,
                    match &self.cached_destinations {
                        Some(cds) => match cds.get(&src) {
                            Some(destinations) => destinations.len() as EdgeT + min_edge_id,
                            None => self.get_unchecked_edge_id_from_tuple(src + 1, 0),
                        },
                        None => self.get_unchecked_edge_id_from_tuple(src + 1, 0),
                    },
                )
            }
        }
    }

    /// Returns the number of outbound neighbours of given node.
    ///
    /// This is implemented as proposed by [S. Vigna here](http://vigna.di.unimi.it/ftp/papers/Broadword.pdf).
    ///
    /// # Arguments
    ///
    /// * `node` - Integer ID of the node.
    ///
    pub fn get_node_degree(&self, node: NodeT) -> NodeT {
        let (min_edge_id, max_edge_id) = self.get_destinations_min_max_edge_ids(node);
        (max_edge_id - min_edge_id) as NodeT
    }

    // TODO: Update docstring
    pub fn get_unchecked_edge_ids_range(
        &self,
        src: NodeT,
        dst: NodeT,
    ) -> impl Iterator<Item = EdgeT> {
        let (min_edge_id, max_edge_id) = self.get_unchecked_edge_types_min_max_edge_ids(src, dst);
        min_edge_id..max_edge_id
    }

    // TODO: Update docstring
    pub fn get_unchecked_destinations_range(&self, src: NodeT) -> impl Iterator<Item = EdgeT> {
        let (min_edge_id, max_edge_id) = self.get_destinations_min_max_edge_ids(src);
        min_edge_id..max_edge_id
    }

    // TODO: Update docstring
    pub fn get_edge_ids(&self, src: NodeT, dst: NodeT) -> Option<impl Iterator<Item = EdgeT>> {
        self.get_edge_types_min_max_edge_ids(src, dst)
            .map(|(min_edge_id, max_edge_id)| min_edge_id..max_edge_id)
    }

    /// Returns edge_types associated to the given edge.
    /// A link is composed by all the edges that starts from src and ends at dst.
    ///
    /// # Arguments
    ///
    /// * `src`: NodeT - Integer ID of the source node.
    /// * `dst`: NodeT - Integer ID of the destination node.
    ///
    pub fn get_unchecked_link_edge_types(
        &self,
        src: NodeT,
        dst: NodeT,
    ) -> Option<Vec<Option<EdgeTypeT>>> {
        self.edge_types.as_ref().map(|ets| {
            self.get_unchecked_edge_ids_range(src, dst)
                .map(|edge_id| ets.ids[edge_id as usize])
                .collect()
        })
    }

    /// Returns weights associated to the given link.
    /// A link is composed by all the edges that starts from src and ends at dst.
    ///
    /// # Arguments
    ///
    /// * `src`: NodeT - Integer ID of the source node.
    /// * `dst`: NodeT - Integer ID of the destination node.
    ///
    pub fn get_unchecked_link_weights(&self, src: NodeT, dst: NodeT) -> Option<Vec<WeightT>> {
        self.weights.as_ref().map(|ws| {
            self.get_unchecked_edge_ids_range(src, dst)
                .map(|edge_id| ws[edge_id as usize])
                .collect()
        })
    }

    /// Returns boolean representing if given node is a trap.
    ///
    /// # Arguments
    ///
    /// * `node` - Integer ID of the node, if this is bigger that the number of nodes it will panic.
    ///
    pub fn is_node_trap(&self, node: NodeT) -> bool {
        self.get_node_degree(node) == 0
    }
    /// Returns boolean representing if given edge is a trap.
    ///
    /// # Arguments
    ///
    /// * `edge_id` - Integer ID of the edge, if this is bigger that the number of edges it will panic.
    ///
    pub fn is_edge_trap(&self, edge_id: EdgeT) -> bool {
        self.is_node_trap(self.get_destination(edge_id))
    }
}
