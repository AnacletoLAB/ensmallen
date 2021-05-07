use super::*;
use graph::{EdgeT, NodeT};

#[pymethods]
impl EnsmallenGraph {
    #[text_signature = "($self)"]
    /// Returns number of triangles in the graph.
    ///
    /// [Automatically generated binding]
    /// [Automatically generated documentation]
    fn get_number_of_triangles(&self) -> EdgeT {
        self.graph.get_number_of_triangles()
    }

    #[text_signature = "($self)"]
    /// Returns transitivity of the graph.
    ///
    /// [Automatically generated binding]
    /// [Automatically generated documentation]
    fn get_transitivity(&self) -> EdgeT {
        self.graph.get_transitivity()
    }

    #[text_signature = "($self)"]
    /// Returns number of triangles for all nodes in the graph.
    ///
    /// [Automatically generated binding]
    /// [Automatically generated documentation]
    fn get_number_of_triangles_per_node(&self) -> Vec<NodeT> {
        self.graph.get_number_of_triangles_per_node()
    }

    #[text_signature = "($self)"]
    /// Returns clustering coefficients for all nodes in the graph.
    ///
    /// [Automatically generated binding]
    /// [Automatically generated documentation]
    fn get_clustering_coefficient_per_node(&self) -> Vec<f64> {
        self.graph.get_clustering_coefficient_per_node()
    }

    #[text_signature = "($self)"]
    /// Returns the graph clustering coefficient.
    ///
    /// [Automatically generated binding]
    /// [Automatically generated documentation]
    fn get_clustering_coefficient(&self) -> f64 {
        self.graph.get_clustering_coefficient()
    }
}
