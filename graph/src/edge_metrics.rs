use super::types::*;
use super::*;
use num_traits::Pow;
use num_traits::Zero;
use rayon::prelude::*;
use std::collections::HashSet;

/// # Properties and measurements of the graph
impl Graph {
    /// Returns the structural distance from the given node IDs.
    ///
    /// # Arguments
    /// * `source_node_id`: NodeT - Node ID of the first node.
    /// * `destination_node_id`: NodeT - Node ID of the second node.
    /// * `maximal_hop_distance`: usize - Maximal hop distance to consider.
    ///
    /// # Reference
    /// This is the structural distance based on the dynamical time warping
    /// of two nodes's neighbours's degree up to a given hop distance.
    ///
    /// # Safety
    /// If either of the provided one and two node IDs are higher than the
    /// number of nodes in the graph.
    pub unsafe fn get_unchecked_structural_distance_from_node_ids(
        &self,
        source_node_id: NodeT,
        destination_node_id: NodeT,
        maximal_hop_distance: usize,
    ) -> Vec<f32> {
        let mut source_frontier = [source_node_id].iter().copied().collect::<HashSet<NodeT>>();
        let mut destination_frontier = [destination_node_id]
            .iter()
            .copied()
            .collect::<HashSet<NodeT>>();

        let step_frontier = |frontier: &HashSet<NodeT>| {
            frontier
                .iter()
                .copied()
                .flat_map(|node_id| {
                    self.iter_unchecked_neighbour_node_ids_from_source_node_id(node_id)
                })
                .collect::<HashSet<NodeT>>()
        };

        let sorted_degree_tuples = |frontier: &HashSet<NodeT>| {
            if frontier.is_empty() {
                return vec![];
            }
            let mut node_degrees = frontier
                .iter()
                .copied()
                .map(|node_id| self.get_unchecked_node_degree_from_node_id(node_id))
                .collect::<Vec<NodeT>>();
            node_degrees.sort_unstable();
            let mut last_degree: Option<NodeT> = None;
            let mut count = 0;
            let mut degree_counts: Vec<(NodeT, NodeT)> = Vec::new();

            node_degrees
                .into_iter()
                .for_each(|degree| match last_degree.as_mut() {
                    Some(last_degree) => {
                        if *last_degree == degree {
                            count += 1;
                        } else {
                            degree_counts.push((count, *last_degree));
                            *last_degree = degree;
                            count = 1;
                        }
                    }
                    None => {
                        last_degree = Some(degree);
                        count = 1;
                    }
                });
            degree_counts.push((count, last_degree.unwrap()));
            degree_counts
        };

        // This is the distance of the degrees defined by
        // the authors of Struct2Vec in Ribeiro et al.
        let degree_distance =
            |(first_degree_count, first_degree): &(NodeT, NodeT),
             (second_degree_count, second_degree): &(NodeT, NodeT)| {
                (*(first_degree.max(second_degree)) as f32
                    / (*(first_degree.min(second_degree)) as f32 + f32::EPSILON)
                    - 1.0)
                    * (*(first_degree_count.max(second_degree_count)) as f32)
            };

        // compute the first layer as just the difference of degrees since we
        // have hop distance of 0
        [degree_distance(
            &(
                self.get_unchecked_node_degree_from_node_id(source_node_id),
                1,
            ),
            &(
                self.get_unchecked_node_degree_from_node_id(destination_node_id),
                1,
            ),
        )]
        .iter()
        .copied()
        .chain(
            // compute the remaining distances
            (0..maximal_hop_distance).map(|_| {
                source_frontier = step_frontier(&source_frontier);
                destination_frontier = step_frontier(&destination_frontier);
                let source_degree_counts = sorted_degree_tuples(&source_frontier);
                let destination_degree_counts = sorted_degree_tuples(&destination_frontier);
                express_measures::dynamic_time_warping(
                    &source_degree_counts,
                    &destination_degree_counts,
                    degree_distance,
                )
            }),
        )
        .scan(0.0, |running_sum, cost| {
            *running_sum += cost;
            Some(*running_sum)
        })
        .collect::<Vec<f32>>()
    }

