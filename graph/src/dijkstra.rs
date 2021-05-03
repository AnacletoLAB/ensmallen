use super::*;
use indicatif::ParallelProgressIterator;
use keyed_priority_queue::KeyedPriorityQueue;
use rayon::iter::{IndexedParallelIterator, IntoParallelIterator};
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use roaring::RoaringBitmap;
use std::cmp::Reverse;
use std::cmp::{Eq, Ord, Ordering, PartialEq, PartialOrd};
use std::collections::{HashSet, VecDeque};
// use std::collections::BinaryHeap;

#[derive(Debug, Copy, Clone)]
struct OrdFloat64(f64);

impl PartialOrd for OrdFloat64 {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(&other))
    }
}

impl Eq for OrdFloat64 {}

impl PartialEq for OrdFloat64 {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(&other) == Ordering::Equal
    }
}

impl Ord for OrdFloat64 {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0
            .partial_cmp(&other.0)
            .unwrap_or(if self.0.is_nan() && other.0.is_nan() {
                Ordering::Equal
            } else if self.0.is_nan() {
                Ordering::Less
            } else {
                Ordering::Greater
            })
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    distance: NodeT,
    node_id: NodeT,
}

// The priority queue depends on `Ord`.
// Explicitly implement the trait so the queue becomes a min-heap
// instead of a max-heap.
impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        // Notice that the we flip the ordering on distances.
        // In case of a tie we compare node_ids - this step is necessary
        // to make implementations of `PartialEq` and `Ord` consistent.
        other
            .distance
            .cmp(&self.distance)
            .then_with(|| self.node_id.cmp(&other.node_id))
    }
}

