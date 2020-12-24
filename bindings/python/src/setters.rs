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

    #[text_signature = "(self, embedding)"]
    /// Set the embedding of the graph.
    ///
    /// Parameters
    /// -----------------------
    /// embedding: np.ndarray,
    ///     Embedding of the graph.
    fn set_embedding(&mut self, embedding: Vec<Vec<f64>>) -> PyResult<()> {
        pyex!(self.graph.set_embedding(embedding))
    }
}
