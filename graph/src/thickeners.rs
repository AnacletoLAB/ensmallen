use super::*;
use indicatif::ParallelProgressIterator;
use num_traits::Pow;
use rayon::prelude::*;

/// # Methods to thicken the graph.
impl Graph {
    /// Returns graph with edges added extracted from given node_features.
    ///
    /// # Arguments
    /// * `node_features`: Vec<Vec<f64>> - node_features to use to identify the new neighbours.
    /// * `neighbours_number`: Option<NodeT> - Number of neighbours to add.
    /// * `distance_name`: Option<&str> - Name of distance to use. Can either be L2 or COSINE. By default COSINE.
    /// * `verbose`: Option<bool> - Whether to show loading bars.
    ///
    /// # Raises
    /// * If the graph does not have nodes.
    /// * If the given node_features are not provided exactly for each node.
    /// * If the node_features do not have a consistent shape.
    /// * If the provided number of neighbours is zero.
    pub fn generate_new_edges_from_node_features(
        &self,
        node_features: Vec<Vec<f64>>,
        neighbours_number: Option<NodeT>,
        distance_name: Option<&str>,
        verbose: Option<bool>,
    ) -> Result<Graph, String> {
        self.must_have_nodes()?;
        if node_features.len() != self.get_nodes_number() as usize {
            return Err(format!(
                concat!(
                    "The node features length need to be provided for each of the node, ",
                    "but the provided node features length is {} while the number of ",
                    "nodes in the graph is {}."
                ),
                node_features.len(),
                self.get_nodes_number()
            ));
        }
        let expected_node_features_length = node_features.first().unwrap().len();
        for node_node_features in node_features.iter() {
            if expected_node_features_length != node_node_features.len() {
                return Err(format!(
                    concat!(
                        "The node features length needs to be consistent: the expected ",
                        "size was {} while the found length was {}."
                    ),
                    expected_node_features_length,
                    node_node_features.len()
                ));
            }
        }

        let verbose = verbose.unwrap_or(true);
        let distance_name = distance_name.unwrap_or("COSINE");
        let neighbours_number =
            neighbours_number.unwrap_or(self.get_node_degrees_mean()?.ceil() as NodeT);
        if neighbours_number == 0 {
            return Err("The number of neighbours to add per node cannot be zero!".to_string());
        }
        let pb = get_loading_bar(
            verbose,
            "Computing additional edges to thicken graph",
            self.get_nodes_number() as usize,
        );

        let new_edges = self
            .par_iter_node_ids()
            .progress_with(pb)
            .map(|source_node_id| {
                let current_node_features = &node_features[source_node_id as usize];
                let mut closest_nodes_distances = vec![f64::INFINITY; neighbours_number as usize];
                let mut closest_nodes = Vec::with_capacity(neighbours_number as usize);
                node_features.iter().zip(self.iter_node_ids()).for_each(
                    |(node_node_features, destination_node_id)| {
                        if source_node_id == destination_node_id {
                            return;
                        }
                        let distance = match distance_name {
                            "L2" => current_node_features
                                .iter()
                                .zip(node_node_features.iter())
                                .map(|(&left, &right)| (left - right).pow(2))
                                .sum(),
                            "COSINE" => {
                                let numerator = current_node_features
                                    .iter()
                                    .zip(node_node_features.iter())
                                    .map(|(&left, &right)| left * right)
                                    .sum::<f64>();
                                let denominator_left = current_node_features
                                    .iter()
                                    .map(|&left| left.pow(2))
                                    .sum::<f64>()
                                    .sqrt();
                                let denominator_right = node_node_features
                                    .iter()
                                    .map(|&right| right.pow(2))
                                    .sum::<f64>()
                                    .sqrt();
                                numerator / (denominator_left * denominator_right)
                            }
                            _ => {
                                unreachable!("The check for the distance name should happen above.")
                            }
                        };
                        let (i, max_distance) = unsafe {
                            closest_nodes_distances
                                .iter()
                                .enumerate()
                                .fold(None, |m, (i, &node_distance)| {
                                    m.map_or(
                                        Some((i, node_distance)),
                                        |(j, current_max_distance)| {
                                            Some(if node_distance > current_max_distance {
                                                (i, node_distance)
                                            } else {
                                                (j, current_max_distance)
                                            })
                                        },
                                    )
                                })
                                .unwrap_unchecked()
                        };
                        if max_distance > distance {
                            if max_distance == f64::INFINITY {
                                closest_nodes.push(destination_node_id);
                            } else {
                                closest_nodes[i] = destination_node_id;
                            }
                            closest_nodes_distances[i] = distance;
                        }
                    },
                );
                closest_nodes
                    .into_iter()
                    .filter(|&destination_node_id| {
                        !self.has_edge_from_node_ids(source_node_id, destination_node_id)
                    })
                    .collect::<Vec<NodeT>>()
            })
            .collect::<Vec<Vec<NodeT>>>();

        Graph::from_integer_unsorted(
            self.iter_edge_node_ids_and_edge_type_id_and_edge_weight(self.is_directed())
                .map(|(_, src_node_id, dst_node_id, edge_type, weight)| {
                    Ok((src_node_id, dst_node_id, edge_type, weight))
                })
                .chain(
                    new_edges
                        .into_iter()
                        .enumerate()
                        .filter(|(_, new_neighbours)| !new_neighbours.is_empty())
                        .map(|(source_node_id, new_neighbours)| {
                            new_neighbours.into_iter().map(move |destination_node_id| {
                                Ok((source_node_id as NodeT, destination_node_id, None, None))
                            })
                        })
                        .flatten(),
                ),
            self.nodes.clone(),
            self.node_types.clone(),
            self.edge_types.as_ref().map(|ets| ets.vocabulary.clone()),
            self.is_directed(),
            self.get_name(),
            true,
            self.has_edge_types(),
            self.has_edge_weights(),
            self.has_singleton_nodes(),
            self.has_singleton_nodes_with_selfloops(),
            self.has_trap_nodes(),
            verbose,
        )
    }
}
