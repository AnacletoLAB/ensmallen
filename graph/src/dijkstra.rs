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
    /// * `use_graph_weights`: bool - Whether to use the graph weights as edge distances.
    /// * `compute_predecessors`: bool - Whether to compute the vector of predecessors or to limit the allocation to exclusively the distances.
    /// * `avoid_visits_above_root`: bool - Whether to avoid computing the paths that include nodes with ID lower than root. By default false.
    /// * `verbose`: bool - Whether to show an indicative progress bar.
    pub fn get_unchecked_dijkstra_from_node_ids(
        &self,
        src_node_id: NodeT,
        maybe_dst_node_id: Option<NodeT>,
        mut maybe_dst_node_ids: Option<RoaringBitmap>,
        use_graph_weights: Option<bool>,
        compute_predecessors: Option<bool>,
        avoid_visits_above_root: Option<bool>,
        verbose: Option<bool>,
    ) -> (Vec<f64>, Option<Vec<NodeT>>) {
        let use_graph_weights = use_graph_weights.unwrap_or(false);
        let compute_predecessors = compute_predecessors.unwrap_or(true);
        let avoid_visits_above_root = avoid_visits_above_root.unwrap_or(false);
        let verbose = verbose.unwrap_or(true);
        let nodes_number = self.get_nodes_number() as usize;
        let mut parents: Option<Vec<NodeT>> = if compute_predecessors {
            Some(vec![NodeT::MAX; nodes_number])
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
            // If the distance to the closest node is infinite, we have
            // completed the exploration of the nodes connected to the source node.
            if distances[closest_node_id as usize] == f64::INFINITY {
                break;
            }
            let weights_iterator: Box<dyn Iterator<Item = WeightT>> = if use_graph_weights {
                Box::new(self.iter_unchecked_edge_weights_from_source_node_id(closest_node_id))
            } else {
                Box::new(
                    self.iter_unchecked_edge_ids_from_source_node_id(closest_node_id)
                        .map(|_| 1.0),
                )
            };
            for (neighbour_node_id, weight) in self
                .iter_unchecked_neighbour_node_ids_from_source_node_id(closest_node_id)
                .zip(weights_iterator)
            {
                // If the user has asked to avoid to compute the paths that go
                // downwards from the current root node ID, which is a usefull
                // thing when computing for instance the diameter of the graph
                // since the path X is surely identified from the other direction
                // if the graph is undirected (the same DOES NOT work for directed graphs)
                // we can avoid a considerable amount of iterations.
                if avoid_visits_above_root && neighbour_node_id <= src_node_id {
                    continue;
                }
                // Since we are not taking in consideration the weights
                // all the node neighbours have distance 1.
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
    /// * `use_graph_weights`: Option<bool> - Whether to use the graph weights as edge distances.
    /// * `compute_predecessors`: Option<bool> - Whether to compute the vector of predecessors or to limit the allocation to exclusively the distances.
    /// * `avoid_visits_above_root`: bool - Whether to avoid computing the paths that include nodes with ID lower than root. By default false.
    /// * `verbose`: Option<bool> - Whether to show an indicative progress bar.
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
        use_graph_weights: Option<bool>,
        compute_predecessors: Option<bool>,
        avoid_visits_above_root: Option<bool>,
        verbose: Option<bool>,
    ) -> Result<(Vec<f64>, Option<Vec<NodeT>>), String> {
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
        // If the weights are to be used, we check that they actually are available
        // in the current graph instance.
        if use_graph_weights.unwrap_or(false) {
            self.must_have_edge_weights()?;
        }
        Ok(self.get_unchecked_dijkstra_from_node_ids(
            src_node_id,
            maybe_dst_node_id,
            maybe_dst_node_ids,
            use_graph_weights,
            compute_predecessors,
            avoid_visits_above_root,
            verbose,
        ))
    }

    /// Returns diameter of the graph.
    ///
    /// # Arguments
    /// * `use_graph_weights`: Option<bool> - Whether to use the graph weights as edge distances. By default, false.
    /// * `ignore_infinity`: Option<bool> - Whether to ignore infinite distances, which are present when in the graph exist multiple components.
    /// * `verbose`: Option<bool> - Whether to show a loading bar.
    ///
    /// # Raises
    /// * If the graph does not contain nodes.
    /// * If the graph does not have weights and weights have been requested.
    pub fn get_diameter(
        &self,
        use_graph_weights: Option<bool>,
        ignore_infinity: Option<bool>,
        verbose: Option<bool>,
    ) -> Result<f64, String> {
        self.must_have_nodes()?;
        let ignore_infinity = ignore_infinity.unwrap_or(true);
        let verbose = verbose.unwrap_or(true);
        // If the weights are to be used, we check that they actually are available
        // in the current graph instance.
        if use_graph_weights.unwrap_or(false) {
            self.must_have_edge_weights()?;
        }
        let pb = get_loading_bar(
            verbose,
            "Computing diameter",
            self.get_nodes_number() as usize,
        );
        Ok(self
            .par_iter_node_ids()
            .progress_with(pb)
            .map(|node_id| {
                self.get_unchecked_dijkstra_from_node_ids(
                    node_id,
                    None,
                    None,
                    use_graph_weights,
                    Some(false),
                    Some(!self.is_directed()),
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
    /// * `use_graph_weights`: Option<bool> - Whether to use the graph weights as edge distances.
    /// * `compute_predecessors`: Option<bool> - Whether to compute the vector of predecessors or to limit the allocation to exclusively the distances.
    /// * `avoid_visits_above_root`: bool - Whether to avoid computing the paths that include nodes with ID lower than root. By default false.
    /// * `verbose`: Option<bool> - Whether to show an indicative progress bar.
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
        use_graph_weights: Option<bool>,
        compute_predecessors: Option<bool>,
        avoid_visits_above_root: Option<bool>,
        verbose: Option<bool>,
    ) -> Result<(Vec<f64>, Option<Vec<NodeT>>), String> {
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
            use_graph_weights,
            compute_predecessors,
            avoid_visits_above_root,
            verbose,
        )
    }
}
