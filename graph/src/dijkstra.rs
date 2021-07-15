use super::*;
use indicatif::ParallelProgressIterator;
use indicatif::ProgressIterator;
use num_traits::Zero;
use rayon::iter::IndexedParallelIterator;
use rayon::iter::IntoParallelIterator;
use rayon::iter::ParallelIterator;
use std::cmp::Ord;
use std::collections::VecDeque;
use std::sync::atomic::{AtomicU32, Ordering};

pub struct ShortestPathsResultBFS {
    distances: Vec<NodeT>,
    predecessors: Option<Vec<NodeT>>,
    eccentricity: NodeT,
    most_distant_node: NodeT,
}

impl ShortestPathsResultBFS {
    pub(crate) fn new(
        distances: Vec<NodeT>,
        predecessors: Option<Vec<NodeT>>,
        eccentricity: NodeT,
        most_distant_node: NodeT,
    ) -> ShortestPathsResultBFS {
        ShortestPathsResultBFS {
            distances,
            predecessors,
            eccentricity,
            most_distant_node,
        }
    }

    pub(crate) fn has_path_to_node_id(&self, node_id: NodeT) -> bool {
        self.get_distance_from_node_id(node_id) != NOT_PRESENT
    }

    pub(crate) fn get_distance_from_node_id(&self, node_id: NodeT) -> NodeT {
        self.distances[node_id as usize]
    }

    pub(crate) fn get_parent_from_node_id(&self, node_id: NodeT) -> Option<NodeT> {
        self.predecessors
            .as_ref()
            .map(|predecessors| predecessors[node_id as usize])
    }

    pub(crate) fn get_node_distance(&self, node_id: NodeT) -> NodeT {
        self.distances[node_id as usize]
    }

    /// Returns node at the `len - k` position on minimum path to given destination node.
    ///
    /// # Arguments
    /// * `dst_node_id`: NodeT - The node to start computing predecessors from.
    /// * `k`: NodeT - Steps to go back.
    ///
    /// # Raises
    /// * If the predecessors vector was not requested.
    pub(crate) fn get_kth_point_on_minimum_path(
        &self,
        mut dst_node_id: NodeT,
        k: NodeT,
    ) -> Result<NodeT> {
        if !self.has_path_to_node_id(dst_node_id) {
            return Err("There is no path to the given destination node.".to_string());
        }
        if self.get_eccentricity() < k {
            return Err(format!(
                concat!(
                    "The current minimum path tree has eccentricity {}, ",
                    "but the requested number of steps is {}."
                ),
                self.get_eccentricity(),
                k
            ));
        }
        if let Some(predecessors) = self.predecessors.as_ref() {
            for _ in 0..k {
                dst_node_id = predecessors[dst_node_id as usize];
            }
            return Ok(dst_node_id);
        }
        Err("Predecessors were not requested and therefore not computed.".to_string())
    }

    pub(crate) fn get_median_point(&self, dst_node_id: NodeT) -> Result<NodeT> {
        if !self.has_path_to_node_id(dst_node_id) {
            return Err("There is no path to the given destination node.".to_string());
        }
        let median_distance = self.get_node_distance(dst_node_id) / 2;
        self.get_kth_point_on_minimum_path(dst_node_id, median_distance)
    }

    pub(crate) fn get_eccentricity(&self) -> NodeT {
        self.eccentricity
    }

    pub(crate) fn get_most_distant_node(&self) -> NodeT {
        self.most_distant_node
    }

    pub(crate) fn into_iter_finite_distances(self) -> impl Iterator<Item = NodeT> {
        self.distances
            .into_iter()
            .filter(|&distance| distance != NOT_PRESENT)
    }

    pub(crate) fn into_par_iter_node_ids_and_finite_distances(
        self,
    ) -> impl ParallelIterator<Item = (NodeT, NodeT)> {
        self.distances
            .into_par_iter()
            .enumerate()
            .filter_map(|(node_id, distance)| {
                if distance != NOT_PRESENT {
                    Some((node_id as NodeT, distance))
                } else {
                    None
                }
            })
    }

    pub(crate) fn into_distances(self) -> Vec<NodeT> {
        self.distances
    }
}

pub struct ShortestPathsDjkstra {
    pub(crate) distances: Vec<f64>,
    pub(crate) predecessors: Option<Vec<Option<NodeT>>>,
    pub(crate) dst_node_distance: Option<f64>,
    pub(crate) eccentricity: f64,
    pub(crate) total_distance: f64,
    pub(crate) total_harmonic_distance: f64,
}

impl ShortestPathsDjkstra {
    pub(crate) fn new(
        distances: Vec<f64>,
        predecessors: Option<Vec<Option<NodeT>>>,
        dst_node_distance: Option<f64>,
        eccentricity: f64,
        total_distance: f64,
        total_harmonic_distance: f64,
    ) -> ShortestPathsDjkstra {
        ShortestPathsDjkstra {
            distances,
            predecessors,
            dst_node_distance,
            eccentricity,
            total_distance,
            total_harmonic_distance,
        }
    }
}

impl Graph {
    #[manual_binding]
    /// Returns vector of minimum paths distances and vector of nodes predecessors, if requested.
    ///
    /// # Arguments
    /// * `src_node_id`: NodeT - Root of the tree of minimum paths.
    /// * `maybe_dst_node_id`: Option<NodeT> - Optional target destination. If provided, Dijkstra will stop upon reaching this node.
    /// * `maybe_dst_node_ids`: Option<Vec<NodeT>> - Optional target destinations. If provided, Dijkstra will stop upon reaching all of these nodes.
    /// * `compute_distances`: Option<bool> - Whether to compute the vector of distances.
    /// * `compute_predecessors`: Option<bool> - Whether to compute the vector of predecessors.
    /// * `compute_visited`: Option<bool> - Whether to compute the vector of visited nodes.
    /// * `maximal_depth`: Option<NodeT> - The maximal depth to execute the DFS for.
    ///
    /// # Safety
    /// If any of the given node IDs does not exist in the graph the method will panic.
    pub unsafe fn get_unchecked_breath_first_search_from_node_ids(
        &self,
        src_node_id: NodeT,
        dst_node_id: Option<NodeT>,
        compute_predecessors: Option<bool>,
        maximal_depth: Option<NodeT>,
    ) -> ShortestPathsResultBFS {
        let compute_predecessors = compute_predecessors.unwrap_or(true);

        let nodes_number = self.get_nodes_number() as usize;
        let mut found_destination = false;

        let mut predecessors: Option<Vec<NodeT>> = if compute_predecessors {
            let mut predecessors = vec![NOT_PRESENT; nodes_number];
            predecessors[src_node_id as usize] = src_node_id;
            Some(predecessors)
        } else {
            None
        };

        let mut distances: Vec<NodeT> = vec![NOT_PRESENT; nodes_number];
        distances[src_node_id as usize] = 0;
        let mut eccentricity = 0;
        let mut most_distant_node = src_node_id;

        let mut nodes_to_explore = VecDeque::with_capacity(nodes_number);
        nodes_to_explore.push_back((src_node_id, 0));

        while let Some((node_id, depth)) = nodes_to_explore.pop_front() {
            // compute the distance of the childs of the current node
            let new_neighbour_distance = depth + 1;

            if eccentricity < depth {
                eccentricity = depth;
                most_distant_node = node_id;
            }

            // check if we need to stop
            if let Some(mi) = maximal_depth {
                if new_neighbour_distance > mi {
                    continue;
                }
            }

            // explore the neighbourhood of the current node
            self.iter_unchecked_neighbour_node_ids_from_source_node_id(node_id)
                .for_each(|neighbour_node_id| {
                    if found_destination {
                        return;
                    }
                    // If the node was not previously visited
                    if distances[neighbour_node_id as usize] == NOT_PRESENT {
                        // Set it's distance
                        distances[neighbour_node_id as usize] = new_neighbour_distance;

                        // and set its parent if we are asked to
                        if let Some(predecessors) = predecessors.as_mut() {
                            predecessors[neighbour_node_id as usize] = node_id;
                        }

                        if let Some(dst_node_id) = dst_node_id {
                            if neighbour_node_id == dst_node_id {
                                found_destination = true;
                            }
                        }

                        // add the node to the nodes to explore
                        nodes_to_explore.push_back((neighbour_node_id, new_neighbour_distance));
                    }
                });
            if found_destination {
                break;
            }
        }
        ShortestPathsResultBFS::new(distances, predecessors, eccentricity, most_distant_node)
    }

