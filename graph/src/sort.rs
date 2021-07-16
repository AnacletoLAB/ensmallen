use permutation::permutation;

use super::*;

/// # Sorting algorithms.
impl Graph {
    /// Returns graph with node IDs sorted by increasing outbound node degree.
    pub fn sort_by_increasing_outbound_node_degree(&self) -> Graph {
        if self.has_nodes_sorted_by_increasing_outbound_node_degree() {
            return self.clone();
        }
        unsafe { self.remap_unchecked_from_permutation(permutation::sort(self.get_node_degrees())) }
    }

    /// Returns graph with node IDs sorted by decreasing outbound node degree.
    pub fn sort_by_decreasing_outbound_node_degree(&self) -> Graph {
        if self.has_nodes_sorted_by_decreasing_outbound_node_degree() {
            return self.clone();
        }
        unsafe {
            self.remap_unchecked_from_permutation(permutation::sort_by(
                self.get_node_degrees(),
                |a, b| b.cmp(a),
            ))
        }
    }
}
