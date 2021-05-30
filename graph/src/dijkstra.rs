use super::*;
use bitvec::prelude::*;
use indicatif::ParallelProgressIterator;
use num_traits::Zero;
use rayon::iter::IndexedParallelIterator;
use rayon::iter::IntoParallelRefIterator;
use rayon::iter::ParallelIterator;
use std::cmp::Ord;
use std::collections::VecDeque;

pub struct ShortestPathsResultBFS {
    pub(crate) distances: Option<Vec<NodeT>>,
    pub(crate) parents: Option<Vec<Option<NodeT>>>,
    pub(crate) visited: Option<BitVec<Lsb0, u8>>,
    pub(crate) dst_node_distance: Option<NodeT>,
    pub(crate) eccentricity: NodeT,
    pub(crate) total_distance: NodeT,
    pub(crate) total_harmonic_distance: f64,
}

impl ShortestPathsResultBFS {
    pub(crate) fn new(
        distances: Option<Vec<NodeT>>,
        parents: Option<Vec<Option<NodeT>>>,
        visited: Option<BitVec<Lsb0, u8>>,
        dst_node_distance: Option<NodeT>,
        eccentricity: NodeT,
        total_distance: NodeT,
        total_harmonic_distance: f64,
    ) -> ShortestPathsResultBFS {
        ShortestPathsResultBFS {
            distances,
            parents,
            visited,
            dst_node_distance,
            eccentricity,
            total_distance,
            total_harmonic_distance,
        }
    }
}

pub struct ShortestPathsDjkstra {
    pub(crate) distances: Vec<f64>,
    pub(crate) parents: Option<Vec<Option<NodeT>>>,
    pub(crate) dst_node_distance: Option<f64>,
    pub(crate) eccentricity: f64,
    pub(crate) total_distance: f64,
    pub(crate) total_harmonic_distance: f64,
}

