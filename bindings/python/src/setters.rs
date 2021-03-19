use super::*;

#[pymethods]
impl EnsmallenGraph {
    #[text_signature = "(self, name)"]
    /// Set the name of the graph.
    ///
    /// Parameters
    /// -----------------------
    /// name: str,
    ///     Name of the graph.
    fn set_name(&mut self, name: String) {
        self.graph.set_name(name)
    }
}
