use indicatif::ParallelProgressIterator;
use rayon::prelude::*;

use super::*;

impl Graph {
    #[manual_binding]
    /// Returns binary dense adjacency matrix.
    ///
    /// Beware of using this method on big graphs!
    /// It'll use all of your RAM!
    ///
    /// # Implementative notes
    /// On multigraphs this method will ignore multi-edges and treat
    /// those occurrences as would an homogeneous graph.
    ///
    /// # Arguments
    /// * `matrix`: &mut [bool] - The matrix to be populated, expected to be full of `false` values.
    pub fn populate_dense_binary_adjacency_matrix(&self, matrix: &mut [bool]) -> Result<()> {
        // We check that the provided matrix has the correct shape.
        if matrix.len() != (self.get_nodes_number() * self.get_nodes_number()) as usize {
            return Err(format!(
                concat!(
                    "The provided matrix has size {} but since this ",
                    "graph has {} nodes and therefore we expected ",
                    "a matrix with size {}."
                ),
                matrix.len(),
                self.get_nodes_number(),
                self.get_nodes_number() * self.get_nodes_number()
            ));
        }
        // Get the number of nodes.
        let number_of_nodes = self.get_nodes_number() as usize;
        // We wrap the adjacency into an object we can share between threads
        let matrix = ThreadDataRaceAware::new(matrix);
        // We iterate on the edges and populate the matrix.
        self.par_iter_directed_edge_node_ids()
            .for_each(|(_, src, dst)| unsafe {
                (*matrix.value.get())[(src as usize) * number_of_nodes + dst as usize] = true;
            });
        Ok(())
    }

    /// Populate the provided slice with the provided edge metric.
    ///
    /// Beware of using this method on big graphs!
    /// It'll use all of your RAM!
    ///
    /// # Implementative notes
    /// On multigraphs this method will ignore multi-edges and treat
    /// those occurrences as would an homogeneous graph.
    ///
    /// # Arguments
    /// * `matrix`: &mut [F] - The matrix to be populated.
    /// * `support`: &S - The support graph.
    /// * `get_edge_weight`: fn(&S, NodeT, NodeT) -> F - The callback to be used to compute the edge weight.
    /// * `verbose`: Option<bool> - Whether to show a loading bar.
    fn populate_dense_adjacency_matrix<F, S>(
        &self,
        matrix: &mut [F],
        support: &S,
        get_edge_weight: fn(&S, NodeT, NodeT) -> F,
        verbose: Option<bool>,
    ) -> Result<()>
    where
        F: Send + Sync,
        S: Send + Sync,
    {
        // We check that the provided matrix has the correct shape.
        if matrix.len() != (self.get_nodes_number() * self.get_nodes_number()) as usize {
            return Err(format!(
                concat!(
                    "The provided matrix has size {} but since this ",
                    "graph has {} nodes and therefore we expected ",
                    "a matrix with size {}."
                ),
                matrix.len(),
                self.get_nodes_number(),
                self.get_nodes_number() * self.get_nodes_number()
            ));
        }

        let pb = get_loading_bar(
            verbose.unwrap_or(true),
            "Computing Matrix",
            self.get_nodes_number() as usize,
        );

        // We iterate on the edges and populate the matrix.
        self.par_iter_node_ids()
            .progress_with(pb)
            .zip(matrix.par_chunks_mut(self.get_nodes_number() as usize))
            .for_each(|(src, row)| {
                self.iter_node_ids()
                    .zip(row.iter_mut())
                    .for_each(|(dst, weight)| {
                        *weight = get_edge_weight(support, src, dst);
                    });
            });
        Ok(())
    }

    #[manual_binding]
    /// Populate the provided slice with the shared ancestor sizes.
    ///
    /// # Arguments
    /// * `matrix`: &mut [f32] - The matrix to be populated.
    /// * `bfs`: &ShortestPathsResultBFS - The BFS object to use for the ancestors.
    /// * `verbose`: Option<bool> - Whether to show a loading bar.
    pub fn populate_shared_ancestors_size_adjacency_matrix(
        &self,
        matrix: &mut [f32],
        bfd: &ShortestPathsResultBFS,
        verbose: Option<bool>,
    ) -> Result<()>
    {
        self.populate_dense_adjacency_matrix(
            matrix,
            bfd,
            |support, src, dst| support.get_shared_ancestors_size(src, dst).unwrap(),
            verbose,
        )
    }

    #[manual_binding]
    /// Populate the provided slice with the shared ancestor Jaccard.
    ///
    /// # Arguments
    /// * `matrix`: &mut [f32] - The matrix to be populated.
    /// * `bfs`: &ShortestPathsResultBFS - The BFS object to use for the ancestors.
    /// * `verbose`: Option<bool> - Whether to show a loading bar.
    pub fn populate_shared_ancestors_jaccard_adjacency_matrix(
        &self,
        matrix: &mut [f32],
        bfd: &ShortestPathsResultBFS,
        verbose: Option<bool>,
    ) -> Result<()>
    {
        self.populate_dense_adjacency_matrix(
            matrix,
            bfd,
            |support, src, dst| support.get_ancestors_jaccard_index(src, dst).unwrap(),
            verbose,
        )
    }

    #[manual_binding]
    /// Returns binary weighted adjacency matrix.
    ///
    /// Beware of using this method on big graphs!
    /// It'll use all of your RAM!
    ///
    /// # Arguments
    /// * `matrix`: &mut [WeightT] - The matrix to be populated, expected to be full of the desired constant value.
    pub fn populate_dense_weighted_adjacency_matrix(&self, matrix: &mut [WeightT]) -> Result<()> {
        // If the graph does not have edge weights we raise an error.
        self.must_have_edge_weights()?;
        if matrix.len() != (self.get_nodes_number() * self.get_nodes_number()) as usize {
            return Err(format!(
                concat!(
                    "The provided matrix has size {} but since this ",
                    "graph has {} nodes and therefore we expected ",
                    "a matrix with size {}."
                ),
                matrix.len(),
                self.get_nodes_number(),
                self.get_nodes_number() * self.get_nodes_number()
            ));
        }
        // Get the number of nodes.
        let number_of_nodes = self.get_nodes_number() as usize;
        // We wrap the adjacency into an object we can share between threads
        let matrix = ThreadDataRaceAware::new(matrix);
        // We iterate on the edges and populate the matrix.
        self.par_iter_edge_node_ids_and_edge_weight()?
            .for_each(|(_, src, dst, weight)| unsafe {
                (*matrix.value.get())[(src as usize) * number_of_nodes + dst as usize] = weight;
            });
        Ok(())
    }
}
