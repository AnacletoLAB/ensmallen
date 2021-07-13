use super::types::*;
use super::*;
use num_traits::Pow;
use num_traits::Zero;

/// # Properties and measurements of the graph
impl Graph {
    /// Returns the minumum unweighted preferential attachment score.
    ///
    /// # Safety
    /// If the graph does not contain nodes, the return value will be undefined.
    pub unsafe fn get_unchecked_minimum_preferential_attachment(&self) -> f64 {
        (self.get_unchecked_minimum_node_degree() as f64).pow(2)
    }

    /// Returns the maximum unweighted preferential attachment score.
    ///
    /// # Safety
    /// If the graph does not contain nodes, the return value will be undefined.
    pub unsafe fn get_unchecked_maximum_preferential_attachment(&self) -> f64 {
        (self.get_unchecked_maximum_node_degree() as f64).pow(2)
    }

    /// Returns the minumum weighted preferential attachment score.
    ///
    /// # Safety
    /// If the graph does not contain nodes, the return value will be undefined.
    pub unsafe fn get_unchecked_weighted_minimum_preferential_attachment(&self) -> f64 {
        (self.get_weighted_minimum_node_degree().unwrap() as f64).pow(2)
    }

    /// Returns the maximum weighted preferential attachment score.
    ///
    /// # Safety
    /// If the graph does not contain nodes, the return value will be undefined.
    pub unsafe fn get_unchecked_weighted_maximum_preferential_attachment(&self) -> f64 {
        (self.get_weighted_maximum_node_degree().unwrap() as f64).pow(2)
    }

    /// Returns the unweighted preferential attachment from the given node IDs.
    ///
    /// # Arguments
    /// * `source_node_id`: NodeT - Node ID of the first node.
    /// * `destination_node_id`: NodeT - Node ID of the second node.
    /// * `normalize`: bool - Whether to normalize within 0 to 1.
    ///
    /// # Safety
    /// If either of the provided one and two node IDs are higher than the
    /// number of nodes in the graph.
    pub unsafe fn get_unchecked_preferential_attachment_from_node_ids(
        &self,
        source_node_id: NodeT,
        destination_node_id: NodeT,
        normalize: bool,
    ) -> f64 {
        let mut preferential_attachment_score =
            self.get_unchecked_node_degree_from_node_id(source_node_id) as f64
                * self.get_unchecked_node_degree_from_node_id(destination_node_id) as f64;
        if normalize {
            let min_preferential_attachment_score =
                self.get_unchecked_minimum_preferential_attachment();
            let max_preferential_attachment_score =
                self.get_unchecked_maximum_preferential_attachment();
            preferential_attachment_score = (preferential_attachment_score
                - min_preferential_attachment_score)
                / (max_preferential_attachment_score - min_preferential_attachment_score);
        }
        preferential_attachment_score
    }

    /// Returns the unweighted preferential attachment from the given node IDs.
    ///
    /// # Arguments
    ///
    /// * `source_node_id`: NodeT - Node ID of the first node.
    /// * `destination_node_id`: NodeT - Node ID of the second node.
    /// * `normalize`: bool - Whether to normalize by the square of maximum degree.
    ///
    /// # Raises
    /// * If either of the node IDs are higher than the number of nodes in the graph.
    pub fn get_preferential_attachment_from_node_ids(
        &self,
        source_node_id: NodeT,
        destination_node_id: NodeT,
        normalize: bool,
    ) -> Result<f64> {
        Ok(unsafe {
            self.get_unchecked_preferential_attachment_from_node_ids(
                self.validate_node_id(source_node_id)?,
                self.validate_node_id(destination_node_id)?,
                normalize,
            )
        })
    }

    /// Returns the unweighted preferential attachment from the given node names.
    ///
    /// # Arguments
    ///
    /// * `first_node_name`: &str - Node name of the first node.
    /// * `second_node_name`: &str - Node name of the second node.
    /// * `normalize`: bool - Whether to normalize by the square of maximum degree.
    ///
    /// # Raises
    /// * If either of the given node names do not exist in the current graph.
    pub fn get_preferential_attachment_from_node_names(
        &self,
        first_node_name: &str,
        second_node_name: &str,
        normalize: bool,
    ) -> Result<f64> {
        Ok(unsafe {
            self.get_unchecked_preferential_attachment_from_node_ids(
                self.get_node_id_from_node_name(first_node_name)?,
                self.get_node_id_from_node_name(second_node_name)?,
                normalize,
            )
        })
    }

