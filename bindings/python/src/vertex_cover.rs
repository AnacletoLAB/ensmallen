use super::*;
use graph::NodeT;
use std::collections::HashSet;

#[pymethods]
impl EnsmallenGraph {
    #[text_signature = "($self, verbose)"]
    /// Returns a 2-approximated vertex cover.
    ///
    /// # Arguments
    /// * `verbose`: Option<bool> - Whether to show a loading bar.
    ///
    /// [Automatically generated binding]
    /// [Automatically generated documentation]
    fn approximated_vertex_cover(&self, verbose: Option<bool>) -> Vec<NodeT> {
        self.graph.approximated_vertex_cover(verbose)
    }
}
