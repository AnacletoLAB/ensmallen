use super::*;
use itertools::Itertools;
use rayon::iter::IntoParallelRefIterator;
use rayon::iter::ParallelIterator;
use std::collections::HashSet;

impl Graph {
    /// Return vector of tuple of Node IDs that form the edges of the required bipartite graph.
    ///
    /// # Arguments
    /// `removed_existing_edges`: Option<bool> - Wether to filter out the existing edges. By default, true.
    /// `first_nodes_set`: Option<HashMap<String>> - Optional set of nodes to use to create the first set of nodes of the graph.
    /// `second_nodes_set`: Option<HashMap<String>> - Optional set of nodes to use to create the second set of nodes of the graph.
    /// `first_node_types_set`: Option<HashMap<String>> - Optional set of node types to create the first set of nodes of the graph.
    /// `second_node_types_set`: Option<HashMap<String>> - Optional set of node types to create the second set of nodes of the graph.
    pub fn get_bipartite_edges(
        &self,
        removed_existing_edges: Option<bool>,
        first_nodes_set: Option<HashSet<String>>,
        second_nodes_set: Option<HashSet<String>>,
        first_node_types_set: Option<HashSet<String>>,
        second_node_types_set: Option<HashSet<String>>,
    ) -> Result<Vec<Vec<NodeT>>, String> {
        let removed_existing_edges_unwrapped = removed_existing_edges.unwrap_or(true);
        let (first_nodes, second_nodes): (Vec<NodeT>, Vec<NodeT>) = [
            (first_nodes_set, first_node_types_set),
            (second_nodes_set, second_node_types_set),
        ]
        .iter()
        .map(|(node_set, node_type_set)| {
            self.get_nodes_names_iter()
                .filter_map(|(node_id, node_name, node_type)| {
                    if let Some(ans) = &node_set {
                        if !ans.contains(&node_name) {
                            return None;
                        }
                    }
                    if let (Some(ants), Some(nt)) = (&node_type_set, &node_type) {
                        if nt
                            .iter()
                            .any(|node_type_name| !ants.contains(node_type_name))
                        {
                            return None;
                        }
                    }
                    Some(node_id)
                })
                .collect::<Vec<NodeT>>()
        })
        .collect_tuple()
        .unwrap();

        if first_nodes.is_empty() {
            return Err("The first nodes set of required bipartite graph is empty!".to_owned());
        }

        if second_nodes.is_empty() {
            return Err("The second nodes set of required bipartite graph is empty!".to_owned());
        }

        if first_nodes
            .par_iter()
            .any(|src| second_nodes.binary_search(src).is_ok())
        {
            return Err(
                "The giving node sets of the required bipartite graph have shared nodes."
                    .to_owned(),
            );
        }

        Ok(first_nodes
            .par_iter()
            .flat_map(|src| {
                second_nodes
                    .iter()
                    .filter_map(|dst| {
                        if removed_existing_edges_unwrapped && self.has_edge_with_type(*src, *dst, None) {
                            return None;
                        }
                        Some(vec![*src, *dst])
                    })
                    .collect::<Vec<Vec<NodeT>>>()
            })
            .collect())
    }

    /// Return vector of tuple of Node IDs that form the edges of the required bipartite graph.
    ///
    /// # Arguments
    /// `removed_existing_edges`: Option<bool> - Wether to filter out the existing edges. By default, true.
    /// `first_nodes_set`: Option<HashMap<String>> - Optional set of nodes to use to create the first set of nodes of the graph.
    /// `second_nodes_set`: Option<HashMap<String>> - Optional set of nodes to use to create the second set of nodes of the graph.
    /// `first_node_types_set`: Option<HashMap<String>> - Optional set of node types to create the first set of nodes of the graph.
    /// `second_node_types_set`: Option<HashMap<String>> - Optional set of node types to create the second set of nodes of the graph.
    pub fn get_bipartite_edge_names(
        &self,
        removed_existing_edges: Option<bool>,
        first_nodes_set: Option<HashSet<String>>,
        second_nodes_set: Option<HashSet<String>>,
        first_node_types_set: Option<HashSet<String>>,
        second_node_types_set: Option<HashSet<String>>,
    ) -> Result<Vec<Vec<String>>, String> {
        Ok(self
            .get_bipartite_edges(
                removed_existing_edges,
                first_nodes_set,
                second_nodes_set,
                first_node_types_set,
                second_node_types_set,
            )?
            .iter()
            .map(|nodes| {
                nodes
                    .iter()
                    .map(|node| self.get_node_name(*node).unwrap())
                    .collect::<Vec<String>>()
            })
            .collect::<Vec<Vec<String>>>())
    }