    /// Returns the weighted preferential attachment from the given node IDs.
    ///
    /// # Arguments
    ///
    /// * `source_node_id`: NodeT - Node ID of the first node.
    /// * `destination_node_id`: NodeT - Node ID of the second node.
    /// * `normalize`: bool - Whether to normalize within 0 to 1.
    ///
    /// # Safety
    /// If either of the provided one and two node IDs are higher than the
    /// number of nodes in the graph.
    pub unsafe fn get_unchecked_weighted_preferential_attachment_from_node_ids(
        &self,
        source_node_id: NodeT,
        destination_node_id: NodeT,
        normalize: bool,
    ) -> f64 {
        let mut preferential_attachment_score =
            self.get_unchecked_weighted_node_degree_from_node_id(source_node_id) as f64
                * self.get_unchecked_weighted_node_degree_from_node_id(destination_node_id) as f64;
        if normalize {
            let min_preferential_attachment_score =
                self.get_unchecked_weighted_minimum_preferential_attachment();
            let max_preferential_attachment_score =
                self.get_unchecked_weighted_maximum_preferential_attachment();
            preferential_attachment_score = (preferential_attachment_score
                - min_preferential_attachment_score)
                / (max_preferential_attachment_score - min_preferential_attachment_score);
        }
        preferential_attachment_score
    }

    /// Returns the weighted preferential attachment from the given node IDs.
    ///
    /// # Arguments
    ///
    /// * `source_node_id`: NodeT - Node ID of the first node.
    /// * `destination_node_id`: NodeT - Node ID of the second node.
    /// * `normalize`: bool - Whether to normalize by the square of maximum degree.
    ///
    /// # Raises
    /// * If either of the node IDs are higher than the number of nodes in the graph.
    pub fn get_weighted_preferential_attachment_from_node_ids(
        &self,
        source_node_id: NodeT,
        destination_node_id: NodeT,
        normalize: bool,
    ) -> Result<f64> {
        self.must_have_edge_weights()?;
        Ok(unsafe {
            self.get_unchecked_weighted_preferential_attachment_from_node_ids(
                self.validate_node_id(source_node_id)?,
                self.validate_node_id(destination_node_id)?,
                normalize,
            )
        })
    }

    /// Returns the weighted preferential attachment from the given node names.
    ///
    /// # Arguments
    ///
    /// * `first_node_name`: &str - Node name of the first node.
    /// * `second_node_name`: &str - Node name of the second node.
    /// * `normalize`: bool - Whether to normalize by the square of maximum degree.
    ///
    /// # Raises
    /// * If either of the given node names do not exist in the current graph.
    pub fn get_weighted_preferential_attachment_from_node_names(
        &self,
        first_node_name: &str,
        second_node_name: &str,
        normalize: bool,
    ) -> Result<f64> {
        self.must_have_edge_weights()?;
        Ok(unsafe {
            self.get_unchecked_weighted_preferential_attachment_from_node_ids(
                self.get_node_id_from_node_name(first_node_name)?,
                self.get_node_id_from_node_name(second_node_name)?,
                normalize,
            )
        })
    }

