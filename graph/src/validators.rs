use super::*;

/// # Validators
/// The naming convention we follow is `validate_X`.
impl Graph {
    /// Validates provided edge ID.
    ///
    /// # Arguments
    /// * edge_id: EdgeT - Edge ID to validate.
    ///
    /// # Example
    /// In order to validate a given edge ID, you can use the following:
    ///
    /// ```rust
    /// # let graph = graph::test_utilities::load_ppi(true, true, true, true, false, false).unwrap();
    /// assert!(graph.validate_edge_id(0).is_ok());
    /// assert!(graph.validate_edge_id(10000000000).is_err());
    /// ```
    pub fn validate_edge_id(&self, edge_id: EdgeT) -> Result<EdgeT, String> {
        if edge_id >= self.get_directed_edges_number() {
            return Err(format!(
                "The given edge id ({}) is higher than the edges of the graph ({}).",
                edge_id,
                self.get_directed_edges_number()
            ));
        }
        Ok(edge_id)
    }
}
