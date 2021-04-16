use super::*;

impl Graph {
    /// Set the name of the graph.
    ///
    /// # Arguments
    ///
    /// * `name`: String - Name of the graph.
    pub fn set_name(&mut self, name: String) {
        self.invalidate_report();
        self.name = name;
    }

    /// Invalidate the cache for the textual report.
    /// This should be called as the first line of every methods that either get
    /// a mutable reference to self or get ownership of self.
    pub(crate) fn invalidate_report(&self) {
        *self.cached_report.write() = None;
    }

    /// Replace all edge types (if present) and set all the edge to edge_type.
    ///
    /// # Arguments
    /// - `edge_type`: String - The edge type to assing to all the edges.
    pub fn set_all_edge_types<S: Into<String>>(mut self, edge_type: S) -> Result<Graph, String> {
        self.invalidate_report();
        let mut vocabulary = Vocabulary::default();
        vocabulary.insert(edge_type.into())?;
        vocabulary.build_reverse_mapping()?;
        let edge_types = EdgeTypeVocabulary::from_structs(
            vec![Some(0); self.get_directed_edges_number() as usize],
            vocabulary,
        );
        self.edge_types = Some(edge_types);
        Ok(self)
    }

    /// Replace all node types (if present) and set all the node to node_type.
    ///
    /// # Arguments
    /// - `node_type`: String - The node type to assing to all the nodes.
    pub fn set_all_node_types<S: Into<String>>(mut self, node_type: S) -> Result<Graph, String> {
        self.invalidate_report();
        let mut vocabulary = Vocabulary::default();
        vocabulary.insert(node_type.into())?;
        vocabulary.build_reverse_mapping()?;
        let node_types = NodeTypeVocabulary::from_structs(
            vec![Some(vec![0]); self.get_nodes_number() as usize],
            Some(vocabulary),
        );
        self.node_types = node_types;
        Ok(self)
    }
}