    /// Returns the minumum unweighted preferential attachment score.
    ///
    /// # Safety
    /// If the graph does not contain nodes, the return value will be undefined.
    pub unsafe fn get_unchecked_minimum_preferential_attachment(&self) -> f32 {
        (self.get_unchecked_minimum_node_degree() as f32).pow(2)
    }

    /// Returns the maximum unweighted preferential attachment score.
    ///
    /// # Safety
    /// If the graph does not contain nodes, the return value will be undefined.
    pub unsafe fn get_unchecked_maximum_preferential_attachment(&self) -> f32 {
        (self.get_unchecked_maximum_node_degree() as f32).pow(2)
    }

    /// Returns the minumum weighted preferential attachment score.
    ///
    /// # Safety
    /// If the graph does not contain nodes, the return value will be undefined.
    pub unsafe fn get_unchecked_weighted_minimum_preferential_attachment(&self) -> f32 {
        (self.get_weighted_minimum_node_degree().clone().unwrap() as f32).pow(2)
    }

    /// Returns the maximum weighted preferential attachment score.
    ///
    /// # Safety
    /// If the graph does not contain nodes, the return value will be undefined.
    pub unsafe fn get_unchecked_weighted_maximum_preferential_attachment(&self) -> f32 {
        (self.get_weighted_maximum_node_degree().clone().unwrap() as f32).pow(2)
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
    ) -> f32 {
        let mut preferential_attachment_score =
            self.get_unchecked_node_degree_from_node_id(source_node_id) as f32
                * self.get_unchecked_node_degree_from_node_id(destination_node_id) as f32;
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
    ) -> Result<f32> {
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
    ) -> Result<f32> {
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
    ) -> f32 {
        let mut preferential_attachment_score =
            self.get_unchecked_weighted_node_degree_from_node_id(source_node_id) as f32
                * self.get_unchecked_weighted_node_degree_from_node_id(destination_node_id) as f32;
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
    ) -> Result<f32> {
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
    ) -> Result<f32> {
        self.must_have_edge_weights()?;
        Ok(unsafe {
            self.get_unchecked_weighted_preferential_attachment_from_node_ids(
                self.get_node_id_from_node_name(first_node_name)?,
                self.get_node_id_from_node_name(second_node_name)?,
                normalize,
            )
        })
    }

