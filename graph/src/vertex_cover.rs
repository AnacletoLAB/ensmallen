use super::*;
use indicatif::ParallelProgressIterator;
use rayon::iter::ParallelIterator;
use std::collections::HashSet;

impl Graph {
    /// Returns 2-approximated verted cover bitvec using greedy algorithm.
    ///
    /// # Arguments
    /// * `verbose`: Option<bool> - Whether to show a loading bar.
    ///
    /// # References
    /// This implementation is described in ["A local-ratio theorem for approximating the weighted vertex cover problem"](http://www.cs.technion.ac.il/~reuven/PDF/vc_lr.pdf).
    ///
    pub fn par_iter_approximated_vertex_cover(
        &self,
        verbose: Option<bool>,
    ) -> impl ParallelIterator<Item = NodeT> + '_ {
        let verbose = verbose.unwrap_or(true);
        let nodes_number = self.get_nodes_number() as usize;
        let thread_shared_vertex_cover = ThreadDataRaceAware {
            value: std::cell::UnsafeCell::new(vec![false; nodes_number]),
        };
        let pb = get_loading_bar(verbose, "Computing approximated vertex cover", nodes_number);
        self.par_iter_node_ids()
            .progress_with(pb)
            .filter_map(move |src_node_id| unsafe {
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
    /// # Arguments
    /// * `verbose`: Option<bool> - Whether to show a loading bar.
    ///
    /// # References
    /// This implementation is described in ["A local-ratio theorem for approximating the weighted vertex cover problem"](http://www.cs.technion.ac.il/~reuven/PDF/vc_lr.pdf).
    pub fn approximated_vertex_cover_set(&self, verbose: Option<bool>) -> HashSet<NodeT> {
        self.par_iter_approximated_vertex_cover(verbose).collect()
    }
}
