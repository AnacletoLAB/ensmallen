use super::*;
use indicatif::ParallelProgressIterator;
use keyed_priority_queue::KeyedPriorityQueue;
use rayon::iter::ParallelIterator;
use roaring::RoaringBitmap;
use std::cmp::Reverse;
use std::cmp::{Eq, Ord, Ordering, PartialEq, PartialOrd};

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

impl Graph {
    /// Returns vector of minimum paths distances and vector of nodes predecessors, if requested.
    ///
    /// # Arguments
    /// * `src_node_id`: NodeT - Root of the tree of minimum paths.
    /// * `maybe_dst_node_id`: Option<NodeT> - Optional target destination. If provided, Dijkstra will stop upon reaching this node.
    /// * `maybe_dst_node_ids`: Option<RoaringBitmap> - Optional target destinations. If provided, Dijkstra will stop upon reaching all of these nodes.
    /// * `compute_predecessors`: bool - Whether to compute the vector of predecessors or to limit the allocation to exclusively the distances.
    /// * `verbose`: bool - Whether to show an indicative progress bar.
    pub fn get_unchecked_unweighted_dijkstra_from_node_ids(
        &self,
        src_node_id: NodeT,
        maybe_dst_node_id: Option<NodeT>,
        mut maybe_dst_node_ids: Option<RoaringBitmap>,
        compute_predecessors: Option<bool>,
        verbose: Option<bool>,
    ) -> (Vec<NodeT>, Option<Vec<NodeT>>) {
        let compute_predecessors = compute_predecessors.unwrap_or(true);
        let verbose = verbose.unwrap_or(true);
        let nodes_number = self.get_nodes_number() as usize;
        let mut parents: Option<Vec<NodeT>> = if compute_predecessors {
            Some(vec![0; nodes_number])
        } else {
            None
        };
        let mut distances: Vec<NodeT> = vec![NodeT::MAX; nodes_number];
        distances[src_node_id as usize] = 0;

        // If the given root node is either a:
        // - singleton
        // - singleton with selfloops
        // - trap node
        // we have already completed the Dijkstra.
        if self.is_unchecked_singleton_from_node_id(src_node_id)
            || self.is_singleton_with_selfloops_from_node_id(src_node_id)
            || self.is_unchecked_trap_node_from_node_id(src_node_id)
        {
            return (distances, parents);
        }

        let mut nodes_to_explore: KeyedPriorityQueue<NodeT, Reverse<NodeT>> =
            KeyedPriorityQueue::new();
        nodes_to_explore.push(src_node_id, Reverse(0));

        let pb = get_loading_bar(verbose, "Computing Dijkstra", nodes_number);

        while !nodes_to_explore.is_empty() {
            // TODO: Use the distance that is returned from the queue instead of getting the one from the vector.
            let (closest_node_id, _) = nodes_to_explore.pop().unwrap();
            // We increase the loading bar by one.
            pb.inc(1);
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

            for neighbour_node_id in
                self.iter_unchecked_neighbour_node_ids_from_source_node_id(closest_node_id)
            {
                // Since we are not taking in consideration the weights
                // all the node neighbours have distance 1.
                let new_neighbour_distance = distances[closest_node_id as usize] + 1;
                if new_neighbour_distance < distances[neighbour_node_id as usize] {
                    distances[neighbour_node_id as usize] = new_neighbour_distance;
                    if let Some(parents) = &mut parents {
                        parents[neighbour_node_id as usize] = closest_node_id;
                    }
                    nodes_to_explore.push(neighbour_node_id, Reverse(new_neighbour_distance));
                }
            }
        }
        (distances, parents)
    }

