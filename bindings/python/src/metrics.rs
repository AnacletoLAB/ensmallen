use super::*;
use graph::{EdgeT, NodeT};
use numpy::{PyArray, PyArray1};
use std::collections::HashMap;

#[pymethods]
impl EnsmallenGraph {
    #[text_signature = "($self)"]
    /// Returns mean node degree of the graph.
    pub fn get_node_degrees_mean(&self) -> PyResult<f64> {
        pe!(self.graph.get_node_degrees_mean())
    }

    #[text_signature = "($self)"]
    /// Returns median node degree of the graph.
    pub fn get_node_degrees_median(&self) -> PyResult<NodeT> {
        pe!(self.graph.get_node_degrees_median())
    }

    #[text_signature = "($self)"]
    /// Returns max node degree of the graph.
    pub fn get_max_node_degree(&self) -> PyResult<NodeT> {
        pe!(self.graph.get_max_node_degree())
    }

    #[text_signature = "($self)"]
    /// Returns min node degree of the graph.
    pub fn get_min_node_degree(&self) -> PyResult<NodeT> {
        pe!(self.graph.get_min_node_degree())
    }

    #[text_signature = "($self)"]
    /// Returns mode node degree of the graph.
    pub fn get_node_degrees_mode(&self) -> PyResult<NodeT> {
        pe!(self.graph.get_node_degrees_mode())
    }

    #[text_signature = "($self)"]
    /// Returns report relative to the graph metrics.
    ///
    /// The report includes a few useful metrics like:
    ///
    /// * degrees_median: the median degree of the nodes.
    /// * degrees_mean: the mean degree of the nodes.
    /// * degrees_mode: the mode degree of the nodes.
    /// * degrees_max: the max degree of the nodes.
    /// * degrees_min: the min degree of the nodes.
    /// * nodes_number: the number of nodes in the graph.
    /// * edges_number: the number of edges in the graph.
    /// * unique_node_types_number: the number of different node types in the graph.
    /// * unique_edge_types_number: the number of different edge types in the graph.
    /// * traps_rate: probability to end up in a trap when starting into any given node.
    /// * selfloops_rate: pecentage of edges that are selfloops.
    ///
    fn report(&self) -> HashMap<&str, String> {
        self.graph.report()
    }

    /// Return report on overlaps of the two graphs.
    ///
    /// Parameters
    /// -------------------
    /// `other`: &EnsmallenGraph,
    ///     Graph to compute the overlaps with.
    /// verbose: bool = True,
    ///     Wether to show loading bars.
    ///
    /// Returns
    /// -------------------
    /// Textual report.
    fn overlap_textual_report(
        &self,
        other: &EnsmallenGraph,
        verbose: Option<bool>,
    ) -> PyResult<String> {
        pe!(self
            .graph
            .overlap_textual_report(&other.graph, verbose.unwrap_or(true)))
    }

    #[text_signature = "($self)"]
    /// Return all the degrees of the nodes graph.
    ///
    /// Returns
    /// ----------------------------
    /// Numpy array with all the degrees of the graph.
    ///
    fn degrees(&self) -> PyResult<Py<PyArray1<NodeT>>> {
        let degrees = self.graph.get_node_degrees();
        let gil = pyo3::Python::acquire_gil();
        Ok(to_ndarray_1d!(gil, degrees, NodeT))
    }

    #[text_signature = "($self, one, two)"]
    /// Return the Jaccard Index for the two given nodes.
    ///
    /// Parameters
    /// ---------------------
    /// one: int,
    ///     First node ID to use to compute Jaccard Index.
    /// two: int,
    ///     Second node ID to use to compute Jaccard Index.
    ///
    /// Returns
    /// ----------------------------
    /// Jaccard Index for the two given nodes.
    ///
    fn jaccard_index(&self, one: NodeT, two: NodeT) -> PyResult<f64> {
        pe!(self.graph.jaccard_index(one, two))
    }

    #[text_signature = "($self, one, two)"]
    /// Return the Adamic/Adar for the two given nodes.
    ///
    /// Parameters
    /// ---------------------
    /// one: int,
    ///     First node ID to use to compute Adamic/Adar.
    /// two: int,
    ///     Second node ID to use to compute Adamic/Adar.
    ///
    /// Returns
    /// ----------------------------
    /// Adamic/Adar for the two given nodes.
    ///
    fn adamic_adar_index(&self, one: NodeT, two: NodeT) -> PyResult<f64> {
        pe!(self.graph.adamic_adar_index(one, two))
    }

    #[text_signature = "($self, one, two)"]
    /// Return the Resource Allocation Index for the two given nodes.
    ///
    /// Parameters
    /// ---------------------
    /// one: int,
    ///     First node ID to use to compute Resource Allocation Index.
    /// two: int,
    ///     Second node ID to use to compute Resource Allocation Index.
    ///
    /// Returns
    /// ----------------------------
    /// Resource Allocation Index for the two given nodes.
    ///
    fn resource_allocation_index(&self, one: NodeT, two: NodeT) -> PyResult<f64> {
        pe!(self.graph.resource_allocation_index(one, two))
    }

    #[text_signature = "($self, one, two)"]
    /// Return the degrees product for the two given nodes.
    ///
    /// Parameters
    /// ---------------------
    /// one: int,
    ///     First node ID to use to compute degrees product.
    /// two: int,
    ///     Second node ID to use to compute degrees product.
    ///
    /// Returns
    /// ----------------------------
    /// degrees product for the two given nodes.
    ///
    fn degrees_product(&self, one: NodeT, two: NodeT) -> PyResult<usize> {
        pe!(self.graph.degrees_product(one, two))
    }
}
