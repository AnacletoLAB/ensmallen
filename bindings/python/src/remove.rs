use super::*;
use graph::{EdgeT, NodeT};
use std::collections::HashSet;

#[pymethods]
impl EnsmallenGraph {
    #[args(py_kwargs = "**")]
    #[text_signature = "($self, *, allow_nodes_set, deny_nodes_set, allow_node_types_set, deny_node_types_set,  allow_edge_set, deny_edge_set, allow_edge_types_set, deny_edge_types_set, weights, node_types, edge_types, singletons, selfloops, verbose)"]
    /// Return new graph object without the indicated attributes.
    ///
    /// Parameters
    /// --------------------
    /// allow_nodes_set: Set[str] = None,
    ///     Optional set of nodes names to keep.
    /// deny_nodes_set: Set[str] = None,
    ///     Optional set of nodes names to remove.
    /// allow_node_types_set: Set[str] = None,
    ///     Optional set of node types names to keep.
    /// deny_node_types_set: Set[str] = None,
    ///     Optional set of node types names to remove.
    /// allow_edge_set: Set[int] = None,
    ///     Optional set of numeric edge IDs to keep.
    /// deny_edge_set: Set[int],
    ///     Optional set of numeric edge IDs to remove.
    /// allow_edge_types_set: Set[str] = None,
    ///     Optional set of edge types names to keep.
    /// deny_edge_types_set: Set[str],
    ///     Optional set of edge types names to remove.
    /// weights: bool = False,
    ///     Wether to remove the weights.
    ///     By default the parameter is false.
    /// node_types: bool = False,
    ///     Wether to remove the node types.
    ///     By default the parameter is false.
    /// edge_types: bool = False,
    ///     Wether to remove the edge types.
    ///     By default the parameter is false.
    /// singletons: bool = False,
    ///     Wether to remove the singleton nodes.
    ///     By default the parameter is false.
    /// selfloops: bool = False,
    ///     Wether to remove the selfloops edges.
    ///     By default the parameter is false.
    /// verbose: bool = True,
    ///     Wether to show a loading bar while building the graph.
    ///     By default the parameter is true.
    ///
    /// Returns
    /// ---------------------
    /// Graph without the required elements.
    fn remove(&self, py_kwargs: Option<&PyDict>) -> PyResult<EnsmallenGraph> {
        let py = pyo3::Python::acquire_gil();
        let kwargs = normalize_kwargs!(py_kwargs, py.python());
        pyex!(validate_kwargs(
            kwargs,
            [
                "allow_nodes_set",
                "deny_nodes_set",
                "allow_node_types_set",
                "deny_node_types_set",
                "allow_edge_set",
                "deny_edge_set",
                "allow_edge_types_set",
                "deny_edge_types_set",
                "weights",
                "node_types",
                "edge_types",
                "singletons",
                "selfloops",
                "verbose"
            ].iter().collect::<Vec<String>>(),
        ))?;

        Ok(EnsmallenGraph {
            graph: pyex!(self.graph.remove(
                pyex!(extract_value!(kwargs, "allow_nodes_set", HashSet<String>))?,
                pyex!(extract_value!(kwargs, "deny_nodes_set", HashSet<String>))?,
                pyex!(extract_value!(
                    kwargs,
                    "allow_node_types_set",
                    HashSet<String>
                ))?,
                pyex!(extract_value!(
                    kwargs,
                    "deny_node_types_set",
                    HashSet<String>
                ))?,
                pyex!(extract_value!(kwargs, "allow_edge_set", HashSet<EdgeT>))?,
                pyex!(extract_value!(kwargs, "deny_edge_set", HashSet<EdgeT>))?,
                pyex!(extract_value!(
                    kwargs,
                    "allow_edge_types_set",
                    HashSet<String>
                ))?,
                pyex!(extract_value!(
                    kwargs,
                    "deny_edge_types_set",
                    HashSet<String>
                ))?,
                pyex!(extract_value!(kwargs, "weights", bool))?.unwrap_or(false),
                pyex!(extract_value!(kwargs, "node_types", bool))?.unwrap_or(false),
                pyex!(extract_value!(kwargs, "edge_types", bool))?.unwrap_or(false),
                pyex!(extract_value!(kwargs, "singletons", bool))?.unwrap_or(false),
                pyex!(extract_value!(kwargs, "selfloops", bool))?.unwrap_or(false),
                pyex!(extract_value!(kwargs, "verbose", bool))?.unwrap_or(true),
            ))?,
        })
    }

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
        pyex!(validate_kwargs(
            kwargs,
            [
                "node_names",
                "node_types",
                "edge_types",
                "minimum_component_size",
                "top_k_components",
                "verbose"
            ].iter().collect::<Vec<String>>(),
        ))?;

        Ok(EnsmallenGraph {
            graph: pyex!(self.graph.remove_components(
                pyex!(extract_value!(kwargs, "node_names", Vec<String>))?,
                pyex!(extract_value!(kwargs, "node_types", Vec<String>))?,
                pyex!(extract_value!(kwargs, "edge_types", Vec<String>))?,
                pyex!(extract_value!(kwargs, "minimum_component_size", NodeT))?,
                pyex!(extract_value!(kwargs, "top_k_components", NodeT))?,
                pyex!(extract_value!(kwargs, "verbose", bool))?.unwrap_or(true),
            ))?,
        })
    }
}
