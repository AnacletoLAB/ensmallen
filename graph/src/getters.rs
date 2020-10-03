use super::*;
use itertools::Itertools;
use rayon::prelude::*;
use std::collections::HashMap;
use std::iter::once;

impl Graph {
    /// Return the number of traps (nodes without any outgoing edges that are not singletons)
    pub fn get_traps_number(&self) -> EdgeT {
        self.not_singleton_nodes_number as EdgeT - self.unique_sources.len() as EdgeT
    }

    // Return if the graph has traps or not
    pub fn has_traps(&self) -> bool {
        self.get_traps_number() > 0
    }

    /// Returns boolean representing if graph is directed.
    pub fn is_directed(&self) -> bool {
        self.directed
    }

    /// Returns boolean representing if graph has weights.
    pub fn has_weights(&self) -> bool {
        self.weights.is_some()
    }

    /// Returns boolean representing if graph has edge types.
    pub fn has_edge_types(&self) -> bool {
        self.edge_types.is_some()
    }

    /// Returns boolean representing if graph has self-loops.
    pub fn has_selfloops(&self) -> bool {
        self.self_loop_number > 0
    }

    /// Returs option with the edge type of the given edge id.
    pub fn get_edge_type(&self, edge_id: EdgeT) -> Option<EdgeTypeT> {
        match &self.edge_types {
            Some(ets) => Some(ets.ids[edge_id as usize]),
            None => None,
        }
    }

    /// Returs option with the node type of the given node id.
    pub fn get_node_type(&self, node_id: NodeT) -> Option<NodeTypeT> {
        match &self.node_types {
            Some(nts) => Some(nts.ids[node_id as usize]),
            None => None,
        }
    }

    /// Returs option with the node type of the given node id.
    pub fn get_node_type_string(&self, node_id: NodeT) -> Option<String> {
        match &self.node_types {
            Some(nts) => Some(
                nts.translate(self.get_node_type(node_id).unwrap())
                    .to_owned(),
            ),
            None => None,
        }
    }

    /// Returs option with the edge type of the given edge id.
    pub fn get_edge_type_string(&self, edge_id: EdgeT) -> Option<String> {
        match &self.edge_types {
            Some(ets) => Some(
                ets.translate(self.get_edge_type(edge_id).unwrap())
                    .to_owned(),
            ),
            None => None,
        }
    }

    /// Returs option with the weight of the given edge id.
    pub fn get_edge_weight(&self, edge_id: EdgeT) -> Option<WeightT> {
        match &self.weights {
            Some(ws) => Some(ws[edge_id as usize]),
            None => None,
        }
    }

    /// Returns boolean representing if graph has node types.
    pub fn has_node_types(&self) -> bool {
        self.node_types.is_some()
    }

    /// Returns number of nodes in the graph.
    pub fn get_nodes_number(&self) -> NodeT {
        self.nodes.len() as NodeT
    }

    /// Returns number of edges in the graph.
    pub fn get_edges_number(&self) -> EdgeT {
        self.edges.len() as EdgeT
    }

    /// Returns number of edge types in the graph.
    pub fn get_edge_types_number(&self) -> EdgeTypeT {
        if let Some(etm) = &self.edge_types {
            etm.len() as EdgeTypeT
        } else {
            0
        }
    }

    /// Returns number of node types in the graph.
    pub fn get_node_types_number(&self) -> NodeTypeT {
        if let Some(etm) = &self.node_types {
            etm.len() as NodeTypeT
        } else {
            0
        }
    }

    /// Returns the degree of every node in the graph.
    pub fn get_node_degrees(&self) -> Vec<NodeT> {
        (0..self.get_nodes_number())
            .into_par_iter()
            .map(|node| self.get_node_degree(node as NodeT))
            .collect::<Vec<NodeT>>()
    }

    pub fn get_not_singletons(&self) -> Vec<NodeT> {
        self.get_edges_iter()
            .flat_map(|(_, src, dst)| once(src).chain(once(dst)))
            .unique()
            .collect()
    }

    /// Return mapping from instance not trap nodes to dense nodes.
    pub fn get_dense_node_mapping(&self) -> HashMap<NodeT, NodeT> {
        self.get_not_singletons()
            .iter()
            .cloned()
            .enumerate()
            .map(|(i, node)| (node as NodeT, i as NodeT))
            .collect()
    }

    pub fn get_edge_type_number(&self, edge_type: EdgeTypeT) -> EdgeTypeT {
        match &self.edge_types {
            None => 0,
            Some(ets) => ets.counts[edge_type as usize] as EdgeTypeT,
        }
    }

    /// Return if there are multiple edges between two nodes
    pub fn is_multigraph(&self) -> bool {
        self.unique_edges_number != self.get_edges_number()
    }

    pub fn get_destination(&self, edge_id: EdgeT) -> NodeT {
        self.get_edge_from_edge_id(edge_id).1
    }

    pub fn get_destinations_range(
        &self,
        min_edge_id: EdgeT,
        max_edge_id: EdgeT,
    ) -> impl Iterator<Item = NodeT> + '_ {
        (min_edge_id..max_edge_id).map(move |edge_id| self.get_destination(edge_id))
    }

    pub fn get_source_destinations_range(&self, src: NodeT) -> impl Iterator<Item = NodeT> + '_ {
        self.get_unchecked_destinations_range(src)
            .map(move |edge_id| self.get_destination(edge_id))
    }

    pub fn get_unique_sources_number(&self) -> NodeT {
        self.unique_sources.len() as NodeT
    }
}
