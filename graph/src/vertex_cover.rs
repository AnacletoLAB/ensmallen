use super::*;
use rayon::iter::ParallelIterator;
use std::{collections::HashSet, sync::atomic::AtomicBool};

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
        let vertex_cover: Vec<AtomicBool> = unsafe {
            std::mem::transmute::<Vec<bool>, Vec<AtomicBool>>(vec![
                false;
                self.get_number_of_nodes()
                    as usize
            ])
        };
        self.par_iter_node_ids().filter(move |&node_id| unsafe {
            let filter = !(self.is_unchecked_singleton_from_node_id(node_id)
                || self
                    .iter_unchecked_neighbour_node_ids_from_source_node_id(node_id)
                    .any(|dst_node_id| {
                        vertex_cover[dst_node_id as usize]
                            .load(std::sync::atomic::Ordering::Relaxed)
                    }));
            if filter {
                vertex_cover[node_id as usize].store(true, std::sync::atomic::Ordering::Relaxed);
            }
            filter
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
    pub fn approximated_vertex_cover(&self) -> Vec<NodeT> {
        self.par_iter_approximated_vertex_cover().collect()
    }

    /// Returns 2-approximated verted cover set using greedy algorithm.
    ///
    /// # Implementative details
    /// We DO NOT provide a loading bar for this method because the loading bar
    /// iterative step is slower than the actual iteration.
    ///
    /// # References
    /// This implementation is described in ["A local-ratio theorem for approximating the weighted vertex cover problem"](http://www.cs.technion.ac.il/~reuven/PDF/vc_lr.pdf).
    pub(crate) fn approximated_vertex_cover_set(&self) -> HashSet<NodeT> {
        self.par_iter_approximated_vertex_cover().collect()
    }
}
