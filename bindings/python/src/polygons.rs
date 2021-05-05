use super::*;
use graph::NodeT;
use std::collections::HashSet;

#[pymethods]
impl EnsmallenGraph {
    #[text_signature = "($self)"]
    /// Returns number of triangles in the graph.
    ///
    /// [Automatically generated binding]
    /// [Automatically generated documentation]
    fn get_triangles_number(&self) -> EdgeT {
        self.graph.get_triangles_number()
    }
}
