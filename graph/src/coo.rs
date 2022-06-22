use super::*;
use num_traits::Zero;
use rayon::prelude::*;

impl Graph {
    /// Returns parallel iterator on coo matrix following the two provided metrics.
    ///
    /// # Arguments
    /// * `get_edge_weight`: fn(&Graph, NodeT, NodeT) -> WeightT - The closure providing the value for the edge weight.
    fn par_iter_transformed_coo_matrix<'a, T: Send + Sync>(
        &'a self,
        support: &'a T,
        get_edge_weight: fn(&T, NodeT, NodeT) -> WeightT,
    ) -> impl ParallelIterator<Item = (NodeT, NodeT, WeightT)> + 'a {
        self.par_iter_node_ids().flat_map(move |src| {
            self.par_iter_node_ids().filter_map(move |dst| {
                let edge_weight = get_edge_weight(support, src, dst);
                if edge_weight.is_zero() {
                    None
                } else {
                    Some((src, dst, edge_weight))
                }
            })
        })
    }

    /// Returns parallel iterator on Jaccard COO matrix.
    pub fn par_iter_jaccard_coo_matrix(
        &self,
    ) -> impl ParallelIterator<Item = (NodeT, NodeT, WeightT)> + '_ {
        self.par_iter_transformed_coo_matrix(self, |support, src, dst| unsafe {
            support.get_unchecked_jaccard_coefficient_from_node_ids(src, dst)
        })
    }

    /// Returns Jaccard coo matrix.
    pub fn get_jaccard_coo_matrix(&self) -> (Vec<(NodeT, NodeT)>, Vec<WeightT>) {
        self.par_iter_jaccard_coo_matrix()
            .map(|(src, dst, weight)| ((src, dst), weight))
            .unzip()
    }

    /// Returns parallel iterator on Adamic-Adar coo matrix following the two provided metrics.
    ///
    /// # Arguments
    /// * `get_edge_weight`: fn(&Graph, NodeT, NodeT) -> WeightT - The closure providing the value for the edge weight.
    pub fn par_iter_adamic_adar_coo_matrix(
        &self,
    ) -> impl ParallelIterator<Item = (NodeT, NodeT, WeightT)> + '_ {
        self.par_iter_transformed_coo_matrix(self, |support, src, dst| unsafe {
            support.get_unchecked_adamic_adar_index_from_node_ids(src, dst)
        })
    }

    /// Returns Adamic-adar coo matrix.
    pub fn get_adamic_adar_coo_matrix(&self) -> (Vec<(NodeT, NodeT)>, Vec<WeightT>) {
        self.par_iter_adamic_adar_coo_matrix()
            .map(|(src, dst, weight)| ((src, dst), weight))
            .unzip()
    }
}