    /// Returns minimum path node IDs and distance from given node ids.
    ///
    /// # Arguments
    /// * `src_node_id`: NodeT - Source node ID.
    /// * `dst_node_id`: NodeT - Destination node ID.
    /// * `maximal_depth`: Option<NodeT> - The maximal depth to execute the BFS for.
    ///
    /// # Safety
    /// If any of the given node IDs does not exist in the graph the method will panic.
    ///
    /// # Raises
    /// * If the given node is a selfloop.
    /// * If there is no path between the two given nodes.
    pub unsafe fn get_unchecked_minimum_path_node_ids_from_node_ids(
        &self,
        src_node_id: NodeT,
        dst_node_id: NodeT,
        maximal_depth: Option<NodeT>,
    ) -> Result<Vec<NodeT>> {
        if src_node_id == dst_node_id {
            return Err("The minimum path on a selfloop is not defined.".to_string());
        }
        let bfs = self.get_unchecked_breath_first_search_from_node_ids(
            src_node_id,
            Some(dst_node_id),
            None,
            maximal_depth,
        );

        // If the distance is infinite, the destination node is not connected.
        if !bfs.has_path_to_node_id(dst_node_id) {
            return Err(format!(
                "There is no path starting from the given source node {} and reaching the given destination node {}.",
                src_node_id, dst_node_id
            ));
        }
        let path_length = bfs.get_distance_from_node_id(dst_node_id) as usize + 1;
        let mut path = vec![0; path_length];

        let mut parent_node_id = dst_node_id;
        (0..path_length).for_each(|index| {
            path[path_length - index - 1] = parent_node_id;
            parent_node_id = bfs.get_parent_from_node_id(parent_node_id).unwrap();
        });
        Ok(path)
    }

    /// Returns minimum path node names from given node ids.
    ///
    /// # Arguments
    /// * `src_node_id`: NodeT - Source node ID.
    /// * `dst_node_id`: NodeT - Destination node ID.
    /// * `maximal_depth`: Option<NodeT> - The maximal depth to execute the BFS for.
    ///
    /// # Safety
    /// If any of the given node IDs does not exist in the graph the method will panic.
    pub unsafe fn get_unchecked_minimum_path_node_names_from_node_ids(
        &self,
        src_node_id: NodeT,
        dst_node_id: NodeT,
        maximal_depth: Option<NodeT>,
    ) -> Result<Vec<String>> {
        Ok(self
            .get_unchecked_minimum_path_node_ids_from_node_ids(
                src_node_id,
                dst_node_id,
                maximal_depth,
            )?
            .into_iter()
            .map(|node_id| self.get_unchecked_node_name_from_node_id(node_id))
            .collect())
    }

    /// Returns minimum path node names from given node ids.
    ///
    /// # Arguments
    /// * `src_node_id`: NodeT - Source node ID.
    /// * `dst_node_id`: NodeT - Destination node ID.
    /// * `maximal_depth`: Option<NodeT> - The maximal depth to execute the BFS for.
    ///
    /// # Raises
    /// * If any of the given node IDs do not exist in the current graph.
    pub fn get_minimum_path_node_ids_from_node_ids(
        &self,
        src_node_id: NodeT,
        dst_node_id: NodeT,
        maximal_depth: Option<NodeT>,
    ) -> Result<Vec<NodeT>> {
        Ok(unsafe {
            self.get_unchecked_minimum_path_node_ids_from_node_ids(
                self.validate_node_id(src_node_id)?,
                self.validate_node_id(dst_node_id)?,
                maximal_depth,
            )?
        })
    }

    /// Returns minimum path node names from given node names.
    ///
    /// # Arguments
    /// * `src_node_name`: &str - Source node name.
    /// * `dst_node_name`: &str - Destination node name.
    /// * `maximal_depth`: Option<NodeT> - The maximal depth to execute the BFS for.
    ///
    /// # Raises
    /// * If any of the given node names do not exist in the current graph.
    pub fn get_minimum_path_node_ids_from_node_names(
        &self,
        src_node_name: &str,
        dst_node_name: &str,
        maximal_depth: Option<NodeT>,
    ) -> Result<Vec<NodeT>> {
        Ok(unsafe {
            self.get_unchecked_minimum_path_node_ids_from_node_ids(
                self.get_node_id_from_node_name(src_node_name)?,
                self.get_node_id_from_node_name(dst_node_name)?,
                maximal_depth,
            )?
        })
    }

    /// Returns minimum path node names from given node names.
    ///
    /// # Arguments
    /// * `src_node_name`: &str - Source node name.
    /// * `dst_node_name`: &str - Destination node name.
    /// * `maximal_depth`: Option<NodeT> - The maximal depth to execute the BFS for.
    ///
    /// # Raises
    /// * If any of the given node names do not exist in the current graph.
    pub fn get_minimum_path_node_names_from_node_names(
        &self,
        src_node_name: &str,
        dst_node_name: &str,
        maximal_depth: Option<NodeT>,
    ) -> Result<Vec<String>> {
        Ok(unsafe {
            self.get_unchecked_minimum_path_node_names_from_node_ids(
                self.get_node_id_from_node_name(src_node_name)?,
                self.get_node_id_from_node_name(dst_node_name)?,
                maximal_depth,
            )?
        })
    }

