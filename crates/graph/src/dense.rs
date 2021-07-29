use std::cell::UnsafeCell;
use crate::graph::Graph;
use shared::*;

use rayon::iter::ParallelIterator;

impl Graph {
    /// Returns binary dense adjacency matrix.
    ///
    /// Beware of using this method on big graphs!
    /// It'll use all of your RAM!
    ///
    /// # Implementative notes
    /// On multigraphs this method will ignore multi-edges and treat
    /// those occurrences as would an homogeneous graph.
    pub fn get_dense_binary_adjacency_matrix(&self) -> Vec<Vec<bool>> {
        // We create the dense binary adjacency matrix.
        let mut ajacency =
            vec![vec![false; self.get_nodes_number() as usize]; self.get_nodes_number() as usize];
        // We wrap the adjacency into an object we can share between threads
        let thread_ajacency = ThreadDataRaceAware {
            value: UnsafeCell::new(&mut ajacency),
        };
        // We iterate on the edges and populate the matrix.
        self.par_iter_edge_node_ids(true)
            .for_each(|(_, src, dst)| unsafe {
                (*thread_ajacency.value.get())[src as usize][dst as usize] = true;
            });
        ajacency
    }

    /// Returns binary weighted adjacency matrix.
    ///
    /// Beware of using this method on big graphs!
    /// It'll use all of your RAM!
    ///
    /// # Arguments
    /// * `weight`: Option<WeightT> - The weight value to use for absent edges. By default, `0.0`.
    ///
    /// # Implementative notes
    /// On multigraphs this method will ignore multi-edges and treat
    /// those occurrences as would an homogeneous graph.
    ///
    /// # Raises
    /// * If the graph does not have edge weights.
    pub fn get_dense_weighted_adjacency_matrix(
        &self,
        weight: Option<WeightT>,
    ) -> Result<Vec<Vec<WeightT>>> {
        // If the graph does not have edge weights we raise an error.
        self.must_have_edge_weights()?;
        // Unwrap the weight to the default 0.0 if it was not given.
        let weight = weight.unwrap_or(0.0);
        // We create the dense binary adjacency matrix.
        let mut ajacency =
            vec![vec![weight; self.get_nodes_number() as usize]; self.get_nodes_number() as usize];
        // We wrap the adjacency into an object we can share between threads
        let thread_ajacency = ThreadDataRaceAware {
            value: UnsafeCell::new(&mut ajacency),
        };
        // We iterate on the edges and populate the matrix.
        self.par_iter_edge_node_ids_and_edge_weight()?
            .for_each(|(_, src, dst, weight)| unsafe {
                (*thread_ajacency.value.get())[src as usize][dst as usize] = weight;
            });
        Ok(ajacency)
    }
}
