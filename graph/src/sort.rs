use rayon::iter::IndexedParallelIterator;
use rayon::iter::IntoParallelIterator;
use rayon::iter::ParallelIterator;
use rayon::slice::ParallelSliceMut;

use super::*;

/// # Sorting algorithms.
impl Graph {
    /// Returns graph with node IDs sorted by increasing outbound node degree.
    pub fn sort_by_increasing_outbound_node_degree(&self) -> Graph {
        if self.has_nodes_sorted_by_increasing_outbound_node_degree() {
            return self.clone();
        }
        let mut node_ids_and_node_degrees =
            vec![(0 as usize, 0 as NodeT); self.get_number_of_nodes() as usize];
        self.par_iter_node_degrees()
            .enumerate()
            .collect_into_vec(&mut node_ids_and_node_degrees);
        node_ids_and_node_degrees.par_sort_unstable_by(|(_, node_degree_a), (_, node_degree_b)| {
            node_degree_a.cmp(node_degree_b)
        });
        let mut new_node_ids = vec![0 as NodeT; self.get_number_of_nodes() as usize];
        node_ids_and_node_degrees
            .into_par_iter()
            .map(|(node_id, _)| node_id as NodeT)
            .collect_into_vec(&mut new_node_ids);
        unsafe { self.remap_unchecked_from_node_ids(new_node_ids) }
    }

    /// Returns graph with node IDs sorted by decreasing outbound node degree.
    pub fn sort_by_decreasing_outbound_node_degree(&self) -> Graph {
        if self.has_nodes_sorted_by_decreasing_outbound_node_degree() {
            return self.clone();
        }
        let mut node_ids_and_node_degrees =
            vec![(0 as usize, 0 as NodeT); self.get_number_of_nodes() as usize];
        self.par_iter_node_degrees()
            .enumerate()
            .collect_into_vec(&mut node_ids_and_node_degrees);
        node_ids_and_node_degrees.par_sort_unstable_by(|(_, node_degree_a), (_, node_degree_b)| {
            node_degree_b.cmp(node_degree_a)
        });
        let mut new_node_ids = vec![0 as NodeT; self.get_number_of_nodes() as usize];
        node_ids_and_node_degrees
            .into_par_iter()
            .map(|(node_id, _)| node_id as NodeT)
            .collect_into_vec(&mut new_node_ids);
        unsafe { self.remap_unchecked_from_node_ids(new_node_ids) }
    }

    /// Returns graph with node IDs sorted by lexicographic order.
    pub fn sort_by_node_lexicographic_order(&self) -> Graph {
        if self.has_nodes_sorted_by_lexicographic_order() {
            return self.clone();
        }
        let mut node_ids_and_node_names =
            vec![(0 as usize, "".to_owned()); self.get_number_of_nodes() as usize];
        self.par_iter_node_names()
            .enumerate()
            .collect_into_vec(&mut node_ids_and_node_names);
        node_ids_and_node_names.par_sort_unstable_by(|(_, node_name_a), (_, node_name_b)| {
            node_name_a.cmp(node_name_b)
        });
        let mut new_node_ids = vec![0 as NodeT; self.get_number_of_nodes() as usize];
        node_ids_and_node_names
            .into_par_iter()
            .map(|(node_id, _)| node_id as NodeT)
            .collect_into_vec(&mut new_node_ids);
        unsafe { self.remap_unchecked_from_node_ids(new_node_ids) }
    }

    /// Returns topological sorting map using breadth-first search from the given node.
    ///
    /// # Arguments
    /// * `root_node_id`: NodeT - Node ID of node to be used as root of BFS
    ///
    /// # Raises
    /// * If the given root node ID does not exist in the graph
    pub fn get_bfs_topological_sorting_from_node_id(
        &self,
        root_node_id: NodeT,
    ) -> Result<Vec<NodeT>> {
        self.validate_node_id(root_node_id)?;
        let mut stack = vec![root_node_id];
        let mut topological_remapping = vec![NODE_NOT_PRESENT; self.get_number_of_nodes() as usize];
        topological_remapping[root_node_id as usize] = 0;
        let mut inserted_nodes_num = 1;

        while inserted_nodes_num != self.get_number_of_nodes() {
            if let Some(src) = stack.pop() {
                unsafe { self.iter_unchecked_neighbour_node_ids_from_source_node_id(src) }
                    .for_each(|dst| {
                        if topological_remapping[dst as usize] == NODE_NOT_PRESENT {
                            topological_remapping[dst as usize] = inserted_nodes_num;
                            inserted_nodes_num += 1;
                            stack.push(dst);
                        }
                    });
            } else {
                // find new node to explore
                let new_root_node = topological_remapping
                    .iter()
                    .position(|&x| x == NODE_NOT_PRESENT)
                    .unwrap() as NodeT;
                topological_remapping[new_root_node as usize] = inserted_nodes_num;
                inserted_nodes_num += 1;
                stack.push(new_root_node);
            }
        }

        Ok(topological_remapping)
    }

    /// Returns topological sorting reversed map using breadth-first search from the given node.
    ///
    /// # Arguments
    /// * `root_node_id`: NodeT - Node ID of node to be used as root of BFS
    ///
    /// # Raises
    /// * If the given root node ID does not exist in the graph
    pub fn get_reversed_bfs_topological_sorting_from_node_id(
        &self,
        root_node_id: NodeT,
    ) -> Result<Vec<NodeT>> {
        let bfs_topological_sorting =
            self.get_bfs_topological_sorting_from_node_id(root_node_id)?;
        let reversed_bfs_topological_sorting =
            ThreadDataRaceAware::new(vec![NODE_NOT_PRESENT; self.get_number_of_nodes() as usize]);

        bfs_topological_sorting
            .into_par_iter()
            .enumerate()
            .for_each(|(i, node_id)| unsafe {
                (&mut (*reversed_bfs_topological_sorting.value.get()))[node_id as usize] = i as NodeT;
            });

        Ok(reversed_bfs_topological_sorting.value.into_inner())
    }

    /// Returns graph with node IDs sorted using a BFS
    ///
    /// # Arguments
    /// * `root_node_id`: NodeT - Node ID of node to be used as root of BFS
    ///
    /// # Raises
    /// * If the given root node ID does not exist in the graph
    pub fn sort_by_bfs_topological_sorting_from_node_id(
        &self,
        root_node_id: NodeT,
    ) -> Result<Graph> {
        Ok(unsafe {
            self.remap_unchecked_from_node_ids(
                self.get_bfs_topological_sorting_from_node_id(root_node_id)?,
            )
        })
    }
}