    #[no_numpy_binding]
    /// Return vector of the k minimum paths node IDs between given source node and destination node ID.
    ///
    /// # Arguments
    /// * `src_node_id`: NodeT - Source node ID.
    /// * `dst_node_id`: NodeT - Destination node ID.
    /// * `k`: usize - Number of paths to find.
    ///
    /// # Implementative details
    /// This method is not converted to a numpy array because it would have
    /// to be a ragged array, as the different paths have different lengths.
    ///
    /// # Safety
    /// If any of the given node IDs does not exist in the graph the method will panic.
    pub unsafe fn get_unchecked_k_shortest_path_node_ids_from_node_ids(
        &self,
        src_node_id: NodeT,
        dst_node_id: NodeT,
        k: usize,
    ) -> Vec<Vec<NodeT>> {
        let nodes_number = self.get_nodes_number() as usize;
        let mut counts = vec![0; nodes_number];
        let mut paths = Vec::new();

        let mut nodes_to_explore = VecDeque::with_capacity(nodes_number);
        nodes_to_explore.push_back(vec![src_node_id]);

        while let Some(path) = nodes_to_explore.pop_front() {
            // If we have found all the required paths we can exit
            if counts[dst_node_id as usize] >= k {
                break;
            }
            let node_id = *path.last().unwrap();
            counts[node_id as usize] += 1;

            if node_id == dst_node_id {
                paths.push(path);
                continue;
            }

            // If the number of identified paths to
            // node ID is greater than k, we can continue.
            if counts[node_id as usize] > k {
                continue;
            }

            self.iter_unchecked_neighbour_node_ids_from_source_node_id(node_id)
                .for_each(|neighbour_node_id| {
                    let mut new_path = path.clone();
                    new_path.push(neighbour_node_id);
                    nodes_to_explore.push_back(new_path);
                });
        }
        paths
    }

    #[fuzz_type(k: u8)]
    #[no_numpy_binding]
    /// Return vector of the k minimum paths node IDs between given source node and destination node ID.
    ///
    /// # Arguments
    /// * `src_node_id`: NodeT - Source node ID.
    /// * `dst_node_id`: NodeT - Destination node ID.
    /// * `maximal_depth`: Option<NodeT> - The maximal depth to execute the BFS for.
    /// * `k`: usize - Number of paths to find.
    ///
    /// # Implementative details
    /// This method is not converted to a numpy array because it would have
    /// to be a ragged array, as the different paths have different lengths.
    ///
    /// # Raises
    /// * If any of the given node IDs does not exist in the graph.
    pub fn get_k_shortest_path_node_ids_from_node_ids(
        &self,
        src_node_id: NodeT,
        dst_node_id: NodeT,
        k: usize,
    ) -> Result<Vec<Vec<NodeT>>> {
        Ok(unsafe {
            self.get_unchecked_k_shortest_path_node_ids_from_node_ids(
                self.validate_node_id(src_node_id)?,
                self.validate_node_id(dst_node_id)?,
                k,
            )
        })
    }

    #[fuzz_type(k: u8)]
    #[no_numpy_binding]
    /// Return vector of the k minimum paths node IDs between given source node and destination node name.
    ///
    /// # Arguments
    /// * `src_node_name`: &str - Source node name.
    /// * `dst_node_name`: &str - Destination node name.
    /// * `k`: usize - Number of paths to find.
    ///
    /// # Implementative details
    /// This method is not converted to a numpy array because it would have
    /// to be a ragged array, as the different paths have different lengths.
    ///
    /// # Raises
    /// * If any of the given node names does not exist in the graph.
    pub fn get_k_shortest_path_node_ids_from_node_names(
        &self,
        src_node_name: &str,
        dst_node_name: &str,
        k: usize,
    ) -> Result<Vec<Vec<NodeT>>> {
        Ok(unsafe {
            self.get_unchecked_k_shortest_path_node_ids_from_node_ids(
                self.get_node_id_from_node_name(src_node_name)?,
                self.get_node_id_from_node_name(dst_node_name)?,
                k,
            )
        })
    }

    #[fuzz_type(k: u8)]
    #[no_numpy_binding]
    /// Return vector of the k minimum paths node names between given source node and destination node name.
    ///
    /// # Arguments
    /// * `src_node_name`: &str - Source node name.
    /// * `dst_node_name`: &str - Destination node name.
    /// * `k`: usize - Number of paths to find.
    ///
    /// # Implementative details
    /// This method is not converted to a numpy array because it would have
    /// to be a ragged array, as the different paths have different lengths.
    ///
    /// # Raises
    /// * If any of the given node names does not exist in the graph.
    pub fn get_k_shortest_path_node_names_from_node_names(
        &self,
        src_node_name: &str,
        dst_node_name: &str,
        k: usize,
    ) -> Result<Vec<Vec<String>>> {
        self.get_k_shortest_path_node_ids_from_node_names(src_node_name, dst_node_name, k)
            .map(|paths| {
                paths
                    .into_iter()
                    .map(|path| {
                        path.into_iter()
                            .map(|node_id| unsafe {
                                self.get_unchecked_node_name_from_node_id(node_id)
                            })
                            .collect()
                    })
                    .collect()
            })
    }

    /// Returns unweighted eccentricity of the given node.
    ///
    /// This method will panic if the given node ID does not exists in the graph.
    ///
    /// # Arguments
    /// * `node_id`: NodeT - Node for which to compute the eccentricity.
    ///
    /// # Safety
    /// If any of the given node IDs does not exist in the graph the method will panic.
    pub unsafe fn get_unchecked_eccentricity_from_node_id(&self, node_id: NodeT) -> NodeT {
        self.get_unchecked_breath_first_search_from_node_ids(node_id, None, None, None)
            .get_eccentricity()
    }

    #[manual_binding]
    /// Returns weighted eccentricity of the given node.
    ///
    /// This method will panic if the given node ID does not exists in the graph.
    ///
    /// # Arguments
    /// * `node_id`: NodeT - Node for which to compute the eccentricity.
    /// * `use_edge_weights_as_probabilities`: Option<bool> - Whether to treat the edge weights as probabilities.
    ///
    /// # Safety
    /// If any of the given node IDs does not exist in the graph the method will panic.
    pub unsafe fn get_unchecked_weighted_eccentricity_from_node_id(
        &self,
        node_id: NodeT,
        use_edge_weights_as_probabilities: Option<bool>,
    ) -> f64 {
        self.get_unchecked_dijkstra_from_node_ids(
            node_id,
            None,
            None,
            Some(false),
            None,
            use_edge_weights_as_probabilities,
        )
        .eccentricity
    }

    /// Returns unweighted eccentricity of the given node ID.
    ///
    /// # Arguments
    /// * `node_id`: NodeT - Node for which to compute the eccentricity.
    /// * `use_edge_weights_as_probabilities`: Option<bool> - Whether to treat the edge weights as probabilities.
    ///
    /// # Raises
    /// * If the given node ID does not exist in the graph.
    pub fn get_eccentricity_from_node_id(&self, node_id: NodeT) -> Result<NodeT> {
        self.validate_node_id(node_id)
            .map(|node_id| unsafe { self.get_unchecked_eccentricity_from_node_id(node_id) })
    }

