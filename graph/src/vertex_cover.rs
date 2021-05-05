use super::*;
use bitvec::prelude::*;
use indicatif::ParallelProgressIterator;
use rayon::iter::ParallelIterator;
use std::collections::HashSet;

impl Graph {
    /// Returns 2-approximated verted cover using greedy algorithm.
    ///
    /// # References
    /// This implementation is described in ["A local-ratio theorem for approximating the weighted vertex cover problem"](http://www.cs.technion.ac.il/~reuven/PDF/vc_lr.pdf).
    pub fn approximated_vertex_cover(&self) -> HashSet<NodeT> {
        let edges_number = self.get_edges_number() as usize;
        let nodes_number = self.get_nodes_number() as usize;
        let mut covered_nodes = bitvec![Lsb0, u8; 0; nodes_number];
        let thread_shared_covered_nodes = ThreadSafe {
            value: std::cell::UnsafeCell::new(&mut covered_nodes),
        };
        self.par_iter_edge_ids(self.is_directed()).for_each(
            |(_, src_node_id, dst_node_id)| unsafe {
                let covered_nodes = thread_shared_covered_nodes.value.get();
                let is_src_inserted = (*covered_nodes)[src_node_id as usize];
                let is_dst_inserted = (*covered_nodes)[dst_node_id as usize];
                if !is_src_inserted && !is_dst_inserted {
                    *(*covered_nodes).get_unchecked_mut(src_node_id as usize) = true;
                }
            },
        );
        covered_nodes
            .iter_ones()
            .map(|node_id| node_id as NodeT)
            .collect()
    }
}