    /// Returns the Jaccard index for the two given nodes from the given node IDs.
    ///
    /// # Arguments
    ///
    /// * `source_node_id`: NodeT - Node ID of the first node.
    /// * `destination_node_id`: NodeT - Node ID of the second node.
    ///
    /// # References
    /// [D. Liben-Nowell, J. Kleinberg.
    /// The Link Prediction Problem for Social Networks (2004).](http://www.cs.cornell.edu/home/kleinber/link-pred.pdf)
    ///
    /// # Example
    ///```rust
    /// # let graph = graph::test_utilities::load_ppi(true, true, true, true, false, false);
    /// println!("The Jaccard Index between node 1 and node 2 is {}", unsafe{ graph.get_unchecked_jaccard_coefficient_from_node_ids(1, 2) });
    /// ```
    ///
    /// # Safety
    /// If either of the provided one and two node IDs are higher than the
    /// number of nodes in the graph.
    pub unsafe fn get_unchecked_jaccard_coefficient_from_node_ids(
        &self,
        source_node_id: NodeT,
        destination_node_id: NodeT,
    ) -> f64 {
        self.iter_unchecked_neighbour_node_ids_intersection_from_source_node_ids(
            source_node_id,
            destination_node_id,
        )
        .count() as f64
            / self
                .iter_unchecked_neighbour_node_ids_union_from_source_node_ids(
                    source_node_id,
                    destination_node_id,
                )
                .count() as f64
    }

    /// Returns the Jaccard index for the two given nodes from the given node IDs.
    ///
    /// # Arguments
    ///
    /// * `source_node_id`: NodeT - Node ID of the first node.
    /// * `destination_node_id`: NodeT - Node ID of the second node.
    ///
    /// # References
    /// [D. Liben-Nowell, J. Kleinberg.
    /// The Link Prediction Problem for Social Networks (2004).](http://www.cs.cornell.edu/home/kleinber/link-pred.pdf)
    ///
    /// # Example
    ///```rust
    /// # let graph = graph::test_utilities::load_ppi(true, true, true, true, false, false);
    /// println!("The Jaccard Index between node 1 and node 2 is {}", graph.get_jaccard_coefficient_from_node_ids(1, 2).unwrap());
    /// ```
    ///
    /// # Raises
    /// * If either of the node IDs are higher than the number of nodes in the graph.
    pub fn get_jaccard_coefficient_from_node_ids(
        &self,
        source_node_id: NodeT,
        destination_node_id: NodeT,
    ) -> Result<f64> {
        Ok(unsafe {
            self.get_unchecked_jaccard_coefficient_from_node_ids(
                self.validate_node_id(source_node_id)?,
                self.validate_node_id(destination_node_id)?,
            )
        })
    }

    /// Returns the Jaccard index for the two given nodes from the given node names.
    ///
    /// # Arguments
    ///
    /// * `first_node_name`: &str - Node name of the first node.
    /// * `second_node_name`: &str - Node name of the second node.
    ///
    /// # References
    /// [D. Liben-Nowell, J. Kleinberg.
    /// The Link Prediction Problem for Social Networks (2004).](http://www.cs.cornell.edu/home/kleinber/link-pred.pdf)
    ///
    /// # Raises
    /// * If either of the given node names do not exist in the current graph.
    pub fn get_jaccard_coefficient_from_node_names(
        &self,
        first_node_name: &str,
        second_node_name: &str,
    ) -> Result<f64> {
        Ok(unsafe {
            self.get_unchecked_jaccard_coefficient_from_node_ids(
                self.get_node_id_from_node_name(first_node_name)?,
                self.get_node_id_from_node_name(second_node_name)?,
            )
        })
    }

    /// Returns the Adamic/Adar Index for the given pair of nodes from the given node IDs.
    ///
    /// # Arguments
    ///
    /// * `source_node_id`: NodeT - Node ID of the first node.
    /// * `destination_node_id`: NodeT - Node ID of the second node.
    ///
    /// # Implementation details
    /// Since the Adamic/Adar Index is only defined for graph not containing
    /// node traps (nodes without any outbound edge) and must support all kind
    /// of graphs, the sinks node are excluded from
    /// the computation because they would result in an infinity.
    ///
    /// # References
    /// [D. Liben-Nowell, J. Kleinberg.
    /// The Link Prediction Problem for Social Networks (2004).](http://www.cs.cornell.edu/home/kleinber/link-pred.pdf)
    ///
    /// # Safety
    /// If either of the provided one and two node IDs are higher than the
    /// number of nodes in the graph.
    pub unsafe fn get_unchecked_adamic_adar_index_from_node_ids(
        &self,
        source_node_id: NodeT,
        destination_node_id: NodeT,
    ) -> f64 {
        self.iter_unchecked_neighbour_node_ids_intersection_from_source_node_ids(
            source_node_id,
            destination_node_id,
        )
        .map(|node_id| self.get_unchecked_node_degree_from_node_id(node_id))
        .filter(|&node_degree| node_degree > 1)
        .map(|node_degree| 1.0 / (node_degree as f64).ln())
        .sum()
    }

