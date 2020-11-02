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
    pub fn set_all_edge_types(&self, edge_type: String) -> EnsmallenGraph {
        EnsmallenGraph {
            graph: self.graph.clone().set_all_edge_types(edge_type),
        }
    }

    #[text_signature = "($self, node_type)"]
    /// Drop all node types (if presents) and set all the node to node_type.
    ///
    /// Arguments
    /// ---------
    /// node_type: str,
    ///     The node type to assing to all the nodes.
    pub fn set_all_node_types(&self, node_type: String) -> EnsmallenGraph {
        EnsmallenGraph {
            graph: self.graph.clone().set_all_node_types(node_type),
        }
    }

    #[args(py_kwargs = "**")]
    #[text_signature = "($self, *, vector_destinations, vector_outbounds, cache_size)"]
    /// Enable fast walk, using more memory.
    ///
    /// Arguments
    /// ------------------
    /// vector_destinations: bool = True,
    ///     wether to cache destinations into a vector for faster walks.
    /// vector_outbounds: bool = True,
    ///     wether to cache outbounds into a vector for faster walks.
    /// cache_size: float = None,
    ///     Rate of nodes destinations to cache.
    ///     Must be a value between 0 and 1.
    ///     This cannot be used with the vector destinations.
    /// 
    /// Raises
    /// -------------------
    /// ValueError,
    ///     If the cache_size parameter is given and vector destinations is enabled.
    /// 
    pub fn enable_fast_walk(&mut self, py_kwargs: Option<&PyDict>) -> PyResult<()> {
        let py = pyo3::Python::acquire_gil();
        let kwargs = normalize_kwargs!(py_kwargs, py.python());
        pyex!(validate_kwargs(
            kwargs,
            ["vector_destinations", "vector_outbounds", "cache_size"]
                .iter()
                .map(|x| x.to_string())
                .collect(),
        ))?;

        pyex!(self.graph.enable_fast_walk(
            pyex!(extract_value!(kwargs, "vector_destinations", bool))?.unwrap_or(true),
            pyex!(extract_value!(kwargs, "vector_outbounds", bool))?.unwrap_or(true),
            pyex!(extract_value!(kwargs, "cache_size", f64))?,
        ))?;
        Ok(())
    }

    #[text_signature = "($self)"]
    /// Disable fast walk, using less memory.
    pub fn disable_fast_walk(&mut self) {
        self.graph.disable_fast_walk()
    }
}
