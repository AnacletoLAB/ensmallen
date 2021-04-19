use super::*;
use graph::{EdgeT, NodeT};
use std::collections::HashSet;

#[pymethods]
impl EnsmallenGraph {
    #[args(py_kwargs = "**")]
    #[text_signature = "($self, *, node_names, node_types, edge_types, minimum_component_size, top_k_components, verbose)"]
    /// remove all the components that are not connected to interesting
    /// nodes and edges.
    ///
    /// Parameters
    /// --------------------
    /// node_names: List[str] = None,
    ///     The name of the nodes of which components to keep.
    /// node_types: List[str] = None,
    ///     The types of the nodes of which components to keep.
    /// edge_types: List[str] = None,
    ///     The types of the edges of which components to keep.
    /// minimum_component_size: int = None,
    ///     Minimum size of the components to keep.
    /// top_k_components: int = None,
    ///     Number of components to keep sorted by number of nodes.
    /// verbose: bool = True,
    ///     Wether to show the loading bar.
    ///
    /// Returns
    /// ---------------------
    /// Graph composed only of filtered components.
    fn remove_components(&self, py_kwargs: Option<&PyDict>) -> PyResult<EnsmallenGraph> {
        let py = pyo3::Python::acquire_gil();
        let kwargs = normalize_kwargs!(py_kwargs, py.python());
        pe!(validate_kwargs(
            kwargs,
            &[
                "node_names",
                "node_types",
                "edge_types",
                "minimum_component_size",
                "top_k_components",
                "verbose"
            ]
        ))?;

        Ok(EnsmallenGraph {
            graph: pe!(self.graph.remove_components(
                extract_value!(kwargs, "node_names", Vec<String>),
                extract_value!(kwargs, "node_types", Vec<Option<String>>),
                extract_value!(kwargs, "edge_types", Vec<Option<String>>),
                extract_value!(kwargs, "minimum_component_size", NodeT),
                extract_value!(kwargs, "top_k_components", NodeT),
                extract_value!(kwargs, "verbose", bool).unwrap_or(true),
            ))?,
        })
    }
}
