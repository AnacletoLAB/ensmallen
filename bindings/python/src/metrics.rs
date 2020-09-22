use super::*;
use graph::{EdgeT, NodeT};
use numpy::{PyArray, PyArray1};
use std::collections::HashMap;

#[pymethods]
impl EnsmallenGraph {
    #[text_signature = "($self)"]
    /// Returns mean node degree of the graph.
    pub fn degrees_mean(&self) -> f64 {
        self.graph.degrees_mean()
    }

    #[text_signature = "($self)"]
    /// Returns number of self-loops.
    pub fn selfloops_number(&self) -> usize {
        self.graph.selfloops_number()
    }

    #[text_signature = "($self)"]
    /// Returns median node degree of the graph.
    pub fn degrees_median(&self) -> NodeT {
        self.graph.degrees_median()
    }

    #[text_signature = "($self)"]
    /// Returns mode node degree of the graph.
    pub fn degrees_mode(&self) -> NodeT {
        self.graph.degrees_mode()
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
    /// * bidirectional_rate: rate of edges that are bidirectional.
    ///
    fn report(&self) -> HashMap<&str, String> {
        self.graph.report()
    }

    #[text_signature = "($self, node)"]
    /// Return the degree for the given node.
    ///
    /// Parameters
    /// ---------------------
    /// node: int,
    ///     Node ID to use to compute degrees product.
    ///
    /// Returns
    /// ----------------------------
    /// degrees product for the two given nodes.
    ///
    fn degree(&self, node: NodeT) -> NodeT {
        self.graph.degree(node)
    }

    #[text_signature = "($self)"]
    /// Return all the degrees of the nodes graph.
    ///
    /// Returns
    /// ----------------------------
    /// Numpy array with all the degrees of the graph.
    ///
    fn degrees(&self) -> PyResult<Py<PyArray1<EdgeT>>> {
        let degrees = self.graph.degrees();
        let gil = pyo3::Python::acquire_gil();
        Ok(to_nparray_1d!(gil, degrees, EdgeT))
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
        pyex!(self.graph.jaccard_index(one, two))
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
        pyex!(self.graph.adamic_adar_index(one, two))
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
        pyex!(self.graph.resource_allocation_index(one, two))
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
        pyex!(self.graph.degrees_product(one, two))
    }

    #[text_signature = "(self)"]
    /// Return the traps rate of the graph.
    ///
    /// This feature is EXPERIMENTAL and still required proving.
    ///
    fn traps_rate(&self) -> f64 {
        self.graph.traps_rate()
    }

    #[text_signature = "(self)"]
    /// Returns the ratio of edges that have multiple types.
    fn get_multigraph_edges_ratio(&self) -> f64 {
        self.graph.get_multigraph_edges_ratio()
    }

    #[text_signature = "(self)"]
    /// Returns the number of edges that have multiple types.
    fn get_multigraph_edges_number(&self) -> usize {
        self.graph.get_multigraph_edges_number()
    }

    #[text_signature = "(self)"]
    /// Returns the mean number of types for each edge.
    fn get_mean_number_of_types_for_edge(&self) -> f64 {
        self.graph.get_mean_number_of_types_for_edge()
    }
}