    /// Returns the Neighbours intersection size for the two given nodes from the given node IDs.
    ///
    /// # Arguments
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
    /// println!("The Neighbours intersection size between node 1 and node 2 is {}", unsafe{ graph.get_unchecked_neighbours_intersection_size_from_node_ids(1, 2) });
    /// ```
    ///
    /// # Safety
    /// If either of the provided one and two node IDs are higher than the
    /// number of nodes in the graph.
    pub unsafe fn get_unchecked_neighbours_intersection_size_from_node_ids(
        &self,
        source_node_id: NodeT,
        destination_node_id: NodeT,
    ) -> f32 {
        self.iter_unchecked_neighbour_node_ids_intersection_from_source_node_ids(
            source_node_id,
            destination_node_id,
        )
        .count() as f32
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
    ) -> f32 {
        let src_neighbours = self
            .edges
            .get_unchecked_neighbours_node_ids_from_src_node_id(source_node_id);
        let dst_neighbours = self
            .edges
            .get_unchecked_neighbours_node_ids_from_src_node_id(destination_node_id);

        let mut intersection_count = 0;
        let mut src_count = 0;
        let mut dst_count = 0;

        let mut dst_index = 0;
        let mut src_index = 0;

        while dst_index < dst_neighbours.len() && src_index < src_neighbours.len() {
            let dst_neighbour = dst_neighbours[dst_index];
            let src_neighbour = src_neighbours[src_index];

            // If this is not an intersection, we march forward
            if dst_neighbour <= src_neighbour {
                dst_count += 1;
                dst_index += 1;
                // on multigraph we ignore repeated edges
                if self.is_multigraph() {
                    while dst_index < dst_neighbours.len()
                        && dst_neighbour == dst_neighbours[dst_index]
                    {
                        dst_index += 1;
                    }
                }
            }
            if dst_neighbour >= src_neighbour {
                src_count += 1;
                src_index += 1;
                // on multigraph we ignore repeated edges
                if self.is_multigraph() {
                    while src_index < src_neighbours.len()
                        && src_neighbour == src_neighbours[src_index]
                    {
                        src_index += 1;
                    }
                }
            }
            // branchless update of equal nodes
            intersection_count += (src_neighbour == dst_neighbour) as usize;
        }

        let union_count = src_count + dst_count - intersection_count;

        if intersection_count == 0 {
            0.0
        } else {
            intersection_count as f32 / union_count as f32
        }
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
    ) -> Result<f32> {
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
    ) -> Result<f32> {
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
    /// node traps (nodes without any outbound edge) and must subgraph all kind
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
    ) -> f32 {
        self.iter_unchecked_neighbour_node_ids_intersection_from_source_node_ids(
            source_node_id,
            destination_node_id,
        )
        .map(|node_id| self.get_unchecked_node_degree_from_node_id(node_id))
        .filter(|&node_degree| node_degree > 1)
        .map(|node_degree| 1.0 / (node_degree as f32).ln())
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
    /// node traps (nodes without any outbound edge) and must subgraph all kind
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
    ) -> Result<f32> {
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
    /// node traps (nodes without any outbound edge) and must subgraph all kind
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
    ) -> Result<f32> {
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
    /// must subgraph all kind of graphs, the sinks node are excluded from
    /// the computation because they would result in an infinity.
    ///
    /// # Safety
    /// If either of the provided one and two node IDs are higher than the
    /// number of nodes in the graph.
    pub unsafe fn get_unchecked_resource_allocation_index_from_node_ids(
        &self,
        source_node_id: NodeT,
        destination_node_id: NodeT,
    ) -> f32 {
        self.iter_unchecked_neighbour_node_ids_intersection_from_source_node_ids(
            source_node_id,
            destination_node_id,
        )
        .map(|node_id| self.get_unchecked_node_degree_from_node_id(node_id))
        .filter(|&node_degree| node_degree > 0)
        .map(|node_degree| 1.0 / node_degree as f32)
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
    /// must subgraph all kind of graphs, the sinks node are excluded from
    /// the computation because they would result in an infinity.
    ///
    /// # Safety
    /// If either of the provided one and two node IDs are higher than the
    /// number of nodes in the graph.
    pub unsafe fn get_unchecked_weighted_resource_allocation_index_from_node_ids(
        &self,
        source_node_id: NodeT,
        destination_node_id: NodeT,
    ) -> f32 {
        self.iter_unchecked_neighbour_node_ids_intersection_from_source_node_ids(
            source_node_id,
            destination_node_id,
        )
        .map(|node_id| self.get_unchecked_weighted_node_degree_from_node_id(node_id))
        .filter(|&node_degree| !node_degree.is_zero())
        .map(|node_degree| 1.0 / node_degree as f32)
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
    /// must subgraph all kind of graphs, the sinks node are excluded from
    /// the computation because they would result in an infinity.
    ///
    /// # Raises
    /// * If either of the node IDs are higher than the number of nodes in the graph.
    pub fn get_resource_allocation_index_from_node_ids(
        &self,
        source_node_id: NodeT,
        destination_node_id: NodeT,
    ) -> Result<f32> {
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
    /// must subgraph all kind of graphs, the sinks node are excluded from
    /// the computation because they would result in an infinity.
    ///
    /// # Raises
    /// * If either of the given node names do not exist in the current graph.
    pub fn get_resource_allocation_index_from_node_names(
        &self,
        first_node_name: &str,
        second_node_name: &str,
    ) -> Result<f32> {
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
    /// must subgraph all kind of graphs, the sinks node are excluded from
    /// the computation because they would result in an infinity.
    ///
    /// # Raises
    /// * If either of the node IDs are higher than the number of nodes in the graph.
    pub fn get_weighted_resource_allocation_index_from_node_ids(
        &self,
        source_node_id: NodeT,
        destination_node_id: NodeT,
    ) -> Result<f32> {
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
    /// must subgraph all kind of graphs, the sinks node are excluded from
    /// the computation because they would result in an infinity.
    ///
    /// # Raises
    /// * If either of the given node names do not exist in the current graph.
    pub fn get_weighted_resource_allocation_index_from_node_names(
        &self,
        first_node_name: &str,
        second_node_name: &str,
    ) -> Result<f32> {
        self.must_have_edge_weights()?;
        Ok(unsafe {
            self.get_unchecked_weighted_resource_allocation_index_from_node_ids(
                self.get_node_id_from_node_name(first_node_name)?,
                self.get_node_id_from_node_name(second_node_name)?,
            )
        })
    }

    /// Returns number of currently subgraphed edge metrics.
    pub fn get_number_of_available_edge_metrics(&self) -> usize {
        4
    }

    /// Returns names of currently subgraphed edge metrics.
    pub fn get_available_edge_metrics_names(&self) -> Vec<&str> {
        vec![
            "Adamic Adar",
            "Jaccard Coefficient",
            "Resource allocation index",
            "Preferential attachment",
        ]
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
    pub unsafe fn get_unchecked_all_edge_metrics_from_node_ids_tuple(
        &self,
        source_node_id: NodeT,
        destination_node_id: NodeT,
        normalize: bool,
    ) -> Vec<f32> {
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
    /// # Raises
    /// * If the provided node IDs do not exist in the current graph instance.
    pub fn get_all_edge_metrics_from_node_ids_tuple(
        &self,
        source_node_id: NodeT,
        destination_node_id: NodeT,
        normalize: bool,
    ) -> Result<Vec<f32>> {
        Ok(unsafe {
            self.get_unchecked_all_edge_metrics_from_node_ids_tuple(
                self.validate_node_id(source_node_id)?,
                self.validate_node_id(destination_node_id)?,
                normalize,
            )
        })
    }

    /// Returns all the implemented edge metrics for the vectors source and destination node IDs.
    ///
    /// Specifically, the returned values are:
    /// * Adamic Adar
    /// * Jaccard coefficient
    /// * Resource allocation index
    /// * Preferential attachment
    ///
    /// # Arguments
    /// * `source_node_ids`: Vec<NodeT> - Node ID of the first node.
    /// * `destination_node_ids`: Vec<NodeT> - Node ID of the second node.
    /// * `normalize`: bool - Whether to normalize within 0 to 1.
    ///
    /// # Safety
    /// If the given node IDs do not exist in the graph this method will panic.
    pub fn get_all_edge_metrics_from_node_ids(
        &self,
        source_node_ids: Vec<NodeT>,
        destination_node_ids: Vec<NodeT>,
        normalize: bool,
    ) -> Result<Vec<Vec<f32>>> {
        source_node_ids
            .into_par_iter()
            .zip(destination_node_ids.into_par_iter())
            .map(|(src, dst)| {
                self.validate_node_id(src)?;
                self.validate_node_id(dst)?;
                Ok(unsafe {
                    self.get_unchecked_all_edge_metrics_from_node_ids_tuple(src, dst, normalize)
                })
            })
            .collect::<Result<Vec<Vec<f32>>>>()
    }

    /// Returns parallel iterator on Preferential Attachment for all edges.
    ///
    /// # Arguments
    /// `normalize`: Option<bool> - Whether to normalize the edge prediction metrics. By default, true.
    /// `subgraph`: Option<&Graph> - Optional subgraph whose edges are to be used when computing the Preferential Attachment.
    ///
    /// # Raises
    /// * If the provided subgraph graph does not share a compatible vocabulary with the current graph instance.
    pub fn par_iter_preferential_attachment_scores<'a>(
        &'a self,
        normalize: Option<bool>,
        subgraph: Option<&'a Graph>,
    ) -> Result<impl IndexedParallelIterator<Item = f32> + 'a> {
        let subgraph = if let Some(subgraph) = subgraph {
            self.must_share_node_vocabulary(subgraph)?;
            subgraph
        } else {
            &self
        };
        let normalize = normalize.unwrap_or(true);
        Ok(subgraph.par_iter_directed_edge_node_ids().map(
            move |(_, source_node_id, destination_node_id)| unsafe {
                self.get_unchecked_preferential_attachment_from_node_ids(
                    source_node_id,
                    destination_node_id,
                    normalize,
                )
            },
        ))
    }

    /// Returns Preferential Attachment for all edges.
    ///
    /// # Arguments
    /// `normalize`: Option<bool> - Whether to normalize the edge prediction metrics. By default, true.
    /// `subgraph`: Option<&Graph> - Optional subgraph whose edges are to be used when computing the Preferential Attachment.
    ///
    /// # Raises
    /// * If the provided subgraph graph does not share a compatible vocabulary with the current graph instance.
    pub fn get_preferential_attachment_scores(
        &self,
        normalize: Option<bool>,
        subgraph: Option<&Graph>,
    ) -> Result<Vec<f32>> {
        self.par_iter_preferential_attachment_scores(normalize, subgraph)
            .map(|iter| {
                let mut result = Vec::with_capacity(iter.len());
                iter.collect_into_vec(&mut result);
                result
            })
    }

    /// Returns parallel iterator on Resource Allocation index for all edges.
    ///
    /// # Arguments
    /// `subgraph`: Option<&Graph> - Optional subgraph whose edges are to be used when computing the Resource Allocation index.
    ///
    /// # Raises
    /// * If the provided subgraph graph does not share a compatible vocabulary with the current graph instance.
    pub fn par_iter_resource_allocation_index_scores<'a>(
        &'a self,
        subgraph: Option<&'a Graph>,
    ) -> Result<impl IndexedParallelIterator<Item = f32> + 'a> {
        let subgraph = if let Some(subgraph) = subgraph {
            self.must_share_node_vocabulary(subgraph)?;
            subgraph
        } else {
            &self
        };
        Ok(subgraph.par_iter_directed_edge_node_ids().map(
            move |(_, source_node_id, destination_node_id)| unsafe {
                self.get_unchecked_resource_allocation_index_from_node_ids(
                    source_node_id,
                    destination_node_id,
                )
            },
        ))
    }

    /// Returns Resource Allocation index for all edges.
    ///
    /// # Arguments
    /// `subgraph`: Option<&Graph> - Optional subgraph whose edges are to be used when computing the Resource Allocation index.
    ///
    /// # Raises
    /// * If the provided subgraph graph does not share a compatible vocabulary with the current graph instance.
    pub fn get_resource_allocation_index_scores(
        &self,
        subgraph: Option<&Graph>,
    ) -> Result<Vec<f32>> {
        self.par_iter_resource_allocation_index_scores(subgraph)
            .map(|iter| {
                let mut result = Vec::with_capacity(iter.len());
                iter.collect_into_vec(&mut result);
                result
            })
    }

    /// Returns parallel iterator on Jaccard Coefficient for all edges.
    ///
    /// # Arguments
    /// `subgraph`: Option<&Graph> - Optional subgraph whose edges are to be used when computing the Jaccard Coefficient.
    ///
    /// # Raises
    /// * If the provided subgraph graph does not share a compatible vocabulary with the current graph instance.
    pub fn par_iter_jaccard_coefficient_scores<'a>(
        &'a self,
        subgraph: Option<&'a Graph>,
    ) -> Result<impl IndexedParallelIterator<Item = f32> + 'a> {
        let subgraph = if let Some(subgraph) = subgraph {
            self.must_share_node_vocabulary(subgraph)?;
            subgraph
        } else {
            &self
        };
        Ok(subgraph.par_iter_directed_edge_node_ids().map(
            move |(_, source_node_id, destination_node_id)| unsafe {
                self.get_unchecked_jaccard_coefficient_from_node_ids(
                    source_node_id,
                    destination_node_id,
                )
            },
        ))
    }

    /// Returns Jaccard Coefficient for all edges.
    ///
    /// # Arguments
    /// `subgraph`: Option<&Graph> - Optional subgraph whose edges are to be used when computing the Jaccard Coefficient.
    ///
    /// # Raises
    /// * If the provided subgraph graph does not share a compatible vocabulary with the current graph instance.
    pub fn get_jaccard_coefficient_scores(&self, subgraph: Option<&Graph>) -> Result<Vec<f32>> {
        self.par_iter_jaccard_coefficient_scores(subgraph)
            .map(|iter| {
                let mut result = Vec::with_capacity(iter.len());
                iter.collect_into_vec(&mut result);
                result
            })
    }

    /// Returns parallel iterator on Adamic-Adar for all edges.
    ///
    /// # Arguments
    /// `subgraph`: Option<&Graph> - Optional subgraph whose edges are to be used when computing the Adamic-Adar.
    ///
    /// # Raises
    /// * If the provided subgraph graph does not share a compatible vocabulary with the current graph instance.
    pub fn par_iter_adamic_adar_scores<'a>(
        &'a self,
        subgraph: Option<&'a Graph>,
    ) -> Result<impl IndexedParallelIterator<Item = f32> + 'a> {
        let subgraph = if let Some(subgraph) = subgraph {
            self.must_share_node_vocabulary(subgraph)?;
            subgraph
        } else {
            &self
        };
        Ok(subgraph.par_iter_directed_edge_node_ids().map(
            move |(_, source_node_id, destination_node_id)| unsafe {
                self.get_unchecked_adamic_adar_index_from_node_ids(
                    source_node_id,
                    destination_node_id,
                )
            },
        ))
    }

