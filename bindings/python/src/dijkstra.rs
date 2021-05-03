use super::*;
use graph::NodeT;
use roaring::RoaringBitmap;

#[pymethods]
impl EnsmallenGraph {
    #[text_signature = "($self, src_node_id, maybe_dst_node_id, maybe_dst_node_ids, compute_distances, compute_predecessors, verbose)"]
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
    /// compute_distances: bool = True,
    ///     Whether to compute the vector of distances.
    /// compute_predecessors: bool = True,
    ///     Whether to compute the vector of predecessors.
    /// verbose: bool = True,
    ///     Whether to show an indicative progress bar.
    ///
    /// # Raises
    /// * If the given source node ID does not exist in the current graph.
    /// * If the given optional destination node ID does not exist in the current graph.
    pub fn get_breath_first_search_from_node_ids(
        &self,
        src_node_id: NodeT,
        maybe_dst_node_id: Option<NodeT>,
        maybe_dst_node_ids: Option<Vec<NodeT>>,
        compute_distances: Option<bool>,
        compute_predecessors: Option<bool>,
        verbose: Option<bool>,
    ) -> PyResult<(Option<Vec<NodeT>>, Option<Vec<Option<NodeT>>>, NodeT)> {
        pe!(self.graph.get_breath_first_search_from_node_ids(
            src_node_id,
            maybe_dst_node_id,
            maybe_dst_node_ids.map(|mut dst_node_ids| {
                dst_node_ids.sort();
                RoaringBitmap::from_sorted_iter(dst_node_ids)
            }),
            compute_distances,
            compute_predecessors,
            verbose,
        ))
    }

    #[text_signature = "($self, src_node_name, maybe_dst_node_name, maybe_dst_node_names, compute_distances, compute_predecessors, verbose)"]
    /// Returns vector of minimum paths distances and vector of nodes predecessors from given source node name and optional destination node name.
    ///
    /// Parameters
    /// -------------------------
    /// src_node_name: int,
    ///     Node name root of the tree of minimum paths.
    /// maybe_dst_node_name: Union[int, None] = None,
    ///     Optional target destination. If provided, Dijkstra will stop upon reaching this node.
    /// maybe_dst_node_names: Union[List[int], None] = None,
    ///     Optional target destinations. If provided, Dijkstra will stop upon reaching all of these nodes.
    /// compute_distances: bool = True,
    ///     Whether to compute the vector of distances.
    /// compute_predecessors: bool = True,
    ///     Whether to compute the vector of predecessors.
    /// verbose: bool = True,
    ///     Whether to show an indicative progress bar.
    ///
    /// # Raises
    /// * If the given source node name does not exist in the current graph.
    /// * If the given optional destination node name does not exist in the current graph.
    pub fn get_breath_first_search_from_node_names(
        &self,
        src_node_name: &str,
        maybe_dst_node_name: Option<&str>,
        maybe_dst_node_names: Option<Vec<&str>>,
        compute_distances: Option<bool>,
        compute_predecessors: Option<bool>,
        verbose: Option<bool>,
    ) -> PyResult<(Option<Vec<NodeT>>, Option<Vec<Option<NodeT>>>, NodeT)> {
        pe!(self.graph.get_breath_first_search_from_node_names(
            src_node_name,
            maybe_dst_node_name,
            maybe_dst_node_names,
            compute_distances,
            compute_predecessors,
            verbose,
        ))
    }

    #[text_signature = "($self, src_node_id, maybe_dst_node_id, maybe_dst_node_ids, compute_predecessors, verbose)"]
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
    /// compute_predecessors: bool = True,
    ///     Whether to compute the vector of predecessors.
    /// verbose: bool = True,
    ///     Whether to show an indicative progress bar.
    ///
    /// # Raises
    /// * If the given source node ID does not exist in the current graph.
    /// * If the given optional destination node ID does not exist in the current graph.
    pub fn get_dijkstra_from_node_ids(
        &self,
        src_node_id: NodeT,
        maybe_dst_node_id: Option<NodeT>,
        maybe_dst_node_ids: Option<Vec<NodeT>>,
        compute_predecessors: Option<bool>,
        verbose: Option<bool>,
    ) -> PyResult<(Vec<f64>, Option<Vec<NodeT>>, f64)> {
        pe!(self.graph.get_dijkstra_from_node_ids(
            src_node_id,
            maybe_dst_node_id,
            maybe_dst_node_ids.map(|mut dst_node_ids| {
                dst_node_ids.sort();
                RoaringBitmap::from_sorted_iter(dst_node_ids)
            }),
            compute_predecessors,
            verbose,
        ))
    }

    #[text_signature = "($self, src_node_name, maybe_dst_node_name, maybe_dst_node_names, compute_predecessors, verbose)"]
    /// Returns vector of minimum paths distances and vector of nodes predecessors from given source node name and optional destination node name.
    ///
    /// Parameters
    /// -------------------------
    /// src_node_name: int,
    ///     Node name root of the tree of minimum paths.
    /// maybe_dst_node_name: Union[int, None] = None,
    ///     Optional target destination. If provided, Dijkstra will stop upon reaching this node.
    /// maybe_dst_node_names: Union[List[int], None] = None,
    ///     Optional target destinations. If provided, Dijkstra will stop upon reaching all of these nodes.
    /// compute_predecessors: bool = True,
    ///     Whether to compute the vector of predecessors.
    /// verbose: bool = True,
    ///     Whether to show an indicative progress bar.
    ///
    /// # Raises
    /// * If the given source node name does not exist in the current graph.
    /// * If the given optional destination node name does not exist in the current graph.
    pub fn get_dijkstra_from_node_names(
        &self,
        src_node_name: &str,
        maybe_dst_node_name: Option<&str>,
        maybe_dst_node_names: Option<Vec<&str>>,
        compute_predecessors: Option<bool>,
        verbose: Option<bool>,
    ) -> PyResult<(Vec<f64>, Option<Vec<NodeT>>, f64)> {
        pe!(self.graph.get_dijkstra_from_node_names(
            src_node_name,
            maybe_dst_node_name,
            maybe_dst_node_names,
            compute_predecessors,
            verbose,
        ))
    }

    /// Returns diameter of the graph.
    ///
    /// # Arguments
    ///     Whether to use the graph weights as edge distances. By default, false.
    /// ignore_infinity: bool = True,
    ///     Whether to ignore infinite distances, which are present when in the graph exist multiple components.
    /// verbose: bool = True,
    ///     Whether to show a loading bar.
    ///
    /// # Raises
    /// * If the graph does not contain nodes.
    /// * If the graph does not have weights and weights have been requested.
    pub fn get_unweighted_diameter(
        &self,
        ignore_infinity: Option<bool>,
        verbose: Option<bool>,
    ) -> PyResult<NodeT> {
        pe!(self.graph.get_unweighted_diameter(ignore_infinity, verbose))
    }

    /// Returns diameter of the graph.
    ///
    /// # Arguments
    ///     Whether to use the graph weights as edge distances. By default, false.
    /// ignore_infinity: bool = True,
    ///     Whether to ignore infinite distances, which are present when in the graph exist multiple components.
    /// verbose: bool = True,
    ///     Whether to show a loading bar.
    ///
    /// # Raises
    /// * If the graph does not contain nodes.
    /// * If the graph does not have weights and weights have been requested.
    pub fn get_weighted_diameter(
        &self,
        ignore_infinity: Option<bool>,
        verbose: Option<bool>,
    ) -> PyResult<f64> {
        pe!(self.graph.get_weighted_diameter(ignore_infinity, verbose))
    }
}
