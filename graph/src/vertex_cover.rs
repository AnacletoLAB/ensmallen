use super::*;
use bitvec::prelude::*;
use rayon::iter::ParallelIterator;
use std::collections::HashSet;

impl Graph {
    /// Returns 2-approximated verted cover bitvec using greedy algorithm.
    ///
    /// # References
    /// This implementation is described in ["A local-ratio theorem for approximating the weighted vertex cover problem"](http://www.cs.technion.ac.il/~reuven/PDF/vc_lr.pdf).
    pub fn approximated_vertex_cover_bitvec(&self) -> BitVec<Lsb0, u8> {
        let nodes_number = self.get_nodes_number() as usize;
        let mut vertex_cover = bitvec![Lsb0, u8; 0; nodes_number];
        let thread_shared_vertex_cover = ThreadSafe {
            value: std::cell::UnsafeCell::new(&mut vertex_cover),
        };
        self.par_iter_edge_ids(self.is_directed()).for_each(
            |(_, src_node_id, dst_node_id)| unsafe {
                let vertex_cover = thread_shared_vertex_cover.value.get();
                let is_src_inserted = (*vertex_cover)[src_node_id as usize];
                let is_dst_inserted = (*vertex_cover)[dst_node_id as usize];
                if !is_src_inserted && !is_dst_inserted {
                    *(*vertex_cover).get_unchecked_mut(src_node_id as usize) = true;
                }
            },
        );
        vertex_cover
    }

    /// Returns 2-approximated verted cover set using greedy algorithm.
    ///
    /// # References
    /// This implementation is described in ["A local-ratio theorem for approximating the weighted vertex cover problem"](http://www.cs.technion.ac.il/~reuven/PDF/vc_lr.pdf).
    pub fn approximated_vertex_cover_set(&self) -> HashSet<NodeT> {
        self.approximated_vertex_cover_bitvec()
            .iter_ones()
            .map(|node_id| node_id as NodeT)
            .collect()
    }
}
