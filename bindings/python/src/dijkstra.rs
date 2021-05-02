use super::*;
use graph::{NodeT};
use roaring::RoaringBitmap;

#[pymethods]
impl EnsmallenGraph {
    #[text_signature = "($self, src)"]
    /// Returns vector of minimum paths distances and vector of nodes predecessors from given source node ID and optional destination node ID.
    ///
    /// Parameters
    /// -------------------------
    /// src_node_id: int,
    ///     Node ID root of the tree of minimum paths.
    /// maybe_dst_node_id: Union[int, None] = None,
    ///     Optional target destination. If provided, Dijkstra will stop upon reaching this node.
    /// maybe_dst_node_ids: Union[List[int], None] = None,
    ///     Optional target destinations. If provided, Dijkstra will stop upon reaching all of these nodes.
    /// use_graph_weights: bool = False,
    ///     Whether to use the graph weights as edge distances.
    /// compute_predecessors: bool = True,
    ///     Whether to compute the vector of predecessors or to limit the allocation to exclusively the distances.
    /// avoid_visits_above_root: bool,
    ///     Whether to avoid computing the paths that include nodes with ID lower than root. By default false.
    /// verbose: bool = True,
    ///     Whether to show an indicative progress bar.
    ///
    /// # Raises
    /// * If the weights are to be used and the graph does not have weights.
    /// * If the given source node ID does not exist in the current graph.
    /// * If the given optional destination node ID does not exist in the current graph.
    pub fn get_dijkstra_from_node_ids(
        &self,
        src_node_id: NodeT,
        maybe_dst_node_id: Option<NodeT>,
        maybe_dst_node_ids: Option<Vec<NodeT>>,
        use_graph_weights: Option<bool>,
        compute_predecessors: Option<bool>,
        avoid_visits_above_root: Option<bool>,
        verbose: Option<bool>,
    ) -> PyResult<(Vec<f64>, Option<Vec<NodeT>>)> {
        pe!(self.graph.get_dijkstra_from_node_ids(
            src_node_id,
            maybe_dst_node_id,
            maybe_dst_node_ids.map(|mut dst_node_ids| {
                dst_node_ids.sort();
                RoaringBitmap::from_sorted_iter(dst_node_ids)
            }),
            use_graph_weights,
            compute_predecessors,
            avoid_visits_above_root,
            verbose,
        ))
    }

    /// Returns diameter of the graph.
    ///
    /// # Arguments
    /// use_graph_weights: bool = False,
    ///     Whether to use the graph weights as edge distances. By default, false.
    /// ignore_infinity: bool = True,
    ///     Whether to ignore infinite distances, which are present when in the graph exist multiple components.
    /// verbose: bool = True,
    ///     Whether to show a loading bar.
    ///
    /// # Raises
    /// * If the graph does not contain nodes.
    /// * If the graph does not have weights and weights have been requested.
    pub fn get_diameter(
        &self,
        use_graph_weights: Option<bool>,
        ignore_infinity: Option<bool>,
        verbose: Option<bool>,
    ) -> PyResult<f64> {
        pe!(self
            .graph
            .get_diameter(use_graph_weights, ignore_infinity, verbose))
    }
}
