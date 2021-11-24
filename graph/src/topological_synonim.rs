use super::*;
use bitvec::prelude::*;
use rayon::prelude::*;

impl Graph {
    /// Returns whether the two given node IDs are topological synonims.
    ///
    /// # Details
    /// Two nodes are topological synonims if they share the same set of neighbours,
    /// have the same node types (if any) and the edges towards the same set of neighbouts
    /// have the same weights and edge types (if it applies).
    /// TODO: add the check for the weights and edge types.
    ///
    /// # Safety
    /// The provided node IDs must exist in the current graph, or the current
    /// function will panic.
    ///
    /// # Arguments
    /// * `first_node_id`: NodeT - The first node to check.
    /// * `second_node_id`: NodeT - The second node to check.
    pub unsafe fn is_unchecked_topological_synonim_from_node_ids(
        &self,
        first_node_id: NodeT,
        second_node_id: NodeT,
    ) -> bool {
        // Preliminarly, we check that the two node IDs are not the same one.
        if first_node_id == second_node_id {
            return false;
        }
        // First we check if the two nodes have the same node degree.
        if self.get_node_degree_from_node_id(first_node_id)
            != self.get_node_degree_from_node_id(second_node_id)
        {
            // If they don't surely they do are not synonims.
            return false;
        }
        // Secondly, we check if the two nodes have the same node type.
        if self.get_node_type_ids_from_node_id(first_node_id)
            != self.get_node_type_ids_from_node_id(second_node_id)
        {
            // If they don't, surely it is not a synonim.
            return false;
        }
        // Thirdly, and the most expensive test, we check if the neighbours are the same.
        self.iter_unchecked_neighbour_node_ids_from_source_node_id(first_node_id)
            .zip(self.iter_unchecked_neighbour_node_ids_from_source_node_id(second_node_id))
            .all(|(first_node_neighbour_id, second_node_neighbour_id)| {
                first_node_neighbour_id == second_node_neighbour_id
            })
    }

