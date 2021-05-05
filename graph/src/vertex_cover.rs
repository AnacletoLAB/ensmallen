use super::*;
use rayon::iter::IntoParallelRefIterator;
use rayon::iter::ParallelIterator;
use rayon::iter::{IndexedParallelIterator, ParallelBridge};
use std::collections::HashSet;

impl Graph {
    /// Returns 2-approximated verted cover using greedy algorithm.
    ///
    /// # Arguments
    /// * `verbose`: Option<bool> - Whether to show a loading bar.
    pub fn approximated_vertex_cover(&self, verbose: Option<bool>) -> HashSet<NodeT> {
        let verbose = verbose.unwrap_or(true);
        let nodes_number = self.get_nodes_number() as usize;
        let mut vertex_cover = HashSet::new();
        let mut degrees = self.get_node_degrees();
        let pb = get_loading_bar(
            verbose,
            "Computing 2-approximated vertex cover",
            nodes_number,
        );
        while let Some((max_degree_node_id, &degree)) = degrees
            .par_iter()
            .enumerate()
            .filter(|(_, &degree)| degree != 0)
            .max_by_key(|&(_, value)| value)
        {
            // We do not check for the selfloop or multigrapjs, but since is only for porposes
            // of visualization of the process of the algorithm, it does not make sense
            // to bother with additional checks.
            pb.inc(1 + degree as u64);
            vertex_cover.insert(max_degree_node_id as NodeT);
            degrees[max_degree_node_id] = 0;
            let thread_shared_degrees = ThreadSafe {
                value: std::cell::UnsafeCell::new(&mut degrees),
            };
            self.iter_unchecked_neighbour_node_ids_from_source_node_id(max_degree_node_id as NodeT)
                .map(|neighbour_node_id| {
                    unsafe { (*thread_shared_degrees.value.get())[neighbour_node_id as usize] = 0 };
                    neighbour_node_id
                })
                .par_bridge()
                .for_each(|neighbour_node_id| {
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