    /// Return vector of tuple of Node IDs that form the edges of the required star.
    ///
    /// # Arguments
    /// `central_node`: String - Name of the node to use as center of the star.
    /// `removed_existing_edges`: Option<bool> - Wether to filter out the existing edges. By default, true.
    /// `star_points_nodes_set`: Option<HashMap<String>> - Optional set of nodes to use to create the set of star points.
    /// `star_points_node_types_set`: Option<HashMap<String>> - Optional set of node types to create the set of star points.
    pub fn get_star_edges(
        &self,
        central_node: String,
        removed_existing_edges: Option<bool>,
        star_points_nodes_set: Option<HashSet<String>>,
        star_points_node_types_set: Option<HashSet<String>>,
    ) -> Result<Vec<Vec<NodeT>>, String> {
        self.get_bipartite_edges(
            removed_existing_edges,
            Some(
                vec![central_node]
                    .iter()
                    .cloned()
                    .collect::<HashSet<String>>(),
            ),
            star_points_nodes_set,
            None,
            star_points_node_types_set,
        )
    }

    /// Return vector of tuple of Node names that form the edges of the required star.
    ///
    /// # Arguments
    /// `central_node`: String - Name of the node to use as center of the star.
    /// `removed_existing_edges`: Option<bool> - Wether to filter out the existing edges. By default, true.
    /// `star_points_nodes_set`: Option<HashMap<String>> - Optional set of nodes to use to create the set of star points.
    /// `star_points_node_types_set`: Option<HashMap<String>> - Optional set of node types to create the set of star points.
    pub fn get_star_edge_names(
        &self,
        central_node: String,
        removed_existing_edges: Option<bool>,
        star_points_nodes_set: Option<HashSet<String>>,
        star_points_node_types_set: Option<HashSet<String>>,
    ) -> Result<Vec<Vec<String>>, String> {
        self.get_bipartite_edge_names(
            removed_existing_edges,
            Some(
                vec![central_node]
                    .iter()
                    .cloned()
                    .collect::<HashSet<String>>(),
            ),
            star_points_nodes_set,
            None,
            star_points_node_types_set,
        )
    }

    /// Return vector of tuple of Node IDs that form the edges of the required clique.
    ///
    /// # Arguments
    /// `directed`: Option<bool> - Wether to return the edges as directed or undirected. By default, equal to the graph.
    /// `allow_self_loops`: Option<bool> - Wether to allow self-loops in the clique. By default, equal to the graph.
    /// `removed_existing_edges`: Option<bool> - Wether to filter out the existing edges. By default, true.
    /// `allow_node_type_set`: Option<HashSet<String>> - Node types to include in the clique.
    /// `allow_node_set`: Option<HashSet<String>> - Nodes to include i the clique.
    pub fn get_clique_edges(
        &self,
        directed: Option<bool>,
        allow_self_loops: Option<bool>,
        removed_existing_edges: Option<bool>,
        allow_node_type_set: Option<HashSet<String>>,
        allow_node_set: Option<HashSet<String>>,
    ) -> Vec<Vec<NodeT>> {
        let directed_unwrapped = directed.unwrap_or(self.directed);
        let allow_self_loops_unwrapped = allow_self_loops.unwrap_or_else(|| self.has_selfloops());
        let removed_existing_edges_unwrapped = removed_existing_edges.unwrap_or(true);
        let nodes: Vec<NodeT> = self
            .get_nodes_names_iter()
            .filter_map(|(node_id, node_name, node_type)| {
                if let (Some(ants), Some(nt)) = (&allow_node_type_set, &node_type) {
                    if nt
                        .iter()
                        .any(|node_type_name| !ants.contains(node_type_name))
                    {
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
                        if !allow_self_loops_unwrapped && src == dst {
                            return None;
                        }
                        if !directed_unwrapped && src > dst {
                            return None;
                        }
                        if removed_existing_edges_unwrapped && self.has_edge_with_type(*src, *dst, None) {
                            return None;
                        }
                        Some(vec![*src, *dst])
                    })
                    .collect::<Vec<Vec<NodeT>>>()
            })
            .collect()
    }

    /// Return vector of tuple of Node names that form the edges of the required clique.
    ///
    /// # Arguments
    /// `directed`: Option<bool> - Wether to return the edges as directed or undirected. By default, equal to the graph.
    /// `allow_self_loops`: Option<bool> - Wether to allow self-loops in the clique. By default, equal to the graph.
    /// `removed_existing_edges`: Option<bool> - Wether to filter out the existing edges. By default, true.
    /// `allow_node_type_set`: Option<HashSet<String>> - Node types to include in the clique.
    /// `allow_node_set`: Option<HashSet<String>> - Nodes to include i the clique.
    pub fn get_clique_edge_names(
        &self,
        directed: Option<bool>,
        allow_self_loops: Option<bool>,
        removed_existing_edges: Option<bool>,
        allow_node_type_set: Option<HashSet<String>>,
        allow_node_set: Option<HashSet<String>>,
    ) -> Vec<Vec<String>> {
        self.get_clique_edges(
            directed,
            allow_self_loops,
            removed_existing_edges,
            allow_node_type_set,
            allow_node_set,
        )
        .iter()
        .map(|nodes| {
            nodes
                .iter()
                .map(|node| self.get_node_name(*node).unwrap())
                .collect::<Vec<String>>()
        })
        .collect::<Vec<Vec<String>>>()
    }
}
