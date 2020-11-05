use super::*;
use rayon::iter::IntoParallelRefIterator;
use rayon::iter::ParallelIterator;
use std::collections::HashSet;

impl Graph {
    /// Return vector of tuple of Node IDs that form the edges of the required clique.
    ///
    /// # Arguments
    /// `directed`: Option<bool> - Wether to return the edges as directed or undirected. By default, equal to the graph.
    /// `removed_existing_edges`: <bool> - Wether to filter out the existing edges. By default, true.
    /// `allow_node_type_set`: Option<HashSet<String>> - Node types to include in the clique.
    /// `allow_node_set`: Option<HashSet<String>> - Nodes to include i the clique.
    pub fn get_bipartite_edges(
        &self,
        directed: Option<bool>,
        removed_existing_edges: Option<bool>,
        allow_node_type_set: Option<HashSet<String>>,
        allow_node_set: Option<HashSet<String>>,
    ) -> Vec<(NodeT, NodeT)> {
        let directed_unwrapped = directed.unwrap_or(self.directed);
        let removed_existing_edges_unwrapped = removed_existing_edges.unwrap_or(true);
        let nodes: Vec<NodeT> = self
            .get_nodes_names_iter()
            .filter_map(|(node_id, node_name, node_type)| {
                if let (Some(ants), Some(nt)) = (&allow_node_type_set, &node_type) {
                    if !ants.contains(nt) {
                        return None;
                    }
                }
                if let Some(ans) = &allow_node_set {
                    if !ans.contains(&node_name) {
                        return None;
                    }
                }
                Some(node_id)
            })
            .collect();

        nodes
            .par_iter()
            .flat_map(|src| {
                nodes
                    .iter()
                    .filter_map(|dst| {
                        if !directed_unwrapped && src > dst {
                            return None;
                        }
                        if removed_existing_edges_unwrapped && self.has_edge(*src, *dst, None) {
                            return None;
                        }
                        Some((*src, *dst))
                    })
                    .collect::<Vec<(NodeT, NodeT)>>()
            })
            .collect()
    }
    
    /// Return vector of tuple of Node IDs that form the edges of the required clique.
    ///
    /// # Arguments
    /// `directed`: Option<bool> - Wether to return the edges as directed or undirected. By default, equal to the graph.
    /// `removed_existing_edges`: <bool> - Wether to filter out the existing edges. By default, true.
    /// `allow_node_type_set`: Option<HashSet<String>> - Node types to include in the clique.
    /// `allow_node_set`: Option<HashSet<String>> - Nodes to include i the clique.
    pub fn get_clique_edges(
        &self,
        directed: Option<bool>,
        removed_existing_edges: Option<bool>,
        allow_node_type_set: Option<HashSet<String>>,
        allow_node_set: Option<HashSet<String>>,
    ) -> Vec<(NodeT, NodeT)> {
        let directed_unwrapped = directed.unwrap_or(self.directed);
        let removed_existing_edges_unwrapped = removed_existing_edges.unwrap_or(true);
        let nodes: Vec<NodeT> = self
            .get_nodes_names_iter()
            .filter_map(|(node_id, node_name, node_type)| {
                if let (Some(ants), Some(nt)) = (&allow_node_type_set, &node_type) {
                    if !ants.contains(nt) {
                        return None;
                    }
                }
                if let Some(ans) = &allow_node_set {
                    if !ans.contains(&node_name) {
                        return None;
                    }
                }
                Some(node_id)
            })
            .collect();

        nodes
            .par_iter()
            .flat_map(|src| {
                nodes
                    .iter()
                    .filter_map(|dst| {
                        if !directed_unwrapped && src > dst {
                            return None;
                        }
                        if removed_existing_edges_unwrapped && self.has_edge(*src, *dst, None) {
                            return None;
                        }
                        Some((*src, *dst))
                    })
                    .collect::<Vec<(NodeT, NodeT)>>()
            })
            .collect()
    }

    /// Return vector of tuple of Node names that form the edges of the required clique.
    ///
    /// # Arguments
    /// `directed`: Option<bool> - Wether to return the edges as directed or undirected. By default, equal to the graph.
    /// `removed_existing_edges`: <bool> - Wether to filter out the existing edges. By default, true.
    /// `allow_node_type_set`: Option<HashSet<String>> - Node types to include in the clique.
    /// `allow_node_set`: Option<HashSet<String>> - Nodes to include i the clique.
    pub fn get_clique_edge_names(
        &self,
        directed: Option<bool>,
        removed_existing_edges: Option<bool>,
        allow_node_type_set: Option<HashSet<String>>,
        allow_node_set: Option<HashSet<String>>,
    ) -> Vec<(String, String)> {
        self.get_clique_edges(
            directed,
            removed_existing_edges,
            allow_node_type_set,
            allow_node_set,
        )
        .iter()
        .map(|(src, dst)| {
            (
                self.get_node_name(*src).unwrap(),
                self.get_node_name(*dst).unwrap(),
            )
        })
        .collect::<Vec<(String, String)>>()
    }
}
