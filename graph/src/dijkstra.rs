use super::*;
use bitvec::prelude::*;
use indicatif::ParallelProgressIterator;
use rayon::iter::IndexedParallelIterator;
use rayon::iter::IntoParallelRefIterator;
use rayon::iter::ParallelIterator;
use roaring::RoaringBitmap;
use std::cmp::Ord;
use std::collections::VecDeque;

impl Graph {
    /// Returns vector of minimum paths distances and vector of nodes predecessors, if requested.
    ///
    /// # Arguments
    /// * `src_node_id`: NodeT - Root of the tree of minimum paths.
    /// * `maybe_dst_node_id`: Option<NodeT> - Optional target destination. If provided, Dijkstra will stop upon reaching this node.
    /// * `maybe_dst_node_ids`: Option<RoaringBitmap> - Optional target destinations. If provided, Dijkstra will stop upon reaching all of these nodes.
    /// * `compute_distances`: Option<bool> - Whether to compute the vector of distances.
    /// * `compute_predecessors`: Option<bool> - Whether to compute the vector of predecessors.
    pub fn get_unchecked_breath_first_search(
        &self,
        src_node_id: NodeT,
        maybe_dst_node_id: Option<NodeT>,
        mut maybe_dst_node_ids: Option<RoaringBitmap>,
        compute_distances: Option<bool>,
        compute_predecessors: Option<bool>,
    ) -> ShortestPathsResultBFS {
        let compute_distances = compute_distances.unwrap_or(true);
        let compute_predecessors = compute_predecessors.unwrap_or(true);
        let nodes_number = self.get_nodes_number() as usize;

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

        let mut visited: Option<_> = if parents.is_some() || distances.is_some() {
            None
        } else {
            let mut visited = bitvec![Lsb0, u8; 0; nodes_number];
            unsafe { *visited.get_unchecked_mut(src_node_id as usize) = true };
            Some(visited)
        };

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
            _ => false,
        };

        let mut nodes_to_explore = VecDeque::with_capacity(nodes_number);
        nodes_to_explore.push_back((src_node_id, 0));
        let mut maximal_distance = 0;
        let mut total_distance = 0;
        let mut total_harmonic_distance: f64 = 0.0;

