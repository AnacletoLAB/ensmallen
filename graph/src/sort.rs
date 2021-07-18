use rayon::iter::IndexedParallelIterator;
use rayon::iter::IntoParallelIterator;
use rayon::iter::ParallelIterator;
use rayon::slice::ParallelSliceMut;

use super::*;

/// # Sorting algorithms.
impl Graph {
    /// Returns graph with node IDs sorted by increasing outbound node degree.
    pub fn sort_by_increasing_outbound_node_degree(&self) -> Graph {
        if self.has_nodes_sorted_by_increasing_outbound_node_degree() {
            return self.clone();
        }
        let mut node_ids_and_node_degrees =
            vec![(0 as usize, 0 as NodeT); self.get_nodes_number() as usize];
        self.par_iter_node_degrees()
            .enumerate()
            .collect_into_vec(&mut node_ids_and_node_degrees);
        node_ids_and_node_degrees.par_sort_unstable_by(|(_, node_degree_a), (_, node_degree_b)| {
            node_degree_a.cmp(node_degree_b)
        });
        let mut new_node_ids = vec![0 as NodeT; self.get_nodes_number() as usize];
        node_ids_and_node_degrees
            .into_par_iter()
            .map(|(node_id, _)| node_id as NodeT)
            .collect_into_vec(&mut new_node_ids);
        unsafe { self.remap_unchecked_from_node_ids(new_node_ids) }
    }

    /// Returns graph with node IDs sorted by decreasing outbound node degree.
    pub fn sort_by_decreasing_outbound_node_degree(&self) -> Graph {
        if self.has_nodes_sorted_by_decreasing_outbound_node_degree() {
            return self.clone();
        }
        let mut node_ids_and_node_degrees =
            vec![(0 as usize, 0 as NodeT); self.get_nodes_number() as usize];
        self.par_iter_node_degrees()
            .enumerate()
            .collect_into_vec(&mut node_ids_and_node_degrees);
        node_ids_and_node_degrees.par_sort_unstable_by(|(_, node_degree_a), (_, node_degree_b)| {
            node_degree_b.cmp(node_degree_a)
        });
        let mut new_node_ids = vec![0 as NodeT; self.get_nodes_number() as usize];
        node_ids_and_node_degrees
            .into_par_iter()
            .map(|(node_id, _)| node_id as NodeT)
            .collect_into_vec(&mut new_node_ids);
        unsafe { self.remap_unchecked_from_node_ids(new_node_ids) }
    }

    /// Returns graph with node IDs sorted by lexicographic order.
    pub fn sort_by_node_lexicographic_order(&self) -> Graph {
        if self.has_nodes_sorted_by_lexicographic_order() {
            return self.clone();
        }
        let mut node_ids_and_node_names =
            vec![(0 as usize, "".to_owned()); self.get_nodes_number() as usize];
        self.par_iter_node_names()
            .enumerate()
            .collect_into_vec(&mut node_ids_and_node_names);
        node_ids_and_node_names.par_sort_unstable_by(|(_, node_name_a), (_, node_name_b)| {
            node_name_b.cmp(node_name_a)
        });
        let mut new_node_ids = vec![0 as NodeT; self.get_nodes_number() as usize];
        node_ids_and_node_names
            .into_par_iter()
            .map(|(node_id, _)| node_id as NodeT)
            .collect_into_vec(&mut new_node_ids);
        unsafe { self.remap_unchecked_from_node_ids(new_node_ids) }
    }
}
