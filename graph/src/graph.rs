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
#[derive(Clone)]
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
    /// How many unique edges the graph has (excluding the multi-graph ones)
    pub(crate) unique_edges_number: EdgeT,

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
    /// weights[10] return the weight of the edge with edge_id 10
    pub(crate) weights: Option<Vec<WeightT>>,
    /// Vocabulary that save the mappings from string to index of every node type
    pub(crate) node_types: Option<VocabularyVec<NodeTypeT>>,
    // This is the next attribute that will be embedded inside of edges once
    // the first refactoring is done
    /// Vocabulary that save the mappings from string to index of every edge type
    pub(crate) edge_types: Option<VocabularyVec<EdgeTypeT>>,
}

/// # Graph utility methods
impl Graph {
    /// Returns node type of given node.
    ///
    /// # Arguments
    ///
    /// * node_id: NodeT - node whose node type is to be returned.
    ///
    /// # Examples
    /// ```rust
    /// # let graph = graph::test_utilities::load_ppi(true, true, true, true, false, false).unwrap();
    /// println!("The node type id of node {} is {}", 0, graph.get_node_type_id(0).unwrap());
    /// ```
    ///
    pub fn get_node_type_id(&self, node_id: NodeT) -> Result<NodeTypeT, String> {
        if let Some(nt) = &self.node_types {
            return if node_id <= nt.ids.len() as NodeT {
                Ok(nt.ids[node_id as usize])
            } else {
                Err(format!(
                    "The node_index {} is too big for the node_types vector which has len {}",
                    node_id,
                    nt.ids.len()
                ))
            };
        }
        Err(String::from(
            "Node types are not defined for current graph instance.",
        ))
    }

    /// Returns edge type of given edge.
    ///
    /// # Arguments
    ///
    /// * edge_id: EdgeT - edge whose edge type is to be returned.
    ///
    /// # Examples
    /// ```rust
    /// # let graph = graph::test_utilities::load_ppi(true, true, true, true, false, false).unwrap();
    /// println!("The edge type id of edge {} is {}", 0, graph.get_edge_type_id(0).unwrap());
    /// ```
    pub fn get_edge_type_id(&self, edge_id: EdgeT) -> Result<EdgeTypeT, String> {
        if let Some(et) = &self.edge_types {
            return if edge_id <= et.ids.len() as EdgeT {
                Ok(et.ids[edge_id as usize])
            } else {
                Err(format!(
                    "The edge_index {} is too big for the edge_types vector which has len {}",
                    edge_id,
                    et.ids.len()
                ))
            };
        }
        Err(String::from(
            "Edge types are not defined for current graph instance.",
        ))
    }

    pub fn get_unchecked_edge_id(
        &self,
        src: NodeT,
        dst: NodeT,
        edge_type: Option<EdgeTypeT>,
    ) -> EdgeT {
        if let Some(et) = edge_type {
            if let Some(ets) = &self.edge_types {
                return self
                    .get_unchecked_edge_ids_range(src, dst)
                    .find(|edge_id| ets.ids[*edge_id as usize] == et)
                    .unwrap();
            }
        }
        self.get_unchecked_edge_from_tuple(src, dst)
    }

