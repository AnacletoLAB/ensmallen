use super::*;
use graph::{WeightT, NodeT};
use numpy::{PyArray, PyArray1};

#[pymethods]
impl EnsmallenGraph {
    #[text_signature = "($self, node_names, node_types, edge_types, min_weight, max_weight, verbose)"]
    /// Return graph filtered by given filters.
    ///
    /// Parameters
    /// -------------
    /// `node_names`: List[str],
    ///     The node names to keep.
    /// `node_types`: List[str],
    ///     The node types to keep.
    /// `edge_types`: List[str],
    ///     The edge types to keep.
    /// `min_weight`: float,
    ///     Minimum weight to use to filter edges.
    /// `max_weight`: float,
    ///     Maximum weight to use to filter edges.
    /// `verbose`: bool,
    ///     Wether to show the loading bar.
    ///
    /// Returns
    /// -------------
    /// The filtered graph.
    pub fn filter(
        &self,
        nodes: Option<Vec<String>>,
        node_types: Option<Vec<String>>,
        edge_types: Option<Vec<String>>,
        min_weight: Option<WeightT>,
        max_weight: Option<WeightT>,
        verbose: Option<bool>,
    ) -> PyResult<EnsmallenGraph> {
        Ok(EnsmallenGraph {
            graph: pyex!(self.graph.filter(
                nodes,
                node_types,
                edge_types,
                min_weight,
                max_weight,
                verbose.unwrap_or(true),
            ))?,
        })
    }

    #[text_signature = "($self, src, node_names, node_types, edge_types, min_weight, max_weight)"]
    /// Return node neighbours filtered by given filters.
    ///
    /// Parameters
    /// -------------
    /// `src`: int,
    ///     The source node.
    /// `node_names`: List[str],
    ///     The node names to keep.
    /// `node_types`: List[str],
    ///     The node types to keep.
    /// `edge_types`: List[str],
    ///     The edge types to keep.
    /// `min_weight`: float,
    ///     Minimum weight to use to filter edges.
    /// `max_weight`: float,
    ///     Maximum weight to use to filter edges.
    ///
    /// Returns
    /// -------------
    /// Integer numpy array with filtered node neighbours.
    pub fn get_filtered_neighbours(
        &self,
        src: NodeT,
        nodes: Option<Vec<String>>,
        node_types: Option<Vec<String>>,
        edge_types: Option<Vec<String>>,
        min_weight: Option<WeightT>,
        max_weight: Option<WeightT>,
    ) -> PyResult<EnsmallenGraph> {
        let neighbours = pyex!(self
            .graph
            .get_filtered_neighbours_range(
                src, nodes, node_types, edge_types, min_weight, max_weight,
            ))?
            .collect::<Vec<NodeT>>();
        let gil = pyo3::Python::acquire_gil();
        to_nparray_1d!(gil, neighbours, NodeT)
    }
}
