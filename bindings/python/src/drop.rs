use super::*;

#[pymethods]
impl EnsmallenGraph {
    #[args(py_kwargs = "**")]
    #[text_signature = "($self)"]
    /// Returns new graph without edge types.
    ///
    /// Raises
    /// -----------------------------
    /// ValueError,
    ///     If the given graph does not have edge types.
    ///
    /// Returns
    /// -----------------------------
    /// Cloned graph without edge types.
    fn drop_edge_types(&self) -> PyResult<EnsmallenGraph> {
        Ok(EnsmallenGraph {
            graph: pyex!(self.graph.drop_edge_types())?,
        })
    }

    #[args(py_kwargs = "**")]
    #[text_signature = "($self, verbose)"]
    /// Returns new graph without singleton nodes.
    ///
    /// Note that this will change the internal nodes mapping.
    ///
    /// Parameters
    /// -----------------------------
    /// verbose: bool = True,
    ///     Wether to show a loading bar.
    ///
    /// Returns
    /// -----------------------------
    /// Cloned graph without singleton nodes.
    fn drop_singletons(&self, verbose: Option<bool>) -> PyResult<EnsmallenGraph> {
        Ok(EnsmallenGraph {
            graph: pyex!(self
                .graph
                .drop_singletons(verbose.unwrap_or(true)))?,
        })
    }

    #[args(py_kwargs = "**")]
    #[text_signature = "($self)"]
    /// Returns new graph without weights.
    ///
    /// Raises
    /// -----------------------------
    /// ValueError,
    ///     If the given graph does not have weights.
    ///
    /// Returns
    /// -----------------------------
    /// Cloned graph without weights.
    fn drop_weights(&self) -> PyResult<EnsmallenGraph> {
        Ok(EnsmallenGraph {
            graph: pyex!(self.graph.drop_weights())?,
        })
    }

    #[args(py_kwargs = "**")]
    #[text_signature = "($self)"]
    /// Returns new graph without node types.
    ///
    /// Raises
    /// -----------------------------
    /// ValueError,
    ///     If the given graph does not have node types.
    ///
    /// Returns
    /// -----------------------------
    /// Cloned graph without node types.
    fn drop_node_types(&self) -> PyResult<EnsmallenGraph> {
        Ok(EnsmallenGraph {
            graph: pyex!(self.graph.drop_node_types())?,
        })
    }

    #[args(py_kwargs = "**")]
    #[text_signature = "($self, *, node_names, node_types, edge_types, verbose)"]
    /// Drop all the components that are not connected to interesting
    /// nodes and edges.
    /// 
    /// Parameters
    /// ----------
    /// node_names: List[str],
    ///     The name of the nodes of which components to keep
    /// node_types: List[str],
    ///     The types of the nodes of which components to keep
    /// edge_types: List[str],
    ///     The types of the edges of which components to keep
    /// 
    fn drop_components(&self, py_kwargs: Option<&PyDict>) -> PyResult<EnsmallenGraph> {
        let py = pyo3::Python::acquire_gil();
        let kwargs = normalize_kwargs!(py_kwargs, py.python());
        pyex!(validate_kwargs(
            kwargs,
            build_walk_parameters_list(&[
                "node_names",
                "node_types",
                "edge_types",
                "verbose"
            ]),
        ))?;

        Ok(EnsmallenGraph {
            graph: pyex!(self.graph.drop_components(
                pyex!(extract_value!(kwargs, "node_names", Vec<String>))?,
                pyex!(extract_value!(kwargs, "node_types", Vec<String>))?,
                pyex!(extract_value!(kwargs, "edge_types", Vec<String>))?,
                pyex!(extract_value!(kwargs, "verbose", bool))?.unwrap_or(true),
            ))?,
        })
    }


    
}
