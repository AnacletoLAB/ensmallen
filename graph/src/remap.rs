use super::*;
use indicatif::ProgressIterator;

impl Graph {
    /// Return wether nodes are remappable to those of the given graph.
    ///
    /// # Arguments
    /// other: &Grap - graph towards remap the nodes to.
    pub fn are_nodes_remappable(&self, other: &Graph) -> bool {
        if self.get_nodes_number() != other.get_nodes_number() {
            return false;
        }
        self.get_nodes_names_iter()
            .all(|(node_name, node_type)| other.has_node_string(&node_name, node_type))
    }

    /// Return graph remapped towards nodes of the given graph.
    ///
    /// # Arguments
    ///
    /// * other: Graph - The graph to remap towards.
    /// * verbose: bool - Wether to show a loding bar.
    ///
    pub fn remap(&self, other: &Graph, verbose: bool) -> Result<Graph, String> {
        let pb = get_loading_bar(
            verbose,
            format!("Building remapped {}", self.name).as_ref(),
            self.get_edges_number() as usize,
        );

        if !self.are_nodes_remappable(&other) {
            return Err("The two graphs nodes sets are not remappable one-another.".to_owned());
        }

        Graph::from_integer_unsorted(
            self.get_edges_string_quadruples().progress_with(pb).map(
                |(_, src, dst, edge_type, weight)| {
                    Ok((
                        other.get_unchecked_node_id(&src),
                        other.get_unchecked_node_id(&dst),
                        edge_type.and_then(|et| self.get_unchecked_edge_type_id(Some(et.as_str()))),
                        weight,
                    ))
                },
            ),
            other.nodes.clone(),
            other.node_types.clone(),
            match &self.edge_types {
                Some(ets) => Some(ets.vocabulary.clone()),
                None => None,
            },
            self.directed,
            self.name.clone(),
            false,
            verbose,
        )
    }
}
