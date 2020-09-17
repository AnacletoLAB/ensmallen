use graph::Graph;
use pyo3::prelude::*;
use pyo3::wrap_pymodule;

#[pymodule]
fn ensmallen_graph(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<EnsmallenGraph>()?;
    m.add_wrapped(wrap_pymodule!(preprocessing))?;
    env_logger::init();
    Ok(())
}

#[pyclass]
#[derive(Clone, PartialEq)]
#[text_signature = "(sources, destinations, *, nodes_mapping, nodes_reverse_mapping, node_types, node_types_mapping, node_types_reverse_mapping, edge_types, edge_types_mapping, edge_types_reverse_mapping, weights, force_conversion_to_undirected)"]
/// Return new EnsmallenGraph.
///
/// sources: List[int],
///     The list of source nodes.
/// destinations: List[int],
///     The list of destination nodes.
/// nodes_mapping: Dict[str, int] = None,
///     The dictionary with mappEnsmallenGraph,
///     List of the node types, must be as long as the nodes mapping.
/// node_types_mapping: Dict[str, int] = None,
///     Mapping between the node types names and their IDs.
/// node_types_reverse_mapping: List[str] = None,
///     Reverse mapping between numeric node Type IDs and their name.
/// edge_types: List[int] = None,
///     List of the egde types, must be as long as the egdes mapping.
/// edge_types_mapping: Dict[str, int] = None,
///     Mapping between the edge types names and their IDs.
/// edge_types_reverse_mapping: List[str] = None,
///     Reverse mapping between numeric egde Type IDs and their name.
/// weights: List[float] = None,
///     List of the weight for each edge.
/// force_conversion_to_undirected: bool = False,
///     Wethever to force the conversion from directed graph to undirected
///     when there are bidirectional directed edges in the given graph.
///
pub(crate) struct EnsmallenGraph {
    graph: Graph,
}