    /// Returns Adamic-Adar for all edges.
    ///
    /// # Arguments
    /// `subgraph`: Option<&Graph> - Optional subgraph whose edges are to be used when computing the Adamic-Adar.
    ///
    /// # Raises
    /// * If the provided subgraph graph does not share a compatible vocabulary with the current graph instance.
    pub fn get_adamic_adar_scores(&self, subgraph: Option<&Graph>) -> Result<Vec<f32>> {
        self.par_iter_adamic_adar_scores(subgraph).map(|iter| {
            let mut result = Vec::with_capacity(iter.len());
            iter.collect_into_vec(&mut result);
            result
        })
    }

    /// Returns parallel iterator on all available edge metrics for all edges.
    ///
    /// The metrics returned are, in order:
    /// - Adamic-Adar
    /// - Jaccard Coefficient
    /// - Resource Allocation index
    /// - Preferential attachment score
    ///
    /// # Arguments
    /// `normalize`: Option<bool> - Whether to normalize the edge prediction metrics. By default, true.
    /// `subgraph`: Option<&Graph> - Optional subgraph whose edges are to be used when computing the metrics.
    ///
    /// # Raises
    /// * If the provided subgraph graph does not share a compatible vocabulary with the current graph instance.
    pub fn par_iter_all_edge_metrics<'a>(
        &'a self,
        normalize: Option<bool>,
        subgraph: Option<&'a Graph>,
    ) -> Result<impl IndexedParallelIterator<Item = Vec<f32>> + 'a> {
        let normalize = normalize.unwrap_or(true);
        let subgraph = if let Some(subgraph) = subgraph {
            self.must_share_node_vocabulary(subgraph)?;
            subgraph
        } else {
            &self
        };
        Ok(subgraph.par_iter_directed_edge_node_ids().map(
            move |(_, source_node_id, destination_node_id)| unsafe {
                self.get_unchecked_all_edge_metrics_from_node_ids_tuple(
                    source_node_id,
                    destination_node_id,
                    normalize,
                )
            },
        ))
    }

    /// Returns all available edge metrics for all edges.
    ///
    /// The metrics returned are, in order:
    /// - Adamic-Adar
    /// - Jaccard Coefficient
    /// - Resource Allocation index
    /// - Preferential attachment score
    ///
    /// # Arguments
    /// `normalize`: Option<bool> - Whether to normalize the edge prediction metrics. By default, true.
    /// `subgraph`: Option<&Graph> - Optional subgraph whose edges are to be used when computing the metrics.
    ///
    /// # Raises
    /// * If the provided subgraph graph does not share a compatible vocabulary with the current graph instance.
    pub fn get_all_edge_metrics(
        &self,
        normalize: Option<bool>,
        subgraph: Option<&Graph>,
    ) -> Result<Vec<Vec<f32>>> {
        self.par_iter_all_edge_metrics(normalize, subgraph)
            .map(|iter| {
                let mut result = Vec::with_capacity(iter.len());
                iter.collect_into_vec(&mut result);
                result
            })
    }
}
