use super::*;
use graph::NodeT;
use std::collections::HashSet;

#[pymethods]
impl EnsmallenGraph {
    #[text_signature = "($self, node_features, neighbours_number, verbose)"]
    /// Returns graph with edges added extracted from given node_features.
    ///
    /// Parameters
    /// -----------------------
    /// node_features: Vec<Vec<f64>>,
    ///     Node_features to use to identify the new neighbours.
    /// neighbours_number: Option<NodeT>,
    ///     Number of neighbours to add.
    /// verbose: Option<bool>,
    ///     Whether to show loading bars.
    pub fn generate_new_edges_from_node_features(
        &self,
        node_features: Vec<Vec<f64>>,
        neighbours_number: Option<NodeT>,
        verbose: Option<bool>,
    ) -> PyResult<EnsmallenGraph> {
        Ok(EnsmallenGraph {
            graph: pe!(self.graph.generate_new_edges_from_node_features(
                node_features,
                neighbours_number,
                verbose
            ))?,
        })
    }
}
