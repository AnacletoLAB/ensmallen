use crate::graph::Graph;
use shared::*;
use rayon::iter::ParallelIterator;
use std::collections::HashSet;

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
    pub fn par_iter_approximated_vertex_cover(&self) -> impl ParallelIterator<Item = NodeT> + '_ {
        let nodes_number = self.get_nodes_number() as usize;
        let thread_shared_vertex_cover = ThreadDataRaceAware {
            value: std::cell::UnsafeCell::new(vec![false; nodes_number]),
        };
        self.par_iter_node_ids()
            .filter_map(move |src_node_id| unsafe {
                if self.is_unchecked_singleton_from_node_id(src_node_id) {
                    return None;
                }
                let vertex_cover = thread_shared_vertex_cover.value.get();
                if self
                    .iter_unchecked_neighbour_node_ids_from_source_node_id(src_node_id)
                    .any(|dst_node_id| !(*vertex_cover)[dst_node_id as usize])
                {
                    *(*vertex_cover).get_unchecked_mut(src_node_id as usize) = true;
                    Some(src_node_id)
                } else {
                    None
                }
            })
    }

    /// Returns 2-approximated verted cover set using greedy algorithm.
    ///
    /// # Implementative details
    /// We DO NOT provide a loading bar for this method because the loading bar
    /// iterative step is slower than the actual iteration.
    ///
    /// # References
    /// This implementation is described in ["A local-ratio theorem for approximating the weighted vertex cover problem"](http://www.cs.technion.ac.il/~reuven/PDF/vc_lr.pdf).
    pub fn approximated_vertex_cover_set(&self) -> HashSet<NodeT> {
        self.par_iter_approximated_vertex_cover().collect()
    }
}
