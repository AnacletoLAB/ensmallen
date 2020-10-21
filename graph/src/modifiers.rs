use super::*;

impl Graph {
    /// Replace all edge types (if present) and set all the edge to edge_type.
    ///
    /// # Arguments
    /// - `edge_type`: String - The edge type to assing to all the edges.
    pub fn set_all_edge_types(mut self, edge_type: String) -> Graph {
        let mut vocabulary = Vocabulary::new(false);
        vocabulary.insert(edge_type).unwrap();
        vocabulary.build_reverse_mapping().unwrap();
        let edge_types = VocabularyVec::from_structs(
            vec![0; self.get_edges_number() as usize],
            Some(vocabulary),
        );
        self.edge_types = edge_types;
        self
    }

    /// Replace all node types (if present) and set all the node to node_type.
    ///
    /// # Arguments
    /// - `node_type`: String - The node type to assing to all the nodes.
    pub fn set_all_node_types(mut self, node_type: String) -> Graph {
        let mut vocabulary = Vocabulary::new(false);
        vocabulary.insert(node_type).unwrap();
        vocabulary.build_reverse_mapping().unwrap();
        let node_types = VocabularyVec::from_structs(
            vec![0; self.get_nodes_number() as usize],
            Some(vocabulary),
        );
        self.node_types = node_types;
        self
    }

    /// Enable fast walk, using more memory.
    ///
    /// # Arguments
    /// - `vector_destinations`: bool, wether to cache destinations into a vector for faster walks.
    /// - `vector_outbounds`: bool, wether to cache outbounds into a vector for faster walks.
    pub fn enable_fast_walk(&mut self, vector_destinations: bool, vector_outbounds: bool) {
        if vector_destinations {
            self.destinations = Some(self.get_destinations());
        }
        if vector_outbounds {
            self.outbounds = Some(self.get_outbounds());
        }
    }

    /// Disable fast walk, using less memory.
    pub fn disable_fast_walk(&mut self) {
        self.destinations = None;
    }
}
