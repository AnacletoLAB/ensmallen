use super::*;
use rayon::prelude::*;

impl Graph {
    /// Returns 2-approximated verted cover bitvec using greedy algorithm.
    ///
    /// # Implementative details
    /// We DO NOT provide a loading bar for this method because the loading bar
    /// iterative step is slower than the actual iteration.
    ///
    /// # References
    /// This implementation is described in ["A local-ratio theorem for approximating the weighted vertex cover problem"](http://www.cs.technion.ac.il/~reuven/PDF/vc_lr.pdf).
    ///
    pub fn get_approximated_vertex_cover(&self) -> Vec<bool> {
        let mut vertex_cover: Vec<bool> = vec![false; self.get_number_of_nodes() as usize];

        let mut node_ids: Vec<NodeT> = self.get_node_ids();
        node_ids.par_sort_unstable_by(|&a, &b| unsafe {
            self.get_unchecked_node_degree_from_node_id(b)
                .partial_cmp(&self.get_unchecked_node_degree_from_node_id(a))
                .unwrap()
        });

        // We iterate the node IDs from higher to lower
        node_ids.into_iter().for_each(|node_id| unsafe {
            if self
                .par_iter_unchecked_neighbour_node_ids_from_source_node_id(node_id)
                .any(|dst_node_id| !vertex_cover[dst_node_id as usize])
            {
                vertex_cover[node_id as usize] = true;
            }
        });

        vertex_cover
    }
}