    /// Returns weighted eccentricity of the given node ID.
    ///
    /// # Arguments
    /// * `node_id`: NodeT - Node for which to compute the eccentricity.
    /// * `use_edge_weights_as_probabilities`: Option<bool> - Whether to treat the edge weights as probabilities.
    ///
    /// # Raises
    /// * If the given node ID does not exist in the graph.
    /// * If weights are requested to be treated as probabilities but are not between 0 and 1.
    /// * If the graph contains negative weights.
    pub fn get_weighted_eccentricity_from_node_id(
        &self,
        node_id: NodeT,
        use_edge_weights_as_probabilities: Option<bool>,
    ) -> Result<f64> {
        if let Some(uewap) = use_edge_weights_as_probabilities {
            if uewap {
                self.must_have_edge_weights_representing_probabilities()?;
            }
        }
        self.must_have_positive_edge_weights()?;
        self.validate_node_id(node_id).map(|node_id| unsafe {
            self.get_unchecked_weighted_eccentricity_from_node_id(
                node_id,
                use_edge_weights_as_probabilities,
            )
        })
    }

    /// Returns unweighted eccentricity of the given node name.
    ///
    /// # Arguments
    /// * `node_name`: &str - Node for which to compute the eccentricity.
    ///
    /// # Raises
    /// * If the given node name does not exist in the current graph instance.
    pub fn get_eccentricity_from_node_name(&self, node_name: &str) -> Result<NodeT> {
        self.get_node_id_from_node_name(node_name)
            .map(|node_id| unsafe { self.get_unchecked_eccentricity_from_node_id(node_id) })
    }

    /// Returns weighted eccentricity of the given node name.
    ///
    /// # Arguments
    /// * `node_name`: &str - Node for which to compute the eccentricity.
    /// * `use_edge_weights_as_probabilities`: Option<bool> - Whether to treat the edge weights as probabilities.
    ///
    /// # Raises
    /// * If the given node name does not exist in the graph.
    /// * If weights are requested to be treated as probabilities but are not between 0 and 1.
    /// * If the graph contains negative weights.
    pub fn get_weighted_eccentricity_from_node_name(
        &self,
        node_name: &str,
        use_edge_weights_as_probabilities: Option<bool>,
    ) -> Result<f64> {
        if let Some(uewap) = use_edge_weights_as_probabilities {
            if uewap {
                self.must_have_edge_weights_representing_probabilities()?;
            }
        }
        self.must_have_positive_edge_weights()?;
        self.get_node_id_from_node_name(node_name)
            .map(|node_id| unsafe {
                self.get_unchecked_weighted_eccentricity_from_node_id(
                    node_id,
                    use_edge_weights_as_probabilities,
                )
            })
    }

    #[manual_binding]
    /// Returns vector of minimum paths distances and vector of nodes predecessors, if requested.
    ///
    /// # Arguments
    /// * `src_node_id`: NodeT - Root of the tree of minimum paths.
    /// * `maybe_dst_node_id`: Option<NodeT> - Optional target destination. If provided, Dijkstra will stop upon reaching this node.
    /// * `maybe_dst_node_ids`: Option<Vec<NodeT>> - Optional target destinations. If provided, Dijkstra will stop upon reaching all of these nodes.
    /// * `compute_predecessors`: bool - Whether to compute the vector of predecessors.
    /// * `maximal_depth`: Option<NodeT> - The maximal number of iterations to execute Dijkstra for.
    /// * `use_edge_weights_as_probabilities`: Option<bool> - Whether to treat the edge weights as probabilities.
    ///
    /// # Safety
    /// If any of the given node IDs does not exist in the graph the method will panic.
    pub unsafe fn get_unchecked_dijkstra_from_node_ids(
        &self,
        src_node_id: NodeT,
        maybe_dst_node_id: Option<NodeT>,
        mut maybe_dst_node_ids: Option<Vec<NodeT>>,
        compute_predecessors: Option<bool>,
        maximal_depth: Option<NodeT>,
        use_edge_weights_as_probabilities: Option<bool>,
    ) -> ShortestPathsDjkstra {
        let compute_predecessors = compute_predecessors.unwrap_or(true);
        let nodes_number = self.get_nodes_number() as usize;
        let use_edge_weights_as_probabilities = use_edge_weights_as_probabilities.unwrap_or(false);
        let mut dst_node_distance = maybe_dst_node_id.map(|_| {
            if use_edge_weights_as_probabilities {
                0.0
            } else {
                f64::INFINITY
            }
        });
        let mut predecessors: Option<Vec<Option<NodeT>>> = if compute_predecessors {
            Some(vec![None; nodes_number])
        } else {
            None
        };

        if self.is_unchecked_disconnected_from_node_id(src_node_id) {
            if use_edge_weights_as_probabilities {
                return ShortestPathsDjkstra::new(
                    vec![0.0; nodes_number],
                    predecessors,
                    dst_node_distance,
                    0.0,
                    0.0,
                    0.0,
                );
            } else {
                return ShortestPathsDjkstra::new(
                    vec![f64::INFINITY; nodes_number],
                    predecessors,
                    dst_node_distance,
                    f64::INFINITY,
                    f64::INFINITY,
                    0.0,
                );
            }
        }

        let bfs: Option<ShortestPathsResultBFS> = maximal_depth.map(|md| {
            self.get_unchecked_breath_first_search_from_node_ids(
                src_node_id,
                maybe_dst_node_id,
                None,
                Some(md),
            )
        });

        let mut nodes_to_explore: DijkstraQueue =
            DijkstraQueue::with_capacity_from_root(nodes_number, src_node_id as usize);
        let mut eccentricity: f64 = 0.0;
        let mut total_distance: f64 = 0.0;
        let mut total_harmonic_distance: f64 = 0.0;

        while let Some(closest_node_id) = nodes_to_explore.pop() {
            // Update the distances metrics
            eccentricity = eccentricity.max(nodes_to_explore[closest_node_id]);
            total_distance += nodes_to_explore[closest_node_id];
            if nodes_to_explore[closest_node_id] > 0.0 {
                total_harmonic_distance += if use_edge_weights_as_probabilities {
                    (-nodes_to_explore[closest_node_id]).exp()
                } else {
                    1.0 / nodes_to_explore[closest_node_id]
                };
            }
            // If the closest node is the optional destination node, we have
            // completed what the user has required.
            if maybe_dst_node_id.map_or(false, |dst| dst == closest_node_id as NodeT) {
                dst_node_distance.insert(if use_edge_weights_as_probabilities {
                    (-nodes_to_explore[closest_node_id]).exp()
                } else {
                    nodes_to_explore[closest_node_id]
                });
                break;
            }
            // If the closest node is in the set of the destination nodes
            if let Some(dst_node_ids) = &mut maybe_dst_node_ids {
                // We remove it
                let node_id_idx = dst_node_ids
                    .iter()
                    .position(|x| *x as usize == closest_node_id);

                if let Some(nii) = node_id_idx {
                    dst_node_ids.remove(nii);
                }
                // And if now the roaringbitmap is empty
                if dst_node_ids.is_empty() {
                    // We have completed the requested task.
                    break;
                }
            }

            self.iter_unchecked_neighbour_node_ids_from_source_node_id(closest_node_id as NodeT)
                .zip(self.iter_unchecked_edge_weights_from_source_node_id(closest_node_id as NodeT))
                .for_each(|(neighbour_node_id, weight)| {
                    if let Some(bfs) = bfs.as_ref() {
                        if !bfs.has_path_to_node_id(neighbour_node_id) {
                            return;
                        }
                    }
                    let new_neighbour_distance = nodes_to_explore[closest_node_id]
                        + if use_edge_weights_as_probabilities {
                            -(weight as f64).ln()
                        } else {
                            weight as f64
                        };
                    if new_neighbour_distance < nodes_to_explore[neighbour_node_id as usize] {
                        if let Some(predecessors) = &mut predecessors {
                            predecessors[neighbour_node_id as usize] =
                                Some(closest_node_id as NodeT);
                        }
                        nodes_to_explore.push(neighbour_node_id as usize, new_neighbour_distance);
                    }
                });
        }

        let mut distances = nodes_to_explore.unwrap();

        if use_edge_weights_as_probabilities {
            distances
                .iter_mut()
                .for_each(|distance| *distance = (-*distance).exp());
            eccentricity = (-eccentricity).exp();
            total_distance = (-total_distance).exp();
        }

        ShortestPathsDjkstra {
            distances,
            predecessors,
            dst_node_distance,
            eccentricity,
            total_distance,
            total_harmonic_distance,
        }
    }

