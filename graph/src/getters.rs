use super::*;

impl Graph{
    
    /// Returns boolean representing if graph has weights.
    pub fn has_weights(&self) -> bool {
        self.weights.is_some()
    }

    /// Returns boolean representing if graph has edge types.
    pub fn has_edge_types(&self) -> bool {
        self.edge_types.is_some()
    }

    /// Returns boolean representing if graph has node types.
    pub fn has_node_types(&self) -> bool {
        self.node_types.is_some()
    }
}