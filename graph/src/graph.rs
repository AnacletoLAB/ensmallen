//! A graph representation optimized for executing random walks on huge graphs.
use super::*;
use counter::Counter;
use itertools::Itertools;
use derive_getters::Getters;
use rayon::prelude::*;
use std::collections::HashMap;

/// A graph representation optimized for executing random walks on huge graphs.
///
/// This class should be initialized using the two constructors:
/// `graph::Graph::new_directed` or `graph::Graph::new_undirected`
///
/// # Examples
///
#[derive(Clone, Getters, PartialEq)]
pub struct Graph {
    // properties
    /// if the graph has traps or not
    pub(crate) has_traps: bool,
    /// if the graph is directed or undirected
    pub(crate) is_directed: bool,
    /// how many singoletons (nodes without any incoming or outgoing edges)
    /// are present in the graph
    pub(crate) singletons_number: NodeT,
    
    // graph structs
    /// vector with the sources of every edge.
    /// sources[10] returns the source of the edge with edge_id 10
    pub(crate) sources: Vec<NodeT>,
    /// vector with the destinations of every edge.
    /// destinations[10] returns the destination of the edge with edge_id 10
    pub(crate) destinations: Vec<NodeT>,
    /// Vocabulary that save the mappings from string to index of every node
    pub(crate) nodes: Vocabulary<NodeT>,
    /// Optional vector of the weights of every edge.
    /// weights[10] return the weight of the edge with edge_id 10
    pub(crate) weights: Option<Vec<WeightT>>,
    /// Vocabulary that save the mappings from string to index of every node type
    pub(crate) node_types: Option<VocabularyVec<NodeTypeT>>,
    /// Vocabulary that save the mappings from string to index of every edge type
    pub(crate) edge_types: Option<VocabularyVec<EdgeTypeT>>,