    /// Returns minimum path node IDs and distance from given node ids.
    ///
    /// # Arguments
    /// * `src_node_id`: NodeT - Source node ID.
    /// * `dst_node_id`: NodeT - Destination node ID.
    /// * `maximal_depth`: Option<NodeT> - The maximal depth to execute the BFS for.
    /// * `use_edge_weights_as_probabilities`: Option<bool> - Whether to treat the edge weights as probabilities.
    /// * `maximal_depth`: Option<NodeT> - The maximal number of iterations to execute Dijkstra for.
    ///
    /// # Safety
    /// If any of the given node IDs does not exist in the graph the method will panic.
    pub unsafe fn get_unchecked_weighted_minimum_path_node_ids_from_node_ids(
        &self,
        src_node_id: NodeT,
        dst_node_id: NodeT,
        use_edge_weights_as_probabilities: Option<bool>,
        maximal_depth: Option<NodeT>,
    ) -> (f64, Vec<NodeT>) {
        let dijkstra = self.get_unchecked_dijkstra_from_node_ids(
            src_node_id,
            Some(dst_node_id),
            None,
            Some(true),
            maximal_depth,
            use_edge_weights_as_probabilities,
        );
        let predecessors = dijkstra.predecessors.unwrap();
        let path_length = dijkstra.dst_node_distance.unwrap();
        if let Some(uewp) = use_edge_weights_as_probabilities {
            // If the path length is to be treated as a probability and the
            // resulting probability is 0, it means that it is impossible
            // to get from the requested source node to the requested
            // destination node.
            if uewp && path_length.is_zero() {
                return (0.0, Vec::new());
            }
        }
        // If the path length is infinite, it means that there is no path
        // between the given source node and the given destination node.
        if path_length.is_infinite() {
            return (f64::INFINITY, Vec::new());
        }
        // Since we need to visit the predecessors vector we will be building
        // the path backwards and we will need to invert it afterwards.
        let mut reverse_path = Vec::new();
        let mut parent = dst_node_id;
        loop {
            reverse_path.push(parent);
            if parent == src_node_id {
                break;
            }
            if let Some(new_parent) = predecessors[parent as usize] {
                parent = new_parent;
            }
        }
        // Now we revert the path.
        (path_length, reverse_path.into_iter().rev().collect())
    }

    /// Returns minimum path node names from given node ids.
    ///
    /// # Arguments
    /// * `src_node_id`: NodeT - Source node ID.
    /// * `dst_node_id`: NodeT - Destination node ID.
    /// * `maximal_depth`: Option<NodeT> - The maximal depth to execute the BFS for.
    /// * `use_edge_weights_as_probabilities`: Option<bool> - Whether to treat the edge weights as probabilities.
    /// * `maximal_depth`: Option<NodeT> - The maximal number of iterations to execute Dijkstra for.
    ///
    /// # Safety
    /// If any of the given node IDs does not exist in the graph the method will panic.
    pub unsafe fn get_unchecked_weighted_minimum_path_node_names_from_node_ids(
        &self,
        src_node_id: NodeT,
        dst_node_id: NodeT,
        use_edge_weights_as_probabilities: Option<bool>,
        maximal_depth: Option<NodeT>,
    ) -> (f64, Vec<String>) {
        let (path_length, path) = self.get_unchecked_weighted_minimum_path_node_ids_from_node_ids(
            src_node_id,
            dst_node_id,
            use_edge_weights_as_probabilities,
            maximal_depth,
        );
        (
            path_length,
            path.into_iter()
                .map(|node_id| self.get_unchecked_node_name_from_node_id(node_id))
                .collect(),
        )
    }

    /// Returns minimum path node names from given node ids.
    ///
    /// # Arguments
    /// * `src_node_id`: NodeT - Source node ID.
    /// * `dst_node_id`: NodeT - Destination node ID.
    /// * `maximal_depth`: Option<NodeT> - The maximal depth to execute the BFS for.
    /// * `use_edge_weights_as_probabilities`: Option<bool> - Whether to treat the edge weights as probabilities.
    /// * `maximal_depth`: Option<NodeT> - The maximal number of iterations to execute Dijkstra for.
    ///
    /// # Raises
    /// * If any of the given node IDs do not exist in the current graph.
    pub fn get_weighted_minimum_path_node_ids_from_node_ids(
        &self,
        src_node_id: NodeT,
        dst_node_id: NodeT,
        use_edge_weights_as_probabilities: Option<bool>,
        maximal_depth: Option<NodeT>,
    ) -> Result<(f64, Vec<NodeT>)> {
        self.must_have_positive_edge_weights()?;
        if let Some(uewp) = use_edge_weights_as_probabilities {
            if uewp {
                self.must_have_edge_weights_representing_probabilities()?;
            }
        }
        Ok(unsafe {
            self.get_unchecked_weighted_minimum_path_node_ids_from_node_ids(
                self.validate_node_id(src_node_id)?,
                self.validate_node_id(dst_node_id)?,
                use_edge_weights_as_probabilities,
                maximal_depth,
            )
        })
    }