impl ShortestPathsDjkstra {
    pub(crate) fn new(
        distances: Vec<f64>,
        parents: Option<Vec<Option<NodeT>>>,
        dst_node_distance: Option<f64>,
        eccentricity: f64,
        total_distance: f64,
        total_harmonic_distance: f64,
    ) -> ShortestPathsDjkstra {
        ShortestPathsDjkstra {
            distances,
            parents,
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
        maybe_dst_node_id: Option<NodeT>,
        mut maybe_dst_node_ids: Option<Vec<NodeT>>,
        compute_distances: Option<bool>,
        compute_predecessors: Option<bool>,
        compute_visited: Option<bool>,
        maximal_depth: Option<NodeT>,
    ) -> ShortestPathsResultBFS {
        let compute_distances = compute_distances.unwrap_or(true);
        let compute_predecessors = compute_predecessors.unwrap_or(true);
        let compute_visited = compute_visited.unwrap_or(false);
        let nodes_number = self.get_nodes_number() as usize;
        let mut dst_node_distance = maybe_dst_node_id.map(|_| NodeT::MAX);

        let mut parents: Option<Vec<Option<NodeT>>> = if compute_predecessors {
            let mut parents = vec![None; nodes_number];
            parents[src_node_id as usize] = Some(src_node_id);
            Some(parents)
        } else {
            None
        };

        let mut distances: Option<Vec<NodeT>> = if compute_distances {
            let mut distances: Vec<NodeT> = vec![NodeT::MAX; nodes_number];
            distances[src_node_id as usize] = 0;
            Some(distances)
        } else {
            None
        };

        let mut visited: Option<_> = if compute_visited || parents.is_none() && distances.is_none()
        {
            let mut visited = bitvec![Lsb0, u8; 0; nodes_number];
            *visited.get_unchecked_mut(src_node_id as usize) = true;
            Some(visited)
        } else {
            None
        };

        if self.is_unchecked_disconnected_from_node_id(src_node_id) {
            return ShortestPathsResultBFS::new(
                distances,
                parents,
                visited,
                dst_node_distance,
                NodeT::MAX,
                NodeT::MAX,
                0.0,
            );
        }

        let mut to_be_added = |neighbour_node_id, new_neighbour_distance, node_id| match (
            &mut distances,
            &mut parents,
            &mut visited,
        ) {
            (None, None, Some(visited)) if !visited[neighbour_node_id as usize] => {
                unsafe { *visited.get_unchecked_mut(neighbour_node_id as usize) = true };
                true
            }
            (Some(distances), None, None)
                if distances[neighbour_node_id as usize] == NodeT::MAX =>
            {
                distances[neighbour_node_id as usize] = new_neighbour_distance;
                true
            }
            (None, Some(parents), None) if parents[neighbour_node_id as usize].is_none() => {
                parents[neighbour_node_id as usize] = Some(node_id);
                true
            }
            (Some(distances), Some(parents), None)
                if distances[neighbour_node_id as usize] == NodeT::MAX =>
            {
                distances[neighbour_node_id as usize] = new_neighbour_distance;
                parents[neighbour_node_id as usize] = Some(node_id);
                true
            }
            (Some(distances), Some(parents), Some(visited))
                if distances[neighbour_node_id as usize] == NodeT::MAX =>
            {
                unsafe { *visited.get_unchecked_mut(neighbour_node_id as usize) = true };
                distances[neighbour_node_id as usize] = new_neighbour_distance;
                parents[neighbour_node_id as usize] = Some(node_id);
                true
            }
            _ => false,
        };

        let mut nodes_to_explore = VecDeque::with_capacity(nodes_number);
        nodes_to_explore.push_back((src_node_id, 0));
        let mut eccentricity = 0;
        let mut total_distance = 0;
        let mut total_harmonic_distance: f64 = 0.0;

        while let Some((node_id, depth)) = nodes_to_explore.pop_front() {
            // Update the metrics
            eccentricity = eccentricity.max(depth);
            total_distance += depth;
            if depth != 0 {
                total_harmonic_distance += 1.0 / depth as f64;
            }
            // If the closest node is the optional destination node, we have
            // completed what the user has required.
            if maybe_dst_node_id.map_or(false, |dst| dst == node_id) {
                dst_node_distance.insert(depth);
                break;
            }

            // If the closest node is in the set of the destination nodes
            if let Some(dst_node_ids) = &mut maybe_dst_node_ids {
                // We remove it
                let node_id_idx = dst_node_ids.iter().position(|x| *x == node_id);

                if let Some(nii) = node_id_idx {
                    dst_node_ids.remove(nii);
                }
                // And if now the roaringbitmap is empty
                if dst_node_ids.is_empty() {
                    // We have completed the requested task.
                    break;
                }
            }

            let new_neighbour_distance = depth + 1;

            if let Some(mi) = maximal_depth {
                if new_neighbour_distance > mi {
                    continue;
                }
            }

            self.iter_unchecked_neighbour_node_ids_from_source_node_id(node_id)
                .for_each(|neighbour_node_id| {
                    if to_be_added(neighbour_node_id, new_neighbour_distance, node_id) {
                        nodes_to_explore.push_back((neighbour_node_id, new_neighbour_distance));
                    }
                });
        }
        ShortestPathsResultBFS::new(
            distances,
            parents,
            visited,
            dst_node_distance,
            eccentricity,
            total_distance,
            total_harmonic_distance,
        )
    }

    /// Returns minimum path node IDs and distance from given node ids.
    ///
    /// # Arguments
    /// * `src_node_id`: NodeT - Source node ID.
    /// * `dst_node_id`: NodeT - Destination node ID.
    ///
    /// # Safety
    /// If any of the given node IDs does not exist in the graph the method will panic.
    pub unsafe fn get_unchecked_unweighted_minimum_path_node_ids_from_node_ids(
        &self,
        src_node_id: NodeT,
        dst_node_id: NodeT,
    ) -> Vec<NodeT> {
        let bfs = self.get_unchecked_breath_first_search_from_node_ids(
            src_node_id,
            Some(dst_node_id),
            None,
            Some(false),
            Some(true),
            Some(false),
            None,
        );
        let parents = bfs.parents.unwrap();
        let path_length = bfs.dst_node_distance.unwrap();
        // If the distance is infinite, the destination node is not connected.
        if path_length == NodeT::MAX {
            return Vec::new();
        }
        let mut path_length = path_length as usize;
        let mut path = vec![0; path_length];
        let mut parent = dst_node_id;
        loop {
            path_length -= 1;
            path[path_length] = parent;
            if parent == src_node_id {
                break;
            }
            if let Some(new_parent) = parents[parent as usize] {
                parent = new_parent;
            }
        }
        path
    }

    /// Returns minimum path node names from given node ids.
    ///
    /// # Arguments
    /// * `src_node_id`: NodeT - Source node ID.
    /// * `dst_node_id`: NodeT - Destination node ID.
    ///
    /// # Safety
    /// If any of the given node IDs does not exist in the graph the method will panic.
    pub unsafe fn get_unchecked_unweighted_minimum_path_node_names_from_node_ids(
        &self,
        src_node_id: NodeT,
        dst_node_id: NodeT,
    ) -> Vec<String> {
        self.get_unchecked_unweighted_minimum_path_node_ids_from_node_ids(src_node_id, dst_node_id)
            .into_iter()
            .map(|node_id| self.get_unchecked_node_name_from_node_id(node_id))
            .collect()
    }

    /// Returns minimum path node names from given node ids.
    ///
    /// # Arguments
    /// * `src_node_id`: NodeT - Source node ID.
    /// * `dst_node_id`: NodeT - Destination node ID.
    ///
    /// # Raises
    /// * If any of the given node IDs do not exist in the current graph.
    pub fn get_unweighted_minimum_path_node_ids_from_node_ids(
        &self,
        src_node_id: NodeT,
        dst_node_id: NodeT,
    ) -> Result<Vec<NodeT>, String> {
        Ok(unsafe {
            self.get_unchecked_unweighted_minimum_path_node_ids_from_node_ids(
                self.validate_node_id(src_node_id)?,
                self.validate_node_id(dst_node_id)?,
            )
        })
    }

    /// Returns minimum path node names from given node names.
    ///
    /// # Arguments
    /// * `src_node_name`: &str - Source node name.
    /// * `dst_node_name`: &str - Destination node name.
    ///
    /// # Raises
    /// * If any of the given node names do not exist in the current graph.
    pub fn get_unweighted_minimum_path_node_ids_from_node_names(
        &self,
        src_node_name: &str,
        dst_node_name: &str,
    ) -> Result<Vec<NodeT>, String> {
        Ok(unsafe {
            self.get_unchecked_unweighted_minimum_path_node_ids_from_node_ids(
                self.get_node_id_from_node_name(src_node_name)?,
                self.get_node_id_from_node_name(dst_node_name)?,
            )
        })
    }

    /// Returns minimum path node names from given node names.
    ///
    /// # Arguments
    /// * `src_node_name`: &str - Source node name.
    /// * `dst_node_name`: &str - Destination node name.
    ///
    /// # Raises
    /// * If any of the given node names do not exist in the current graph.
    pub fn get_unweighted_minimum_path_node_names_from_node_names(
        &self,
        src_node_name: &str,
        dst_node_name: &str,
    ) -> Result<Vec<String>, String> {
        Ok(unsafe {
            self.get_unchecked_unweighted_minimum_path_node_names_from_node_ids(
                self.get_node_id_from_node_name(src_node_name)?,
                self.get_node_id_from_node_name(dst_node_name)?,
            )
        })
    }

    #[no_numpy_binding]
    /// Returns vector of k minimum paths distances and vector of nodes predecessors.
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
    pub unsafe fn get_unchecked_unweighted_k_shortest_path_from_node_ids(
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

    /// Returns unweighted eccentricity of the given node.
    ///
    /// This method will panic if the given node ID does not exists in the graph.
    ///
    /// # Arguments
    /// * `node_id`: NodeT - Node for which to compute the eccentricity.
    ///
    /// # Safety
    /// If any of the given node IDs does not exist in the graph the method will panic.
    pub unsafe fn get_unchecked_unweighted_eccentricity_from_node_id(
        &self,
        node_id: NodeT,
    ) -> NodeT {
        self.get_unchecked_breath_first_search_from_node_ids(
            node_id,
            None,
            None,
            Some(false),
            Some(false),
            Some(false),
            None,
        )
        .eccentricity
    }

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
    pub fn get_unweighted_eccentricity_from_node_id(
        &self,
        node_id: NodeT,
    ) -> Result<NodeT, String> {
        self.validate_node_id(node_id).map(|node_id| unsafe {
            self.get_unchecked_unweighted_eccentricity_from_node_id(node_id)
        })
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
    ) -> Result<f64, String> {
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
    pub fn get_unweighted_eccentricity_from_node_name(
        &self,
        node_name: &str,
    ) -> Result<NodeT, String> {
        self.get_node_id_from_node_name(node_name)
            .map(|node_id| unsafe {
                self.get_unchecked_unweighted_eccentricity_from_node_id(node_id)
            })
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
    ) -> Result<f64, String> {
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
        let mut parents: Option<Vec<Option<NodeT>>> = if compute_predecessors {
            Some(vec![None; nodes_number])
        } else {
            None
        };

        if self.is_unchecked_disconnected_from_node_id(src_node_id) {
            if use_edge_weights_as_probabilities {
                let mut distances = vec![0.0; nodes_number];
                distances[src_node_id as usize] = 1.0;
                return ShortestPathsDjkstra::new(
                    distances,
                    parents,
                    dst_node_distance,
                    0.0,
                    0.0,
                    0.0,
                );
            } else {
                let mut distances = vec![f64::INFINITY; nodes_number];
                distances[src_node_id as usize] = 0.0;
                return ShortestPathsDjkstra::new(
                    distances,
                    parents,
                    dst_node_distance,
                    f64::INFINITY,
                    f64::INFINITY,
                    0.0,
                );
            }
        }

        let to_visit = if maximal_depth.is_some() {
            self.get_unchecked_breath_first_search_from_node_ids(
                src_node_id,
                maybe_dst_node_id,
                maybe_dst_node_ids.clone(),
                Some(false),
                Some(false),
                Some(true),
                maximal_depth,
            )
            .visited
        } else {
            None
        };

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
                    if let Some(tv) = to_visit.as_ref() {
                        if !tv[neighbour_node_id as usize] {
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
                        if let Some(parents) = &mut parents {
                            parents[neighbour_node_id as usize] = Some(closest_node_id as NodeT);
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
            parents,
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
    /// * `use_edge_weights_as_probabilities`: Option<bool> - Whether to treat the edge weights as probabilities.
    ///
    /// # Safety
    /// If any of the given node IDs does not exist in the graph the method will panic.
    pub unsafe fn get_unchecked_weighted_minimum_path_node_ids_from_node_ids(
        &self,
        src_node_id: NodeT,
        dst_node_id: NodeT,
        use_edge_weights_as_probabilities: Option<bool>,
    ) -> (f64, Vec<NodeT>) {
        let dijkstra = self.get_unchecked_dijkstra_from_node_ids(
            src_node_id,
            Some(dst_node_id),
            None,
            Some(true),
            None,
            use_edge_weights_as_probabilities,
        );
        let parents = dijkstra.parents.unwrap();
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
        // Since we need to visit the parents vector we will be building
        // the path backwards and we will need to invert it afterwards.
        let mut reverse_path = Vec::new();
        let mut parent = dst_node_id;
        loop {
            reverse_path.push(parent);
            if parent == src_node_id {
                break;
            }
            if let Some(new_parent) = parents[parent as usize] {
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
    /// * `use_edge_weights_as_probabilities`: Option<bool> - Whether to treat the edge weights as probabilities.
    ///
    /// # Safety
    /// If any of the given node IDs does not exist in the graph the method will panic.
    pub unsafe fn get_unchecked_weighted_minimum_path_node_names_from_node_ids(
        &self,
        src_node_id: NodeT,
        dst_node_id: NodeT,
        use_edge_weights_as_probabilities: Option<bool>,
    ) -> (f64, Vec<String>) {
        let (path_length, path) = self.get_unchecked_weighted_minimum_path_node_ids_from_node_ids(
            src_node_id,
            dst_node_id,
            use_edge_weights_as_probabilities,
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
    /// * `use_edge_weights_as_probabilities`: Option<bool> - Whether to treat the edge weights as probabilities.
    ///
    /// # Raises
    /// * If any of the given node IDs do not exist in the current graph.
    pub fn get_weighted_minimum_path_node_ids_from_node_ids(
        &self,
        src_node_id: NodeT,
        dst_node_id: NodeT,
        use_edge_weights_as_probabilities: Option<bool>,
    ) -> Result<(f64, Vec<NodeT>), String> {
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
            )
        })
    }

    /// Returns minimum path node names from given node names.
    ///
    /// # Arguments
    /// * `src_node_name`: &str - Source node name.
    /// * `dst_node_name`: &str - Destination node name.
    /// * `use_edge_weights_as_probabilities`: Option<bool> - Whether to treat the edge weights as probabilities.
    ///
    /// # Raises
    /// * If any of the given node names do not exist in the current graph.
    pub fn get_weighted_minimum_path_node_ids_from_node_names(
        &self,
        src_node_name: &str,
        dst_node_name: &str,
        use_edge_weights_as_probabilities: Option<bool>,
    ) -> Result<(f64, Vec<NodeT>), String> {
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
            )
        })
    }

    /// Returns minimum path node names from given node names.
    ///
    /// # Arguments
    /// * `src_node_name`: &str - Source node name.
    /// * `dst_node_name`: &str - Destination node name.
    /// * `use_edge_weights_as_probabilities`: Option<bool> - Whether to treat the edge weights as probabilities.
    ///
    /// # Raises
    /// * If any of the given node names do not exist in the current graph.
    pub fn get_weighted_minimum_path_node_names_from_node_names(
        &self,
        src_node_name: &str,
        dst_node_name: &str,
        use_edge_weights_as_probabilities: Option<bool>,
    ) -> Result<(f64, Vec<String>), String> {
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
            )
        })
    }

    #[manual_binding]
    /// Returns vector of minimum paths distances and vector of nodes predecessors from given source node ID and optional destination node ID.
    ///
    /// # Arguments
    /// * `src_node_id`: NodeT - Node ID root of the tree of minimum paths.
    /// * `maybe_dst_node_id`: Option<NodeT> - Optional target destination. If provided, Dijkstra will stop upon reaching this node.
    /// * `maybe_dst_node_ids`: Option<Vec<NodeT>> - Optional target destinations. If provided, Dijkstra will stop upon reaching all of these nodes.
    /// * `compute_distances`: Option<bool> - Whether to compute the vector of distances.
    /// * `compute_predecessors`: Option<bool> - Whether to compute the vector of predecessors.
    /// * `compute_visited`: Option<bool> - Whether to compute the vector of visited nodes.
    /// * `maximal_depth`: Option<NodeT> - The maximal number of iterations to execute the DFS for.
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
        compute_visited: Option<bool>,
        maximal_depth: Option<NodeT>,
    ) -> Result<ShortestPathsResultBFS, String> {
        // Check if the given root exists in the graph
        self.validate_node_id(src_node_id)?;
        // If given, check if the given destination node ID exists in the graph
        if let Some(dst) = &maybe_dst_node_id {
            self.validate_node_id(*dst)?;
        }
        // If given, check if the given destination node IDs exist in the graph
        let maybe_dst_node_ids = maybe_dst_node_ids.map_or(Ok::<_, String>(None), |node_ids| {
            Ok(Some(self.validate_node_ids(node_ids)?))
        })?;
        Ok(unsafe {
            self.get_unchecked_breath_first_search_from_node_ids(
                src_node_id,
                maybe_dst_node_id,
                maybe_dst_node_ids,
                compute_distances,
                compute_predecessors,
                compute_visited,
                maximal_depth,
            )
        })
    }

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
    ) -> Result<ShortestPathsDjkstra, String> {
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

    /// Returns diameter of an UNDIRECTED and UNWEIGHTED graph.
    ///
    /// # Referencences
    /// This method is based on the algorithm described in ["On computing the diameter of real-world undirected graphs" by Crescenzi et al](https://who.rocq.inria.fr/Laurent.Viennot/road/papers/ifub.pdf).
    fn get_unweighted_ifub(&self) -> f64 {
        if self.is_directed() {
            panic!("This method is not defined for directed graphs!")
        }
        let most_central_node_id = unsafe { self.get_unchecked_argmax_node_degree() };
        if self.is_singleton_with_selfloops_from_node_id(most_central_node_id) {
            return f64::INFINITY;
        }
        let bfs = unsafe {
            self.get_unchecked_breath_first_search_from_node_ids(
                most_central_node_id,
                None,
                None,
                Some(true),
                Some(false),
                Some(false),
                None,
            )
        };
        let mut root_eccentricity = bfs.eccentricity;
        let distances = bfs.distances.unwrap();
        assert!(
            root_eccentricity != NodeT::MAX,
            "The central node eccentricity cannot be infinite!"
        );
        assert!(
            root_eccentricity != 0,
            "The central node eccentricity cannot be zero!"
        );
        let mut lower_bound_diameter = root_eccentricity;
        let mut upper_bound_diameter = 2 * root_eccentricity;
        while lower_bound_diameter < upper_bound_diameter {
            if let Some(maximal_eccentricity) = distances
                .par_iter()
                .enumerate()
                .filter(|(_, &distance)| distance == root_eccentricity)
                .map(|(node_id, _)| unsafe {
                    self.get_unchecked_unweighted_eccentricity_from_node_id(node_id as NodeT)
                })
                .max()
            {
                assert!(
                    maximal_eccentricity != NodeT::MAX,
                    "The maximal eccentricity here cannot be infinite!"
                );
                assert!(
                    maximal_eccentricity != 0,
                    "The maximal eccentricity here cannot be zero!"
                );
                assert!(
                    root_eccentricity != 0,
                    "The root eccentricity cannot be zero!"
                );
                lower_bound_diameter = lower_bound_diameter.max(maximal_eccentricity);
            }
            root_eccentricity -= 1;
            upper_bound_diameter = 2 * root_eccentricity;
        }
        lower_bound_diameter as f64
    }

    /// Returns diameter of an UNDIRECTED and WEIGHTED graph.
    ///
    /// # Arguments
    /// * `use_edge_weights_as_probabilities`: Option<bool> - Whether to treat the edge weights as probabilities.
    ///
    /// # Safety
    /// This method will raise a panic if it is called on a directed graph.
    ///
    /// # Referencences
    /// This method is based on the algorithm described in ["On Computing the Diameter of Real-World Directed (Weighted) Graphs" by Crescenzi et al](https://link.springer.com/chapter/10.1007/978-3-642-30850-5_10).
    fn get_weighted_ifub(&self, use_edge_weights_as_probabilities: Option<bool>) -> f64 {
        if self.is_directed() {
            panic!("This method is not defined for directed graphs!")
        }
        let most_central_node_id = unsafe { self.get_unchecked_argmax_node_degree() };
        if self.is_singleton_with_selfloops_from_node_id(most_central_node_id) {
            return f64::INFINITY;
        }
        let dijkstra = unsafe {
            self.get_unchecked_dijkstra_from_node_ids(
                most_central_node_id,
                None,
                None,
                Some(false),
                None,
                use_edge_weights_as_probabilities,
            )
        };

        let mut root_eccentricity = dijkstra.eccentricity;
        let distances = dijkstra.distances;

        assert!(
            root_eccentricity != f64::INFINITY,
            "The central node eccentricity cannot be infinite!"
        );
        assert!(
            root_eccentricity != 0.0,
            "The central node eccentricity cannot be zero!"
        );
        let mut lower_bound_diameter = root_eccentricity;
        let mut upper_bound_diameter = 2.0 * root_eccentricity;
        while upper_bound_diameter < lower_bound_diameter {
            if let Some(maximal_eccentricity) = distances
                .par_iter()
                .enumerate()
                .filter(|(_, &distance)| (distance - root_eccentricity).abs() < f64::EPSILON)
                .map(|(node_id, _)| unsafe {
                    Some(self.get_unchecked_weighted_eccentricity_from_node_id(
                        node_id as NodeT,
                        use_edge_weights_as_probabilities,
                    ))
                })
                .reduce(
                    || None,
                    |old, new| {
                        if let (Some(old), Some(new)) = (old, new) {
                            Some(f64::max(old, new))
                        } else {
                            new
                        }
                    },
                )
            {
                assert!(
                    maximal_eccentricity != f64::INFINITY,
                    "The maximal eccentricity here cannot be infinite!"
                );
                assert!(
                    maximal_eccentricity != 0.0,
                    "The maximal eccentricity here cannot be zero!"
                );
                assert!(
                    root_eccentricity != 0.0,
                    "The root eccentricity cannot be zero!"
                );
                lower_bound_diameter = lower_bound_diameter.max(maximal_eccentricity);
            }
            root_eccentricity -= 1.0;
            upper_bound_diameter = 2.0 * root_eccentricity;
        }
        lower_bound_diameter
    }

    /// Returns diameter of the graph.
    ///
    /// # Arguments
    /// * `ignore_infinity`: Option<bool> - Whether to ignore infinite distances, which are present when in the graph exist multiple components.
    /// * `verbose`: Option<bool> - Whether to show a loading bar.
    ///
    /// # Raises
    /// * If the graph does not contain nodes.
    /// * If the graph does not have weights and weights have been requested.
    ///
    /// TODO! Add better implementation for directed graphs
    /// To make the better implementation for directed graphs we will first
    /// need to make the Elias-Fano encode the directed graph in a better way.
    pub fn get_unweighted_diameter(
        &self,
        ignore_infinity: Option<bool>,
        verbose: Option<bool>,
    ) -> Result<f64, String> {
        self.must_have_nodes()?;
        let ignore_infinity = ignore_infinity.unwrap_or(false);
        let verbose = verbose.unwrap_or(true);

        if !self.has_edges() || !ignore_infinity && !self.is_connected(Some(verbose)) {
            return Ok(f64::INFINITY);
        }

        if self.is_directed() {
            let pb = get_loading_bar(
                verbose,
                "Computing unweighted diameter",
                self.get_nodes_number() as usize,
            );
            // TODO: Add a better implementation for the directed case
            Ok(self
                .par_iter_node_ids()
                .progress_with(pb)
                .map(|node_id| unsafe {
                    self.get_unchecked_unweighted_eccentricity_from_node_id(node_id)
                })
                .filter(|&distance| !ignore_infinity || distance != NodeT::MAX)
                .max()
                .unwrap_or(0) as f64)
        } else {
            Ok(self.get_unweighted_ifub())
        }
    }

    /// Returns diameter of the graph.
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
    pub fn get_weighted_diameter(
        &self,
        ignore_infinity: Option<bool>,
        use_edge_weights_as_probabilities: Option<bool>,
        verbose: Option<bool>,
    ) -> Result<f64, String> {
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

        if self.is_directed() {
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
        } else {
            Ok(self.get_weighted_ifub(Some(use_edge_weights_as_probabilities)) as f64)
        }
    }

    #[manual_binding]
    /// Returns vector of minimum paths distances and vector of nodes predecessors from given source node name and optional destination node name.
    ///
    /// # Arguments
    /// * `src_node_name`: &str - Node name root of the tree of minimum paths.
    /// * `maybe_dst_node_name`: Option<&str> - Optional target destination node name. If provided, Dijkstra will stop upon reaching this node.
    /// * `maybe_dst_node_names`: Option<Vec<&str>> - Optional target destination node names. If provided, Dijkstra will stop upon reaching all of these nodes.
    /// * `compute_distances`: Option<bool> - Whether to compute the vector of distances.
    /// * `compute_predecessors`: Option<bool> - Whether to compute the vector of predecessors.
    /// * `compute_visited`: Option<bool> - Whether to compute the vector of visited nodes.
    /// * `maximal_depth`: Option<NodeT> - The maximal depth to execute the DFS for.

    ///
    /// # Raises
    /// * If the weights are to be used and the graph does not have weights.
    /// * If the given source node name does not exist in the current graph.
    /// * If the given optional destination node name does not exist in the current graph.
    pub fn get_breath_first_search_from_node_names(
        &self,
        src_node_name: &str,
        maybe_dst_node_name: Option<&str>,
        maybe_dst_node_names: Option<Vec<&str>>,
        compute_distances: Option<bool>,
        compute_predecessors: Option<bool>,
        compute_visited: Option<bool>,
        maximal_depth: Option<NodeT>,
    ) -> Result<ShortestPathsResultBFS, String> {
        Ok(unsafe {
            self.get_unchecked_breath_first_search_from_node_ids(
                self.get_node_id_from_node_name(src_node_name)?,
                maybe_dst_node_name.map_or(Ok::<_, String>(None), |dst_node_name| {
                    Ok(Some(self.get_node_id_from_node_name(dst_node_name)?))
                })?,
                maybe_dst_node_names.map_or(Ok::<_, String>(None), |dst_node_names| {
                    Ok(Some(
                        dst_node_names
                            .into_iter()
                            .map(|node_name| self.get_node_id_from_node_name(node_name))
                            .collect::<Result<_, _>>()?,
                    ))
                })?,
                compute_distances,
                compute_predecessors,
                compute_visited,
                maximal_depth,
            )
        })
    }

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
    ) -> Result<ShortestPathsDjkstra, String> {
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
                        .collect::<Result<_, _>>()?,
                ))
            })?,
            compute_predecessors,
            maximal_depth,
            use_edge_weights_as_probabilities,
        )
    }
}