// `PartialOrd` needs to be implemented as well.
impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

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
    ) -> (Option<Vec<NodeT>>, Option<Vec<Option<NodeT>>>, NodeT) {
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

        let mut visited: Option<Vec<bool>> = if parents.is_some() || distances.is_some() {
            None
        } else {
            let mut visited = vec![false; nodes_number];
            visited[src_node_id as usize] = true;
            Some(visited)
        };

        // If the given root node is either a:
        // - singleton
        // - singleton with selfloops
        // - trap node
        // we have already completed the Dijkstra.
        if self.is_unchecked_singleton_from_node_id(src_node_id)
            || self.is_singleton_with_selfloops_from_node_id(src_node_id)
            || self.is_unchecked_trap_node_from_node_id(src_node_id)
        {
            return (distances, parents, NodeT::MAX);
        }

        let mut nodes_to_explore = VecDeque::with_capacity(nodes_number);
        nodes_to_explore.push_back((src_node_id, 0));
        // We are counting the number of elements in the queue because the
        // internal .len() method does a lot more stuff than it is needed.
        let mut elements_in_queue: NodeT = 1;
        let mut maximal_distance = 0;

        while let Some((node_id, depth)) = nodes_to_explore.pop_front() {
            // Reduce number of elements in queue
            elements_in_queue -= 1;
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
            let queue_length = elements_in_queue;
            for neighbour_node_id in
                self.iter_unchecked_neighbour_node_ids_from_source_node_id(node_id)
            {
                match (&mut distances, &mut parents, &mut visited) {
                    (None, None, Some(visited)) if !visited[neighbour_node_id as usize] => {
                        visited[neighbour_node_id as usize] = true;
                        nodes_to_explore.push_back((neighbour_node_id, new_neighbour_distance));
                        elements_in_queue += 1;
                    }
                    (Some(distances), None, None)
                        if distances[neighbour_node_id as usize] == NodeT::MAX =>
                    {
                        distances[neighbour_node_id as usize] = new_neighbour_distance;
                        nodes_to_explore.push_back((neighbour_node_id, new_neighbour_distance));
                        elements_in_queue += 1;
                    }
                    (None, Some(parents), None)
                        if parents[neighbour_node_id as usize].is_none() =>
                    {
                        parents[neighbour_node_id as usize] = Some(node_id);
                        nodes_to_explore.push_back((neighbour_node_id, new_neighbour_distance));
                        elements_in_queue += 1;
                    }
                    (Some(distances), Some(parents), None)
                        if distances[neighbour_node_id as usize] == NodeT::MAX =>
                    {
                        distances[neighbour_node_id as usize] = new_neighbour_distance;
                        parents[neighbour_node_id as usize] = Some(node_id);
                        nodes_to_explore.push_back((neighbour_node_id, new_neighbour_distance));
                        elements_in_queue += 1;
                    }
                    _ => {}
                };
            }
            if queue_length < elements_in_queue {
                maximal_distance = maximal_distance.max(new_neighbour_distance);
            }
        }
        (distances, parents, maximal_distance)
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
    ) -> (Vec<f64>, Option<Vec<NodeT>>, f64) {
        let compute_predecessors = compute_predecessors.unwrap_or(true);
        let nodes_number = self.get_nodes_number() as usize;
        let mut parents: Option<Vec<NodeT>> = if compute_predecessors {
            Some(vec![0; nodes_number])
        } else {
            None
        };
        let mut distances: Vec<f64> = vec![f64::INFINITY; nodes_number];
        distances[src_node_id as usize] = 0.0;

        // If the given root node is either a:
        // - singleton
        // - singleton with selfloops
        // - trap node
        // we have already completed the Dijkstra.
        if self.is_unchecked_singleton_from_node_id(src_node_id)
            || self.is_singleton_with_selfloops_from_node_id(src_node_id)
            || self.is_unchecked_trap_node_from_node_id(src_node_id)
        {
            return (distances, parents, f64::INFINITY);
        }

        let mut nodes_to_explore: KeyedPriorityQueue<NodeT, Reverse<OrdFloat64>> =
            KeyedPriorityQueue::new();
        nodes_to_explore.push(src_node_id, Reverse(OrdFloat64(0.0)));
        let mut maximal_distance: f64 = 0.0;

        while let Some((closest_node_id, closest_distance)) = nodes_to_explore.pop() {
            // If the closest node is the optional destination node, we have
            // completed what the user has required.
            if maybe_dst_node_id.map_or(false, |dst| dst == closest_node_id) {
                break;
            }
            // If the closest node is in the set of the destination nodes
            if let Some(dst_node_ids) = &mut maybe_dst_node_ids {
                // We remove it
                dst_node_ids.remove(closest_node_id);
                // And if now the roaringbitmap is empty
                if dst_node_ids.is_empty() {
                    // We have completed the requested task.
                    break;
                }
            }
            for (neighbour_node_id, weight) in self
                .iter_unchecked_neighbour_node_ids_from_source_node_id(closest_node_id)
                .zip(self.iter_unchecked_edge_weights_from_source_node_id(closest_node_id))
            {
                let new_neighbour_distance = distances[closest_node_id as usize] + weight as f64;
                if new_neighbour_distance < distances[neighbour_node_id as usize] {
                    distances[neighbour_node_id as usize] = new_neighbour_distance;
                    maximal_distance = maximal_distance.max(new_neighbour_distance);
                    if let Some(parents) = &mut parents {
                        parents[neighbour_node_id as usize] = closest_node_id;
                    }
                    nodes_to_explore.push(
                        neighbour_node_id,
                        Reverse(OrdFloat64(new_neighbour_distance)),
                    );
                }
            }
        }
        (distances, parents, maximal_distance)
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
    ) -> Result<(Option<Vec<NodeT>>, Option<Vec<Option<NodeT>>>, NodeT), String> {
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
    ) -> Result<(Vec<f64>, Option<Vec<NodeT>>, f64), String> {
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
    ) -> Result<NodeT, String> {
        self.must_have_nodes()?;
        let ignore_infinity = ignore_infinity.unwrap_or(true);
        let verbose = verbose.unwrap_or(true);
        let pb = get_loading_bar(
            verbose,
            "Computing unweighted diameter",
            self.get_nodes_number() as usize,
        );
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
            .reduce(|| 0, NodeT::max))
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
    pub fn get_experimental_unweighted_diameter(
        &self,
        ignore_infinity: Option<bool>,
        verbose: Option<bool>,
    ) -> Result<NodeT, String> {
        if self.is_directed(){
            return self.get_unweighted_diameter(ignore_infinity, verbose);
        }
        self.must_have_nodes()?;
        let ignore_infinity = ignore_infinity.unwrap_or(true);
        let verbose = verbose.unwrap_or(true);
        let pb = get_loading_bar(
            verbose,
            "Computing approximated unweighted diameter",
            self.get_nodes_number() as usize,
        );
        let components = self.get_node_connected_component_ids(verbose);
        let node_degrees = self.par_iter_node_degrees().collect::<Vec<NodeT>>();
        let components_with_tendrils = components
            .par_iter()
            .cloned()
            .zip(node_degrees.par_iter().cloned())
            .filter_map(|(component_id, degree)| {
                if degree == 1 {
                    Some(component_id)
                } else {
                    None
                }
            })
            .collect::<Vec<NodeT>>()
            .into_iter()
            .collect::<HashSet<NodeT>>();
        Ok(self
            .par_iter_node_ids()
            .zip(components.into_par_iter())
            .zip(node_degrees.par_iter().cloned())
            .progress_with(pb)
            .filter_map(|((node_id, component_id), degree)| {
                if components_with_tendrils.contains(&component_id) && degree != 1 {
                    None
                } else {
                    Some(
                        self.get_unchecked_breath_first_search(
                            node_id,
                            None,
                            None,
                            Some(false),
                            Some(false),
                        )
                        .2,
                    )
                }
            })
            .filter(|&distance| !ignore_infinity || distance != NodeT::MAX)
            .reduce(|| 0, NodeT::max))
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
        let ignore_infinity = ignore_infinity.unwrap_or(true);
        let verbose = verbose.unwrap_or(true);
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
    ) -> Result<(Option<Vec<NodeT>>, Option<Vec<Option<NodeT>>>, NodeT), String> {
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
    ) -> Result<(Vec<f64>, Option<Vec<NodeT>>, f64), String> {
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
