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

    #[args(py_kwargs = "**")]
    #[text_signature = "($self, *, vector_destinations, vector_outbounds)"]
    /// Enable fast walk, using more memory.
    ///
    /// Arguments
    /// ------------------
    /// vector_destinations: bool,
    ///     wether to cache destinations into a vector for faster walks.
    /// vector_outbounds: bool,
    ///     wether to cache outbounds into a vector for faster walks.
    pub fn enable_fast_walk(&mut self, py_kwargs: Option<&PyDict>) -> PyResult<()> {
        let py = pyo3::Python::acquire_gil();
        let kwargs = normalize_kwargs!(py_kwargs, py.python());
        pyex!(validate_kwargs(
            kwargs,
            ["vector_destinations", "vector_outbounds"]
                .iter()
                .map(|x| x.to_string())
                .collect(),
        ))?;

        self.graph.enable_fast_walk(
            pyex!(extract_value!(kwargs, "vector_destinations", bool))?
                .or_else(|| Some(true))
                .unwrap(),
            pyex!(extract_value!(kwargs, "vector_outbounds", bool))?
                .or_else(|| Some(true))
                .unwrap(),
        );
        Ok(())
    }

    #[text_signature = "($self)"]
    /// Disable fast walk, using less memory.
    pub fn disable_fast_walk(&mut self) {
        self.graph.disable_fast_walk()
    }
}