    /// Returns vector of minimum paths distances and vector of nodes predecessors, if requested.
    ///
    /// # Arguments
    /// * `src_node_id`: NodeT - Root of the tree of minimum paths.
    /// * `maybe_dst_node_id`: Option<NodeT> - Optional target destination. If provided, Dijkstra will stop upon reaching this node.
    /// * `maybe_dst_node_ids`: Option<RoaringBitmap> - Optional target destinations. If provided, Dijkstra will stop upon reaching all of these nodes.
    /// * `compute_predecessors`: bool - Whether to compute the vector of predecessors or to limit the allocation to exclusively the distances.
    /// * `verbose`: bool - Whether to show an indicative progress bar.
    pub fn get_unchecked_weighted_dijkstra_from_node_ids(
        &self,
        src_node_id: NodeT,
        maybe_dst_node_id: Option<NodeT>,
        mut maybe_dst_node_ids: Option<RoaringBitmap>,
        compute_predecessors: Option<bool>,
        verbose: Option<bool>,
    ) -> (Vec<f64>, Option<Vec<NodeT>>) {
        let compute_predecessors = compute_predecessors.unwrap_or(true);
        let verbose = verbose.unwrap_or(true);
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
            return (distances, parents);
        }

        let mut nodes_to_explore: KeyedPriorityQueue<NodeT, Reverse<OrdFloat64>> =
            KeyedPriorityQueue::new();
        nodes_to_explore.push(src_node_id, Reverse(OrdFloat64(0.0)));

        let pb = get_loading_bar(verbose, "Computing Dijkstra", nodes_number);

        while !nodes_to_explore.is_empty() {
            // TODO: Use the distance that is returned from the queue instead of getting the one from the vector.
            let (closest_node_id, _) = nodes_to_explore.pop().unwrap();
            // We increase the loading bar by one.
            pb.inc(1);
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
        (distances, parents)
    }

    /// Returns vector of minimum paths distances and vector of nodes predecessors from given source node ID and optional destination node ID.
    ///
    /// # Arguments
    /// * `src_node_id`: NodeT - Node ID root of the tree of minimum paths.
    /// * `maybe_dst_node_id`: Option<NodeT> - Optional target destination. If provided, Dijkstra will stop upon reaching this node.
    /// * `maybe_dst_node_ids`: Option<RoaringBitmap> - Optional target destinations. If provided, Dijkstra will stop upon reaching all of these nodes.
    /// * `compute_predecessors`: Option<bool> - Whether to compute the vector of predecessors or to limit the allocation to exclusively the distances.
    /// * `verbose`: Option<bool> - Whether to show an indicative progress bar.
    ///
    /// # Raises
    /// * If the given source node ID does not exist in the current graph.
    /// * If the given optional destination node ID does not exist in the current graph.
    pub fn get_unweighted_dijkstra_from_node_ids(
        &self,
        src_node_id: NodeT,
        maybe_dst_node_id: Option<NodeT>,
        maybe_dst_node_ids: Option<RoaringBitmap>,
        compute_predecessors: Option<bool>,
        verbose: Option<bool>,
    ) -> Result<(Vec<NodeT>, Option<Vec<NodeT>>), String> {
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
        Ok(self.get_unchecked_unweighted_dijkstra_from_node_ids(
            src_node_id,
            maybe_dst_node_id,
            maybe_dst_node_ids,
            compute_predecessors,
            verbose,
        ))
    }