        while let Some((node_id, depth)) = nodes_to_explore.pop_front() {
            // Update the metrics
            maximal_distance = maximal_distance.max(depth);
            total_distance += depth;
            total_harmonic_distance += 1.0 / depth as f64;
            // If the closest node is the optional destination node, we have
            // completed what the user has required.
            if maybe_dst_node_id.map_or(false, |dst| dst == node_id) {
                break;
            }

            // If the closest node is in the set of the destination nodes
            if let Some(dst_node_ids) = &mut maybe_dst_node_ids {
                // We remove it
                dst_node_ids.remove(node_id);
                // And if now the roaringbitmap is empty
                if dst_node_ids.is_empty() {
                    // We have completed the requested task.
                    break;
                }
            }

            let new_neighbour_distance = depth + 1;

            self.iter_unchecked_neighbour_node_ids_from_source_node_id(node_id)
                .for_each(|neighbour_node_id| {
                    if to_be_added(neighbour_node_id, new_neighbour_distance, node_id) {
                        nodes_to_explore.push_back((neighbour_node_id, new_neighbour_distance));
                    }
                });
        }
        (
            distances,
            parents,
            maximal_distance,
            total_distance,
            total_harmonic_distance,
        )
    }

    /// Returns vector of k minimum paths distances and vector of nodes predecessors.
    ///
    /// # Arguments
    /// * `src_node_id`: NodeT - Source node ID.
    /// * `dst_node_id`: NodeT - Destination node ID.
    /// * `k`: usize - Number of paths to find.
    pub fn get_unchecked_unweighted_k_shortest_path(
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
    pub fn get_unchecked_unweighted_eccentricity_from_node_id(&self, node_id: NodeT) -> NodeT {
        self.get_unchecked_breath_first_search(node_id, None, None, None, None)
            .2
    }

    /// Returns weighted eccentricity of the given node.
    ///
    /// This method will panic if the given node ID does not exists in the graph.
    ///
    /// # Arguments
    /// * `node_id`: NodeT - Node for which to compute the eccentricity.
    ///
    pub fn get_unchecked_weighted_eccentricity_from_node_id(&self, node_id: NodeT) -> f64 {
        self.get_unchecked_dijkstra_from_node_ids(node_id, None, None, None)
            .2
    }

    /// Returns unweighted eccentricity of the given node ID.
    ///
    /// # Arguments
    /// * `node_id`: NodeT - Node for which to compute the eccentricity.
    ///
    pub fn get_unweighted_eccentricity_from_node_id(
        &self,
        node_id: NodeT,
    ) -> Result<NodeT, String> {
        self.validate_node_id(node_id)
            .map(|node_id| self.get_unchecked_unweighted_eccentricity_from_node_id(node_id))
    }

    /// Returns weighted eccentricity of the given node ID.
    ///
    /// # Arguments
    /// * `node_id`: NodeT - Node for which to compute the eccentricity.
    ///
    pub fn get_weighted_eccentricity_from_node_id(&self, node_id: NodeT) -> Result<f64, String> {
        self.validate_node_id(node_id)
            .map(|node_id| self.get_unchecked_weighted_eccentricity_from_node_id(node_id))
    }

    /// Returns unweighted eccentricity of the given node name.
    ///
    /// # Arguments
    /// * `node_name`: &str - Node for which to compute the eccentricity.
    ///
    pub fn get_unweighted_eccentricity_from_node_name(
        &self,
        node_name: &str,
    ) -> Result<NodeT, String> {
        self.get_node_id_from_node_name(node_name)
            .map(|node_id| self.get_unchecked_unweighted_eccentricity_from_node_id(node_id))
    }

    /// Returns weighted eccentricity of the given node name.
    ///
    /// # Arguments
    /// * `node_name`: &str - Node for which to compute the eccentricity.
    ///
    pub fn get_weighted_eccentricity_from_node_name(&self, node_name: &str) -> Result<f64, String> {
        self.get_node_id_from_node_name(node_name)
            .map(|node_id| self.get_unchecked_weighted_eccentricity_from_node_id(node_id))
    }

    /// Returns vector of minimum paths distances and vector of nodes predecessors, if requested.
    ///
    /// # Arguments
    /// * `src_node_id`: NodeT - Root of the tree of minimum paths.
    /// * `maybe_dst_node_id`: Option<NodeT> - Optional target destination. If provided, Dijkstra will stop upon reaching this node.
    /// * `maybe_dst_node_ids`: Option<RoaringBitmap> - Optional target destinations. If provided, Dijkstra will stop upon reaching all of these nodes.
    /// * `compute_predecessors`: bool - Whether to compute the vector of predecessors.
    pub fn get_unchecked_dijkstra_from_node_ids(
        &self,
        src_node_id: NodeT,
        maybe_dst_node_id: Option<NodeT>,
        mut maybe_dst_node_ids: Option<RoaringBitmap>,
        compute_predecessors: Option<bool>,
    ) -> ShortestPathsDjkstra {
        let compute_predecessors = compute_predecessors.unwrap_or(true);
        let nodes_number = self.get_nodes_number() as usize;
        let mut parents: Option<Vec<Option<NodeT>>> = if compute_predecessors {
            Some(vec![None; nodes_number])
        } else {
            None
        };

        let mut nodes_to_explore: DijkstraQueue =
            DijkstraQueue::with_capacity_from_root(nodes_number, src_node_id as usize);
        let mut maximal_distance: f64 = 0.0;
        let mut total_distance: f64 = 0.0;
        let mut total_harmonic_distance: f64 = 0.0;

        while let Some(closest_node_id) = nodes_to_explore.pop() {
            // Update the distances metrics
            maximal_distance = maximal_distance.max(nodes_to_explore[closest_node_id]);
            total_distance += nodes_to_explore[closest_node_id];
            total_harmonic_distance += 1.0 / nodes_to_explore[closest_node_id];
            // If the closest node is the optional destination node, we have
            // completed what the user has required.
            if maybe_dst_node_id.map_or(false, |dst| dst == closest_node_id as NodeT) {
                break;
            }
            // If the closest node is in the set of the destination nodes
            if let Some(dst_node_ids) = &mut maybe_dst_node_ids {
                // We remove it
                dst_node_ids.remove(closest_node_id as NodeT);
                // And if now the roaringbitmap is empty
                if dst_node_ids.is_empty() {
                    // We have completed the requested task.
                    break;
                }
            }
            self.iter_unchecked_neighbour_node_ids_from_source_node_id(closest_node_id as NodeT)
                .zip(self.iter_unchecked_edge_weights_from_source_node_id(closest_node_id as NodeT))
                .for_each(|(neighbour_node_id, weight)| {
                    let new_neighbour_distance = nodes_to_explore[closest_node_id] + weight as f64;
                    if new_neighbour_distance < nodes_to_explore[neighbour_node_id as usize] {
                        if let Some(parents) = &mut parents {
                            parents[neighbour_node_id as usize] = Some(closest_node_id as NodeT);
                        }
                        nodes_to_explore.push(neighbour_node_id as usize, new_neighbour_distance);
                    }
                });
        }
        (
            nodes_to_explore.unwrap(),
            parents,
            maximal_distance,
            total_distance,
            total_harmonic_distance,
        )
    }

    /// Returns vector of minimum paths distances and vector of nodes predecessors from given source node ID and optional destination node ID.
    ///
    /// # Arguments
    /// * `src_node_id`: NodeT - Node ID root of the tree of minimum paths.
    /// * `maybe_dst_node_id`: Option<NodeT> - Optional target destination. If provided, Dijkstra will stop upon reaching this node.
    /// * `maybe_dst_node_ids`: Option<RoaringBitmap> - Optional target destinations. If provided, Dijkstra will stop upon reaching all of these nodes.
    /// * `compute_distances`: Option<bool> - Whether to compute the vector of distances.
    /// * `compute_predecessors`: Option<bool> - Whether to compute the vector of predecessors.
    ///
    /// # Raises
    /// * If the given source node ID does not exist in the current graph.
    /// * If the given optional destination node ID does not exist in the current graph.
    pub fn get_breath_first_search_from_node_ids(
        &self,
        src_node_id: NodeT,
        maybe_dst_node_id: Option<NodeT>,
        maybe_dst_node_ids: Option<RoaringBitmap>,
        compute_distances: Option<bool>,
        compute_predecessors: Option<bool>,
    ) -> Result<ShortestPathsResultBFS, String> {
        // Check if the given root exists in the graph
        self.validate_node_id(src_node_id)?;
        // If given, check if the given destination node ID exists in the graph
        if let Some(dst) = &maybe_dst_node_id {
            self.validate_node_id(*dst)?;
        }
        // If given, check if the given destination node IDs exist in the graph
        if let Some(dst_node_ids) = &maybe_dst_node_ids {
            for dst_node_id in dst_node_ids.iter() {
                self.validate_node_id(dst_node_id)?;
            }
        }
        Ok(self.get_unchecked_breath_first_search(
            src_node_id,
            maybe_dst_node_id,
            maybe_dst_node_ids,
            compute_distances,
            compute_predecessors,
        ))
    }

    /// Returns vector of minimum paths distances and vector of nodes predecessors from given source node ID and optional destination node ID.
    ///
    /// # Arguments
    /// * `src_node_id`: NodeT - Node ID root of the tree of minimum paths.
    /// * `maybe_dst_node_id`: Option<NodeT> - Optional target destination. If provided, Dijkstra will stop upon reaching this node.
    /// * `maybe_dst_node_ids`: Option<RoaringBitmap> - Optional target destinations. If provided, Dijkstra will stop upon reaching all of these nodes.
    /// * `compute_predecessors`: Option<bool> - Whether to compute the vector of predecessors.
    ///
    /// # Raises
    /// * If the weights are to be used and the graph does not have weights.
    /// * If the given source node ID does not exist in the current graph.
    /// * If the given optional destination node ID does not exist in the current graph.
    pub fn get_dijkstra_from_node_ids(
        &self,
        src_node_id: NodeT,
        maybe_dst_node_id: Option<NodeT>,
        maybe_dst_node_ids: Option<RoaringBitmap>,
        compute_predecessors: Option<bool>,
    ) -> Result<ShortestPathsDjkstra, String> {
        // Check if the given root exists in the graph
        self.validate_node_id(src_node_id)?;
        self.must_have_edge_weights()?;
        // If given, check if the given destination node ID exists in the graph
        if let Some(dst) = &maybe_dst_node_id {
            self.validate_node_id(*dst)?;
        }
        // If given, check if the given destination node IDs exist in the graph
        if let Some(dst_node_ids) = &maybe_dst_node_ids {
            for dst_node_id in dst_node_ids.iter() {
                self.validate_node_id(dst_node_id)?;
            }
        }
        Ok(self.get_unchecked_dijkstra_from_node_ids(
            src_node_id,
            maybe_dst_node_id,
            maybe_dst_node_ids,
            compute_predecessors,
        ))
    }

    /// Returns diameter of an UNDIRECTED and UNWEIGHTED graph.
    ///
    /// # Referencences
    /// This method is based on the algorithm described in ["On computing the diameter of real-world undirected graphs" by Crescenzi et al](https://who.rocq.inria.fr/Laurent.Viennot/road/papers/ifub.pdf).
    fn get_unweighted_ifub(&self) -> NodeT {
        let most_central_node_id = unsafe { self.get_unchecked_argmax_node_degree() };
        let (distances, _, mut root_eccentricity, _, _) = self.get_unchecked_breath_first_search(
            most_central_node_id,
            None,
            None,
            Some(true),
            Some(false),
        );
        let mut lower_bound_diameter = root_eccentricity;
        let distances = unsafe { distances.unwrap_unchecked() };
        let mut upper_bound_diameter = 2 * root_eccentricity;
        while lower_bound_diameter != upper_bound_diameter {
            if let Some(maximal_eccentricity) = distances
                .par_iter()
                .enumerate()
                .filter(|(_, &distance)| distance == root_eccentricity)
                .map(|(node_id, _)| {
                    self.get_unchecked_unweighted_eccentricity_from_node_id(node_id as NodeT)
                })
                .max()
            {
                lower_bound_diameter = lower_bound_diameter.max(maximal_eccentricity);
                root_eccentricity -= 1;
                upper_bound_diameter = 2 * root_eccentricity;
            }
        }
        lower_bound_diameter
    }

    /// Returns diameter of an UNDIRECTED and WEIGHTED graph.
    ///
    /// # Referencences
    /// This method is based on the algorithm described in ["On Computing the Diameter of Real-World Directed (Weighted) Graphs" by Crescenzi et al](https://link.springer.com/chapter/10.1007/978-3-642-30850-5_10).
    fn get_weighted_ifub(&self) -> f64 {
        let most_central_node_id = unsafe { self.get_unchecked_argmax_node_degree() };
        let (distances, _, mut root_eccentricity, _, _) = self
            .get_unchecked_dijkstra_from_node_ids(most_central_node_id, None, None, Some(false));
        let mut lower_bound_diameter = root_eccentricity;
        let mut upper_bound_diameter = 2.0 * root_eccentricity;
        while upper_bound_diameter - lower_bound_diameter > f64::EPSILON {
            if let Some(maximal_eccentricity) = distances
                .par_iter()
                .enumerate()
                .filter(|(_, &distance)| (distance - root_eccentricity).abs() < f64::EPSILON)
                .map(|(node_id, _)| {
                    Some(self.get_unchecked_weighted_eccentricity_from_node_id(node_id as NodeT))
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
                lower_bound_diameter = lower_bound_diameter.max(maximal_eccentricity);
                root_eccentricity -= 1.0;
                upper_bound_diameter = 2.0 * root_eccentricity;
            }
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
    pub fn get_unweighted_diameter(
        &self,
        ignore_infinity: Option<bool>,
        verbose: Option<bool>,
    ) -> Result<f64, String> {
        self.must_have_nodes()?;
        let ignore_infinity = ignore_infinity.unwrap_or(false);
        let verbose = verbose.unwrap_or(true);

        if !ignore_infinity && !self.is_connected(verbose) {
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
                .map(|node_id| {
                    self.get_unchecked_breath_first_search(
                        node_id,
                        None,
                        None,
                        Some(false),
                        Some(false),
                    )
                    .2
                })
                .filter(|&distance| !ignore_infinity || distance != NodeT::MAX)
                .max()
                .unwrap_or(0) as f64)
        } else {
            Ok(self.get_unweighted_ifub() as f64)
        }
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
    pub fn get_weighted_diameter(
        &self,
        ignore_infinity: Option<bool>,
        verbose: Option<bool>,
    ) -> Result<f64, String> {
        self.must_have_nodes()?;
        self.must_have_edge_weights()?;
        let ignore_infinity = ignore_infinity.unwrap_or(true);
        let verbose = verbose.unwrap_or(true);

        if !ignore_infinity && !self.is_connected(verbose) {
            return Ok(f64::INFINITY);
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
                .map(|node_id| {
                    self.get_unchecked_dijkstra_from_node_ids(node_id, None, None, Some(false))
                        .2
                })
                .filter(|&distance| !ignore_infinity || distance != f64::INFINITY)
                .reduce(|| f64::NEG_INFINITY, f64::max))
        } else {
            Ok(self.get_weighted_ifub() as f64)
        }
    }

    /// Returns vector of minimum paths distances and vector of nodes predecessors from given source node name and optional destination node name.
    ///
    /// # Arguments
    /// * `src_node_name`: &str - Node name root of the tree of minimum paths.
    /// * `maybe_dst_node_name`: Option<&str> - Optional target destination node name. If provided, Dijkstra will stop upon reaching this node.
    /// * `maybe_dst_node_names`: Option<Vec<&str>> - Optional target destination node names. If provided, Dijkstra will stop upon reaching all of these nodes.
    /// * `compute_distances`: Option<bool> - Whether to compute the vector of distances.
    /// * `compute_predecessors`: Option<bool> - Whether to compute the vector of predecessors.
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
    ) -> Result<ShortestPathsResultBFS, String> {
        Ok(self.get_unchecked_breath_first_search(
            self.get_node_id_from_node_name(src_node_name)?,
            maybe_dst_node_name.map_or(Ok::<_, String>(None), |dst_node_name| {
                Ok(Some(self.get_node_id_from_node_name(dst_node_name)?))
            })?,
            maybe_dst_node_names.map_or(Ok::<_, String>(None), |dst_node_names| {
                let mut bitmap = RoaringBitmap::new();
                for node_name in dst_node_names.iter() {
                    bitmap.push(self.get_node_id_from_node_name(node_name)?);
                }
                Ok(Some(bitmap))
            })?,
            compute_distances,
            compute_predecessors,
        ))
    }

    /// Returns vector of minimum paths distances and vector of nodes predecessors from given source node name and optional destination node name.
    ///
    /// # Arguments
    /// * `src_node_name`: &str - Node name root of the tree of minimum paths.
    /// * `maybe_dst_node_name`: Option<&str> - Optional target destination node name. If provided, Dijkstra will stop upon reaching this node.
    /// * `maybe_dst_node_names`: Option<Vec<&str>> - Optional target destination node names. If provided, Dijkstra will stop upon reaching all of these nodes.
    /// * `compute_predecessors`: Option<bool> - Whether to compute the vector of predecessors.
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
    ) -> Result<ShortestPathsDjkstra, String> {
        self.get_dijkstra_from_node_ids(
            self.get_node_id_from_node_name(src_node_name)?,
            maybe_dst_node_name.map_or(Ok::<_, String>(None), |dst_node_name| {
                Ok(Some(self.get_node_id_from_node_name(dst_node_name)?))
            })?,
            maybe_dst_node_names.map_or(Ok::<_, String>(None), |dst_node_names| {
                let mut bitmap = RoaringBitmap::new();
                for node_name in dst_node_names.iter() {
                    bitmap.push(self.get_node_id_from_node_name(node_name)?);
                }
                Ok(Some(bitmap))
            })?,
            compute_predecessors,
        )
    }
}