    /// Returns the Adamic/Adar Index for the given pair of nodes from the given node IDs.
    ///
    /// # Arguments
    ///
    /// * `source_node_id`: NodeT - Node ID of the first node.
    /// * `destination_node_id`: NodeT - Node ID of the second node.
    ///
    /// # Implementation details
    /// Since the Adamic/Adar Index is only defined for graph not containing
    /// node traps (nodes without any outbound edge) and must support all kind
    /// of graphs, the sinks node are excluded from
    /// the computation because they would result in an infinity.
    ///
    /// # References
    /// [D. Liben-Nowell, J. Kleinberg.
    /// The Link Prediction Problem for Social Networks (2004).](http://www.cs.cornell.edu/home/kleinber/link-pred.pdf)
    ///
    /// # Raises
    /// * If either of the node IDs are higher than the number of nodes in the graph.
    pub fn get_adamic_adar_index_from_node_ids(
        &self,
        source_node_id: NodeT,
        destination_node_id: NodeT,
    ) -> Result<f64> {
        Ok(unsafe {
            self.get_unchecked_adamic_adar_index_from_node_ids(
                self.validate_node_id(source_node_id)?,
                self.validate_node_id(destination_node_id)?,
            )
        })
    }

    /// Returns the Adamic/Adar Index for the given pair of nodes from the given node names.
    ///
    /// # Arguments
    ///
    /// * `first_node_name`: &str - Node name of the first node.
    /// * `second_node_name`: &str - Node name of the second node.
    ///
    /// # Implementation details
    /// Since the Adamic/Adar Index is only defined for graph not containing
    /// node traps (nodes without any outbound edge) and must support all kind
    /// of graphs, the sinks node are excluded from
    /// the computation because they would result in an infinity.
    ///
    /// # References
    /// [D. Liben-Nowell, J. Kleinberg.
    /// The Link Prediction Problem for Social Networks (2004).](http://www.cs.cornell.edu/home/kleinber/link-pred.pdf)
    ///
    /// # Raises
    /// * If either of the given node names do not exist in the current graph.
    pub fn get_adamic_adar_index_from_node_names(
        &self,
        first_node_name: &str,
        second_node_name: &str,
    ) -> Result<f64> {
        Ok(unsafe {
            self.get_unchecked_adamic_adar_index_from_node_ids(
                self.get_node_id_from_node_name(first_node_name)?,
                self.get_node_id_from_node_name(second_node_name)?,
            )
        })
    }

    /// Returns the unweighted Resource Allocation Index for the given pair of nodes from the given node IDs.
    ///
    /// # Arguments
    ///
    /// * `source_node_id`: NodeT - Node ID of the first node.
    /// * `destination_node_id`: NodeT - Node ID of the second node.
    ///
    /// # References
    /// [T. Zhou, L. Lu, Y.-C. Zhang.
    /// Predicting missing links via local information.
    /// Eur. Phys. J. B 71 (2009) 623.](http://arxiv.org/pdf/0901.0553.pdf)
    ///
    /// # Implementation details
    /// Since the Resource Allocation Index is only defined for graph not
    /// containing node traps (nodes without any outbound edge) and
    /// must support all kind of graphs, the sinks node are excluded from
    /// the computation because they would result in an infinity.
    ///
    /// # Safety
    /// If either of the provided one and two node IDs are higher than the
    /// number of nodes in the graph.
    pub unsafe fn get_unchecked_resource_allocation_index_from_node_ids(
        &self,
        source_node_id: NodeT,
        destination_node_id: NodeT,
    ) -> f64 {
        self.iter_unchecked_neighbour_node_ids_intersection_from_source_node_ids(
            source_node_id,
            destination_node_id,
        )
        .map(|node_id| self.get_unchecked_node_degree_from_node_id(node_id))
        .filter(|&node_degree| node_degree > 0)
        .map(|node_degree| 1.0 / node_degree as f64)
        .sum()
    }