    /// Returns minimum path node names from given node names.
    ///
    /// # Arguments
    /// * `src_node_name`: &str - Source node name.
    /// * `dst_node_name`: &str - Destination node name.
    /// * `use_edge_weights_as_probabilities`: Option<bool> - Whether to treat the edge weights as probabilities.
    /// * `maximal_depth`: Option<NodeT> - The maximal number of iterations to execute Dijkstra for.
    ///
    /// # Raises
    /// * If any of the given node names do not exist in the current graph.
    pub fn get_weighted_minimum_path_node_ids_from_node_names(
        &self,
        src_node_name: &str,
        dst_node_name: &str,
        use_edge_weights_as_probabilities: Option<bool>,
        maximal_depth: Option<NodeT>,
    ) -> Result<(f64, Vec<NodeT>)> {
        self.must_have_positive_edge_weights()?;
        if let Some(uewp) = use_edge_weights_as_probabilities {
            if uewp {
                self.must_have_edge_weights_representing_probabilities()?;
            }
        }
        Ok(unsafe {
            self.get_unchecked_weighted_minimum_path_node_ids_from_node_ids(
                self.get_node_id_from_node_name(src_node_name)?,
                self.get_node_id_from_node_name(dst_node_name)?,
                use_edge_weights_as_probabilities,
                maximal_depth,
            )
        })
    }

    /// Returns minimum path node names from given node names.
    ///
    /// # Arguments
    /// * `src_node_name`: &str - Source node name.
    /// * `dst_node_name`: &str - Destination node name.
    /// * `use_edge_weights_as_probabilities`: Option<bool> - Whether to treat the edge weights as probabilities.
    /// * `maximal_depth`: Option<NodeT> - The maximal number of iterations to execute Dijkstra for.
    ///
    /// # Raises
    /// * If any of the given node names do not exist in the current graph.
    pub fn get_weighted_minimum_path_node_names_from_node_names(
        &self,
        src_node_name: &str,
        dst_node_name: &str,
        use_edge_weights_as_probabilities: Option<bool>,
        maximal_depth: Option<NodeT>,
    ) -> Result<(f64, Vec<String>)> {
        self.must_have_positive_edge_weights()?;
        if let Some(uewp) = use_edge_weights_as_probabilities {
            if uewp {
                self.must_have_edge_weights_representing_probabilities()?;
            }
        }
        Ok(unsafe {
            self.get_unchecked_weighted_minimum_path_node_names_from_node_ids(
                self.get_node_id_from_node_name(src_node_name)?,
                self.get_node_id_from_node_name(dst_node_name)?,
                use_edge_weights_as_probabilities,
                maximal_depth,
            )
        })
    }

    #[manual_binding]
    /// Returns vector of minimum paths distances and vector of nodes predecessors from given source node ID and optional destination node ID.
    ///
    /// # Arguments
    /// * `src_node_id`: NodeT - Node ID root of the tree of minimum paths.
    /// * `compute_predecessors`: Option<bool> - Whether to compute the vector of predecessors.
    /// * `maximal_depth`: Option<NodeT> - The maximal number of iterations to execute the DFS for.
    ///
    /// # Raises
    /// * If the given source node ID does not exist in the current graph.
    /// * If the given optional destination node ID does not exist in the current graph.
    pub fn get_breath_first_search_from_node_ids(
        &self,
        src_node_id: NodeT,
        dst_node_id: Option<NodeT>,
        compute_predecessors: Option<bool>,
        maximal_depth: Option<NodeT>,
    ) -> Result<ShortestPathsResultBFS> {
        // Check if the given root exists in the graph
        self.validate_node_id(src_node_id)?;
        unsafe {
            Ok(self.get_unchecked_breath_first_search_from_node_ids(
                src_node_id,
                dst_node_id,
                compute_predecessors,
                maximal_depth,
            ))
        }
    }

    #[manual_binding]
    /// Returns vector of minimum paths distances and vector of nodes predecessors from given source node ID and optional destination node ID.
    ///
    /// # Arguments
    /// * `src_node_id`: NodeT - Node ID root of the tree of minimum paths.
    /// * `maybe_dst_node_id`: Option<NodeT> - Optional target destination. If provided, Dijkstra will stop upon reaching this node.
    /// * `maybe_dst_node_ids`: Option<Vec<NodeT>> - Optional target destinations. If provided, Dijkstra will stop upon reaching all of these nodes.
    /// * `compute_predecessors`: Option<bool> - Whether to compute the vector of predecessors.
    /// * `maximal_depth`: Option<NodeT> - The maximal depth to execute the DFS for.
    /// * `use_edge_weights_as_probabilities`: Option<bool> - Whether to treat the edge weights as probabilities.
    ///
    /// # Raises
    /// * If the weights are to be used and the graph does not have weights.
    /// * If the given source node ID does not exist in the current graph.
    /// * If the given optional destination node ID does not exist in the current graph.
    /// * If weights are requested to be treated as probabilities but are not between 0 and 1.
    /// * If the graph contains negative weights.
    pub fn get_dijkstra_from_node_ids(
        &self,
        src_node_id: NodeT,
        maybe_dst_node_id: Option<NodeT>,
        maybe_dst_node_ids: Option<Vec<NodeT>>,
        compute_predecessors: Option<bool>,
        maximal_depth: Option<NodeT>,
        use_edge_weights_as_probabilities: Option<bool>,
    ) -> Result<ShortestPathsDjkstra> {
        // Check if the given root exists in the graph
        self.validate_node_id(src_node_id)?;
        self.must_have_positive_edge_weights()?;
        if let Some(uewap) = use_edge_weights_as_probabilities {
            if uewap {
                self.must_have_edge_weights_representing_probabilities()?;
            }
        };
        // If given, check if the given destination node ID exists in the graph
        if let Some(dst) = &maybe_dst_node_id {
            self.validate_node_id(*dst)?;
        }

        let maybe_dst_node_ids = maybe_dst_node_ids.map_or(Ok::<_, String>(None), |node_ids| {
            Ok(Some(self.validate_node_ids(node_ids)?))
        })?;

        Ok(unsafe {
            self.get_unchecked_dijkstra_from_node_ids(
                src_node_id,
                maybe_dst_node_id,
                maybe_dst_node_ids,
                compute_predecessors,
                maximal_depth,
                use_edge_weights_as_probabilities,
            )
        })
    }