    // helper structs
    /// Vector that has the cumulative sum of the degree of each node.
    /// This is used as an offset array to quickly retreive the outgoing edges
    pub(crate) outbounds: Vec<EdgeT>,
    /// Hashmap with as keys (src, dst) and the id of the first edge from src to dst (just the first since 
    /// all these edges have consecutive edge_ids) and a set of edge types present.
    pub(crate) unique_edges: HashMap<(NodeT, NodeT), EdgeMetadata>,
    /// All the nodes that are not traps.
    /// This is used to speed up the walk.
    pub(crate) not_trap_nodes: Vec<NodeT>,
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
            return if node_id <= nt.ids.len() {
                Ok(nt.ids[node_id])
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
            return if edge_id <= et.ids.len() {
                Ok(et.ids[edge_id])
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
    ///
    pub fn has_edge(&self, src: NodeT, dst: NodeT) -> bool {
        self.unique_edges.contains_key(&(src, dst))
    }

    /// Returns a boolean representing if the graph contains an edge that has
    /// source == destination.
    pub fn has_selfloops(&self) -> bool {
        self.sources
            .iter()
            .zip(self.destinations.iter())
            .any(|(src, dst)| src == dst)
    }

    /// Returns a boolean representing if the graph contains a pair of nodes
    /// which have edges of multiple types.
    pub fn is_multigraph(&self) -> bool {
        self.unique_edges.values().any(|data| {
            if let Some(edt) = &data.edge_types {
                edt.len() > 1
            } else {
                false
            }
        })
    }

    /// Private method that check if a triple (src, dst, edge_type) is present in another graph.
    /// This is used in overlaps and contains and it must be a method because we need to convert
    /// from the indexing of one graph to the other.
    ///
    /// # Arguments
    /// * src: NodeT - The source of the edge
    /// * dst: NodeT - The destination of the edge
    /// * et: Option<EdgeTypeT> - The optional edge type of the edge.
    ///
    pub(crate) fn check_edge_overlap(&self, src: NodeT, dst: NodeT, et: Option<EdgeTypeT>) -> bool {
        match self.unique_edges.get(&(src, dst)) {
            Some(metadata) => match &metadata.edge_types {
                Some(ets) => ets.contains(&et.unwrap()),
                None => true,
            },
            None => false,
        }
    }

    /// Return true if given graph has any edge overlapping with current graph.
    ///
    /// # Arguments
    ///
    /// * graph: Graph - The graph to check against.
    ///
    pub fn overlaps(&self, graph: &Graph) -> Result<bool, String> {
        if self.has_edge_types() ^ graph.has_edge_types() {
            return Err("One of the graph has edge types while the other has not. This is an undefined behaviour.".to_string());
        }

        Ok(graph
            .sources
            .par_iter()
            .zip(graph.destinations.par_iter())
            .enumerate()
            .map(|(edge_id, (src, dst))| {
                (
                    src,
                    dst,
                    match &graph.edge_types {
                        Some(et) => {
                            // The ids list can be empty with a filled vocabulary when
                            // handling negative edges graphs.
                            if et.ids.is_empty() {
                                None
                            } else {
                                Some(et.ids[edge_id])
                            }
                        }
                        None => None,
                    },
                )
            })
            .any(|(src, dst, et)| self.check_edge_overlap(*src, *dst, et)))
    }

    /// Return true if given graph edges are all contained within current graph.
    ///
    /// # Arguments
    ///
    /// * graph: Graph - The graph to check against.
    ///
    pub fn contains(&self, graph: &Graph) -> Result<bool, String> {
        if self.edge_types.is_some() ^ graph.edge_types.is_some() {
            return Err("One of the graph has edge types while the other has not. This is an undefined behaviour.".to_string());
        }

        Ok(graph
            .sources
            .par_iter()
            .zip(graph.destinations.par_iter())
            .enumerate()
            .map(|(edge_id, (src, dst))| {
                (
                    src,
                    dst,
                    match &graph.edge_types {
                        Some(et) => Some(et.ids[edge_id]),
                        None => None,
                    },
                )
            })
            .all(|(src, dst, et)| self.check_edge_overlap(*src, *dst, et)))
    }

    /// Returns number of nodes in the graph.
    pub fn get_nodes_number(&self) -> usize {
        self.nodes.len()
    }

    /// Returns number of not node nodes in the graph.
    pub fn get_not_trap_nodes_number(&self) -> usize {
        self.not_trap_nodes.len()
    }

    /// Returns number of edges in the graph.
    pub fn get_edges_number(&self) -> usize {
        self.sources.len()
    }

    /// Returns the number of edges (ignoring the edge type)
    pub fn get_unique_edges_number(&self) -> usize {
        self.unique_edges.len()
    }

    /// Returns number of edge types in the graph.
    pub fn get_edge_types_number(&self) -> usize {
        if let Some(etm) = &self.edge_types {
            etm.len()
        } else {
            0
        }
    }

    /// Returns number of node types in the graph.
    pub fn get_node_types_number(&self) -> usize {
        if let Some(etm) = &self.node_types {
            etm.len()
        } else {
            0
        }
    }

    /// Return range of outbound edges IDs for given Node.
    ///
    /// # Arguments
    ///
    /// * node: NodeT - Node for which we need to compute the outbounds range.
    ///
    pub(crate) fn get_min_max_edge(&self, node: NodeT) -> (EdgeT, EdgeT) {
        let min_edge: EdgeT = if node == 0 {
            0
        } else {
            self.outbounds[node - 1]
        };
        let max_edge: EdgeT = self.outbounds[node];
        (min_edge, max_edge)
    }

    /// Return mapping from instance not trap nodes to dense nodes.
    pub fn get_dense_nodes_mapping(&self) -> HashMap<NodeT, NodeT> {
        self.sources
            .iter()
            .chain(self.destinations.iter())
            .cloned()
            .unique()
            .enumerate()
            .map(|(i, node)| (node, i))
            .collect()
    }

    /// Returns the number of outbound neighbours of given node.
    ///
    ///
    /// # Arguments
    ///
    /// * `node` - Integer ID of the node.
    ///
    pub fn degree(&self, node: NodeT) -> NodeT {
        let (_min, _max) = self.get_min_max_edge(node);
        _max - _min
    }

    /// Returns the degree of every node in the graph.
    pub fn degrees(&self) -> Vec<NodeT> {
        (0..self.get_nodes_number())
            .into_par_iter()
            .map(|node| self.degree(node))
            .collect()
    }

    // Return a vector with the ids of all the edges that start from src
    // and ends at dst. This is meaningful on multigraphs.
    /// A link is composed by all the edges that starts from src and ends at dst.
    ///
    /// # Arguments
    ///
    /// * `src`: NodeT - Integer ID of the source node.
    /// * `dst`: NodeT - Integer ID of the destination node.
    ///
    pub fn get_link_ids(&self, src: NodeT, dst: NodeT) -> Option<Vec<EdgeT>> {
        match self.unique_edges.get(&(src, dst)) {
            Some(metadata) => {
                let edge_id = metadata.edge_id;
                let number_of_types = match &metadata.edge_types {
                    Some(et) => et.len(),
                    None => 1,
                };
                Some((edge_id..edge_id + number_of_types).collect())
            }
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
    pub fn get_link_edge_types(&self, src: NodeT, dst: NodeT) -> Option<Vec<EdgeTypeT>> {
        if let Some(ets) = &self.edge_types {
            match self.get_link_ids(src, dst) {
                Some(ids) => Some(ids.iter().map(|i| ets.ids[*i]).collect()),
                None => None,
            }
        } else {
            None
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
    pub fn get_link_weights(&self, src: NodeT, dst: NodeT) -> Option<Vec<WeightT>> {
        if let Some(w) = &self.weights {
            match self.get_link_ids(src, dst) {
                Some(ids) => Some(ids.iter().map(|i| w[*i]).collect()),
                None => None,
            }
        } else {
            None
        }
    }

    /// Returns boolean representing if given node is a trap.
    ///
    /// # Arguments
    ///
    /// * `node` - Integer ID of the node, if this is bigger that the number of nodes it will panic.
    ///
    pub fn is_node_trap(&self, node: NodeT) -> bool {
        self.degree(node) == 0
    }
    /// Returns boolean representing if given edge is a trap.
    ///
    /// # Arguments
    ///
    /// * `edge` - Integer ID of the edge, if this is bigger that the number of edges it will panic.
    ///
    pub fn is_edge_trap(&self, edge: EdgeT) -> bool {
        self.is_node_trap(self.destinations[edge])
    }

    /// Returns list of neigbours of given node.
    ///
    /// # Arguments
    ///
    /// * `node` - Integer ID of the node, if this is bigger that the number of nodes it will panic.
    ///
    pub fn get_node_neighbours(&self, node: NodeT) -> Vec<NodeT> {
        let (min_edge, max_edge) = self.get_min_max_edge(node);
        self.destinations[min_edge..max_edge].to_vec()
    }
}
