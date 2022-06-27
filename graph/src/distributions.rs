use super::*;

/// Implementation of methods relative to statistical tools.
impl Graph {
    /// Return threshold representing cutuoff point in graph node degree geometric distribution to have the given amount of elements above cutoff.
    ///
    /// # Implementative details
    /// Note that if the number of required elements is higher than the number of elements in the array,
    /// the threshold returned will be equal to zero.
    ///
    /// # Arguments
    /// * `number_of_elements_above_threshold`: usize - Number of elements expected to be above cutoff threshold.
    pub fn get_node_degree_geometric_distribution_threshold(
        &self,
        number_of_nodes_above_threshold: NodeT,
    ) -> f64 {
        // If the number of requested elements is higher than the number of available elements
        // the threshold to cutoff that numbeer of elements is surely zero.
        if number_of_nodes_above_threshold >= self.get_number_of_nodes() {
            return 0.0;
        }
        // We compute the mean of the node degrees
        // We can surely unwrap because if the number of nodes were to be zero
        // the check above would handle that.
        let mean_node_degree = self.get_node_degrees_mean().unwrap();
        // Check if the graph contains zero degree nodes
        let has_zero_degree_nodes = self.has_singleton_nodes() | self.has_trap_nodes();
        // And then, using the geometric distribution formula,
        // we compute the cutoff threshold.
        let numerator =
            (number_of_nodes_above_threshold as f64 / self.get_number_of_nodes() as f64).ln();
        if has_zero_degree_nodes {
            numerator / (1.0 - 1.0 / (mean_node_degree + 1.0)) - 1.0
        } else {
            numerator / (1.0 - 1.0 / mean_node_degree)
        }
    }
}
