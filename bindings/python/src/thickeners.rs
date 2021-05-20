use super::*;
use graph::NodeT;

#[pymethods]
impl EnsmallenGraph {
    #[text_signature = "($self, node_features, neighbours_number, distance_name, verbose)"]
    /// Returns graph with edges added extracted from given node_features.
    ///
    /// Parameters
    /// -----------------------
    /// node_features: List[List[float]],
    ///     Node_features to use to identify the new neighbours.
    /// neighbours_number: Option<NodeT>,
    ///     Number of neighbours to add.
    /// distance_name: Optional[str],
    ///     Name of distance to use. Can either be L2 or COSINE. By default COSINE.
    /// verbose: Option<bool>,
    ///     Whether to show loading bars.
    pub fn generate_new_edges_from_node_features(
        &self,
        node_features: Vec<Vec<f64>>,
        neighbours_number: Option<NodeT>,
        distance_name: Option<&str>,
        verbose: Option<bool>,
    ) -> PyResult<EnsmallenGraph> {
        Ok(EnsmallenGraph {
            graph: pe!(self.graph.generate_new_edges_from_node_features(
                node_features,
                neighbours_number,
                distance_name,
                verbose
            ))?,
        })
    }
}