    /// Returns vector of minimum paths distances and vector of nodes predecessors from given source node ID and optional destination node ID.
    ///
    /// # Arguments
    /// * `src_node_id`: NodeT - Node ID root of the tree of minimum paths.
    /// * `maybe_dst_node_id`: Option<NodeT> - Optional target destination. If provided, Dijkstra will stop upon reaching this node.
    /// * `maybe_dst_node_ids`: Option<RoaringBitmap> - Optional target destinations. If provided, Dijkstra will stop upon reaching all of these nodes.
    /// * `compute_predecessors`: Option<bool> - Whether to compute the vector of predecessors or to limit the allocation to exclusively the distances.
    /// * `verbose`: Option<bool> - Whether to show an indicative progress bar.
    ///
    /// # Raises
    /// * If the weights are to be used and the graph does not have weights.
    /// * If the given source node ID does not exist in the current graph.
    /// * If the given optional destination node ID does not exist in the current graph.
    pub fn get_weighted_dijkstra_from_node_ids(
        &self,
        src_node_id: NodeT,
        maybe_dst_node_id: Option<NodeT>,
        maybe_dst_node_ids: Option<RoaringBitmap>,
        compute_predecessors: Option<bool>,
        verbose: Option<bool>,
    ) -> Result<(Vec<f64>, Option<Vec<NodeT>>), String> {
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
        Ok(self.get_unchecked_weighted_dijkstra_from_node_ids(
            src_node_id,
            maybe_dst_node_id,
            maybe_dst_node_ids,
            compute_predecessors,
            verbose,
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
                self.get_unchecked_unweighted_dijkstra_from_node_ids(
                    node_id,
                    None,
                    None,
                    Some(false),
                    Some(false),
                )
                .0
                .into_iter()
                .filter(|distance| !ignore_infinity || *distance != NodeT::MAX)
                .fold(0, NodeT::max)
            })
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
                self.get_unchecked_weighted_dijkstra_from_node_ids(
                    node_id,
                    None,
                    None,
                    Some(false),
                    Some(false),
                )
                .0
                .into_iter()
                .filter(|distance| !ignore_infinity || *distance != f64::INFINITY)
                .fold(f64::NEG_INFINITY, f64::max)
            })
            .reduce(|| f64::NEG_INFINITY, f64::max))
    }

    /// Returns vector of minimum paths distances and vector of nodes predecessors from given source node name and optional destination node name.
    ///
    /// # Arguments
    /// * `src_node_name`: &str - Node name root of the tree of minimum paths.
    /// * `maybe_dst_node_name`: Option<&str> - Optional target destination node name. If provided, Dijkstra will stop upon reaching this node.
    /// * `maybe_dst_node_names`: Option<Vec<&str>> - Optional target destination node names. If provided, Dijkstra will stop upon reaching all of these nodes.
    /// * `compute_predecessors`: Option<bool> - Whether to compute the vector of predecessors or to limit the allocation to exclusively the distances.
    /// * `verbose`: Option<bool> - Whether to show an indicative progress bar.
    ///
    /// # Raises
    /// * If the weights are to be used and the graph does not have weights.
    /// * If the given source node name does not exist in the current graph.
    /// * If the given optional destination node name does not exist in the current graph.
    pub fn get_unweighted_dijkstra_from_node_names(
        &self,
        src_node_name: &str,
        maybe_dst_node_name: Option<&str>,
        maybe_dst_node_names: Option<Vec<&str>>,
        compute_predecessors: Option<bool>,
        verbose: Option<bool>,
    ) -> Result<(Vec<NodeT>, Option<Vec<NodeT>>), String> {
        self.get_unweighted_dijkstra_from_node_ids(
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
            verbose,
        )
    }

    /// Returns vector of minimum paths distances and vector of nodes predecessors from given source node name and optional destination node name.
    ///
    /// # Arguments
    /// * `src_node_name`: &str - Node name root of the tree of minimum paths.
    /// * `maybe_dst_node_name`: Option<&str> - Optional target destination node name. If provided, Dijkstra will stop upon reaching this node.
    /// * `maybe_dst_node_names`: Option<Vec<&str>> - Optional target destination node names. If provided, Dijkstra will stop upon reaching all of these nodes.
    /// * `compute_predecessors`: Option<bool> - Whether to compute the vector of predecessors or to limit the allocation to exclusively the distances.
    /// * `verbose`: Option<bool> - Whether to show an indicative progress bar.
    ///
    /// # Raises
    /// * If the weights are to be used and the graph does not have weights.
    /// * If the given source node name does not exist in the current graph.
    /// * If the given optional destination node name does not exist in the current graph.
    pub fn get_weighted_dijkstra_from_node_names(
        &self,
        src_node_name: &str,
        maybe_dst_node_name: Option<&str>,
        maybe_dst_node_names: Option<Vec<&str>>,
        compute_predecessors: Option<bool>,
        verbose: Option<bool>,
    ) -> Result<(Vec<f64>, Option<Vec<NodeT>>), String> {
        self.get_weighted_dijkstra_from_node_ids(
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
            verbose,
        )
    }
}