    /// Returns the weighted Resource Allocation Index for the given pair of nodes from the given node IDs.
    ///
    /// # Arguments
    ///
    /// * `source_node_id`: NodeT - Node ID of the first node.
    /// * `destination_node_id`: NodeT - Node ID of the second node.
    ///
    /// # References
    /// [T. Zhou, L. Lu, Y.-C. Zhang.
    /// Predicting missing links via local information.
    /// Eur. Phys. J. B 71 (2009) 623.](http://arxiv.org/pdf/0901.0553.pdf)
    ///
    /// # Implementation details
    /// Since the Resource Allocation Index is only defined for graph not
    /// containing node traps (nodes without any outbound edge) and
    /// must support all kind of graphs, the sinks node are excluded from
    /// the computation because they would result in an infinity.
    ///
    /// # Safety
    /// If either of the provided one and two node IDs are higher than the
    /// number of nodes in the graph.
    pub unsafe fn get_unchecked_weighted_resource_allocation_index_from_node_ids(
        &self,
        source_node_id: NodeT,
        destination_node_id: NodeT,
    ) -> f64 {
        self.iter_unchecked_neighbour_node_ids_intersection_from_source_node_ids(
            source_node_id,
            destination_node_id,
        )
        .map(|node_id| self.get_unchecked_weighted_node_degree_from_node_id(node_id))
        .filter(|&node_degree| !node_degree.is_zero())
        .map(|node_degree| 1.0 / node_degree as f64)
        .sum()
    }

    /// Returns the unweighted Resource Allocation Index for the given pair of nodes from the given node IDs.
    ///
    /// # Arguments
    ///
    /// * `source_node_id`: NodeT - Node ID of the first node.
    /// * `destination_node_id`: NodeT - Node ID of the second node.
    ///
    /// # References
    /// [T. Zhou, L. Lu, Y.-C. Zhang.
    /// Predicting missing links via local information.
    /// Eur. Phys. J. B 71 (2009) 623.](http://arxiv.org/pdf/0901.0553.pdf)
    ///
    /// # Implementation details
    /// Since the Resource Allocation Index is only defined for graph not
    /// containing node traps (nodes without any outbound edge) and
    /// must support all kind of graphs, the sinks node are excluded from
    /// the computation because they would result in an infinity.
    ///
    /// # Raises
    /// * If either of the node IDs are higher than the number of nodes in the graph.
    pub fn get_resource_allocation_index_from_node_ids(
        &self,
        source_node_id: NodeT,
        destination_node_id: NodeT,
    ) -> Result<f64> {
        Ok(unsafe {
            self.get_unchecked_resource_allocation_index_from_node_ids(
                self.validate_node_id(source_node_id)?,
                self.validate_node_id(destination_node_id)?,
            )
        })
    }

    /// Returns the unweighted Resource Allocation Index for the given pair of nodes from the given node names.
    ///
    /// # Arguments
    ///
    /// * `first_node_name`: &str - Node name of the first node.
    /// * `second_node_name`: &str - Node name of the second node.
    ///
    /// # References
    /// [T. Zhou, L. Lu, Y.-C. Zhang.
    /// Predicting missing links via local information.
    /// Eur. Phys. J. B 71 (2009) 623.](http://arxiv.org/pdf/0901.0553.pdf)
    ///
    /// # Implementation details
    /// Since the Resource Allocation Index is only defined for graph not
    /// containing node traps (nodes without any outbound edge) and
    /// must support all kind of graphs, the sinks node are excluded from
    /// the computation because they would result in an infinity.
    ///
    /// # Raises
    /// * If either of the given node names do not exist in the current graph.
    pub fn get_resource_allocation_index_from_node_names(
        &self,
        first_node_name: &str,
        second_node_name: &str,
    ) -> Result<f64> {
        Ok(unsafe {
            self.get_unchecked_resource_allocation_index_from_node_ids(
                self.get_node_id_from_node_name(first_node_name)?,
                self.get_node_id_from_node_name(second_node_name)?,
            )
        })
    }