    /// Returns approximated diameter and tentative low eccentricity node for an UNDIRECTED graph.
    /// This method returns a lowerbound of the diameter by doing the following steps:
    /// * Find the most central node
    /// * Find the most distant node from the most central one (and get a first
    ///    approximation of the diameter lowerbound)
    /// * Get the median node in this path
    /// * Find the most distant node from the median node
    /// * Find the most distant node form the last one, and get the second approx
    ///     of the diameter lowerbound.
    ///
    /// This basically creates a "cross" that spans the graph.
    fn get_four_sweep(&self) -> Result<(NodeT, NodeT)> {
        let most_central_node_id = unsafe { *self.get_unchecked_most_central_node_id() };
        let first_candidate_most_eccentric_node_id = unsafe {
            self.get_unchecked_breath_first_search_from_node_ids(
                most_central_node_id,
                None,
                None,
                None,
            )
            .get_most_distant_node()
        };

        let bfs1 = unsafe {
            self.get_unchecked_breath_first_search_from_node_ids(
                first_candidate_most_eccentric_node_id,
                None,
                Some(true),
                None,
            )
        };

        let second_candidate_most_eccentric_node_id = unsafe {
            self.get_unchecked_breath_first_search_from_node_ids(
                bfs1.get_median_point(bfs1.get_most_distant_node())?,
                None,
                Some(true),
                None,
            )
            .get_most_distant_node()
        };
        let bfs2 = unsafe {
            self.get_unchecked_breath_first_search_from_node_ids(
                second_candidate_most_eccentric_node_id,
                None,
                Some(true),
                None,
            )
        };

        Ok((
            bfs1.get_eccentricity().max(bfs2.get_eccentricity()),
            bfs2.get_median_point(bfs2.get_most_distant_node())?,
        ))
    }

    /// Returns diameter of an UNDIRECTED graph.
    ///
    /// # Arguments
    /// * `verbose`: Option<bool> - Whether to show a loading bar.
    ///
    /// # Referencences
    /// This method is based on the algorithm described in ["On computing the diameter of real-world undirected graphs" by Crescenzi et al](https://who.rocq.inria.fr/Laurent.Viennot/road/papers/ifub.pdf).
    fn get_ifub(&self, verbose: Option<bool>) -> Result<f64> {
        if self.is_directed() {
            panic!(
                "This method is not defined YET for directed graphs! We will add it in the future!"
            )
        }

        let most_central_node_id = unsafe { *self.get_unchecked_most_central_node_id() };
        if unsafe { self.is_unchecked_disconnected_from_node_id(most_central_node_id) } {
            return Ok(0.0);
        }

        // get the lowerbound of the diameter
        let (tentative_diameter, low_eccentricity_node) = self.get_four_sweep()?;
        // find the distances of all the nodes from the node with low eccentricty,
        // and thus with high centrality
        let bfs = unsafe {
            self.get_unchecked_breath_first_search_from_node_ids(
                low_eccentricity_node,
                None,
                None,
                None,
            )
        };
        assert!(
            tentative_diameter != NodeT::MAX,
            "The central node eccentricity cannot be infinite!"
        );
        assert!(
            tentative_diameter != 0,
            "The central node eccentricity cannot be zero!"
        );

        // filter out all the nodes which are surely not start or end of the diameter
        // because they are too close to the central node, this keeps only the outer
        // crown of nodes.
        let mut node_ids_and_distances = bfs
            .into_par_iter_node_ids_and_finite_distances()
            .filter(|&(_, distance)| tentative_diameter < distance * 2)
            .collect::<Vec<(NodeT, NodeT)>>();

        // sort the nodes by distance, so that we will start checking from the
        // most distant ones which are the most probable to be an extreme of the
        // diameter.
        // Since this vector is generally expected to be quite small,
        // we proceed with a non-parallell approach to avoid spinning up
        // threads for no good reason.
        node_ids_and_distances.sort_by(|(_, a), &(_, b)| b.cmp(a));

        // Fold it into groups
        let mut current_distance = node_ids_and_distances[0].1;
        let mut distance_groups: Vec<(NodeT, Vec<NodeT>)> = vec![(current_distance, Vec::new())];
        for (node_id, distance) in node_ids_and_distances {
            if current_distance == distance {
                distance_groups.last_mut().unwrap().1.push(node_id);
            } else {
                current_distance = distance;
                distance_groups.push((distance, vec![node_id]));
            }
        }

        // Put tentative diameter into an AtomicU32
        let tentative_diameter = AtomicU32::new(tentative_diameter);

        let pb = get_loading_bar(
            verbose.unwrap_or(true) && distance_groups.len() > 1,
            "Computing diameter groups",
            distance_groups.len(),
        );

        // for each possible node of the outer crown compute the maximum path
        // from there, this way we can find the exact diameter
        distance_groups
            .into_iter()
            .progress_with(pb)
            .for_each(|(distance, node_ids)| unsafe {
                // If we have not yet reached the bound
                if tentative_diameter.load(Ordering::Relaxed) < distance * 2 {
                    let pb2 = get_loading_bar(
                        verbose.unwrap_or(true) && node_ids.len() > 1,
                        &format!("Computing diameter of nodes at distance {}", distance),
                        node_ids.len(),
                    );
                    // We compute the new candidate diameter.
                    tentative_diameter.fetch_max(
                        node_ids
                            .into_par_iter()
                            .progress_with(pb2)
                            .map(|node_id| self.get_unchecked_eccentricity_from_node_id(node_id))
                            .max()
                            .unwrap(),
                        Ordering::Relaxed,
                    );
                }
            });

        Ok(tentative_diameter.into_inner() as f64)
    }

    /// Returns diameter of the graph using naive method.
    ///
    /// Note that there exists the non-naive method for undirected graphs
    /// and it is possible to implement a faster method for directed graphs
    /// but we still need to get to it, as it will require an updated
    /// succinct data structure.
    ///
    /// # Arguments
    /// * `ignore_infinity`: Option<bool> - Whether to ignore infinite distances, which are present when in the graph exist multiple components.
    /// * `verbose`: Option<bool> - Whether to show a loading bar.
    ///
    /// # Raises
    /// * If the graph does not contain nodes.
    pub fn get_diameter_naive(
        &self,
        ignore_infinity: Option<bool>,
        verbose: Option<bool>,
    ) -> Result<f64> {
        self.must_have_nodes()?;
        let ignore_infinity = ignore_infinity.unwrap_or(false);
        let verbose = verbose.unwrap_or(true);

        if !self.has_edges() || !ignore_infinity && !self.is_connected(Some(verbose)) {
            return Ok(f64::INFINITY);
        }

        let pb = get_loading_bar(
            verbose,
            "Computing diameter",
            self.get_nodes_number() as usize,
        );
        Ok(self
            .par_iter_node_ids()
            .progress_with(pb)
            .map(|node_id| unsafe { self.get_unchecked_eccentricity_from_node_id(node_id) })
            .filter(|&distance| !ignore_infinity || distance != NOT_PRESENT)
            .max()
            .unwrap_or(0) as f64)
    }

