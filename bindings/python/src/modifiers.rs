use super::*;

#[pymethods]
impl EnsmallenGraph {
    #[text_signature = "($self, edge_type)"]
    /// Drop all edge types (if presents) and set all the edge to edge_type.
    /// 
    /// Arguments
    /// ---------
    /// edge_type: str,
    ///     The edge type to assing to all the edges.
    pub fn set_all_edge_types(&mut self, edge_type: String) {
        self.graph.set_all_edge_types(edge_type);
    }

    /// Enable fast walk, using more memory.
    pub fn enable_fast_walk(&mut self) {
        self.graph.enable_fast_walk()
    }

    /// Disable fast walk, using less memory.
    pub fn disable_fast_walk(&mut self) {
        self.graph.disable_fast_walk()
    }
}