use super::*;
use atomic_float::AtomicF32;
use indicatif::ParallelProgressIterator;
use indicatif::ProgressIterator;
use itertools::Itertools;
use num_traits::pow::Pow;
use num_traits::Zero;
use parallel_frontier::prelude::*;
use rayon::iter::IndexedParallelIterator;
use rayon::iter::IntoParallelRefIterator;
use rayon::iter::{IntoParallelRefMutIterator, ParallelIterator};
use std::cell::SyncUnsafeCell;
use std::sync::atomic::Ordering;
use std::sync::atomic::{AtomicU32, AtomicU64};
use visited_rs::prelude::*;

#[inline(always)]
pub(crate) unsafe fn non_temporal_store<T>(ptr: &mut T, value: T) {
    #[cfg(feature = "nts")]
    std::intrinsics::nontemporal_store(ptr as *mut T, value);

    #[cfg(not(feature = "nts"))]
    std::ptr::write(ptr as *mut T, value)
}

impl Graph {
    /// Returns iterator over the unweighted degree centrality for all nodes.
    pub fn iter_degree_centrality(&self) -> Result<impl Iterator<Item = f32> + '_> {
        self.must_have_edges()?;

        let max_degree = unsafe { self.get_unchecked_maximum_node_degree() as f32 };
        Ok(self
            .iter_node_degrees()
            .map(move |degree| degree as f32 / max_degree))
    }

    /// Returns parallel iterator over the unweighted degree centrality for all nodes.
    pub fn par_iter_degree_centrality(
        &self,
    ) -> Result<impl IndexedParallelIterator<Item = f32> + '_> {
        self.must_have_edges()?;

        let max_degree = unsafe { self.get_unchecked_maximum_node_degree() as f32 };
        Ok(self
            .par_iter_node_degrees()
            .map(move |degree| degree as f32 / max_degree))
    }

    /// Returns iterator over the weighted degree centrality for all nodes.
    pub fn par_iter_weighted_degree_centrality(
        &self,
    ) -> Result<impl IndexedParallelIterator<Item = f32> + '_> {
        self.must_have_edges()?;
        self.must_have_positive_edge_weights()?;

        let weighted_max_degree = self.get_weighted_maximum_node_degree().clone()? as f32;
        Ok(self
            .par_iter_weighted_node_degrees()?
            .map(move |degree| degree as f32 / weighted_max_degree))
    }

    /// Returns vector of unweighted degree centrality for all nodes.
    pub fn get_degree_centrality(&self) -> Result<Vec<f32>> {
        let mut degree_centralities = vec![0.0; self.get_number_of_nodes() as usize];
        self.par_iter_degree_centrality()?
            .collect_into_vec(&mut degree_centralities);
        Ok(degree_centralities)
    }

    /// Returns vector of weighted degree centrality for all nodes.
    pub fn get_weighted_degree_centrality(&self) -> Result<Vec<f32>> {
        let mut weighted_degree_centralities = vec![0.0; self.get_number_of_nodes() as usize];
        self.par_iter_weighted_degree_centrality()?
            .collect_into_vec(&mut weighted_degree_centralities);
        Ok(weighted_degree_centralities)
    }

    /// Return closeness centrality of the requested node.
    ///
    /// If the given node ID does not exist in the current graph the method
    /// will panic.
    ///
    /// # Arguments
    /// * `node_id`: NodeT - The node ID whose closeness centrality is to be computed.
    /// * `verbose`: Option<bool> - Whether to show an indicative progress bar.
    ///
    /// # References
    /// The metric is described in [Centrality in Social Networks by Freeman](https://www.bebr.ufl.edu/sites/default/files/Centrality%20in%20Social%20Networks.pdf)
    ///
    /// # Safety
    /// If the given node ID does not exist in the graph the method will panic.
    pub unsafe fn get_unchecked_closeness_centrality_from_node_id(&self, node_id: NodeT) -> f32 {
        if self.is_unchecked_disconnected_node_from_node_id(node_id) {
            return 0.0;
        }
        1.0 / self
            .get_unchecked_breadth_first_search_from_node_id(node_id, None, None, None)
            .into_iter_finite_distances()
            .sum::<NodeT>() as f32
    }

    /// Return closeness centrality of the requested node.
    ///
    /// If the given node ID does not exist in the current graph the method
    /// will panic.
    ///
    /// # Arguments
    /// * `node_id`: NodeT - The node ID whose closeness centrality is to be computed.
    /// * `use_edge_weights_as_probabilities`: bool - Whether to treat the edge weights as probabilities.
    ///
    /// # References
    /// The metric is described in [Centrality in Social Networks by Freeman](https://www.bebr.ufl.edu/sites/default/files/Centrality%20in%20Social%20Networks.pdf)
    ///
    /// # Implementative notes
    /// When the user provides the information that the graph contains weights
    /// representing probabilities (which is impossible to detect automatically)
    /// we return instead of `1 / total_distance` directly the total distance,
    /// as `1 / total_distance` when the weights represent a distance basically
    /// represent the probability to sample all those paths. This value is
    /// already captured by the product of the probabilities, which composes
    /// the `total_distance` value when it is known that the graph is composed
    /// of probabilities.
    ///
    /// # Safety
    /// If the given node ID does not exist in the graph the method will panic.
    pub unsafe fn get_unchecked_weighted_closeness_centrality_from_node_id(
        &self,
        node_id: NodeT,
        use_edge_weights_as_probabilities: bool,
    ) -> f32 {
        if self.is_unchecked_disconnected_node_from_node_id(node_id) {
            return 0.0;
        }
        let dijkstra = self.get_unchecked_dijkstra_from_node_id(
            node_id,
            None,
            None,
            Some(false),
            None,
            Some(use_edge_weights_as_probabilities),
        );
        1.0 / if use_edge_weights_as_probabilities {
            dijkstra.get_log_total_distance()
        } else {
            dijkstra.get_total_distance()
        }
    }

    /// Return parallel iterator over closeness centrality for all nodes.
    ///
    /// # References
    /// The metric is described in [Centrality in Social Networks by Freeman](https://www.bebr.ufl.edu/sites/default/files/Centrality%20in%20Social%20Networks.pdf)
    pub fn get_closeness_centrality(&self) -> Vec<f32> {
        let visited: SyncUnsafeCell<Vec<Visited<u16>>> = SyncUnsafeCell::from(
            (0..rayon::current_num_threads().max(1))
                .map(|_| Visited::zero(self.get_number_of_nodes() as usize))
                .collect::<Vec<Visited<u16>>>(),
        );
        let mut centralities = vec![0.0; self.get_number_of_nodes() as usize];

        centralities
            .par_iter_mut()
            .enumerate()
            .for_each(move |(root, centrality)| {
                let mut current_depth = 0;
                let mut total_distance = 0;
                let thread_id = rayon::current_thread_index().unwrap_or(0);
                let mut frontier = vec![root as NodeT];
                let visited = unsafe { &mut (*visited.get())[thread_id] };
                visited.set_visited(root);
                while !frontier.is_empty() {
                    current_depth += 1;
                    frontier = frontier
                        .into_iter()
                        .flat_map(|src| unsafe {
                            self.iter_unchecked_neighbour_node_ids_from_source_node_id(src)
                        })
                        .filter(|&dst| !visited.set_and_get_visited(dst))
                        .collect::<Vec<NodeT>>();

                    total_distance += current_depth * frontier.len();
                }
                if !total_distance.is_zero() {
                    *centrality = 1.0 / total_distance as f32;
                }
                visited.clear();
            });
        centralities
    }

    /// Return parallel iterator over closeness centrality for all nodes.
    ///
    /// # Arguments
    /// * `use_edge_weights_as_probabilities`: bool - Whether to treat the edge weights as probabilities.
    /// * `verbose`: Option<bool> - Whether to show an indicative progress bar.
    ///
    /// # References
    /// The metric is described in [Centrality in Social Networks by Freeman](https://www.bebr.ufl.edu/sites/default/files/Centrality%20in%20Social%20Networks.pdf)
    ///
    /// # Implementative notes
    /// When the user provides the information that the graph contains weights
    /// representing probabilities (which is impossible to detect automatically)
    /// we return instead of `1 / total_distance` directly the total distance,
    /// as `1 / total_distance` when the weights represent a distance basically
    /// represent the probability to sample all those paths. This value is
    /// already captured by the product of the probabilities, which composes
    /// the `total_distance` value when it is known that the graph is composed
    /// of probabilities.
    ///
    /// # References
    /// The metric is described in [Centrality in Social Networks by Freeman](https://www.bebr.ufl.edu/sites/default/files/Centrality%20in%20Social%20Networks.pdf)
    ///
    /// # Raises
    /// * If the graph does not have weights.
    /// * If the graph contains negative weights.
    /// * If the user has asked for the weights to be treated as probabilities but the weights are not between 0 and 1.
    pub fn par_iter_weighted_closeness_centrality(
        &self,
        use_edge_weights_as_probabilities: Option<bool>,
        verbose: Option<bool>,
    ) -> Result<impl ParallelIterator<Item = f32> + '_> {
        self.must_have_positive_edge_weights()?;
        let use_edge_weights_as_probabilities = use_edge_weights_as_probabilities.unwrap_or(false);
        if use_edge_weights_as_probabilities {
            self.must_have_edge_weights_representing_probabilities()?;
        }
        let verbose = verbose.unwrap_or(true);
        let pb = get_loading_bar(
            verbose,
            "Computing closeness centrality",
            self.get_number_of_nodes() as usize,
        );
        Ok(self
            .par_iter_node_ids()
            .progress_with(pb)
            .map(move |node_id| unsafe {
                self.get_unchecked_weighted_closeness_centrality_from_node_id(
                    node_id,
                    use_edge_weights_as_probabilities,
                )
            }))
    }

    /// Return closeness centrality for all nodes.
    ///
    /// # Arguments
    /// * `use_edge_weights_as_probabilities`: bool - Whether to treat the edge weights as probabilities.
    /// * `verbose`: Option<bool> - Whether to show an indicative progress bar.
    ///
    /// # References
    /// The metric is described in [Centrality in Social Networks by Freeman](https://www.bebr.ufl.edu/sites/default/files/Centrality%20in%20Social%20Networks.pdf)
    ///
    /// # Implementative notes
    /// When the user provides the information that the graph contains weights
    /// representing probabilities (which is impossible to detect automatically)
    /// we return instead of `1 / total_distance` directly the total distance,
    /// as `1 / total_distance` when the weights represent a distance basically
    /// represent the probability to sample all those paths. This value is
    /// already captured by the product of the probabilities, which composes
    /// the `total_distance` value when it is known that the graph is composed
    /// of probabilities.
    ///
    /// # Raises
    /// * If the graph does not have weights.
    /// * If the graph contains negative weights.
    /// * If the user has asked for the weights to be treated as probabilities but the weights are not between 0 and 1.
    pub fn get_weighted_closeness_centrality(
        &self,
        use_edge_weights_as_probabilities: Option<bool>,
        verbose: Option<bool>,
    ) -> Result<Vec<f32>> {
        self.par_iter_weighted_closeness_centrality(use_edge_weights_as_probabilities, verbose)
            .map(|x| x.collect())
    }

    /// Return harmonic centrality of the requested node.
    ///
    /// If the given node ID does not exist in the current graph the method
    /// will panic.
    ///
    /// # Arguments
    /// * `node_id`: NodeT - The node ID whose harmonic centrality is to be computed.
    ///
    /// # References
    /// The metric is described in [Axioms for centrality by Boldi and Vigna](https://www.tandfonline.com/doi/abs/10.1080/15427951.2013.865686).
    ///
    /// # Safety
    /// If the given node ID does not exist in the graph the method will panic.
    pub unsafe fn get_unchecked_harmonic_centrality_from_node_id(&self, node_id: NodeT) -> f32 {
        self.get_unchecked_breadth_first_search_from_node_id(node_id, None, None, None)
            .into_iter_finite_distances()
            .map(|distance| {
                if distance != 0 {
                    1.0 / distance as f32
                } else {
                    0.0
                }
            })
            .sum()
    }

    /// Return harmonic centrality of the requested node.
    ///
    /// If the given node ID does not exist in the current graph the method
    /// will panic.
    ///
    /// # Arguments
    /// * `node_id`: NodeT - The node ID whose harmonic centrality is to be computed.
    /// * `use_edge_weights_as_probabilities`: bool - Whether to treat the edge weights as probabilities.
    ///
    /// # References
    /// The metric is described in [Axioms for centrality by Boldi and Vigna](https://www.tandfonline.com/doi/abs/10.1080/15427951.2013.865686).
    ///
    /// # Safety
    /// If the given node ID does not exist in the graph the method will panic.
    pub unsafe fn get_unchecked_weighted_harmonic_centrality_from_node_id(
        &self,
        node_id: NodeT,
        use_edge_weights_as_probabilities: bool,
    ) -> f32 {
        self.get_unchecked_dijkstra_from_node_id(
            node_id,
            None,
            None,
            Some(false),
            None,
            Some(use_edge_weights_as_probabilities),
        )
        .total_harmonic_distance
    }

    /// Return vector of harmonic centrality for all nodes.
    ///
    /// # References
    /// The metric is described in [Axioms for centrality by Boldi and Vigna](https://www.tandfonline.com/doi/abs/10.1080/15427951.2013.865686).
    ///
    pub fn get_harmonic_centrality(&self) -> Vec<f32> {
        let visited: SyncUnsafeCell<Vec<Visited<u16>>> = SyncUnsafeCell::from(
            (0..rayon::current_num_threads().max(1))
                .map(|_| Visited::zero(self.get_number_of_nodes() as usize))
                .collect::<Vec<Visited<u16>>>(),
        );
        let mut centralities = vec![0.0; self.get_number_of_nodes() as usize];

        centralities
            .par_iter_mut()
            .enumerate()
            .for_each(move |(root, centrality)| {
                let mut current_depth = 0;
                let mut total_reciprocal_distance: f32 = 0.0;
                let thread_id = rayon::current_thread_index().unwrap_or(0);
                let mut frontier = vec![root as NodeT];
                let visited = unsafe { &mut (*visited.get())[thread_id] };
                visited.set_visited(root);
                while !frontier.is_empty() {
                    current_depth += 1;
                    frontier = frontier
                        .into_iter()
                        .flat_map(|src| unsafe {
                            self.iter_unchecked_neighbour_node_ids_from_source_node_id(src)
                        })
                        .filter(|&dst| !visited.set_and_get_visited(dst))
                        .collect::<Vec<NodeT>>();

                    total_reciprocal_distance +=
                        (current_depth as f32).recip() * (frontier.len() as f32);
                }
                *centrality = total_reciprocal_distance;
                visited.clear();
            });
        centralities
    }

    /// Return parallel iterator over harmonic centrality for all nodes.
    ///
    /// # Arguments
    /// * `use_edge_weights_as_probabilities`: Option<bool> - Whether to treat the edge weights as probabilities.
    /// * `verbose`: Option<bool> - Whether to show an indicative progress bar.
    ///
    /// # References
    /// The metric is described in [Axioms for centrality by Boldi and Vigna](https://www.tandfonline.com/doi/abs/10.1080/15427951.2013.865686).
    ///
    /// # Raises
    /// * If the graph does not have weights.
    /// * If the graph contains negative weights.
    /// * If the user has asked for the weights to be treated as probabilities but the weights are not between 0 and 1.
    pub fn par_iter_weighted_harmonic_centrality(
        &self,
        use_edge_weights_as_probabilities: Option<bool>,
        verbose: Option<bool>,
    ) -> Result<impl ParallelIterator<Item = f32> + '_> {
        self.must_have_positive_edge_weights()?;
        let use_edge_weights_as_probabilities = use_edge_weights_as_probabilities.unwrap_or(false);
        if use_edge_weights_as_probabilities {
            self.must_have_edge_weights_representing_probabilities()?;
        }

        let verbose = verbose.unwrap_or(true);
        let pb = get_loading_bar(
            verbose,
            "Computing harmonic centrality",
            self.get_number_of_nodes() as usize,
        );
        Ok(self
            .par_iter_node_ids()
            .progress_with(pb)
            .map(move |node_id| unsafe {
                self.get_unchecked_weighted_harmonic_centrality_from_node_id(
                    node_id,
                    use_edge_weights_as_probabilities,
                )
            }))
    }

    /// Return harmonic centrality for all nodes.
    ///
    /// # Arguments
    /// * `use_edge_weights_as_probabilities`: Option<bool> - Whether to treat the edge weights as probabilities.
    /// * `verbose`: Option<bool> - Whether to show an indicative progress bar.
    ///
    /// # References
    /// The metric is described in [Axioms for centrality by Boldi and Vigna](https://www.tandfonline.com/doi/abs/10.1080/15427951.2013.865686).
    pub fn get_weighted_harmonic_centrality(
        &self,
        use_edge_weights_as_probabilities: Option<bool>,
        verbose: Option<bool>,
    ) -> Result<Vec<f32>> {
        self.par_iter_weighted_harmonic_centrality(use_edge_weights_as_probabilities, verbose)
            .map(|x| x.collect())
    }

    /// Returns vector of stress centrality for all nodes.
    ///
    /// # Arguments
    /// * `verbose`: Option<bool> - Whether to show a loading bar while computing the stress centrality. By default, true.
    ///
    /// # References
    /// The algorithm is implemented as described in [Parallel Algorithms for Evaluating Centrality Indices in Real-World Networks](https://ieeexplore.ieee.org/abstract/document/1690659), by Bader et al.
    ///
    /// # Raises
    /// * If the graph is a multigraph.
    pub fn get_stress_centrality(&self, verbose: Option<bool>) -> Result<Vec<f32>> {
        self.must_not_be_multigraph()?;
        if !self.has_nodes() {
            return Ok(Vec::new());
        }

        let pb = get_loading_bar(
            verbose.unwrap_or(true),
            "Computing betweennes centralities",
            self.get_number_of_nodes() as usize,
        );

        // We allocate the vector we are going to use to store
        // the computed node betwenness centralities.
        // Since we are going to add the portion of different
        // betwenness centralities scores across different threads,
        // we need to make use to atomics.
        let mut centralities: Vec<f32> = vec![0.0; self.get_number_of_nodes() as usize];

        // Similarly, since we are going to extend the successors of a node
        // from multiple threads at once, and we want to do it in a sync-free
        // manner, we employ the `Frontier` object, which simply allocates a vector
        // for each thread.
        let mut successors: Vec<u32> = vec![0; self.get_number_of_directed_edges() as usize];
        let mut successor_counts: Vec<u32> = vec![0; self.get_number_of_nodes() as usize];

        let shortest_path_counts: Vec<AtomicU64> = (0..self.get_number_of_nodes() as usize)
            .map(|_| AtomicU64::default())
            .collect();

        const UNVISITED: u8 = u8::MAX;
        const VISITED: u8 = 0;
        const JUST_VISITED: u8 = 1;
        let mut visited_status: Vec<u8> = vec![UNVISITED; self.get_number_of_nodes() as usize];

        let mut dependencies: Vec<f32> = vec![0.0; self.get_number_of_nodes() as usize];

        let mut frontiers: Vec<Frontier<NodeT>> = vec![Frontier::default(), Frontier::default()];

        self.iter_node_ids().progress_with(pb).for_each(|root| {
            // First, we prepare the data-structrues for this iteration.

            // We begin by resetting the shortest path counts to zero.
            shortest_path_counts.par_iter().for_each(|count| {
                count.store(0, Ordering::Relaxed);
            });

            // We set the number of paths from root as equal to one.
            shortest_path_counts[root as usize].store(1, Ordering::Relaxed);

            // We set the number of paths from root as equal to one.
            visited_status[root as usize] = VISITED;

            // We clear the first frontier and insert the root node.
            frontiers[0].clear();
            frontiers[0].push(root);

            let mut current_depth = 0;

            loop {
                current_depth += 1;
                // Every time the frontiers has become too small
                // the current depth, we need to add another frontier
                // layer that we will be reusing.
                if frontiers.len() < 1 + current_depth {
                    frontiers.push(Frontier::default());
                }

                let shared_visited_status = ThreadDataRaceAware::new(&mut visited_status);
                let shared_successor_counts = ThreadDataRaceAware::new(&mut successor_counts);
                let shared_successors = ThreadDataRaceAware::new(&mut successors);
                frontiers[current_depth - 1]
                    .par_iter()
                    .for_each(|&src| unsafe {
                        let source_paths =
                            shortest_path_counts[src as usize].load(Ordering::Relaxed);
                        let current_number_of_successors =
                            &mut (*shared_successor_counts.get())[src as usize];
                        let mut number_of_successors = *current_number_of_successors;
                        let mut offset = self
                            .edges
                            .get_unchecked_minmax_edge_ids_from_source_node_id(src)
                            .0 as usize
                            + number_of_successors as usize;
                        self.iter_unchecked_neighbour_node_ids_from_source_node_id(src)
                            .for_each(|dst: u32| {
                                let status_ref = &mut (*shared_visited_status.get())[dst as usize];
                                let status = *status_ref;

                                // If the node was not yet visited
                                if status == UNVISITED {
                                    // We push this node to the new frontier to be visited.
                                    non_temporal_store(status_ref, JUST_VISITED);
                                }

                                // We now handle the updates of the neighbourhoods.
                                // NOTE: we CANNOT do this in the previous loop because of
                                // possible collisions with other parallel iterations. For
                                // instance a node `X` may have a neighbour `K` shared with
                                // another node in the current frontier `Y`. Both the neighbour
                                // exploration of `K` starting from `X` and `Y` has to be considered
                                // for the following if statement.
                                if status == JUST_VISITED || status == UNVISITED {
                                    // We increase the degree of the successors
                                    // of this node by one, and we get the previous
                                    // number of successors.
                                    shortest_path_counts[dst as usize]
                                        .fetch_add(source_paths, Ordering::Relaxed);
                                    non_temporal_store(
                                        &mut (*shared_successors.get())[offset],
                                        dst,
                                    );
                                    offset += 1;
                                    number_of_successors += 1;
                                }
                            });
                        non_temporal_store(current_number_of_successors, number_of_successors);
                    });

                #[cfg(feature = "nts")]
                sfence();

                frontiers[current_depth].clear();

                visited_status
                    .par_iter_mut()
                    .enumerate()
                    .filter(|(_, distance)| **distance == JUST_VISITED)
                    .for_each(|(node_id, distance)| {
                        *distance = VISITED;
                        frontiers[current_depth].push(node_id as NodeT);
                    });

                if frontiers[current_depth].is_empty() {
                    break;
                }
            }

            successor_counts[root as usize] = 0;
            visited_status[root as usize] = UNVISITED;
            let shared_visited_status = ThreadDataRaceAware::new(&mut visited_status);
            let shared_dependencies = ThreadDataRaceAware::new(&mut dependencies);
            let shared_centralities = ThreadDataRaceAware::new(&mut centralities);
            let shared_successor_counts = ThreadDataRaceAware::new(&mut successor_counts);

            frontiers[..current_depth]
                .iter()
                .enumerate()
                .skip(1)
                .rev()
                .for_each(|(depth, frontier)| {
                    frontier.par_iter().copied().for_each(|src| {
                        let path_counts =
                            shortest_path_counts[src as usize].load(Ordering::Relaxed) as f32;
                        unsafe {
                            (*shared_visited_status.get())[src as usize] = UNVISITED;
                        }
                        let offset = unsafe {
                            self.edges
                                .get_unchecked_minmax_edge_ids_from_source_node_id(src)
                                .0
                        };
                        let number_of_successors =
                            unsafe { &mut (*shared_successor_counts.get())[src as usize] };
                        let dependency: f32 = path_counts
                            * if current_depth == depth + 1 {
                                // If this is the leafs, these nodes do not have any dependency.
                                *number_of_successors as f32
                            } else {
                                // Otherwise, we need to access the dependencies.
                                // Note that all dependencies are weighted by their
                                // own shortest path counts.
                                successors[offset as usize
                                    ..(offset as usize + *number_of_successors as usize)]
                                    .iter()
                                    .map(|&dst| {
                                        1.0 + unsafe { (*shared_dependencies.get())[dst as usize] }
                                    })
                                    .sum::<f32>()
                            };
                        *number_of_successors = 0;
                        // Since we are always setting the dependency of the previous
                        // layer before reading them, we do not need to reset them.
                        unsafe { (*shared_dependencies.get())[src as usize] = dependency };
                        // Similarly, since the node `src` by design can only appear once
                        // in the frontier, we do not need an atomic check using fetch-add.
                        unsafe { (*shared_centralities.get())[src as usize] += dependency };
                    });
                });
        });

        if !self.is_directed() {
            centralities.par_iter_mut().for_each(|value| {
                *value /= 2.0;
            });
        }

        Ok(centralities)
    }

    /// Returns vector of betweenness centrality for all nodes.
    ///
    /// # Arguments
    /// * `edges_normalization`: Option<bool> - Whether to normalize the values by the number of edges of the complete graph. By default, false.
    /// * `min_max_normalization`: Option<bool> - Whether to normalize the values between 0 and 1. By default, false.
    /// * `verbose`: Option<bool> - Whether to show a loading bar while computing the betweenness centrality. By default, true.
    ///
    /// # References
    /// The algorithm is implemented as described in [Parallel Algorithms for Evaluating Centrality Indices in Real-World Networks](https://ieeexplore.ieee.org/abstract/document/1690659), by Bader et al.
    ///
    /// # Raises
    /// * If the graph is a multigraph.
    pub fn get_betweenness_centrality(
        &self,
        edges_normalization: Option<bool>,
        min_max_normalization: Option<bool>,
        verbose: Option<bool>,
    ) -> Result<Vec<f32>> {
        self.must_not_be_multigraph()?;
        if !self.has_nodes() {
            return Ok(Vec::new());
        }
        let edges_normalization = edges_normalization.unwrap_or(false);
        let min_max_normalization = min_max_normalization.unwrap_or(false);

        let pb = get_loading_bar(
            verbose.unwrap_or(true),
            "Computing betweennes centralities",
            self.get_number_of_nodes() as usize,
        );

        // We allocate the vector we are going to use to store
        // the computed node betwenness centralities.
        // Since we are going to add the portion of different
        // betwenness centralities scores across different threads,
        // we need to make use to atomics.
        let mut centralities: Vec<f32> = vec![0.0; self.get_number_of_nodes() as usize];

        // Similarly, since we are going to extend the successors of a node
        // from multiple threads at once, and we want to do it in a sync-free
        // manner, we employ the `Frontier` object, which simply allocates a vector
        // for each thread.
        let mut successors: Vec<u32> = vec![0; self.get_number_of_directed_edges() as usize];
        let mut successor_counts: Vec<u32> = vec![0; self.get_number_of_nodes() as usize];

        let shortest_path_counts: Vec<AtomicU32> = (0..self.get_number_of_nodes() as usize)
            .map(|_| AtomicU32::default())
            .collect();

        const UNVISITED: u8 = u8::MAX;
        const VISITED: u8 = 0;
        const JUST_VISITED: u8 = 1;
        let mut visited_status: Vec<u8> = vec![UNVISITED; self.get_number_of_nodes() as usize];

        let mut dependencies: Vec<f32> = vec![0.0; self.get_number_of_nodes() as usize];

        let mut frontiers: Vec<Frontier<NodeT>> = vec![Frontier::default(), Frontier::default()];

        self.iter_node_ids().progress_with(pb).for_each(|root| {
            // First, we prepare the data-structrues for this iteration.

            // We begin by resetting the shortest path counts to zero.
            shortest_path_counts.par_iter().for_each(|count| {
                count.store(0, Ordering::Relaxed);
            });

            // We set the number of paths from root as equal to one.
            shortest_path_counts[root as usize].store(1, Ordering::Relaxed);

            // We set the number of paths from root as equal to one.
            visited_status[root as usize] = VISITED;

            // We clear the first frontier and insert the root node.
            frontiers[0].clear();
            frontiers[0].push(root);

            let mut current_depth = 0;

            loop {
                current_depth += 1;
                // Every time the frontiers has become too small
                // the current depth, we need to add another frontier
                // layer that we will be reusing.
                if frontiers.len() < 1 + current_depth {
                    frontiers.push(Frontier::default());
                }

                let shared_visited_status = ThreadDataRaceAware::new(&mut visited_status);
                let shared_successor_counts = ThreadDataRaceAware::new(&mut successor_counts);
                let shared_successors = ThreadDataRaceAware::new(&mut successors);
                frontiers[current_depth - 1]
                    .par_iter()
                    .for_each(|&src| unsafe {
                        let source_paths =
                            shortest_path_counts[src as usize].load(Ordering::Relaxed);
                        let current_number_of_successors =
                            &mut (*shared_successor_counts.get())[src as usize];
                        let mut number_of_successors = *current_number_of_successors;
                        let mut offset = self
                            .edges
                            .get_unchecked_minmax_edge_ids_from_source_node_id(src)
                            .0 as usize
                            + number_of_successors as usize;
                        self.iter_unchecked_neighbour_node_ids_from_source_node_id(src)
                            .for_each(|dst: u32| {
                                let status_ref = &mut (*shared_visited_status.get())[dst as usize];
                                let status = *status_ref;

                                // If the node was not yet visited
                                if status == UNVISITED {
                                    // We push this node to the new frontier to be visited.
                                    non_temporal_store(status_ref, JUST_VISITED);
                                }

                                // We now handle the updates of the neighbourhoods.
                                // NOTE: we CANNOT do this in the previous loop because of
                                // possible collisions with other parallel iterations. For
                                // instance a node `X` may have a neighbour `K` shared with
                                // another node in the current frontier `Y`. Both the neighbour
                                // exploration of `K` starting from `X` and `Y` has to be considered
                                // for the following if statement.
                                if status == JUST_VISITED || status == UNVISITED {
                                    // We increase the degree of the successors
                                    // of this node by one, and we get the previous
                                    // number of successors.
                                    shortest_path_counts[dst as usize]
                                        .fetch_add(source_paths, Ordering::Relaxed);
                                    non_temporal_store(
                                        &mut (*shared_successors.get())[offset],
                                        dst,
                                    );
                                    offset += 1;
                                    number_of_successors += 1;
                                }
                            });
                        non_temporal_store(current_number_of_successors, number_of_successors);
                    });

                #[cfg(feature = "nts")]
                sfence();

                frontiers[current_depth].clear();

                visited_status
                    .par_iter_mut()
                    .enumerate()
                    .filter(|(_, distance)| **distance == JUST_VISITED)
                    .for_each(|(node_id, distance)| {
                        *distance = VISITED;
                        frontiers[current_depth].push(node_id as NodeT);
                    });

                if frontiers[current_depth].is_empty() {
                    break;
                }
            }

            successor_counts[root as usize] = 0;
            visited_status[root as usize] = UNVISITED;
            let shared_visited_status = ThreadDataRaceAware::new(&mut visited_status);
            let shared_dependencies = ThreadDataRaceAware::new(&mut dependencies);
            let shared_centralities = ThreadDataRaceAware::new(&mut centralities);
            let shared_successor_counts = ThreadDataRaceAware::new(&mut successor_counts);

            frontiers[..current_depth]
                .iter()
                .enumerate()
                .skip(1)
                .rev()
                .for_each(|(depth, frontier)| {
                    frontier.par_iter().copied().for_each(|src| {
                        let path_counts =
                            shortest_path_counts[src as usize].load(Ordering::Relaxed) as f32;
                        unsafe {
                            (*shared_visited_status.get())[src as usize] = UNVISITED;
                        }
                        let offset = unsafe {
                            self.edges
                                .get_unchecked_minmax_edge_ids_from_source_node_id(src)
                                .0
                        };
                        let number_of_successors =
                            unsafe { &mut (*shared_successor_counts.get())[src as usize] };
                        let dependency: f32 = path_counts
                            * if current_depth == depth + 1 {
                                // If this is the leafs, these nodes do not have any dependency.
                                successors[offset as usize
                                    ..(offset as usize + *number_of_successors as usize)]
                                    .iter()
                                    .map(|&dst| {
                                        1.0 / shortest_path_counts[dst as usize]
                                            .load(Ordering::Relaxed)
                                            as f32
                                    })
                                    .sum::<f32>()
                            } else {
                                // Otherwise, we need to access the dependencies.
                                // Note that all dependencies are weighted by their
                                // own shortest path counts.
                                successors[offset as usize
                                    ..(offset as usize + *number_of_successors as usize)]
                                    .iter()
                                    .map(|&dst| {
                                        (1.0 + unsafe {
                                            (*shared_dependencies.get())[dst as usize]
                                        }) / shortest_path_counts[dst as usize]
                                            .load(Ordering::Relaxed)
                                            as f32
                                    })
                                    .sum::<f32>()
                            };
                        *number_of_successors = 0;
                        // Since we are always setting the dependency of the previous
                        // layer before reading them, we do not need to reset them.
                        unsafe { (*shared_dependencies.get())[src as usize] = dependency };
                        // Similarly, since the node `src` by design can only appear once
                        // in the frontier, we do not need an atomic check using fetch-add.
                        unsafe { (*shared_centralities.get())[src as usize] += dependency };
                    });
                });
        });

        if !self.is_directed() {
            centralities.par_iter_mut().for_each(|value| {
                *value /= 2.0;
            });
        }

        if min_max_normalization {
            let (min_centrality, max_centrality) =
                centralities.iter().copied().minmax().into_option().unwrap();
            let delta = max_centrality - min_centrality;
            centralities.par_iter_mut().for_each(|value| {
                *value = (*value - min_centrality) / delta;
            });
        } else if edges_normalization {
            let denominator = (self.get_number_of_nodes() as f32 - 1.0)
                * (self.get_number_of_nodes() as f32 - 2.0)
                / if self.is_directed() { 1.0 } else { 2.0 };
            centralities.par_iter_mut().for_each(|value| {
                *value /= denominator;
            });
        }
        Ok(centralities)
    }

    #[no_binding]
    /// Returns the unweighted pair dependency from the given node ID.
    ///
    /// # Arguments
    /// `node_id`: NodeT - The node ID for which to compute the approximated betweenness centrality.
    /// `sssp`: &ShortestPathsResultBFS - Reference to shortest paths object.
    ///
    /// # Returns
    /// The pair dependency from the given graphs.
    pub fn get_pair_dependency_from_node_id(
        &self,
        node_id: NodeT,
        sssp: &ShortestPathsResultBFS,
    ) -> Result<f32> {
        self.validate_node_id(node_id)?;
        let number_of_shortest_paths =
            sssp.get_number_of_shortest_paths_from_node_id(node_id)? as f32;
        Ok(sssp
            .get_successors_from_node_id(node_id)?
            .into_iter()
            .map(|successor_node_id| {
                (1.0 + self
                    .get_pair_dependency_from_node_id(successor_node_id, sssp)
                    .unwrap())
                    * number_of_shortest_paths
                    / sssp
                        .get_number_of_shortest_paths_from_node_id(successor_node_id)
                        .unwrap() as f32
            })
            .sum::<f32>())
    }

    #[no_binding]
    /// Returns the weighted pair dependency from the given node ID.
    ///
    /// # Arguments
    /// `node_id`: NodeT - The node ID for which to compute the approximated betweenness centrality.
    /// `sssp`: &ShortestPathsDjkstra - Reference to dijkstra shortest paths object.
    ///
    /// # Returns
    /// The pair dependency from the given graphs.
    pub fn get_weighted_pair_dependency_from_node_id(
        &self,
        node_id: NodeT,
        sssp: &ShortestPathsDjkstra,
    ) -> Result<f32> {
        self.validate_node_id(node_id)?;
        let number_of_shortest_paths =
            sssp.get_number_of_shortest_paths_from_node_id(node_id)? as f32;
        Ok(sssp
            .get_successors_from_node_id(node_id)?
            .into_iter()
            .map(|successor_node_id| {
                (1.0 + self
                    .get_weighted_pair_dependency_from_node_id(successor_node_id, sssp)
                    .unwrap())
                    * number_of_shortest_paths
                    / sssp
                        .get_number_of_shortest_paths_from_node_id(successor_node_id)
                        .unwrap() as f32
            })
            .sum::<f32>())
    }

    /// Returns the unweighted approximated betweenness centrality of the given node id.
    ///
    /// # Arguments
    /// * `node_id`: NodeT - The node ID for which to compute the approximated betweenness centrality.
    /// * `constant`: Option<f32> - The constant factor to use to regulate the sampling. By default 2.0. It must be greater or equal than 2.0.
    /// * `maximum_samples_number`: Option<f32> - The maximum number of samples to sample. By default `nodes_number / 20`, as suggested in the paper.
    /// * `random_state`: Option<u64> - The random state to use for the sampling. By default 42.
    ///
    /// # Raises
    /// * If the provided node ID does not exist in the current graph instance.
    ///
    /// # References
    /// This method is an implementation of the [Approximating Betweenness Centrality](https://link.springer.com/chapter/10.1007/978-3-540-77004-6_10)
    /// work by David Bader et al.
    ///
    /// The algorithm repeatedly samples a vertex \(v_i \in V\),
    /// then performs single-source shortest paths from \(v_i\)
    /// and maintain a running sum \(S\) of the dependency scores \(\delta_{v_i∗}(v)\).
    /// Sample nodes until \(S\) is greater than cn for some constant \(c \geq 2\).
    /// Let the total number of samples be \(k\).
    /// The estimated betweenness centrality score of \(v\), \(BC(v)\) is given by \(\frac{nS}{k}\).
    ///
    /// # Example
    /// In order to compute the approximated betweenness centrality of the first node of the graph
    /// Homo Sapiens from STRING PPI you can use the following:
    ///
    /// ```rust
    /// let graph = graph::test_utilities::load_ppi(true, true, true, true, false, false);
    /// graph.get_approximated_betweenness_centrality_from_node_id(
    ///     0,
    ///     None,
    ///     None,
    ///     None
    /// );
    /// ```
    ///
    /// # Returns
    /// Float value with the approximated betweenness centrality of the provided node id.
    pub fn get_approximated_betweenness_centrality_from_node_id(
        &self,
        node_id: NodeT,
        constant: Option<f32>,
        maximum_samples_number: Option<f32>,
        random_state: Option<u64>,
    ) -> Result<f32> {
        self.validate_node_id(node_id)?;
        // The running sum, which in the paper is
        // referred to as \(S\).
        let mut running_sum: f32 = 0.0;
        // The number of samples nodes considered, which in the paper
        // is referred to as \(k\).
        let mut number_of_sampled_nodes: f32 = 0.0;
        // The number of the nodes in the graph, which in the paper
        // is referred to as \(n\).
        let nodes_number = self.get_number_of_nodes() as f32;
        // The random state to use to sample the nodes.
        let mut random_state = random_state.unwrap_or(42);
        let maximum_samples_number = maximum_samples_number.unwrap_or(nodes_number / 20.0);
        // The factor for the convergence of the approximated sampling for the considered node.
        // In the paper it is referred to a \(c\), and must be at least \(2.0\).
        let constant = constant.unwrap_or(2.0);
        if constant < 2.0 {
            return Err(format!(
                concat!(
                    "The constant parameter must be at least 2.0, but the provided ",
                    "value for the parameter's value is {}."
                ),
                constant
            ));
        }
        // Repeatedly sample the vertices.
        unsafe {
            for neighbour_node_id in
                self.iter_unchecked_neighbour_node_ids_from_source_node_id(node_id)
            {
                if running_sum >= nodes_number * constant
                    || number_of_sampled_nodes > maximum_samples_number
                {
                    break;
                }
                // Increase the number of sampled nodes.
                number_of_sampled_nodes += 1.0;
                // Compute the SSSP starting from the samples node.
                let sssp = self
                    .get_unchecked_breadth_first_search_predecessors_parallel_from_node_id(
                        neighbour_node_id,
                    );
                // Compute the pair dependency.
                let pair_dependency = self.get_pair_dependency_from_node_id(node_id, &sssp)?;
                // Update the running sum.
                running_sum += pair_dependency;
            }
        }
        // If the running sum is still zero,
        // it means that there are functionally no shortest paths
        // unless we explicitly build them, therefore
        // the approximated betweenness centrality can
        // be considered zero.
        if running_sum.is_zero() {
            return Ok(0.0);
        }
        // Repeatedly sample the vertices.
        while running_sum < nodes_number * constant
            && number_of_sampled_nodes < maximum_samples_number
        {
            // Sample random node.
            let sampled_node_id = self.get_random_node(random_state);
            // Increase the random state, using a wrapping add in order to avoid
            // possible overflows when a very high random state is provided.
            random_state = random_state.wrapping_add(1);
            // If the sampled node is a disconnected ones, we need to skip it.
            if unsafe { self.is_unchecked_disconnected_node_from_node_id(sampled_node_id) } {
                continue;
            }
            // Increase the number of sampled nodes.
            number_of_sampled_nodes += 1.0;
            // Compute the SSSP starting from the samples node.
            let sssp = unsafe {
                self.get_unchecked_breadth_first_search_predecessors_parallel_from_node_id(
                    sampled_node_id,
                )
            };
            // Compute the pair dependency.
            let pair_dependency = self.get_pair_dependency_from_node_id(node_id, &sssp)?;
            // Update the running sum.
            running_sum += pair_dependency;
        }
        // Compute the approximated betweenness centrality from the considered samples
        let approximated_betweenness_centrality =
            nodes_number / number_of_sampled_nodes * running_sum;
        // Return the computed betweenness centrality score
        Ok(approximated_betweenness_centrality)
    }

    /// Returns the unweighted approximated betweenness centrality of the given node id.
    ///
    /// # Arguments
    /// * `node_name`: &str - The node name for which to compute the approximated betweenness centrality.
    /// * `constant`: Option<f32> - The constant factor to use to regulate the sampling. By default 2.0. It must be greater or equal than 2.0.
    /// * `maximum_samples_number`: Option<f32> - The maximum number of samples to sample. By default `nodes_number / 20`, as suggested in the paper.
    /// * `random_state`: Option<u64> - The random state to use for the sampling. By default 42.
    ///
    /// # Raises
    /// * If the provided node name does not exist in the current graph instance.
    ///
    /// # References
    /// This method is an implementation of the [Approximating Betweenness Centrality](https://link.springer.com/chapter/10.1007/978-3-540-77004-6_10)
    /// work by David Bader et al.
    ///
    /// The algorithm repeatedly samples a vertex \(v_i \in V\),
    /// then performs single-source shortest paths from \(v_i\)
    /// and maintain a running sum \(S\) of the dependency scores \(\delta_{v_i∗}(v)\).
    /// Sample nodes until \(S\) is greater than cn for some constant \(c \geq 2\).
    /// Let the total number of samples be \(k\).
    /// The estimated betweenness centrality score of \(v\), \(BC(v)\) is given by \(\frac{nS}{k}\).
    ///
    /// # Example
    /// In order to compute the approximated weighted betweenness centrality of the node `ENSG00000178607` of the graph
    /// Homo Sapiens from STRING PPI you can use the following:
    ///
    /// ```rust
    /// let graph = graph::test_utilities::load_ppi(true, true, true, true, false, false);
    /// graph.get_approximated_betweenness_centrality_from_node_name(
    ///     "ENSG00000178607",
    ///     None,
    ///     None,
    ///     None   
    /// );
    /// ```
    ///
    /// # Returns
    /// Float value with the approximated betweenness centrality of the provided node id.
    pub fn get_approximated_betweenness_centrality_from_node_name(
        &self,
        node_name: &str,
        constant: Option<f32>,
        maximum_samples_number: Option<f32>,
        random_state: Option<u64>,
    ) -> Result<f32> {
        self.get_approximated_betweenness_centrality_from_node_id(
            self.get_node_id_from_node_name(node_name)?,
            constant,
            maximum_samples_number,
            random_state,
        )
    }

    /// Returns the weighted approximated betweenness centrality of the given node id.
    ///
    /// # Arguments
    /// * `node_id`: NodeT - The node ID for which to compute the approximated betweenness centrality.
    /// * `constant`: Option<f32> - The constant factor to use to regulate the sampling. By default 2.0. It must be greater or equal than 2.0.
    /// * `use_edge_weights_as_probabilities`: Option<bool> - Whether to consider the edge weights as probabilities.
    /// * `maximum_samples_number`: Option<f32> - The maximum number of samples to sample. By default `nodes_number / 20`, as suggested in the paper.
    /// * `random_state`: Option<u64> - The random state to use for the sampling. By default 42.
    ///
    /// # Raises
    /// * If the provided node ID does not exist in the current graph instance.
    ///
    /// # References
    /// This method is an implementation of the [Approximating Betweenness Centrality](https://link.springer.com/chapter/10.1007/978-3-540-77004-6_10)
    /// work by David Bader et al.
    ///
    /// The algorithm repeatedly samples a vertex \(v_i \in V\),
    /// then performs single-source shortest paths from \(v_i\)
    /// and maintain a running sum \(S\) of the dependency scores \(\delta_{v_i∗}(v)\).
    /// Sample nodes until \(S\) is greater than cn for some constant \(c \geq 2\).
    /// Let the total number of samples be \(k\).
    /// The estimated betweenness centrality score of \(v\), \(BC(v)\) is given by \(\frac{nS}{k}\).
    ///
    /// # Example
    /// In order to compute the approximated weighted betweenness centrality of the first node of the graph
    /// Homo Sapiens from STRING PPI you can use the following:
    ///
    /// ```rust
    /// let graph = graph::test_utilities::load_ppi(true, true, true, true, false, false);
    /// graph.get_weighted_approximated_betweenness_centrality_from_node_id(
    ///     0,
    ///     None,
    ///     None,
    ///     None,
    ///     None   
    /// );
    /// ```
    ///
    /// # Returns
    /// Float value with the weighted approximated betweenness centrality of the provided node id.
    pub fn get_weighted_approximated_betweenness_centrality_from_node_id(
        &self,
        node_id: NodeT,
        constant: Option<f32>,
        use_edge_weights_as_probabilities: Option<bool>,
        maximum_samples_number: Option<f32>,
        random_state: Option<u64>,
    ) -> Result<f32> {
        self.validate_node_id(node_id)?;
        // The running sum, which in the paper is
        // referred to as \(S\).
        let mut running_sum: f32 = 0.0;
        // The number of samples nodes considered, which in the paper
        // is referred to as \(k\).
        let mut number_of_sampled_nodes: f32 = 0.0;
        // The number of the nodes in the graph, which in the paper
        // is referred to as \(n\).
        let nodes_number = self.get_number_of_nodes() as f32;
        let maximum_samples_number = maximum_samples_number.unwrap_or(nodes_number / 20.0);
        // The random state to use to sample the nodes.
        let mut random_state = random_state.unwrap_or(42);
        // The factor for the convergence of the approximated sampling for the considered node.
        // In the paper it is referred to a \(c\), and must be at least \(2.0\).
        let constant = constant.unwrap_or(2.0);
        if constant < 2.0 {
            return Err(format!(
                concat!(
                    "The constant parameter must be at least 2.0, but the provided ",
                    "value for the parameter's value is {}."
                ),
                constant
            ));
        }
        // Repeatedly sample the vertices.
        unsafe {
            for neighbour_node_id in
                self.iter_unchecked_neighbour_node_ids_from_source_node_id(node_id)
            {
                if running_sum >= nodes_number * constant
                    || number_of_sampled_nodes > maximum_samples_number
                {
                    break;
                }
                // Increase the number of sampled nodes.
                number_of_sampled_nodes += 1.0;
                // Compute the SSSP starting from the samples node.
                let sssp = self.get_unchecked_dijkstra_from_node_id(
                    neighbour_node_id,
                    None,
                    None,
                    Some(true),
                    None,
                    use_edge_weights_as_probabilities,
                );
                // Compute the pair dependency.
                let pair_dependency =
                    self.get_weighted_pair_dependency_from_node_id(node_id, &sssp)?;
                // Update the running sum.
                running_sum += pair_dependency;
            }
        }
        // If the running sum is still zero,
        // it means that there are functionally no shortest paths
        // unless we explicitly build them, therefore
        // the approximated betweenness centrality can
        // be considered zero.
        if running_sum.is_zero() {
            return Ok(0.0);
        }
        // Repeatedly sample the vertices.
        while running_sum < nodes_number * constant
            && number_of_sampled_nodes < maximum_samples_number
        {
            // Sample random node.
            let sampled_node_id = self.get_random_node(random_state);
            // Increase the random state, using a wrapping add in order to avoid
            // possible overflows when a very high random state is provided.
            random_state = random_state.wrapping_add(1);
            // If the sampled node is a disconnected ones, we need to skip it.
            if unsafe { self.is_unchecked_disconnected_node_from_node_id(sampled_node_id) } {
                continue;
            }
            // Increase the number of sampled nodes.
            number_of_sampled_nodes += 1.0;
            // Compute the SSSP starting from the samples node.
            let sssp = unsafe {
                self.get_unchecked_dijkstra_from_node_id(
                    sampled_node_id,
                    None,
                    None,
                    Some(true),
                    None,
                    use_edge_weights_as_probabilities,
                )
            };
            // Compute the pair dependency.
            let pair_dependency = self.get_weighted_pair_dependency_from_node_id(node_id, &sssp)?;
            // Update the running sum.
            running_sum += pair_dependency;
        }
        // Compute the approximated betweenness centrality from the considered samples
        let approximated_betweenness_centrality =
            nodes_number / number_of_sampled_nodes * running_sum;
        // Return the computed betweenness centrality score
        Ok(approximated_betweenness_centrality)
    }

    /// Returns the weighted approximated betweenness centrality of the given node id.
    ///
    /// # Arguments
    /// * `node_name`: &str - The node name for which to compute the approximated betweenness centrality.
    /// * `constant`: Option<f32> - The constant factor to use to regulate the sampling. By default 2.0. It must be greater or equal than 2.0.
    /// * `use_edge_weights_as_probabilities`: Option<bool> - Whether to consider the edge weights as probabilities.
    /// * `maximum_samples_number`: Option<f32> - The maximum number of samples to sample. By default `nodes_number / 20`, as suggested in the paper.
    /// * `random_state`: Option<u64> - The random state to use for the sampling. By default 42.
    ///
    /// # Raises
    /// * If the provided node name does not exist in the current graph instance.
    ///
    /// # References
    /// This method is an implementation of the [Approximating Betweenness Centrality](https://link.springer.com/chapter/10.1007/978-3-540-77004-6_10)
    /// work by David Bader et al.
    ///
    /// The algorithm repeatedly samples a vertex \(v_i \in V\),
    /// then performs single-source shortest paths from \(v_i\)
    /// and maintain a running sum \(S\) of the dependency scores \(\delta_{v_i∗}(v)\).
    /// Sample nodes until \(S\) is greater than cn for some constant \(c \geq 2\).
    /// Let the total number of samples be \(k\).
    /// The estimated betweenness centrality score of \(v\), \(BC(v)\) is given by \(\frac{nS}{k}\).
    ///
    /// # Example
    /// In order to compute the approximated weighted betweenness centrality of the node `ENSG00000178607` of the graph
    /// Homo Sapiens from STRING PPI you can use the following:
    ///
    /// ```rust
    /// let graph = graph::test_utilities::load_ppi(true, true, true, true, false, false);
    /// graph.get_weighted_approximated_betweenness_centrality_from_node_name(
    ///     "ENSG00000178607",
    ///     None,
    ///     None,
    ///     None,
    ///     None
    /// );
    /// ```
    ///
    /// # Returns
    /// Float value with the weighted approximated betweenness centrality of the provided node id.
    pub fn get_weighted_approximated_betweenness_centrality_from_node_name(
        &self,
        node_name: &str,
        constant: Option<f32>,
        use_edge_weights_as_probabilities: Option<bool>,
        maximum_samples_number: Option<f32>,
        random_state: Option<u64>,
    ) -> Result<f32> {
        self.get_weighted_approximated_betweenness_centrality_from_node_id(
            self.get_node_id_from_node_name(node_name)?,
            constant,
            use_edge_weights_as_probabilities,
            maximum_samples_number,
            random_state,
        )
    }

    #[fuzz_type(maximum_iterations_number: Option<u8>)]
    /// Returns vector with unweighted eigenvector centrality.
    ///
    /// # Arguments
    /// * `maximum_iterations_number`: Option<usize> - The maximum number of iterations to consider.
    /// * `tollerance`: Option<f32> - The maximum error tollerance for convergence.
    pub fn get_eigenvector_centrality(
        &self,
        maximum_iterations_number: Option<usize>,
        tollerance: Option<f32>,
    ) -> Result<Vec<f32>> {
        let maximum_iterations_number = maximum_iterations_number.unwrap_or(1000);
        let tollerance = tollerance.unwrap_or(1e-6) * self.get_number_of_nodes() as f32;
        if tollerance < f32::EPSILON {
            return Err(
                "The tollerance must be a non-zero positive value bigger than epislon (1e-16)."
                    .to_string(),
            );
        }
        let mut centralities: Vec<AtomicF32> = self
            .iter_node_ids()
            .map(|_| AtomicF32::new(1.0 / self.get_number_of_nodes() as f32))
            .collect();
        let mut last_centralities =
            vec![1.0 / self.get_number_of_nodes() as f32; self.get_number_of_nodes() as usize];
        for _ in 0..maximum_iterations_number {
            self.par_iter_node_ids().for_each(|src| {
                unsafe { self.iter_unchecked_neighbour_node_ids_from_source_node_id(src) }
                    .for_each(|dst| {
                        centralities[dst as usize]
                            .fetch_add(last_centralities[src as usize], Ordering::Relaxed);
                    });
            });
            let norm: f32 = centralities
                .par_iter()
                .map(|centrality| centrality.load(Ordering::Relaxed).pow(2))
                .sum::<f32>()
                .sqrt();
            centralities.par_iter_mut().for_each(|centrality| {
                centrality
                    .fetch_update(Ordering::Relaxed, Ordering::Relaxed, |x| Some(x / norm))
                    .unwrap();
            });
            let updated_centrality = centralities
                .iter()
                .map(|centrality| centrality.load(Ordering::Relaxed))
                .collect::<Vec<f32>>();
            let differences = updated_centrality
                .par_iter()
                .zip(last_centralities.par_iter())
                .map(|(centrality, old_centrality)| (centrality - old_centrality).abs())
                .sum::<f32>();
            if differences < tollerance {
                return Ok(updated_centrality);
            }
            last_centralities = updated_centrality;
        }
        Err(format!(
            "Unable to reach convergence in {} iterations.",
            maximum_iterations_number
        ))
    }

    #[fuzz_type(maximum_iterations_number: Option<u8>)]
    /// Returns vector with unweighted eigenvector centrality.
    ///
    /// # Arguments
    /// * `maximum_iterations_number`: Option<usize> - The maximum number of iterations to consider.
    /// * `tollerance`: Option<f32> - The maximum error tollerance for convergence.
    pub fn get_weighted_eigenvector_centrality(
        &self,
        maximum_iterations_number: Option<usize>,
        tollerance: Option<f32>,
    ) -> Result<Vec<f32>> {
        self.must_have_positive_edge_weights()?;
        let maximum_iterations_number = maximum_iterations_number.unwrap_or(1000);
        let tollerance = tollerance.unwrap_or(1e-6) * self.get_number_of_nodes() as f32;
        if tollerance < f32::EPSILON {
            return Err(
                "The tollerance must be a non-zero positive value bigger than epsilon (1e-16)."
                    .to_string(),
            );
        }
        let mut centralities: Vec<AtomicF32> = self
            .iter_node_ids()
            .map(|_| AtomicF32::new(1.0 / self.get_number_of_nodes() as f32))
            .collect();
        let mut last_centralities =
            vec![1.0 / self.get_number_of_nodes() as f32; self.get_number_of_nodes() as usize];
        for _ in 0..maximum_iterations_number {
            self.par_iter_node_ids().for_each(|src| {
                // TODO: this can be done in a faster way
                unsafe { self.iter_unchecked_neighbour_node_ids_from_source_node_id(src) }
                    .for_each(|dst| unsafe {
                        centralities[dst as usize].fetch_add(
                            last_centralities[src as usize]
                                * self.get_unchecked_edge_weight_from_node_ids(src, dst) as f32,
                            Ordering::Relaxed,
                        );
                    });
            });
            let norm: f32 = centralities
                .par_iter()
                .map(|centrality| centrality.load(Ordering::Relaxed).pow(2))
                .sum::<f32>()
                .sqrt();
            centralities.par_iter_mut().for_each(|centrality| {
                centrality
                    .fetch_update(Ordering::Relaxed, Ordering::Relaxed, |x| Some(x / norm))
                    .unwrap();
            });
            let updated_centrality = centralities
                .iter()
                .map(|centrality| centrality.load(Ordering::Relaxed))
                .collect::<Vec<f32>>();
            let differences = updated_centrality
                .par_iter()
                .zip(last_centralities.par_iter())
                .map(|(centrality, old_centrality)| (centrality - old_centrality).abs())
                .sum::<f32>();
            if differences < tollerance {
                return Ok(updated_centrality);
            }
            last_centralities = updated_centrality;
        }
        Err(format!(
            "Unable to reach convergence in {} iterations.",
            maximum_iterations_number
        ))
    }
}
