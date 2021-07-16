use super::*;
use permutation::permutation;

/// # Sorting algorithms.
impl Graph {
    /// Returns graph with node IDs sorted by increasing outbound node degree.
    pub fn sort_by_increasing_outbound_node_degree(&self) -> Graph {
        if self.has_nodes_sorted_by_increasing_outbound_node_degree() {
            return self.clone();
        }
        let sorted_node_ids_permutation = permutation::sort(self.get_node_degrees());
        let new_sorted_node_ids =
            sorted_node_ids_permutation.apply_slice(self.iter_node_ids().collect::<Vec<NodeT>>());
        unsafe { self.remap_unchecked_from_node_ids(new_sorted_node_ids) }
    }

    /// Returns graph with node IDs sorted by decreasing outbound node degree.
    pub fn sort_by_decreasing_outbound_node_degree(&self) -> Graph {
        if self.has_nodes_sorted_by_decreasing_outbound_node_degree() {
            return self.clone();
        }
        let sorted_node_ids_permutation =
            permutation::sort_by(self.get_node_degrees(), |a, b| b.cmp(a));
        let new_sorted_node_ids =
            sorted_node_ids_permutation.apply_slice(self.iter_node_ids().collect::<Vec<NodeT>>());
        unsafe { self.remap_unchecked_from_node_ids(new_sorted_node_ids) }
    }
}
