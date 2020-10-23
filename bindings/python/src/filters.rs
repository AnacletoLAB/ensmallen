use super::*;
use graph::WeightT;

#[pymethods]
impl EnsmallenGraph {
    #[text_signature = "($self, min_weight, max_weight, verbose)"]
    /// Return graph filtered by given weights range.
    ///
    /// Parameters
    /// -------------
    /// min_weight: float=None,
    ///     Minimum weight to use to filter edges.
    /// max_weight: float=None,
    ///     Maximum weight to use to filter edges.
    /// verbose: bool=True,
    ///     Wether to show the loading bar.
    ///
    /// Returns
    /// -------------
    /// The filtered graph.
    pub fn filter_weights(
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
                verbose.or_else(|| Some(true)).unwrap(),
            ))?,
        })
    }
}
