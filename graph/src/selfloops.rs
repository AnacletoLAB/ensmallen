use super::*;

/// # Selfloops.
impl Graph {
    /// Returns new graph with added in missing self-loops with given edge type and weight.
    ///
    /// # Arguments
    /// `edge_type_name`: Option<&str> - The edge type to use for the selfloops.
    /// `weight`: Option<WeightT> - The weight to use for the new selfloops edges.
    /// `verbose`: Option<bool> - Whether to show loading bars while building the graph.
    ///
    /// # Raises
    /// * If the edge type for the new singletons is provided but the graph does not have edge types.
    /// * If the edge weight for the new singletons is provided but the graph does not have edge weights.
    /// * If the edge weight for the new singletons is NOT provided but the graph does have edge weights.
    pub fn add_selfloops(
        &self,
        edge_type_name: Option<&str>,
        weight: Option<WeightT>,
        verbose: Option<bool>,
    ) -> Result<Graph, String> {
        let verbose = verbose.unwrap_or(true);
        let edge_type_id = if edge_type_name.is_some() {
            self.get_edge_type_id_from_edge_type_name(edge_type_name)?
        } else {
            None
        };
        if weight.is_some() ^ self.has_edge_weights() {
            return Err(concat!(
                "The weight for the self-loops must be specified ",
                "only and exclusively if the graph has edge weights."
            )
            .to_string());
        }
        Graph::from_integer_unsorted(
            self.iter_edge_node_ids_and_edge_type_id_and_edge_weight(true)
                .map(|(_, src, dst, edge_type_id, weight)| Ok((src, dst, edge_type_id, weight)))
                .chain(
                    self.iter_node_ids()
                        .filter(|&node_id| !self.has_selfloop_from_node_id(node_id))
                        .map(|node_id| Ok((node_id, node_id, edge_type_id, weight))),
                ),
            self.nodes.clone(),
            self.node_types.clone(),
            self.edge_types.as_ref().map(|ets| ets.vocabulary.clone()),
            self.is_directed(),
            self.get_name(),
            true,
            self.has_edge_types(),
            self.has_edge_weights(),
            false,
            false,
            self.has_singleton_nodes_with_selfloops() || self.has_singleton_nodes(),
            self.has_trap_nodes(),
            verbose,
        )
    }
}