    /// Returns diameter of the graph.
    ///
    /// # Arguments
    /// * `ignore_infinity`: Option<bool> - Whether to ignore infinite distances, which are present when in the graph exist multiple components.
    /// * `verbose`: Option<bool> - Whether to show a loading bar.
    ///
    /// # Raises
    /// * If the graph does not contain nodes.
    ///
    /// TODO! Add better implementation for directed graphs
    /// To make the better implementation for directed graphs we will first
    /// need to make the Elias-Fano encode the directed graph in a better way.
    pub fn get_diameter(
        &self,
        ignore_infinity: Option<bool>,
        verbose: Option<bool>,
    ) -> Result<f64> {
        self.must_have_nodes()?;
        let ignore_infinity = ignore_infinity.unwrap_or(false);
        let verbose = verbose.unwrap_or(true);

        if !self.has_edges() || !ignore_infinity && !self.is_connected(Some(verbose)) {
            return Ok(f64::INFINITY);
        }

        if self.is_directed() {
            self.get_diameter_naive(Some(true), Some(verbose))
        } else {
            self.get_ifub(Some(verbose))
        }
    }

    /// Returns diameter of the graph using naive method.
    ///
    /// Note that there exists the non-naive method for undirected graphs
    /// and it is possible to implement a faster method for directed graphs
    /// but we still need to get to it, as it will require an updated
    /// succinct data structure.
    ///
    /// # Arguments
    /// * `ignore_infinity`: Option<bool> - Whether to ignore infinite distances, which are present when in the graph exist multiple components.
    /// * `use_edge_weights_as_probabilities`: Option<bool> - Whether to treat the edge weights as probabilities.
    /// * `verbose`: Option<bool> - Whether to show a loading bar.
    ///
    /// # Raises
    /// * If the graph does not contain nodes.
    /// * If the graph does not have weights.
    /// * If the graph contains negative weights.
    /// * If the user has asked for the weights to be treated as probabilities but the weights are not between 0 and 1.
    ///
    /// TODO! Add better implementation for directed graphs
    /// To make the better implementation for directed graphs we will first
    /// need to make the Elias-Fano encode the directed graph in a better way.
    pub fn get_weighted_diameter_naive(
        &self,
        ignore_infinity: Option<bool>,
        use_edge_weights_as_probabilities: Option<bool>,
        verbose: Option<bool>,
    ) -> Result<f64> {
        self.must_have_nodes()?;
        self.must_have_positive_edge_weights()?;
        let use_edge_weights_as_probabilities = use_edge_weights_as_probabilities.unwrap_or(false);
        if use_edge_weights_as_probabilities {
            self.must_have_edge_weights_representing_probabilities()?;
        }
        let ignore_infinity = ignore_infinity.unwrap_or(true);
        let verbose = verbose.unwrap_or(true);

        if !self.has_edges() || !ignore_infinity && !self.is_connected(Some(verbose)) {
            return Ok(if use_edge_weights_as_probabilities {
                0.0
            } else {
                f64::INFINITY
            });
        }

        let pb = get_loading_bar(
            verbose,
            "Computing weighted diameter",
            self.get_nodes_number() as usize,
        );
        Ok(self
            .par_iter_node_ids()
            .progress_with(pb)
            .map(|node_id| unsafe {
                self.get_unchecked_weighted_eccentricity_from_node_id(
                    node_id,
                    Some(use_edge_weights_as_probabilities),
                )
            })
            .filter(|&distance| {
                !ignore_infinity
                    || if use_edge_weights_as_probabilities {
                        !distance.is_zero()
                    } else {
                        distance.is_finite()
                    }
            })
            .reduce(|| f64::NEG_INFINITY, f64::max))
    }

    #[manual_binding]
    /// Returns vector of minimum paths distances and vector of nodes predecessors from given source node name and optional destination node name.
    ///
    /// # Arguments
    /// * `src_node_name`: &str - Node name root of the tree of minimum paths.
    /// * `dst_node_name`: Option<&str> - Destination node name.
    /// * `compute_predecessors`: Option<bool> - Whether to compute the vector of predecessors.
    /// * `maximal_depth`: Option<NodeT> - The maximal depth to execute the DFS for.

    ///
    /// # Raises
    /// * If the weights are to be used and the graph does not have weights.
    /// * If the given source node name does not exist in the current graph.
    /// * If the given optional destination node name does not exist in the current graph.
    pub fn get_breath_first_search_from_node_names(
        &self,
        src_node_name: &str,
        dst_node_name: Option<&str>,
        compute_predecessors: Option<bool>,
        maximal_depth: Option<NodeT>,
    ) -> Result<ShortestPathsResultBFS> {
        unsafe {
            Ok(self.get_unchecked_breath_first_search_from_node_ids(
                self.get_node_id_from_node_name(src_node_name)?,
                dst_node_name.map_or(Ok::<_, String>(None), |dst_node_name| {
                    Ok(Some(self.get_node_id_from_node_name(dst_node_name)?))
                })?,
                compute_predecessors,
                maximal_depth,
            ))
        }
    }

    #[manual_binding]
    /// Returns vector of minimum paths distances and vector of nodes predecessors from given source node name and optional destination node name.
    ///
    /// # Arguments
    /// * `src_node_name`: &str - Node name root of the tree of minimum paths.
    /// * `maybe_dst_node_name`: Option<&str> - Optional target destination node name. If provided, Dijkstra will stop upon reaching this node.
    /// * `maybe_dst_node_names`: Option<Vec<&str>> - Optional target destination node names. If provided, Dijkstra will stop upon reaching all of these nodes.
    /// * `compute_predecessors`: Option<bool> - Whether to compute the vector of predecessors.
    /// * `maximal_depth`: Option<NodeT> - The maximal depth to execute the DFS for.
    /// * `use_edge_weights_as_probabilities`: Option<bool> - Whether to treat the edge weights as probabilities.
    ///
    /// # Raises
    /// * If the weights are to be used and the graph does not have weights.
    /// * If the given source node name does not exist in the current graph.
    /// * If the given optional destination node name does not exist in the current graph.
    pub fn get_dijkstra_from_node_names(
        &self,
        src_node_name: &str,
        maybe_dst_node_name: Option<&str>,
        maybe_dst_node_names: Option<Vec<&str>>,
        compute_predecessors: Option<bool>,
        maximal_depth: Option<NodeT>,
        use_edge_weights_as_probabilities: Option<bool>,
    ) -> Result<ShortestPathsDjkstra> {
        self.get_dijkstra_from_node_ids(
            self.get_node_id_from_node_name(src_node_name)?,
            maybe_dst_node_name.map_or(Ok::<_, String>(None), |dst_node_name| {
                Ok(Some(self.get_node_id_from_node_name(dst_node_name)?))
            })?,
            maybe_dst_node_names.map_or(Ok::<_, String>(None), |dst_node_names| {
                Ok(Some(
                    dst_node_names
                        .into_iter()
                        .map(|node_name| self.get_node_id_from_node_name(node_name))
                        .collect::<Result<_>>()?,
                ))
            })?,
            compute_predecessors,
            maximal_depth,
            use_edge_weights_as_probabilities,
        )
    }
}
