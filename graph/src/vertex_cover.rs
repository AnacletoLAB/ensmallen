use super::*;
use bitvec::prelude::*;
use rayon::iter::IntoParallelRefIterator;
use rayon::iter::ParallelIterator;
use rayon::iter::{IndexedParallelIterator, ParallelBridge};

impl Graph {
    /// Returns 2-approximated verted cover using greedy algorithm.
    ///
    /// # Arguments
    /// * `verbose`: Option<bool> - Whether to show a loading bar.
    pub fn approximated_vertex_cover(&self, verbose: Option<bool>) -> Vec<NodeT> {
        let verbose = verbose.unwrap_or(true);
        let nodes_number = self.get_nodes_number() as usize;
        let mut vertex_cover = Vec::with_capacity(nodes_number);
        let mut covered_nodes = bitvec![Lsb0, u8; 0; nodes_number as usize];
        let mut degrees = self.get_node_degrees();
        let pb = get_loading_bar(
            verbose,
            "Computing 2-approximated vertex cover",
            nodes_number,
        );
        while let Some((max_degree_node_id, &degree)) = degrees
            .par_iter()
            .enumerate()
            .filter(|(node_id, _)| !covered_nodes[*node_id])
            .max_by_key(|&(_, degree)| degree)
        {
            // We do not check for the selfloop or multigraphs, but since is only for porposes
            // of visualization of the process of the algorithm, it does not make sense
            // to bother with additional checks.
            pb.inc(1 + degree as u64);
            vertex_cover.push(max_degree_node_id as NodeT);
            unsafe {
                *covered_nodes.get_unchecked_mut(max_degree_node_id) = true;
            }
            if degree == 0 {
                // If we reached a 0 degree max node, it means that all the
                // remaining nodes are either singletons or all their neighbours
                // have been inserted. Therefore, we can now complete sequentially.
                vertex_cover.extend(covered_nodes.iter_zeros().map(|node_id| node_id as NodeT));
                break;
            }
            let thread_shared_degrees = ThreadSafe {
                value: std::cell::UnsafeCell::new(&mut degrees),
            };
            let thread_shared_covered_nodes = ThreadSafe {
                value: std::cell::UnsafeCell::new(&mut covered_nodes),
            };
            self.iter_unchecked_neighbour_node_ids_from_source_node_id(max_degree_node_id as NodeT)
                .par_bridge()
                .filter(|&neighbour_node_id| unsafe {
                    !(*thread_shared_covered_nodes.value.get())[neighbour_node_id as usize]
                })
                .for_each(|neighbour_node_id| {
                    unsafe {
                        *(*thread_shared_covered_nodes.value.get())
                            .get_unchecked_mut(neighbour_node_id as usize) = true;
                    }
                    let degrees = thread_shared_degrees.value.get();
                    self.iter_unchecked_neighbour_node_ids_from_source_node_id(neighbour_node_id)
                        .for_each(|second_order_neighbour_id| {
                            unsafe {
                                (*degrees)[second_order_neighbour_id as usize] =
                                    (*degrees)[second_order_neighbour_id as usize].saturating_sub(1)
                            };
                        })
                });
        }
        vertex_cover
    }
}
