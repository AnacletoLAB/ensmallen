use super::*;
use indicatif::ProgressIterator;

impl Graph {
    /// Return whether nodes are remappable to those of the given graph.
    ///
    /// # Arguments
    /// * `other`: &Graph - graph towards remap the nodes to.
    ///
    /// # Example
    /// A graph is always remappable to itself:
    /// ```rust
    /// # let graph = graph::test_utilities::load_ppi(true, true, true, true, false, false);
    /// assert!(graph.are_nodes_remappable(&graph));
    /// ```
    /// Two different graphs, like Cora and STRING, are not remappable:
    /// ```rust
    /// # let cora = graph::test_utilities::load_cora();
    /// # let ppi = graph::test_utilities::load_ppi(true, true, true, true, false, false);
    /// assert!(!cora.are_nodes_remappable(&ppi));
    /// ```
    ///
    pub fn are_nodes_remappable(&self, other: &Graph) -> bool {
        if self.get_nodes_number() != other.get_nodes_number() {
            return false;
        }
        self.iter_node_names_and_node_type_names().all(|(_, node_name, _, node_type)| {
            other.has_node_name_and_node_type_name(&node_name, node_type)
        })
    }

    /// Return graph remapped towards nodes of the given graph.
    ///
    /// # Arguments
    ///
    /// * `other`: &Graph - The graph to remap towards.
    /// * `verbose`: bool - Whether to show a loding bar.
    ///
    /// # Example
    /// A graph is always remappable to itself:
    /// ```rust
    /// # let graph = graph::test_utilities::load_ppi(true, true, true, true, false, false);
    /// assert_eq!(graph, graph.remap(&graph, false).unwrap());
    /// ```
    ///
    pub fn remap(&self, other: &Graph, verbose: bool) -> Result<Graph, String> {
        let pb = get_loading_bar(
            verbose,
            format!("Building remapped {}", self.name).as_ref(),
            self.get_directed_edges_number() as usize,
        );

        if !self.are_nodes_remappable(&other) {
            return Err("The two graphs nodes sets are not remappable one-another.".to_owned());
        }

        Graph::from_integer_unsorted(
            self.iter_edge_node_names_and_edge_type_name_and_edge_weight(true)
                .progress_with(pb)
                .map(|(_, _, src_name, _, dst_name, _, edge_type, weight)| unsafe {
                    Ok((
                        other.get_unchecked_node_id_from_node_name(&src_name),
                        other.get_unchecked_node_id_from_node_name(&dst_name),
                        edge_type.and_then(|et| {
                            self.get_unchecked_edge_type_id_from_edge_type_name(et.as_str())
                        }),
                        weight,
                    ))
                }),
            other.nodes.clone(),
            other.node_types.clone(),
            self.edge_types.as_ref().map(|ets| ets.vocabulary.clone()),
            self.directed,
            self.name.clone(),
            false,
            self.has_edge_types(),
            self.has_edge_weights(),
            self.has_singleton_nodes(),
            self.has_singleton_nodes_with_selfloops(),
            self.has_trap_nodes(),
            verbose,
        )
    }
}
