use super::*;
use graph::{WeightT};

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
        node_types: Option<Vec<Option<String>>>,
        edge_types: Option<Vec<Option<String>>>,
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
}
