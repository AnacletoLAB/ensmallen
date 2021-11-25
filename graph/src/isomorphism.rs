use super::*;
use rayon::prelude::*;
use std::sync::atomic::{AtomicBool, Ordering};

impl Graph {
    /// Returns vector of vectors of isomorphic node groups IDs.
    ///
    /// # Arguments
    /// * `minimum_node_degree`: Option<NodeT> - Minimum node degree for the topological synonims. By default, 5.
    pub fn par_iter_isomorphic_node_groups(
        &self,
        minimum_node_degree: Option<NodeT>,
    ) -> impl ParallelIterator<Item = Vec<NodeT>> + '_ {
        let minimum_node_degree = minimum_node_degree.unwrap_or(5);
        let isomorphisms: Vec<AtomicBool> = self
            .par_iter_node_ids()
            .map(|_| AtomicBool::new(false))
            .collect();
        self.par_iter_node_ids().filter_map(move |node_id| unsafe {
            if isomorphisms[node_id as usize].load(Ordering::Relaxed) {
                return None;
            }
            let node_degree = self.get_unchecked_node_degree_from_node_id(node_id);
            if node_degree < minimum_node_degree {
                return None;
            }
            let node_type = self.get_unchecked_node_type_ids_from_node_id(node_id);
            let mut isomorphic_group: Vec<NodeT> = (node_id + 1..self.get_nodes_number())
                .into_par_iter()
                .filter(|&other_node_id| {
                    self.get_unchecked_node_degree_from_node_id(other_node_id) == node_degree
                        && self.get_unchecked_node_type_ids_from_node_id(other_node_id) == node_type
                        && self
                            .iter_unchecked_neighbour_node_ids_from_source_node_id(node_id)
                            .zip(self.iter_unchecked_neighbour_node_ids_from_source_node_id(
                                other_node_id,
                            ))
                            .all(|(first_node_neighbour_id, second_node_neighbour_id)| {
                                first_node_neighbour_id == second_node_neighbour_id
                            })
                })
                .collect();
            if isomorphic_group.is_empty() {
                None
            } else {
                isomorphic_group.push(node_id);
                isomorphic_group.iter().for_each(|&node_id| {
                    isomorphisms[node_id as usize].store(true, Ordering::Relaxed);
                });
                Some(isomorphic_group)
            }
        })
    }

    /// Returns vector with isomorphic node groups IDs.
    ///
    /// # Implementative details
    /// Nodes not associated with no particular group are mapped with the maximum U32.
    ///
    /// # Arguments
    /// * `minimum_node_degree`: Option<NodeT> - Minimum node degree for the topological synonims. By default, 5.
    pub fn get_isomorphic_node_groups(
        &self,
        minimum_node_degree: Option<NodeT>,
    ) -> Vec<Vec<NodeT>> {
        self.par_iter_isomorphic_node_groups(minimum_node_degree)
            .collect()
    }

    /// Returns whether the current graph has topological synonims.
    ///
    /// # Arguments
    /// * `minimum_node_degree`: Option<NodeT> - Minimum node degree for the topological synonims.
    pub fn has_isomorphic_nodes(&self, minimum_node_degree: Option<NodeT>) -> bool {
        let minimum_node_degree = minimum_node_degree.unwrap_or(5);
        self.par_iter_node_ids().any(move |node_id| unsafe {
            let node_degree = self.get_unchecked_node_degree_from_node_id(node_id);
            if node_degree < minimum_node_degree {
                return false;
            }
            let node_type = self.get_unchecked_node_type_ids_from_node_id(node_id);
            (node_id + 1..self.get_nodes_number())
                .into_par_iter()
                .any(|other_node_id| {
                    self.get_unchecked_node_degree_from_node_id(other_node_id) == node_degree
                        && self.get_unchecked_node_type_ids_from_node_id(other_node_id) == node_type
                        && self
                            .iter_unchecked_neighbour_node_ids_from_source_node_id(node_id)
                            .zip(self.iter_unchecked_neighbour_node_ids_from_source_node_id(
                                other_node_id,
                            ))
                            .all(|(first_node_neighbour_id, second_node_neighbour_id)| {
                                first_node_neighbour_id == second_node_neighbour_id
                            })
                })
        })
    }
}
