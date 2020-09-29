use super::*;

/// # Drop.
impl Graph {
    /// Returns a **NEW** Graph that have no edge types.
    /// If the Graph have weights, the new unique edge will have a weight that
    /// equal to the sum of all the weights of the edges with same src and dst.
    pub fn drop_edge_types(&self) -> Result<Graph, String> {
        if self.edge_types.is_none() {
            return Err("Cannot drop edge types from a graph without edge types".to_string());
        }

        let mut unique_edges_tree = GraphDictionary::new();

        self.get_unique_edges_iter().for_each(|(src, dst)| {
            if !self.directed && src > dst {
                return;
            }
            let average_weight = if let Some(ws) = self.get_unchecked_link_weights(src, dst) {
                Some(ws.iter().sum::<f64>() / ws.len() as f64)
            } else {
                None
            };

            unique_edges_tree.extend(self, src, dst, None, average_weight, false);
        });

        Ok(build_graph(
            &mut unique_edges_tree,
            self.nodes.clone(),
            self.node_types.clone(),
            None,
            self.directed,
        ))
    }

    /// Returns a **NEW** Graph that have no weights.
    pub fn drop_weights(&self) -> Result<Graph, String> {
        if self.weights.is_none() {
            return Err("Cannot drop weights from a graph without weights".to_string());
        }

        let mut new = self.clone();
        new.weights = None;
        Ok(new)
    }

    /// Returns a **NEW** Graph that have no nodes types.
    pub fn drop_node_types(&self) -> Result<Graph, String> {
        if self.node_types.is_none() {
            return Err("Cannot drop node types from a graph without node types".to_string());
        }
        let mut new = self.clone();
        new.node_types = None;
        Ok(new)
    }
}
