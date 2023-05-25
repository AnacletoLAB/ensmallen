use super::*;
use indicatif::ParallelProgressIterator;
use num_traits::{PrimInt, Zero};
use parallel_frontier::prelude::*;
use std::cmp::Ord;
use std::collections::VecDeque;
use std::convert::TryFrom;
use std::hash::{Hash, Hasher};
use std::string::ToString;

#[derive(Hash, Clone, Debug)]
pub struct ShortestPathsResultBFS {
    distances: Option<Vec<NodeT>>,
    predecessors: Option<Vec<NodeT>>,
    eccentricity: NodeT,
    most_distant_node: NodeT,
}

impl ToString for ShortestPathsResultBFS {
    fn to_string(&self) -> String {
        format!("{:#4?}", self)
    }
}

impl ShortestPathsResultBFS {
    pub(crate) fn new(
        distances: Option<Vec<NodeT>>,
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

    pub fn has_path_to_node_id(&self, node_id: NodeT) -> Result<bool> {
        Ok(self.get_distance_from_node_id(node_id)? != NODE_NOT_PRESENT)
    }

    fn validate_node_id(&self, node_id: NodeT) -> Result<NodeT> {
        let number_of_nodes = match (self.distances.as_ref(), self.predecessors.as_ref()) {
            (Some(distances), _) => distances.len(),
            (None, Some(predecessors)) => predecessors.len(),
            (None, None) => unreachable!(
                "Either distances or predecessors must be set for this method to be called."
            ),
        };
        if node_id as usize >= number_of_nodes {
            return Err(format!(
                "The request node ID `{}` is higher than the available numbers of nodes `{}`.",
                node_id, number_of_nodes
            ));
        }

        Ok(node_id)
    }

    pub fn get_distance_from_node_id(&self, node_id: NodeT) -> Result<NodeT> {
        self.validate_node_id(node_id)?;
        match &self.distances {
            Some(distances) => Ok(distances[node_id as usize]),
            None => Err(concat!(
                "Distance from node was requested but the distances ",
                "where not computed for this BFS run."
            )
            .to_string()),
        }
    }

    pub fn get_parent_from_node_id(&self, node_id: NodeT) -> Result<NodeT> {
        self.validate_node_id(node_id)?;
        match &self.predecessors {
            Some(predecessors) => Ok(predecessors[node_id as usize]),
            None => Err(concat!(
                "Parent node was requested but the predecessors ",
                "where not computed for this BFS run."
            )
            .to_string()),
        }
    }

    /// Returns node at the `len - k` position on minimum path to given destination node.
    ///
    /// # Arguments
    /// * `dst_node_id`: NodeT - The node to start computing predecessors from.
    /// * `k`: NodeT - Steps to go back.
    ///
    /// # Raises
    /// * If the predecessors vector was not requested.
    pub unsafe fn get_unchecked_kth_point_on_shortest_path(
        &self,
        mut dst_node_id: NodeT,
        k: NodeT,
    ) -> Result<NodeT> {
        if let Some(predecessors) = self.predecessors.as_ref() {
            for _ in 0..k {
                dst_node_id = predecessors[dst_node_id as usize];
            }
            return Ok(dst_node_id);
        }
        Err("Predecessors were not requested and therefore not computed.".to_string())
    }

    /// Returns node at the `len - k` position on minimum path to given destination node.
    ///
    /// # Arguments
    /// * `dst_node_id`: NodeT - The node to start computing predecessors from.
    /// * `k`: NodeT - Steps to go back.
    ///
    /// # Raises
    /// * If the predecessors vector was not requested.
    pub fn get_kth_point_on_shortest_path(&self, dst_node_id: NodeT, k: NodeT) -> Result<NodeT> {
        if !self.has_path_to_node_id(dst_node_id)? {
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
        unsafe { self.get_unchecked_kth_point_on_shortest_path(dst_node_id, k) }
    }

    pub fn get_median_point(&self, dst_node_id: NodeT) -> Result<NodeT> {
        if !self.has_path_to_node_id(dst_node_id)? {
            return Err("There is no path to the given destination node.".to_string());
        }
        let median_distance = self.get_distance_from_node_id(dst_node_id)? / 2;
        self.get_kth_point_on_shortest_path(dst_node_id, median_distance)
    }

    pub fn get_median_point_to_most_distant_node(&self) -> Result<NodeT> {
        let median_distance = self.eccentricity / 2;
        unsafe {
            self.get_unchecked_kth_point_on_shortest_path(self.most_distant_node, median_distance)
        }
    }

    pub fn get_eccentricity(&self) -> NodeT {
        self.eccentricity
    }

    pub fn get_most_distant_node(&self) -> NodeT {
        self.most_distant_node
    }

    #[no_binding]
    pub fn into_iter_finite_distances(self) -> impl Iterator<Item = NodeT> {
        self.distances
            .unwrap()
            .into_iter()
            .filter(|&distance| distance != NODE_NOT_PRESENT)
    }

    #[no_binding]
    pub fn into_par_iter_node_ids_and_finite_distances(
        self,
    ) -> impl ParallelIterator<Item = (NodeT, NodeT)> {
        self.distances
            .unwrap()
            .into_par_iter()
            .enumerate()
            .filter_map(|(node_id, distance)| {
                if distance != NODE_NOT_PRESENT {
                    Some((node_id as NodeT, distance))
                } else {
                    None
                }
            })
    }

    /// Returns the number of shortest paths starting from the root node.
    ///
    /// # Raises
    /// * If neither predecessors nor distances were computed for this BFS.
    ///
    /// # Returns
    /// Number of shortest paths starting from the root node.
    pub fn get_number_of_shortest_paths(&self) -> Result<NodeT> {
        if let Some(predecessors) = self.predecessors.as_ref() {
            return Ok(predecessors
                .par_iter()
                .filter(|&&predecessor| predecessor != NODE_NOT_PRESENT)
                .count() as NodeT);
        }
        if let Some(distances) = self.distances.as_ref() {
            return Ok(distances
                .par_iter()
                .filter(|&&distance| distance != NODE_NOT_PRESENT)
                .count() as NodeT);
        }
        Err(concat!(
            "Neither predecessors nor distances were computed (as it was requested) ",
            "when creating this breath shortest paths object.\n",
            "It is not possible to compute the number of shortest paths from the current ",
            "root node when neither predecessors nor distances were computed."
        )
        .to_string())
    }

    /// Returns the number of shortest paths passing through the given node.
    ///
    /// # Arguments
    /// * `node_id`: NodeT - The node id.
    ///
    /// # Raises
    /// * If neither predecessors nor distances were computed for this BFS.
    /// * If the given node ID does not exist in the current graph instance.
    ///
    /// # Returns
    /// The number of nodes passing by the node ID.
    pub fn get_number_of_shortest_paths_from_node_id(&self, node_id: NodeT) -> Result<NodeT> {
        self.validate_node_id(node_id)?;
        if let Some(predecessors) = self.predecessors.as_ref() {
            if predecessors[node_id as usize] == NODE_NOT_PRESENT {
                return Ok(0);
            }
            // There is also the path to the given node ID
            // that has that node as a destinaton.
            return Ok(1 + predecessors
                .par_iter()
                .filter(|&&predecessor| predecessor == node_id)
                .count() as NodeT);
        }
        Err(concat!(
            "The predecessors were computed (as it was requested) ",
            "when creating this breath shortest paths object.\n",
            "It is not possible to compute the number of shortest paths from the current ",
            "root node passing to the given node ID when predecessors were not computed."
        )
        .to_string())
    }

    /// Return list of successors of a given node.
    ///
    /// # Arguments
    /// * `source_node_id`: NodeT - The node for which to return the successors.
    ///
    /// # Raises
    /// * If the given node ID does not exist in the graph.
    ///
    /// # Returns
    /// List of successors of the given node.
    pub fn get_successors_from_node_id(&self, source_node_id: NodeT) -> Result<Vec<NodeT>> {
        self.validate_node_id(source_node_id)?;
        if let Some(predecessors) = self.predecessors.as_ref() {
            // If the node is not reacheable in the
            // considered shortest paths, we can stop.
            if predecessors[source_node_id as usize] == NODE_NOT_PRESENT {
                return Ok(Vec::new());
            }
            // Get the number of nodes in the graph.
            let nodes_number = predecessors.len() as NodeT;
            // We iterate over the nodes in the graph.
            return Ok((0..nodes_number)
                // Convert to parallel iterator
                .into_par_iter()
                // Remove the nodes that do not have the
                // provided source node as predecessor
                .filter(move |&node_id| {
                    // Otherwise we start to climb over the
                    // predecessors tree, starting from the current node.
                    let mut node_id = node_id;
                    while predecessors[node_id as usize] != node_id {
                        // We retrieve the node predecessor
                        // and climb up the predecessors ladder.
                        node_id = predecessors[node_id as usize];
                        // If the node is not reacheable in the
                        // considered shortest paths, we can stop.
                        if node_id == NODE_NOT_PRESENT {
                            return false;
                        }
                        // If the node is equal to the source node ID
                        // we have finished and found that this node
                        // is indeed a successor of the source nodes
                        // and we need to keep it.
                        if source_node_id == node_id {
                            return true;
                        }
                    }
                    false
                })
                .collect::<Vec<NodeT>>());
        }
        Err(concat!(
            "The predecessors were not computed (as it was requested) ",
            "when creating this breath shortest paths object.\n",
            "It is not possible to compute the successors from the current ",
            "root node passing to the given node ID when predecessors were not computed."
        )
        .to_string())
    }

    /// Return list of predecessors of a given node.
    ///
    /// # Arguments
    /// * `source_node_id`: NodeT - The node for which to return the predecessors.
    ///
    /// # Raises
    /// * If the given node ID does not exist in the graph.
    ///
    /// # Returns
    /// List of predecessors of the given node.
    pub fn get_predecessors_from_node_id(&self, source_node_id: NodeT) -> Result<Vec<NodeT>> {
        self.validate_node_id(source_node_id)?;
        if let Some(predecessors) = self.predecessors.as_ref() {
            // If the node is not reacheable in the
            // considered shortest paths, we can stop.
            if predecessors[source_node_id as usize] == NODE_NOT_PRESENT {
                return Ok(vec![source_node_id]);
            }

            let mut node_predecessors = vec![source_node_id];

            let mut node_id = source_node_id;
            while predecessors[node_id as usize] != node_id {
                // We retrieve the node predecessor
                // and climb up the predecessors ladder.
                node_id = predecessors[node_id as usize];
                node_predecessors.push(node_id);
            }

            return Ok(node_predecessors);
        }
        Err(concat!(
            "The predecessors were not computed (as it was requested) ",
            "when creating this breath shortest paths object.\n",
            "It is not possible to compute the predecessors from the current ",
            "root node passing to the given node ID when predecessors were not computed."
        )
        .to_string())
    }

    /// Return Shared Ancestors number.
    ///
    /// # Arguments
    /// * `first_node_id`: NodeT - The first node for which to compute the predecessors Jaccard index.
    /// * `second_node_id`: NodeT - The second node for which to compute the predecessors Jaccard index.
    ///
    /// # Raises
    /// * If the given node IDs do not exist in the graph.
    ///
    /// # Returns
    /// Ancestors Jaccard Index.
    pub fn get_shared_ancestors_size(
        &self,
        first_node_id: NodeT,
        second_node_id: NodeT,
    ) -> Result<f32> {
        Ok(self
            .get_predecessors_from_node_id(first_node_id)?
            .iter()
            .rev()
            .zip(
                self.get_predecessors_from_node_id(second_node_id)?
                    .iter()
                    .rev(),
            )
            .take_while(|(a, b)| a == b)
            .count() as f32)
    }

    /// Return Ancestors Jaccard Index.
    ///
    /// # Arguments
    /// * `first_node_id`: NodeT - The first node for which to compute the predecessors Jaccard index.
    /// * `second_node_id`: NodeT - The second node for which to compute the predecessors Jaccard index.
    ///
    /// # Raises
    /// * If the given node IDs do not exist in the graph.
    ///
    /// # Returns
    /// Ancestors Jaccard Index.
    pub fn get_ancestors_jaccard_index(
        &self,
        first_node_id: NodeT,
        second_node_id: NodeT,
    ) -> Result<f32> {
        let first_node_predecessors = self.get_predecessors_from_node_id(first_node_id)?;
        let second_node_predecessors = self.get_predecessors_from_node_id(second_node_id)?;

        let intersection_size = first_node_predecessors
            .iter()
            .rev()
            .zip(second_node_predecessors.iter().rev())
            .take_while(|(a, b)| a == b)
            .count();

        let union_size =
            first_node_predecessors.len() + second_node_predecessors.len() - intersection_size;

        Ok(if union_size.is_zero() {
            0.0
        } else {
            intersection_size as f32 / union_size as f32
        })
    }

    pub fn get_distances(&self) -> Result<Vec<NodeT>> {
        match &self.distances {
            Some(distances) => Ok(distances.clone()),
            None => Err(concat!(
                "Distance of node was requested but the distances ",
                "where not computed for this BFS run."
            )
            .to_string()),
        }
    }

    pub fn get_predecessors(&self) -> Result<Vec<NodeT>> {
        match &self.predecessors {
            Some(predecessors) => Ok(predecessors.clone()),
            None => Err(concat!(
                "Distance of node was requested but the predecessors ",
                "where not computed for this BFS run."
            )
            .to_string()),
        }
    }

    #[no_binding]
    pub fn into_distances(self) -> Vec<NodeT> {
        self.distances.unwrap()
    }
}

#[derive(Clone, Debug)]
pub struct ShortestPathsDjkstra {
    pub(crate) distances: Vec<f32>,
    most_distant_node: NodeT,
    pub(crate) predecessors: Option<Vec<Option<NodeT>>>,
    pub(crate) dst_node_distance: Option<f32>,
    pub(crate) eccentricity: f32,
    total_distance: f32,
    log_total_distance: f32,
    pub(crate) total_harmonic_distance: f32,
}

impl ToString for ShortestPathsDjkstra {
    fn to_string(&self) -> String {
        format!("{:#4?}", self)
    }
}

impl Hash for ShortestPathsDjkstra {
    fn hash<H: Hasher>(&self, state: &mut H) {
        for d in &self.distances {
            crate::hash::hash_f32(*d, state);
        }
        self.predecessors.hash(state);

        if let Some(d) = self.dst_node_distance {
            1.hash(state);
            crate::hash::hash_f32(d, state);
        } else {
            0.hash(state);
        }

        crate::hash::hash_f32(self.eccentricity, state);
        crate::hash::hash_f32(self.total_distance, state);
        crate::hash::hash_f32(self.total_harmonic_distance, state);
    }
}

impl ShortestPathsDjkstra {
    pub(crate) fn new(
        distances: Vec<f32>,
        most_distant_node: NodeT,
        predecessors: Option<Vec<Option<NodeT>>>,
        dst_node_distance: Option<f32>,
        eccentricity: f32,
        total_distance: f32,
        log_total_distance: f32,
        total_harmonic_distance: f32,
    ) -> ShortestPathsDjkstra {
        ShortestPathsDjkstra {
            distances,
            most_distant_node,
            predecessors,
            dst_node_distance,
            eccentricity,
            total_distance,
            log_total_distance,
            total_harmonic_distance,
        }
    }

    pub fn has_path_to_node_id(&self, node_id: NodeT) -> Result<bool> {
        Ok(self.get_distance_from_node_id(node_id)?.is_infinite())
    }

    fn validate_node_id(&self, node_id: NodeT) -> Result<NodeT> {
        if node_id as usize >= self.distances.len() {
            return Err(format!(
                "The request node ID `{}` is higher than the available numbers of nodes `{}`.",
                node_id,
                self.distances.len()
            ));
        }
        Ok(node_id)
    }

    pub fn get_distance_from_node_id(&self, node_id: NodeT) -> Result<f32> {
        self.validate_node_id(node_id)
            .map(|node_id| self.distances[node_id as usize])
    }

    pub fn get_parent_from_node_id(&self, node_id: NodeT) -> Result<Option<NodeT>> {
        self.validate_node_id(node_id)?;
        match &self.predecessors {
            Some(predecessors) => Ok(predecessors[node_id as usize]),
            None => Err(concat!(
                "Parent node was requested but the predecessors ",
                "where not computed for this Dijkstra run."
            )
            .to_string()),
        }
    }

    /// Returns node at just before given distance on minimum path to given destination node.
    ///
    /// # Arguments
    /// * `dst_node_id`: NodeT - The node to start computing predecessors from.
    /// * `distance`: f32 - The distance to aim for.
    ///
    /// # Raises
    /// * If the predecessors vector was not requested.
    pub fn get_point_at_given_distance_on_shortest_path(
        &self,
        mut dst_node_id: NodeT,
        distance: f32,
    ) -> Result<NodeT> {
        if !self.has_path_to_node_id(dst_node_id)? {
            return Err("There is no path to the given destination node.".to_string());
        }
        if self.get_distance_from_node_id(dst_node_id)? < distance {
            return Err(format!(
                concat!(
                    "The path to the requested node {} has distance {}, ",
                    "but the requested distance is {}."
                ),
                dst_node_id,
                self.get_eccentricity(),
                distance
            ));
        }
        if let Some(predecessors) = self.predecessors.as_ref() {
            while self.get_distance_from_node_id(dst_node_id)? < distance {
                if let Some(node_id) = predecessors[dst_node_id as usize] {
                    dst_node_id = node_id;
                } else {
                    break;
                }
            }
            return Ok(dst_node_id);
        }
        Err("Predecessors were not requested and therefore not computed.".to_string())
    }

    pub fn get_median_point(&self, dst_node_id: NodeT) -> Result<NodeT> {
        if !self.has_path_to_node_id(dst_node_id)? {
            return Err("There is no path to the given destination node.".to_string());
        }
        let median_distance = self.get_distance_from_node_id(dst_node_id)? / 2.0;
        self.get_point_at_given_distance_on_shortest_path(dst_node_id, median_distance)
    }

    pub fn get_eccentricity(&self) -> f32 {
        self.eccentricity
    }

    pub fn get_total_distance(&self) -> f32 {
        self.total_distance
    }
    pub fn get_log_total_distance(&self) -> f32 {
        self.log_total_distance
    }

    pub fn get_most_distant_node(&self) -> NodeT {
        self.most_distant_node
    }

    /// Returns the number of shortest paths starting from the root node.
    pub fn get_number_of_shortest_paths(&self) -> NodeT {
        self.distances
            .par_iter()
            .filter(|&distances| distances.is_finite())
            .count() as NodeT
    }

    /// Returns the number of shortest paths passing through the given node.
    ///
    /// # Arguments
    /// * `node_id`: NodeT - The node id.
    ///
    /// # Raises
    /// * If neither predecessors nor distances were computed for this BFS.
    /// * If the given node ID does not exist in the current graph instance.
    ///
    /// # Returns
    /// The number of nodes passing by the node ID.
    pub fn get_number_of_shortest_paths_from_node_id(&self, node_id: NodeT) -> Result<NodeT> {
        self.validate_node_id(node_id)?;
        if let Some(predecessors) = self.predecessors.as_ref() {
            return Ok(predecessors
                .par_iter()
                .filter(|&&predecessor| predecessor.map_or(false, |pred| pred == node_id))
                .count() as NodeT);
        }
        Err(concat!(
            "The predecessors were computed (as it was requested) ",
            "when creating this breath shortest paths object.\n",
            "It is not possible to compute the number of shortest paths from the current ",
            "root node passing to the given node ID when predecessors were not computed."
        )
        .to_string())
    }

    /// Return list of successors of a given node.
    ///
    /// # Arguments
    /// * `source_node_id`: NodeT - The node for which to return the successors.
    ///
    /// # Raises
    /// * If the given node ID does not exist in the graph.
    ///
    /// # Returns
    /// List of successors of the given node.
    pub fn get_successors_from_node_id(&self, source_node_id: NodeT) -> Result<Vec<NodeT>> {
        self.validate_node_id(source_node_id)?;
        if let Some(predecessors) = self.predecessors.as_ref() {
            let nodes_number = predecessors.len() as NodeT;
            return Ok((0..nodes_number)
                .into_par_iter()
                .filter(move |&node_id| {
                    let mut node_id = node_id;
                    while predecessors[node_id as usize]
                        .map_or(false, |predecessor| predecessor != node_id)
                    {
                        if predecessors[node_id as usize].is_none() {
                            return false;
                        }
                        node_id = predecessors[node_id as usize].unwrap();
                        if source_node_id == node_id {
                            return true;
                        }
                    }
                    false
                })
                .collect::<Vec<NodeT>>());
        }
        Err(concat!(
            "The predecessors were computed (as it was requested) ",
            "when creating this breath shortest paths object.\n",
            "It is not possible to compute the number of shortest paths from the current ",
            "root node passing to the given node ID when predecessors were not computed."
        )
        .to_string())
    }

    #[no_binding]
    pub fn into_iter_finite_distances(self) -> impl Iterator<Item = f32> {
        self.distances
            .into_iter()
            .filter(|&distance| distance.is_finite())
    }

    #[no_binding]
    pub fn into_par_iter_node_ids_and_finite_distances(
        self,
    ) -> impl ParallelIterator<Item = (NodeT, f32)> {
        self.distances
            .into_par_iter()
            .enumerate()
            .filter_map(|(node_id, distance)| {
                if distance.is_finite() {
                    Some((node_id as NodeT, distance))
                } else {
                    None
                }
            })
    }

    #[no_binding]
    pub fn into_distances(self) -> Vec<f32> {
        self.distances
    }
}

impl Graph {
    /// Returns shortest path result for the BFS from given source node ID.
    ///
    /// # Arguments
    /// * `src_node_id`: NodeT - Root of the tree of minimum paths.
    ///
    /// # Safety
    /// If any of the given node ID does not exist in the graph the method will panic.
    ///
    pub unsafe fn get_unchecked_breadth_first_search_predecessors_parallel_from_node_id(
        &self,
        src_node_id: NodeT,
    ) -> ShortestPathsResultBFS {
        let nodes_number = self.get_number_of_nodes() as usize;
        let thread_shared_predecessors =
            ThreadDataRaceAware::new(vec![NODE_NOT_PRESENT; nodes_number]);
        (*thread_shared_predecessors.value.get())[src_node_id as usize] = src_node_id;
        let mut eccentricity = 0;
        let mut most_distant_node = src_node_id;

        let mut frontier_new = Frontier::new();
        let mut frontier = Frontier::new();
        frontier.push(src_node_id);

        while !frontier.is_empty() {
            eccentricity += 1;
            most_distant_node = *frontier.iter().next().unwrap();
            frontier.par_iter().for_each(|node_id| {
                self.iter_unchecked_neighbour_node_ids_from_source_node_id(*node_id)
                    .for_each(|neighbour_node_id| {
                        if (*thread_shared_predecessors.value.get())[neighbour_node_id as usize]
                            == NODE_NOT_PRESENT
                        {
                            // Set it's distance
                            (*thread_shared_predecessors.value.get())[neighbour_node_id as usize] =
                                *node_id;
                            // add the node to the nodes to explore
                            frontier_new.push(neighbour_node_id);
                        }
                    });
            });
            frontier.clear();
            std::mem::swap(&mut frontier, &mut frontier_new);
        }
        eccentricity -= 1;

        ShortestPathsResultBFS::new(
            None,
            Some(thread_shared_predecessors.value.into_inner()),
            eccentricity,
            most_distant_node,
        )
    }

    #[no_binding]
    /// Returns shortest path result for the BFS from given source node IDs, treating the set of source nodes as an hyper-node.
    ///
    /// # Arguments
    /// * `src_node_ids`: Vec<NodeT> - Roots of the tree of minimum paths.
    /// * `maximal_depth`: Option<NodeT> - The maximal depth to run the BFS for.
    ///
    /// # Safety
    /// If any of the given node IDs does not exist in the graph the method will panic.
    /// The provided list of node ids must be non-empty, or the method will panic.
    ///
    pub unsafe fn get_unchecked_generic_breadth_first_search_distances_parallel_from_node_ids<
        T: Send + Sync + PrimInt + TryFrom<usize> + std::ops::AddAssign,
    >(
        &self,
        src_node_ids: Vec<NodeT>,
        maximal_depth: Option<T>,
    ) -> (Vec<T>, T, NodeT) {
        let nodes_number = self.get_number_of_nodes() as usize;
        let node_not_present = T::max_value();
        let mut distances = vec![node_not_present; nodes_number];
        let thread_shared_distances = ThreadDataRaceAware::new(&mut distances);
        for src_node_id in src_node_ids.iter().cloned() {
            (*thread_shared_distances.value.get())[src_node_id as usize] =
                T::try_from(0).ok().unwrap();
        }
        let mut eccentricity: T = T::try_from(0).ok().unwrap();
        let mut most_distant_node = src_node_ids[0];

        let mut frontier: Frontier<NodeT> = src_node_ids.into();
        let mut frontier_new = Frontier::new();

        while !frontier.is_empty() {
            eccentricity += T::try_from(1).ok().unwrap();
            most_distant_node = *frontier.iter().next().unwrap();
            if maximal_depth.map_or(false, |maximal_depth| maximal_depth > eccentricity) {
                break;
            }

            frontier.par_iter().for_each(|node_id| {
                self.iter_unchecked_neighbour_node_ids_from_source_node_id(*node_id)
                    .for_each(|neighbour_node_id| {
                        if (*thread_shared_distances.value.get())[neighbour_node_id as usize]
                            == node_not_present
                        {
                            // Set it's distance
                            (*thread_shared_distances.value.get())[neighbour_node_id as usize] =
                                eccentricity;
                            // add the node to the nodes to explore
                            frontier_new.push(neighbour_node_id);
                        }
                    });
            });
            frontier.clear();
            std::mem::swap(&mut frontier, &mut frontier_new);
        }
        eccentricity = eccentricity.saturating_sub(T::try_from(1).ok().unwrap());
        (distances, eccentricity, most_distant_node)
    }

    /// Returns shortest path result for the BFS from given source node IDs, treating the set of source nodes as an hyper-node.
    ///
    /// # Arguments
    /// * `src_node_ids`: Vec<NodeT> - Roots of the tree of minimum paths.
    /// * `maximal_depth`: Option<NodeT> - The maximal depth to run the BFS for.
    ///
    /// # Safety
    /// If any of the given node IDs does not exist in the graph the method will panic.
    /// The provided list of node ids must be non-empty, or the method will panic.
    ///
    pub unsafe fn get_unchecked_breadth_first_search_distances_parallel_from_node_ids(
        &self,
        src_node_ids: Vec<NodeT>,
        maximal_depth: Option<NodeT>,
    ) -> ShortestPathsResultBFS {
        let (distances, eccentricity, most_distant_node) = self
            .get_unchecked_generic_breadth_first_search_distances_parallel_from_node_ids::<u32>(
                src_node_ids,
                maximal_depth,
            );
        ShortestPathsResultBFS::new(Some(distances), None, eccentricity, most_distant_node)
    }

    /// Returns shortest path result for the BFS from given source node ID.
    ///
    /// # Arguments
    /// * `src_node_id`: NodeT - Root of the tree of minimum paths.
    /// * `maximal_depth`: Option<NodeT> - The maximal depth to run the BFS for.
    ///
    /// # Safety
    /// If any of the given node ID does not exist in the graph the method will panic.
    ///
    pub unsafe fn get_unchecked_breadth_first_search_distances_parallel_from_node_id(
        &self,
        src_node_id: NodeT,
        maximal_depth: Option<NodeT>,
    ) -> ShortestPathsResultBFS {
        self.get_unchecked_breadth_first_search_distances_parallel_from_node_ids(
            vec![src_node_id],
            maximal_depth,
        )
    }

    /// Returns shortest path result for the BFS from given source node ID.
    ///
    /// # Arguments
    /// * `src_node_id`: NodeT - Root of the tree of minimum paths.
    ///
    /// # Safety
    /// If any of the given node ID does not exist in the graph the method will panic.
    ///
    /// TODO! Explore chains accelerations!
    pub unsafe fn get_unchecked_breadth_first_search_distances_sequential_from_node_id(
        &self,
        src_node_id: NodeT,
    ) -> ShortestPathsResultBFS {
        let nodes_number = self.get_number_of_nodes() as usize;
        let mut distances = vec![NODE_NOT_PRESENT; nodes_number];
        distances[src_node_id as usize] = 0;
        let mut eccentricity = 0;
        let mut most_distant_node = src_node_id;

        let mut frontier = vec![src_node_id];

        while !frontier.is_empty() {
            eccentricity += 1;
            most_distant_node = frontier[0];
            frontier = frontier
                .into_iter()
                .flat_map(|node_id| {
                    self.iter_unchecked_neighbour_node_ids_from_source_node_id(node_id)
                })
                .filter_map(|neighbour_node_id| {
                    if distances[neighbour_node_id as usize] == NODE_NOT_PRESENT {
                        // Set it's distance
                        distances[neighbour_node_id as usize] = eccentricity;
                        // add the node to the nodes to explore
                        Some(neighbour_node_id)
                    } else {
                        None
                    }
                })
                .collect::<Vec<NodeT>>();
        }
        eccentricity -= 1;
        ShortestPathsResultBFS::new(Some(distances), None, eccentricity, most_distant_node)
    }

    /// Returns vector of minimum paths distances and vector of nodes predecessors, if requested, treating the set of source nodes as an hyper-node.
    ///
    /// # Arguments
    /// * `src_node_ids`: Vec<NodeT> - Root of the tree of minimum paths.
    /// * `maybe_dst_node_id`: Option<NodeT> - Optional target destination. If provided, the breadth first search will stop upon reaching this node.
    /// * `compute_predecessors`: Option<bool> - Whether to compute the vector of predecessors.
    /// * `maximal_depth`: Option<NodeT> - The maximal depth to execute the DFS for.
    ///
    /// # Safety
    /// If any of the given node IDs does not exist in the graph the method will panic.
    ///
    pub unsafe fn get_unchecked_breadth_first_search_from_node_ids(
        &self,
        src_node_ids: Vec<NodeT>,
        dst_node_id: Option<NodeT>,
        compute_predecessors: Option<bool>,
        maximal_depth: Option<NodeT>,
    ) -> ShortestPathsResultBFS {
        let compute_predecessors = compute_predecessors.unwrap_or(true);

        let nodes_number = self.get_number_of_nodes() as usize;
        let mut found_destination = false;

        let mut predecessors: Option<Vec<NodeT>> = if compute_predecessors {
            let mut predecessors = vec![NODE_NOT_PRESENT; nodes_number];
            for src_node_id in src_node_ids.iter().cloned() {
                predecessors[src_node_id as usize] = src_node_id;
            }
            Some(predecessors)
        } else {
            None
        };

        let mut distances: Vec<NodeT> = vec![NODE_NOT_PRESENT; nodes_number];
        for src_node_id in src_node_ids.iter().cloned() {
            distances[src_node_id as usize] = 0;
        }
        let mut eccentricity = 0;
        let mut most_distant_node = src_node_ids[0];

        let mut nodes_to_explore = VecDeque::with_capacity(nodes_number);
        for src_node_id in src_node_ids.iter().cloned() {
            nodes_to_explore.push_back((src_node_id, 0));
        }

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
                    if distances[neighbour_node_id as usize] > new_neighbour_distance {
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
        ShortestPathsResultBFS::new(
            Some(distances),
            predecessors,
            eccentricity,
            most_distant_node,
        )
    }

    /// Returns vector of minimum paths distances and vector of nodes predecessors, if requested.
    ///
    /// # Arguments
    /// * `src_node_id`: NodeT - Root of the tree of minimum paths.
    /// * `maybe_dst_node_id`: Option<NodeT> - Optional target destination. If provided, breadth first search will stop upon reaching this node.
    /// * `compute_predecessors`: Option<bool> - Whether to compute the vector of predecessors.
    /// * `maximal_depth`: Option<NodeT> - The maximal depth to execute the DFS for.
    ///
    /// # Safety
    /// If any of the given node IDs does not exist in the graph the method will panic.
    ///
    pub unsafe fn get_unchecked_breadth_first_search_from_node_id(
        &self,
        src_node_id: NodeT,
        dst_node_id: Option<NodeT>,
        compute_predecessors: Option<bool>,
        maximal_depth: Option<NodeT>,
    ) -> ShortestPathsResultBFS {
        self.get_unchecked_breadth_first_search_from_node_ids(
            vec![src_node_id],
            dst_node_id,
            compute_predecessors,
            maximal_depth,
        )
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
    pub unsafe fn get_unchecked_shortest_path_node_ids_from_node_ids(
        &self,
        src_node_id: NodeT,
        dst_node_id: NodeT,
        maximal_depth: Option<NodeT>,
    ) -> Result<Vec<NodeT>> {
        if src_node_id == dst_node_id {
            return Err("The minimum path on a selfloop is not defined.".to_string());
        }
        let bfs = self.get_unchecked_breadth_first_search_from_node_ids(
            vec![src_node_id],
            Some(dst_node_id),
            None,
            maximal_depth,
        );

        // If the distance is infinite, the destination node is not connected.
        if !bfs.has_path_to_node_id(dst_node_id)? {
            return Err(format!(
                "There is no path starting from the given source node {} and reaching the given destination node {}.",
                src_node_id, dst_node_id
            ));
        }
        let path_length = bfs.get_distance_from_node_id(dst_node_id)? as usize + 1;
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
    pub unsafe fn get_unchecked_shortest_path_node_names_from_node_ids(
        &self,
        src_node_id: NodeT,
        dst_node_id: NodeT,
        maximal_depth: Option<NodeT>,
    ) -> Result<Vec<String>> {
        Ok(self
            .get_unchecked_shortest_path_node_ids_from_node_ids(
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
    pub fn get_shortest_path_node_ids_from_node_ids(
        &self,
        src_node_id: NodeT,
        dst_node_id: NodeT,
        maximal_depth: Option<NodeT>,
    ) -> Result<Vec<NodeT>> {
        Ok(unsafe {
            self.get_unchecked_shortest_path_node_ids_from_node_ids(
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
    pub fn get_shortest_path_node_ids_from_node_names(
        &self,
        src_node_name: &str,
        dst_node_name: &str,
        maximal_depth: Option<NodeT>,
    ) -> Result<Vec<NodeT>> {
        Ok(unsafe {
            self.get_unchecked_shortest_path_node_ids_from_node_ids(
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
    pub fn get_shortest_path_node_names_from_node_names(
        &self,
        src_node_name: &str,
        dst_node_name: &str,
        maximal_depth: Option<NodeT>,
    ) -> Result<Vec<String>> {
        Ok(unsafe {
            self.get_unchecked_shortest_path_node_names_from_node_ids(
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
        let nodes_number = self.get_number_of_nodes() as usize;
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
    pub unsafe fn get_unchecked_eccentricity_and_most_distant_node_id_from_node_id(
        &self,
        node_id: NodeT,
    ) -> (NodeT, NodeT) {
        let nodes_number = self.get_number_of_nodes() as usize;
        let thread_shared_visited = ThreadDataRaceAware::new(vec![false; nodes_number]);
        (*thread_shared_visited.value.get())[node_id as usize] = true;
        let mut eccentricity = 0;
        let mut most_distant_node = node_id;

        let mut frontier_new = Frontier::new();
        let mut frontier = Frontier::new();
        frontier.push(node_id);

        while !frontier.is_empty() {
            eccentricity += 1;
            most_distant_node = *frontier.iter().next().unwrap();
            frontier.par_iter().for_each(|node_id| {
                self.iter_unchecked_neighbour_node_ids_from_source_node_id(*node_id)
                    .for_each(|neighbour_node_id| {
                        if !(*thread_shared_visited.value.get())[neighbour_node_id as usize] {
                            // Set it's distance
                            (*thread_shared_visited.value.get())[neighbour_node_id as usize] = true;
                            // add the node to the nodes to explore
                            frontier_new.push(neighbour_node_id);
                        }
                    })
            });
            frontier.clear();
            std::mem::swap(&mut frontier, &mut frontier_new);
        }
        eccentricity -= 1;
        (eccentricity, most_distant_node)
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
    ) -> f32 {
        self.get_unchecked_dijkstra_from_node_id(
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
    pub fn get_eccentricity_and_most_distant_node_id_from_node_id(
        &self,
        node_id: NodeT,
    ) -> Result<(NodeT, NodeT)> {
        self.validate_node_id(node_id).map(|node_id| unsafe {
            self.get_unchecked_eccentricity_and_most_distant_node_id_from_node_id(node_id)
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
    ) -> Result<f32> {
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
            .map(|node_id| unsafe {
                self.get_unchecked_eccentricity_and_most_distant_node_id_from_node_id(node_id)
                    .0
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
    ) -> Result<f32> {
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

    /// Returns vector of minimum paths distances and vector of nodes predecessors, if requested, from the given root nodes (treated as an hyper-node).
    ///
    /// # Arguments
    /// * `src_node_id`: Vec<NodeT> - Root of the tree of minimum paths.
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
        src_node_ids: Vec<NodeT>,
        maybe_dst_node_id: Option<NodeT>,
        mut maybe_dst_node_ids: Option<Vec<NodeT>>,
        compute_predecessors: Option<bool>,
        maximal_depth: Option<NodeT>,
        use_edge_weights_as_probabilities: Option<bool>,
    ) -> ShortestPathsDjkstra {
        let compute_predecessors = compute_predecessors.unwrap_or(true);
        let nodes_number = self.get_number_of_nodes() as usize;
        let mut most_distant_node = src_node_ids[0];
        let use_edge_weights_as_probabilities = use_edge_weights_as_probabilities.unwrap_or(false);
        let mut dst_node_distance = maybe_dst_node_id.map(|_| {
            if use_edge_weights_as_probabilities {
                0.0
            } else {
                f32::INFINITY
            }
        });
        let mut predecessors: Option<Vec<Option<NodeT>>> = if compute_predecessors {
            Some(vec![None; nodes_number])
        } else {
            None
        };

        if src_node_ids
            .iter()
            .cloned()
            .all(|src_node_id| self.is_unchecked_disconnected_node_from_node_id(src_node_id))
        {
            if use_edge_weights_as_probabilities {
                return ShortestPathsDjkstra::new(
                    vec![0.0; nodes_number],
                    most_distant_node,
                    predecessors,
                    dst_node_distance,
                    0.0,
                    0.0,
                    0.0,
                    0.0,
                );
            } else {
                return ShortestPathsDjkstra::new(
                    vec![f32::INFINITY; nodes_number],
                    most_distant_node,
                    predecessors,
                    dst_node_distance,
                    f32::INFINITY,
                    f32::INFINITY,
                    f32::INFINITY,
                    0.0,
                );
            }
        }

        let bfs: Option<ShortestPathsResultBFS> = maximal_depth.map(|md| {
            self.get_unchecked_breadth_first_search_from_node_ids(
                src_node_ids.clone(),
                maybe_dst_node_id,
                None,
                Some(md),
            )
        });

        let mut distances = vec![f32::MAX; nodes_number];
        let mut nodes_to_explore: DijkstraQueue<f32> =
            DijkstraQueue::with_capacity_from_roots(nodes_number, src_node_ids, &mut distances);
        let mut eccentricity: f32 = 0.0;
        let mut total_distance: f32 = 0.0;
        let mut total_harmonic_distance: f32 = 0.0;

        while let Some(closest_node_id) = nodes_to_explore.pop() {
            // Update the distances metrics
            let closest_node_id_distance = nodes_to_explore[closest_node_id];
            if closest_node_id_distance > eccentricity {
                eccentricity = closest_node_id_distance;
                most_distant_node = closest_node_id as NodeT;
            }
            total_distance += if use_edge_weights_as_probabilities {
                (-nodes_to_explore[closest_node_id]).exp()
            } else {
                nodes_to_explore[closest_node_id]
            };
            if nodes_to_explore[closest_node_id] > 0.0 {
                total_harmonic_distance += nodes_to_explore[closest_node_id].recip();
            }
            // If the closest node is the optional destination node, we have
            // completed what the user has required.
            if maybe_dst_node_id.map_or(false, |dst| dst == closest_node_id as NodeT) {
                let _ = dst_node_distance.insert(if use_edge_weights_as_probabilities {
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
                        if !bfs.has_path_to_node_id(neighbour_node_id).unwrap() {
                            return;
                        }
                    }
                    let new_neighbour_distance = nodes_to_explore[closest_node_id]
                        + if use_edge_weights_as_probabilities {
                            -(weight as f32).ln()
                        } else {
                            weight as f32
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

        // If the edge weights are to be treated as probabilities
        // we need to adjust the distances back using the exponentiation.
        let log_total_distance = if use_edge_weights_as_probabilities {
            distances
                .par_iter_mut()
                .for_each(|distance| *distance = (-*distance).exp());
            eccentricity = (-eccentricity).exp();
            let log_total_distance = total_distance;
            total_distance = (-total_distance).exp();
            log_total_distance
        } else {
            total_distance.ln()
        };

        ShortestPathsDjkstra {
            distances,
            most_distant_node,
            predecessors,
            dst_node_distance,
            eccentricity,
            total_distance,
            log_total_distance,
            total_harmonic_distance,
        }
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
    pub unsafe fn get_unchecked_dijkstra_from_node_id(
        &self,
        src_node_id: NodeT,
        maybe_dst_node_id: Option<NodeT>,
        maybe_dst_node_ids: Option<Vec<NodeT>>,
        compute_predecessors: Option<bool>,
        maximal_depth: Option<NodeT>,
        use_edge_weights_as_probabilities: Option<bool>,
    ) -> ShortestPathsDjkstra {
        self.get_unchecked_dijkstra_from_node_ids(
            vec![src_node_id],
            maybe_dst_node_id,
            maybe_dst_node_ids,
            compute_predecessors,
            maximal_depth,
            use_edge_weights_as_probabilities,
        )
    }

    /// Returns minimum path node IDs and distance from given node ids.
    ///
    /// # Arguments
    /// * `src_node_id`: NodeT - Source node ID.
    /// * `dst_node_id`: NodeT - Destination node ID.
    /// * `use_edge_weights_as_probabilities`: Option<bool> - Whether to treat the edge weights as probabilities.
    /// * `maximal_depth`: Option<NodeT> - The maximal number of iterations to execute Dijkstra for.
    ///
    /// # Safety
    /// If any of the given node IDs does not exist in the graph the method will panic.
    pub unsafe fn get_unchecked_weighted_shortest_path_node_ids_from_node_ids(
        &self,
        src_node_id: NodeT,
        dst_node_id: NodeT,
        use_edge_weights_as_probabilities: Option<bool>,
        maximal_depth: Option<NodeT>,
    ) -> (f32, Vec<NodeT>) {
        let dijkstra = self.get_unchecked_dijkstra_from_node_id(
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
            return (f32::INFINITY, Vec::new());
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
    /// * `use_edge_weights_as_probabilities`: Option<bool> - Whether to treat the edge weights as probabilities.
    /// * `maximal_depth`: Option<NodeT> - The maximal number of iterations to execute Dijkstra for.
    ///
    /// # Safety
    /// If any of the given node IDs does not exist in the graph the method will panic.
    pub unsafe fn get_unchecked_weighted_shortest_path_node_names_from_node_ids(
        &self,
        src_node_id: NodeT,
        dst_node_id: NodeT,
        use_edge_weights_as_probabilities: Option<bool>,
        maximal_depth: Option<NodeT>,
    ) -> (f32, Vec<String>) {
        let (path_length, path) = self.get_unchecked_weighted_shortest_path_node_ids_from_node_ids(
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
    /// * `use_edge_weights_as_probabilities`: Option<bool> - Whether to treat the edge weights as probabilities.
    /// * `maximal_depth`: Option<NodeT> - The maximal number of iterations to execute Dijkstra for.
    ///
    /// # Raises
    /// * If any of the given node IDs do not exist in the current graph.
    pub fn get_weighted_shortest_path_node_ids_from_node_ids(
        &self,
        src_node_id: NodeT,
        dst_node_id: NodeT,
        use_edge_weights_as_probabilities: Option<bool>,
        maximal_depth: Option<NodeT>,
    ) -> Result<(f32, Vec<NodeT>)> {
        self.must_have_positive_edge_weights()?;
        if let Some(uewp) = use_edge_weights_as_probabilities {
            if uewp {
                self.must_have_edge_weights_representing_probabilities()?;
            }
        }
        Ok(unsafe {
            self.get_unchecked_weighted_shortest_path_node_ids_from_node_ids(
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
    pub fn get_weighted_shortest_path_node_ids_from_node_names(
        &self,
        src_node_name: &str,
        dst_node_name: &str,
        use_edge_weights_as_probabilities: Option<bool>,
        maximal_depth: Option<NodeT>,
    ) -> Result<(f32, Vec<NodeT>)> {
        self.must_have_positive_edge_weights()?;
        if let Some(uewp) = use_edge_weights_as_probabilities {
            if uewp {
                self.must_have_edge_weights_representing_probabilities()?;
            }
        }
        Ok(unsafe {
            self.get_unchecked_weighted_shortest_path_node_ids_from_node_ids(
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
    pub fn get_weighted_shortest_path_node_names_from_node_names(
        &self,
        src_node_name: &str,
        dst_node_name: &str,
        use_edge_weights_as_probabilities: Option<bool>,
        maximal_depth: Option<NodeT>,
    ) -> Result<(f32, Vec<String>)> {
        self.must_have_positive_edge_weights()?;
        if let Some(uewp) = use_edge_weights_as_probabilities {
            if uewp {
                self.must_have_edge_weights_representing_probabilities()?;
            }
        }
        Ok(unsafe {
            self.get_unchecked_weighted_shortest_path_node_names_from_node_ids(
                self.get_node_id_from_node_name(src_node_name)?,
                self.get_node_id_from_node_name(dst_node_name)?,
                use_edge_weights_as_probabilities,
                maximal_depth,
            )
        })
    }

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
    pub fn get_breadth_first_search_from_node_ids(
        &self,
        src_node_id: NodeT,
        dst_node_id: Option<NodeT>,
        compute_predecessors: Option<bool>,
        maximal_depth: Option<NodeT>,
    ) -> Result<ShortestPathsResultBFS> {
        // Check if the given root exists in the graph
        self.validate_node_id(src_node_id)?;
        unsafe {
            Ok(self.get_unchecked_breadth_first_search_from_node_ids(
                vec![src_node_id],
                dst_node_id,
                compute_predecessors,
                maximal_depth,
            ))
        }
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
            self.get_unchecked_dijkstra_from_node_id(
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
    pub fn get_four_sweep(&self) -> (NodeT, NodeT) {
        let most_central_node_id = unsafe { self.get_unchecked_most_central_node_id() };
        let first_candidate_most_eccentric_node_id = unsafe {
            self.get_unchecked_eccentricity_and_most_distant_node_id_from_node_id(
                most_central_node_id,
            )
            .1
        };

        let bfs1 = unsafe {
            self.get_unchecked_breadth_first_search_predecessors_parallel_from_node_id(
                first_candidate_most_eccentric_node_id,
            )
        };

        let second_candidate_most_eccentric_node_id = unsafe {
            self.get_unchecked_eccentricity_and_most_distant_node_id_from_node_id(
                bfs1.get_median_point_to_most_distant_node().unwrap(),
            )
            .1
        };
        let bfs2 = unsafe {
            self.get_unchecked_breadth_first_search_predecessors_parallel_from_node_id(
                second_candidate_most_eccentric_node_id,
            )
        };

        (
            bfs1.get_eccentricity().max(bfs2.get_eccentricity()),
            bfs2.get_median_point_to_most_distant_node().unwrap(),
        )
    }

    /// Returns diameter of an UNDIRECTED graph.
    ///
    /// # Referencences
    /// This method is based on the algorithm described in ["On computing the diameter of real-world undirected graphs" by Crescenzi et al](https://who.rocq.inria.fr/Laurent.Viennot/road/papers/ifub.pdf).
    fn get_ifub(&self) -> Result<f32> {
        if self.is_directed() {
            panic!(
                "This method is not defined YET for directed graphs! We will add it in the future!"
            )
        }

        let most_central_node_id = unsafe { self.get_unchecked_most_central_node_id() };
        if unsafe { self.is_unchecked_disconnected_node_from_node_id(most_central_node_id) } {
            return Ok(0.0);
        }

        // get the lowerbound of the diameter
        let (mut tentative_diameter, low_eccentricity_node) = self.get_four_sweep();
        // find the distances of all the nodes from the node with low eccentricty,
        // and thus with high centrality
        let bfs = unsafe {
            self.get_unchecked_breadth_first_search_distances_parallel_from_node_id(
                low_eccentricity_node,
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

        // If all the test cases are empty, it means
        // that our tentative diameter is already the actual diameter.
        if node_ids_and_distances.is_empty() {
            return Ok(tentative_diameter as f32);
        }

        // sort the nodes by distance, so that we will start checking from the
        // most distant ones which are the most probable to be an extreme of the
        // diameter.
        node_ids_and_distances.par_sort_unstable_by(|(_, a), &(_, b)| b.cmp(a));

        let mut current_distance = node_ids_and_distances[0].1;

        for (node_id, distance) in node_ids_and_distances {
            // If the distance has changed, it means we have finished
            // a distance block and therefore we can check if we have
            // found the diameter inside this distances block.
            if current_distance != distance {
                current_distance = distance;
                if tentative_diameter >= current_distance * 2 {
                    break;
                }
            }

            // Alternatively, we compute for another node ID
            // its eccentricity.
            tentative_diameter = tentative_diameter.max(
                unsafe {
                    self.get_unchecked_eccentricity_and_most_distant_node_id_from_node_id(node_id)
                }
                .0,
            );
        }

        Ok(tentative_diameter as f32)
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
    ) -> Result<f32> {
        self.must_have_nodes()?;
        let ignore_infinity = ignore_infinity.unwrap_or(false);
        let verbose = verbose.unwrap_or(true);

        if !self.has_edges() || !ignore_infinity && !self.is_connected(Some(verbose)) {
            return Ok(f32::INFINITY);
        }

        let pb = get_loading_bar(
            verbose,
            "Computing diameter",
            self.get_number_of_nodes() as usize,
        );

        Ok(self
            .par_iter_node_ids()
            .progress_with(pb)
            .map(|node_id| unsafe {
                self.get_unchecked_eccentricity_and_most_distant_node_id_from_node_id(node_id)
                    .0
            })
            .filter(|&distance| !ignore_infinity || distance != NODE_NOT_PRESENT)
            .max()
            .unwrap_or(0) as f32)
    }

    #[cache_property(diameter)]
    /// Returns diameter of the graph.
    ///
    /// # Arguments
    /// * `ignore_infinity`: Option<bool> - Whether to ignore infinite distances, which are present when in the graph exist multiple components. By default True.
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
    ) -> Result<f32> {
        self.must_have_nodes()?;
        let ignore_infinity = ignore_infinity.unwrap_or(true);
        let verbose = verbose.unwrap_or(true);

        if !self.has_edges() || !ignore_infinity && !self.is_connected(Some(verbose)) {
            return Ok(f32::INFINITY);
        }

        if self.is_directed() {
            self.get_diameter_naive(Some(true), Some(verbose))
        } else {
            self.get_ifub()
        }
    }

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
    pub fn get_breadth_first_search_from_node_names(
        &self,
        src_node_name: &str,
        dst_node_name: Option<&str>,
        compute_predecessors: Option<bool>,
        maximal_depth: Option<NodeT>,
    ) -> Result<ShortestPathsResultBFS> {
        unsafe {
            Ok(self.get_unchecked_breadth_first_search_from_node_id(
                self.get_node_id_from_node_name(src_node_name)?,
                dst_node_name.map_or(Ok::<_, String>(None), |dst_node_name| {
                    Ok(Some(self.get_node_id_from_node_name(dst_node_name)?))
                })?,
                compute_predecessors,
                maximal_depth,
            ))
        }
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