    /// Returns the weighted Resource Allocation Index for the given pair of nodes from the given node IDs.
    ///
    /// # Arguments
    ///
    /// * `source_node_id`: NodeT - Node ID of the first node.
    /// * `destination_node_id`: NodeT - Node ID of the second node.
    ///
    /// # References
    /// [T. Zhou, L. Lu, Y.-C. Zhang.
    /// Predicting missing links via local information.
    /// Eur. Phys. J. B 71 (2009) 623.](http://arxiv.org/pdf/0901.0553.pdf)
    ///
    /// # Implementation details
    /// Since the Resource Allocation Index is only defined for graph not
    /// containing node traps (nodes without any outbound edge) and
    /// must support all kind of graphs, the sinks node are excluded from
    /// the computation because they would result in an infinity.
    ///
    /// # Raises
    /// * If either of the node IDs are higher than the number of nodes in the graph.
    pub fn get_weighted_resource_allocation_index_from_node_ids(
        &self,
        source_node_id: NodeT,
        destination_node_id: NodeT,
    ) -> Result<f64> {
        self.must_have_edge_weights()?;
        Ok(unsafe {
            self.get_unchecked_weighted_resource_allocation_index_from_node_ids(
                self.validate_node_id(source_node_id)?,
                self.validate_node_id(destination_node_id)?,
            )
        })
    }

    /// Returns the weighted Resource Allocation Index for the given pair of nodes from the given node names.
    ///
    /// # Arguments
    ///
    /// * `first_node_name`: &str - Node name of the first node.
    /// * `second_node_name`: &str - Node name of the second node.
    ///
    /// # References
    /// [T. Zhou, L. Lu, Y.-C. Zhang.
    /// Predicting missing links via local information.
    /// Eur. Phys. J. B 71 (2009) 623.](http://arxiv.org/pdf/0901.0553.pdf)
    ///
    /// # Implementation details
    /// Since the Resource Allocation Index is only defined for graph not
    /// containing node traps (nodes without any outbound edge) and
    /// must support all kind of graphs, the sinks node are excluded from
    /// the computation because they would result in an infinity.
    ///
    /// # Raises
    /// * If either of the given node names do not exist in the current graph.
    pub fn get_weighted_resource_allocation_index_from_node_names(
        &self,
        first_node_name: &str,
        second_node_name: &str,
    ) -> Result<f64> {
        self.must_have_edge_weights()?;
        Ok(unsafe {
            self.get_unchecked_weighted_resource_allocation_index_from_node_ids(
                self.get_node_id_from_node_name(first_node_name)?,
                self.get_node_id_from_node_name(second_node_name)?,
            )
        })
    }

    /// Returns all the implemented edge metrics for the two given node IDs.
    ///
    /// Specifically, the returned values are:
    /// * Adamic Adar
    /// * Jaccard coefficient
    /// * Resource allocation index
    /// * Preferential attachment
    ///
    /// # Arguments
    /// * `source_node_id`: NodeT - Node ID of the first node.
    /// * `destination_node_id`: NodeT - Node ID of the second node.
    /// * `normalize`: bool - Whether to normalize within 0 to 1.
    ///
    /// # Safety
    /// If the given node IDs do not exist in the graph this method will panic.
    pub unsafe fn get_unchecked_all_edge_metrics_from_node_ids(
        &self,
        source_node_id: NodeT,
        destination_node_id: NodeT,
        normalize: bool,
    ) -> Vec<f64> {
        vec![
            self.get_unchecked_adamic_adar_index_from_node_ids(source_node_id, destination_node_id),
            self.get_unchecked_jaccard_coefficient_from_node_ids(
                source_node_id,
                destination_node_id,
            ),
            self.get_unchecked_resource_allocation_index_from_node_ids(
                source_node_id,
                destination_node_id,
            ),
            self.get_unchecked_preferential_attachment_from_node_ids(
                source_node_id,
                destination_node_id,
                normalize,
            ),
        ]
    }
}