    /// Returns iterator over the topological synonims of the provide node types.
    ///
    /// # Arguments
    /// * `node_id`: NodeT - The node ID to check for.
    /// * `skip_lower_node_ids`: Option<bool> - Whether to check lower node IDs. By default false.
    pub fn iter_topological_synonim_from_node_id(
        &self,
        node_id: NodeT,
        skip_lower_node_ids: Option<bool>,
    ) -> impl Iterator<Item = NodeT> + '_ {
        let skip_lower_node_ids = skip_lower_node_ids.unwrap_or(false);
        self.iter_node_ids()
            .filter(move |&this_node_id| !skip_lower_node_ids || this_node_id > node_id)
            .filter(move |&this_node_id| unsafe {
                self.is_unchecked_topological_synonim_from_node_ids(node_id, this_node_id)
            })
    }

    /// Returns parallel iterator over the topological synonims of the provide node types.
    ///
    /// # Arguments
    /// * `node_id`: NodeT - The node ID to check for.
    /// * `skip_lower_node_ids`: Option<bool> - Whether to check lower node IDs. By default false.
    pub fn par_iter_topological_synonim_from_node_id(
        &self,
        node_id: NodeT,
        skip_lower_node_ids: Option<bool>,
    ) -> impl ParallelIterator<Item = NodeT> + '_ {
        let skip_lower_node_ids = skip_lower_node_ids.unwrap_or(false);
        self.par_iter_node_ids()
            .filter(move |&this_node_id| !skip_lower_node_ids || this_node_id > node_id)
            .filter(move |&this_node_id| unsafe {
                self.is_unchecked_topological_synonim_from_node_ids(node_id, this_node_id)
            })
    }

    /// Returns topological synonims detected in the current graph.
    ///
    /// # Arguments
    /// * `minimum_node_degree`: Option<NodeT> - Minimum node degree for the topological synonims.
    pub fn iter_topological_synonims_node_ids(
        &self,
        minimum_node_degree: Option<NodeT>,
    ) -> impl Iterator<Item = Vec<NodeT>> + '_ {
        let minimum_node_degree = minimum_node_degree.unwrap_or(1);
        let mut topological_synonim_mask = bitvec![Lsb0, u8; 0; self.get_nodes_number() as usize];
        self.iter_node_ids().filter_map(move |node_id| unsafe {
            if topological_synonim_mask[node_id as usize] {
                return None;
            }
            let node_degree = self.get_unchecked_node_degree_from_node_id(node_id);
            if node_degree < minimum_node_degree {
                return None;
            }
            let mut topological_synonims = self
                .par_iter_topological_synonim_from_node_id(node_id, Some(true))
                .collect::<Vec<NodeT>>();
            if topological_synonims.is_empty() {
                None
            } else {
                topological_synonims.push(node_id);
                topological_synonims.iter().for_each(|&other_node_id| {
                    *topological_synonim_mask
                        .get_mut(other_node_id as usize)
                        .unwrap() = true;
                });
                Some(topological_synonims)
            }
        })
    }

    #[no_numpy_binding]
    /// Returns topological synonims detected in the current graph.
    ///
    /// # Arguments
    /// * `minimum_node_degree`: Option<NodeT> - Minimum node degree for the topological synonims.
    pub fn get_topological_synonims_node_ids(
        &self,
        minimum_node_degree: Option<NodeT>,
    ) -> Vec<Vec<NodeT>> {
        self.iter_topological_synonims_node_ids(minimum_node_degree)
            .collect()
    }

    /// Returns whether the current graph has topological synonims.
    ///
    /// # Arguments
    /// * `minimum_node_degree`: Option<NodeT> - Minimum node degree for the topological synonims.
    pub fn has_topological_synonims(&self, minimum_node_degree: Option<NodeT>) -> bool {
        self.iter_topological_synonims_node_ids(minimum_node_degree)
            .any(|_| true)
    }

    /// Returns whether the two given node IDs are topological synonims.
    ///
    /// # Details
    /// Two nodes are topological synonims if they share the same set of neighbours,
    /// have the same node types (if any) and the edges towards the same set of neighbouts
    /// have the same weights and edge types (if it applies).
    /// TODO: add the check for the weights and edge types.
    ///
    /// # Arguments
    /// * `first_node_id`: NodeT - The first node to check.
    /// * `second_node_id`: NodeT - The second node to check.
    ///
    /// # Raises
    /// * If one of more of the provided node IDs do not exist in the current graph.
    pub fn is_topological_synonim_from_node_ids(
        &self,
        first_node_id: NodeT,
        second_node_id: NodeT,
    ) -> Result<bool> {
        self.validate_node_id(first_node_id)?;
        self.validate_node_id(second_node_id)?;
        Ok(unsafe {
            self.is_unchecked_topological_synonim_from_node_ids(first_node_id, second_node_id)
        })
    }

    /// Returns whether the two given node names are topological synonims.
    ///
    /// # Details
    /// Two nodes are topological synonims if they share the same set of neighbours,
    /// have the same node types (if any) and the edges towards the same set of neighbouts
    /// have the same weights and edge types (if it applies).
    /// TODO: add the check for the weights and edge types.
    ///
    /// # Arguments
    /// * `first_node_name`: &str - The first node to check.
    /// * `second_node_name`: &str - The second node to check.
    ///
    /// # Raises
    /// * If one of more of the provided node names do not exist in the current graph.
    pub fn is_topological_synonim_from_node_names(
        &self,
        first_node_name: &str,
        second_node_name: &str,
    ) -> Result<bool> {
        let first_node_id = self.get_node_id_from_node_name(first_node_name)?;
        let second_node_id = self.get_node_id_from_node_name(second_node_name)?;
        Ok(unsafe {
            self.is_unchecked_topological_synonim_from_node_ids(first_node_id, second_node_id)
        })
    }
}
