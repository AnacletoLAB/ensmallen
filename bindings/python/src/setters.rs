use super::*;

#[pymethods]
impl EnsmallenGraph {
    #[text_signature = "(self)"]
    /// Set the name of the graph.
    fn set_name(&mut self, name: String) {
        self.graph.set_name(name)
    }
}