    pub fn get_edge_id(
        &self,
        src: NodeT,
        dst: NodeT,
        edge_type: Option<EdgeTypeT>,
    ) -> Option<EdgeT> {
        if let Some(et) = edge_type {
            if let Some(ets) = &self.edge_types {
                return self.get_edge_ids(src, dst).and_then(|mut edge_ids| {
                    edge_ids.find(|edge_id| ets.ids[*edge_id as usize] == et)
                });
            }
        }
        self.get_edge_from_tuple(src, dst)
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
    pub fn get_edge_type_counts(&self) -> Result<HashMap<EdgeTypeT, usize>, String> {
        if let Some(et) = &self.edge_types {
            Ok(Counter::init(et.ids.clone()).into_map())
        } else {
            Err(String::from(
                "Edge types are not defined for current graph instance.",
            ))
        }
    }

    /// Return translated edge types from string to internal edge ID.
    ///
    /// # Arguments
    ///
    /// * `edge_types`: Vec<String> - Vector of edge types to be converted.
    pub fn translate_edge_types(&self, edge_types: Vec<String>) -> Result<Vec<EdgeTypeT>, String> {
        Ok(match &self.edge_types {
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
        }?)
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
    pub fn get_node_type_counts(&self) -> Result<HashMap<NodeTypeT, usize>, String> {
        if let Some(nt) = &self.node_types {
            Ok(Counter::init(nt.ids.clone()).into_map())
        } else {
            Err(String::from(
                "Node types are not defined for current graph instance.",
            ))
        }
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
        if self.edges.contains(self.encode_edge(src, dst)) {
            return match &edge_type {
                Some(et) => self
                    .get_unchecked_link_edge_types(src, dst)
                    .unwrap()
                    .contains(et),
                None => true,
            };
        }
        false
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
        if let Some(src) = self.nodes.get(src_name) {
            if let Some(dst) = self.nodes.get(dst_name) {
                let edge_type_id = edge_type_name.and_then(|etn| match &self.edge_types {
                    Some(ets) => ets.get(etn).copied(),
                    None => None,
                });
                if edge_type_id.is_none() && edge_type_name.is_some() {
                    return false;
                }
                return self.has_edge(*src, *dst, edge_type_id);
            }
        }
        false
    }

    /// Return true if given graph has any edge overlapping with current graph.
    ///
    /// # Arguments
    ///
    /// * graph: Graph - The graph to check against.
    ///
    pub fn overlaps(&self, graph: &Graph) -> bool {
        graph
            .get_edges_par_string_triples()
            .any(|(_, src, dst, et)| self.has_edge_string(&src, &dst, et.as_ref()))
    }

    /// Return true if given graph edges are all contained within current graph.
    ///
    /// # Arguments
    ///
    /// * graph: Graph - The graph to check against.
    ///
    pub fn contains(&self, graph: &Graph) -> bool {
        graph
            .get_edges_par_string_triples()
            .all(|(_, src, dst, et)| self.has_edge_string(&src, &dst, et.as_ref()))
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
            self.edges.unchecked_rank(self.encode_edge(src, dst)) as EdgeT,
            self.edges.unchecked_rank(self.encode_edge(src, dst + 1)) as EdgeT,
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
        if let Some(min_edge) = self.get_edge_from_tuple(src, dst) {
            let max_edge = self.get_unchecked_edge_from_tuple(src, dst + 1);
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
        (
            self.edges.unchecked_rank(self.encode_edge(src, 0)) as EdgeT,
            self.edges.unchecked_rank(self.encode_edge(src + 1, 0)) as EdgeT,
        )
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
        match self.get_edge_types_min_max_edge_ids(src, dst) {
            Some((min_edge_id, max_edge_id)) => Some(min_edge_id..max_edge_id),
            None => None,
        }
    }

    /// Returns edge_types associated to the given edge.
    /// A link is composed by all the edges that starts from src and ends at dst.
    ///
    /// # Arguments
    ///
    /// * `src`: NodeT - Integer ID of the source node.
    /// * `dst`: NodeT - Integer ID of the destination node.
    ///
    pub fn get_unchecked_link_edge_types(&self, src: NodeT, dst: NodeT) -> Option<Vec<EdgeTypeT>> {
        match &self.edge_types {
            Some(ets) => Some(
                self.get_unchecked_edge_ids_range(src, dst)
                    .map(|edge_id| ets.ids[edge_id as usize])
                    .collect(),
            ),
            None => None,
        }
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
        match &self.weights {
            Some(ws) => Some(
                self.get_unchecked_edge_ids_range(src, dst)
                    .map(|edge_id| ws[edge_id as usize])
                    .collect(),
            ),
            None => None,
        }
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
