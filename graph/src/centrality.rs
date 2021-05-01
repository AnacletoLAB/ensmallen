use super::*;
use atomic_float::AtomicF64;
use indicatif::ParallelProgressIterator;
use itertools::Itertools;
use rayon::iter::{IntoParallelRefMutIterator, ParallelIterator};
use std::collections::VecDeque;
use std::sync::atomic::Ordering;

impl Graph {
    /// Returns iterator over degree centrality for all nodes.
    pub fn iter_degree_centrality(&self) -> Box<dyn Iterator<Item = f64> + '_> {
        if self.has_nodes() {
            let max_degree = self.get_max_node_degree().unwrap() as f64;
            Box::new(
                self.iter_node_degrees()
                    .map(move |degree| degree as f64 / max_degree),
            )
        } else {
            Box::new(::std::iter::empty())
        }
    }

    /// Returns vector of degree centrality for all nodes.
    pub fn get_degree_centrality(&self) -> Vec<f64> {
        self.iter_degree_centrality().collect()
    }

    /// Returns vector of betweenness centrality for all nodes.
    ///
    /// # Arguments
    /// * `normalize`: Option<bool> - Whether to normalize the values. By default, it is false.
    /// * `verbose`: Option<bool> - Whether to show a loading bar. By default, it is true.
    pub fn get_betweenness_centrality(
        &self,
        normalize: Option<bool>,
        verbose: Option<bool>,
    ) -> Vec<f64> {
        if !self.has_nodes() {
            return Vec::new();
        }
        let normalize = normalize.unwrap_or(false);
        let verbose = verbose.unwrap_or(true);
        let nodes_number = self.get_nodes_number() as usize;
        let centralities: Vec<AtomicF64> =
            self.iter_node_ids().map(|_| AtomicF64::new(0.0)).collect();
        let factor = if self.is_directed() { 1.0 } else { 2.0 };
        let pb = get_loading_bar(verbose, "Computing betweennes centralities", nodes_number);
        self.par_iter_node_ids()
            .progress_with(pb)
            .for_each(|src_node_id| {
                let mut stack: Vec<NodeT> = Vec::new();
                let mut node_lists: Vec<Vec<NodeT>> =
                    self.iter_node_ids().map(|_| Vec::new()).collect();
                let mut shortest_path_counts = vec![0; nodes_number];
                shortest_path_counts[src_node_id as usize] = 1;
                let mut distance_from_root = vec![-1; nodes_number];
                distance_from_root[src_node_id as usize] = 0;
                let mut nodes_to_visit: VecDeque<NodeT> = VecDeque::new();
                nodes_to_visit.push_back(src_node_id);
                while !nodes_to_visit.is_empty() {
                    let current_node_id = nodes_to_visit.pop_front().unwrap();
                    stack.push(current_node_id);
                    // Bader says to do the following step in parallel
                    // Currently it is not parallel because the EliasFano implementation
                    // does not supporting a range of values in parallel, and currently
                    // it is not possible to Box a parallel iterator from Rayon.
                    self.iter_unchecked_neighbour_node_ids_from_source_node_id(current_node_id)
                        .for_each(|neighbour_node_id| {
                            if distance_from_root[neighbour_node_id as usize] < 0 {
                                nodes_to_visit.push_back(neighbour_node_id);
                                distance_from_root[neighbour_node_id as usize] =
                                    distance_from_root[current_node_id as usize] + 1;
                            }
                            if distance_from_root[neighbour_node_id as usize]
                                == distance_from_root[current_node_id as usize] + 1
                            {
                                shortest_path_counts[neighbour_node_id as usize] +=
                                    shortest_path_counts[current_node_id as usize];
                                node_lists[neighbour_node_id as usize].push(current_node_id);
                            }
                        });
                }
                let mut dependencies = vec![0.0; nodes_number];
                stack.into_iter().rev().for_each(|current_node_id| {
                    node_lists[current_node_id as usize]
                        .iter()
                        .for_each(|&neighbour_node_id| {
                            dependencies[neighbour_node_id as usize] +=
                                shortest_path_counts[neighbour_node_id as usize] as f64
                                    / shortest_path_counts[current_node_id as usize] as f64
                                    * (1.0 + dependencies[current_node_id as usize]);
                        });
                    if current_node_id != src_node_id {
                        centralities[current_node_id as usize].fetch_add(
                            dependencies[current_node_id as usize] / factor,
                            Ordering::SeqCst,
                        );
                    }
                });
            });
        let mut centralities =
            unsafe { std::mem::transmute::<Vec<AtomicF64>, Vec<f64>>(centralities) };
        if normalize {
            let (min_centrality, max_centrality) =
                centralities.iter().cloned().minmax().into_option().unwrap();
            let delta = max_centrality - min_centrality;
            centralities.par_iter_mut().for_each(|value| {
                *value = (*value - min_centrality) / delta;
            });
        }
        centralities
    }
}
