use super::*;
use rayon::prelude::*;
use std::sync::atomic::{AtomicBool, AtomicU32, Ordering};

impl Graph {
    /// Returns parallel iterator of vectors of isomorphic node groups IDs.
    ///
    /// # Arguments
    /// * `minimum_node_degree`: Option<NodeT> - Minimum node degree for the topological synonims. By default, 5.
    /// * `k`: Option<NodeT> - Maximum number of groups to retrieve.
    pub fn par_iter_isomorphic_node_ids_groups(
        &self,
        minimum_node_degree: Option<NodeT>,
        k: Option<NodeT>,
    ) -> impl ParallelIterator<Item = Vec<NodeT>> + '_ {
        let minimum_node_degree = minimum_node_degree.unwrap_or(5);
        let isomorphisms: Vec<AtomicBool> = self
            .par_iter_node_ids()
            .map(|_| AtomicBool::new(false))
            .collect();
        let neighbours_sums = self
            .par_iter_node_ids()
            .map(|node_id| unsafe {
                self.iter_unchecked_neighbour_node_ids_from_source_node_id(node_id)
                    .map(|neighbour_id| neighbour_id as usize)
                    .sum()
            })
            .collect::<Vec<usize>>();
        let k = k.unwrap_or(self.get_nodes_number());
        let number_of_isomorphisms = AtomicU32::new(0);
        self.par_iter_node_ids().filter_map(move |node_id| unsafe {
            if number_of_isomorphisms.load(Ordering::Relaxed) >= k {
                return None;
            }
            if isomorphisms[node_id as usize].load(Ordering::Relaxed) {
                return None;
            }
            let node_degree = self.get_unchecked_node_degree_from_node_id(node_id);
            if node_degree < minimum_node_degree {
                return None;
            }
            let neughbour_sum = neighbours_sums[node_id as usize];
            let node_type = self.get_unchecked_node_type_ids_from_node_id(node_id);
            let mut isomorphic_group: Vec<NodeT> = (node_id + 1..self.get_nodes_number())
                .into_par_iter()
                .filter(|&other_node_id| {
                    neighbours_sums[other_node_id as usize] == neughbour_sum
                        && self.get_unchecked_node_degree_from_node_id(other_node_id) == node_degree
                        && self.get_unchecked_node_type_ids_from_node_id(other_node_id) == node_type
                        && self
                            .iter_unchecked_neighbour_node_ids_from_source_node_id(node_id)
                            .zip(self.iter_unchecked_neighbour_node_ids_from_source_node_id(
                                other_node_id,
                            ))
                            .all(|(first_node_neighbour_id, second_node_neighbour_id)| {
                                first_node_neighbour_id == second_node_neighbour_id
                            })
                })
                .collect();
            if isomorphic_group.is_empty() {
                None
            } else {
                number_of_isomorphisms.fetch_add(1, Ordering::Relaxed);
                isomorphic_group.push(node_id);
                isomorphic_group.iter().for_each(|&node_id| {
                    isomorphisms[node_id as usize].store(true, Ordering::Relaxed);
                });
                Some(isomorphic_group)
            }
        })
    }

    /// Returns parallel iterator of vectors of isomorphic node type groups IDs.
    ///
    /// # Arguments
    /// * `k`: Option<NodeT> - Maximum number of groups to retrieve.
    pub fn par_iter_isomorphic_node_type_ids_groups(
        &self,
        k: Option<NodeTypeT>,
    ) -> Result<impl ParallelIterator<Item = Vec<NodeTypeT>> + '_> {
        let isomorphisms: Vec<AtomicBool> = self
            .par_iter_unique_node_type_ids()?
            .map(|_| AtomicBool::new(false))
            .collect();
        let k = k.unwrap_or(self.get_node_types_number()?);
        let number_of_isomorphisms = AtomicU32::new(0);
        Ok(self
            .par_iter_unique_node_type_ids()?
            .filter_map(move |node_type_id| unsafe {
                if number_of_isomorphisms.load(Ordering::Relaxed) >= k {
                    return None;
                }
                if isomorphisms[node_type_id as usize].load(Ordering::Relaxed) {
                    return None;
                }
                let node_type_count =
                    self.get_unchecked_number_of_nodes_from_node_type_id(node_type_id);
                let mut isomorphic_group = (node_type_id + 1
                    ..self.get_node_types_number().unwrap())
                    .into_par_iter()
                    .filter(|&other_node_type_id| {
                        self.get_unchecked_number_of_nodes_from_node_type_id(other_node_type_id)
                            == node_type_count
                            && self.par_iter_node_ids_and_node_type_ids().all(
                                |(_, node_type_ids)| {
                                    node_type_ids.map_or(true, |node_type_ids| {
                                        !(node_type_ids.contains(&other_node_type_id)
                                            ^ node_type_ids.contains(&node_type_id))
                                    })
                                },
                            )
                    })
                    .collect::<Vec<NodeTypeT>>();
                if isomorphic_group.is_empty() {
                    None
                } else {
                    number_of_isomorphisms.fetch_add(1, Ordering::Relaxed);
                    isomorphic_group.push(node_type_id);
                    isomorphic_group.iter().for_each(|&node_type_id| {
                        isomorphisms[node_type_id as usize].store(true, Ordering::Relaxed);
                    });
                    Some(isomorphic_group)
                }
            }))
    }

    /// Returns parallel iterator of vectors of isomorphic edge type groups IDs.
    ///
    /// # Arguments
    /// * `k`: Option<EdgeT> - Maximum number of groups to retrieve.
    pub fn par_iter_isomorphic_edge_type_ids_groups(
        &self,
        k: Option<EdgeTypeT>,
    ) -> Result<impl ParallelIterator<Item = Vec<EdgeTypeT>> + '_> {
        let isomorphisms: Vec<AtomicBool> = self
            .par_iter_unique_edge_type_ids()?
            .map(|_| AtomicBool::new(false))
            .collect();
        let k = k.unwrap_or(self.get_edge_types_number()?);
        let number_of_isomorphisms = AtomicU32::new(0);
        Ok(self
            .par_iter_unique_edge_type_ids()?
            .filter_map(move |edge_type_id| unsafe {
                if number_of_isomorphisms.load(Ordering::Relaxed) >= k {
                    return None;
                }
                if isomorphisms[edge_type_id as usize].load(Ordering::Relaxed) {
                    return None;
                }
                let edge_type_count =
                    self.get_unchecked_number_of_edges_from_edge_type_id(edge_type_id);
                let mut isomorphic_group = (edge_type_id + 1
                    ..self.get_edge_types_number().unwrap())
                    .into_par_iter()
                    .filter(|&other_edge_type_id| {
                        self.get_unchecked_number_of_edges_from_edge_type_id(other_edge_type_id)
                            == edge_type_count
                            && self.par_iter_edge_ids_with_known_edge_types().unwrap().all(
                                |edge_id| {
                                    let edge_type_ids = self
                                        .iter_unchecked_edge_type_ids_from_edge_id(edge_id)
                                        .map(|edge_type_id| edge_type_id.unwrap())
                                        .collect::<Vec<EdgeTypeT>>();
                                    !(edge_type_ids.contains(&other_edge_type_id)
                                        ^ edge_type_ids.contains(&edge_type_id))
                                },
                            )
                    })
                    .collect::<Vec<EdgeTypeT>>();
                if isomorphic_group.is_empty() {
                    None
                } else {
                    number_of_isomorphisms.fetch_add(1, Ordering::Relaxed);
                    isomorphic_group.push(edge_type_id);
                    isomorphic_group.iter().for_each(|&edge_type_id| {
                        isomorphisms[edge_type_id as usize].store(true, Ordering::Relaxed);
                    });
                    Some(isomorphic_group)
                }
            }))
    }

    /// Returns parallel iterator of vectors of isomorphic node groups names.
    ///
    /// # Arguments
    /// * `minimum_node_degree`: Option<NodeT> - Minimum node degree for the topological synonims. By default, 5.
    /// * `k`: Option<NodeT> - Maximum number of groups to retrieve.
    pub fn par_iter_isomorphic_node_names_groups(
        &self,
        minimum_node_degree: Option<NodeT>,
        k: Option<NodeT>,
    ) -> impl ParallelIterator<Item = Vec<String>> + '_ {
        self.par_iter_isomorphic_node_ids_groups(minimum_node_degree, k)
            .map(move |group| {
                group
                    .into_iter()
                    .map(|node_id| unsafe { self.get_unchecked_node_name_from_node_id(node_id) })
                    .collect()
            })
    }

    #[no_numpy_binding]
    /// Returns vector with isomorphic node groups IDs.
    ///
    /// # Arguments
    /// * `minimum_node_degree`: Option<NodeT> - Minimum node degree for the topological synonims. By default, 5.
    pub fn get_isomorphic_node_ids_groups(
        &self,
        minimum_node_degree: Option<NodeT>,
    ) -> Vec<Vec<NodeT>> {
        self.par_iter_isomorphic_node_ids_groups(minimum_node_degree, None)
            .collect()
    }

    #[no_numpy_binding]
    /// Returns vector with isomorphic node groups names.
    ///
    /// # Arguments
    /// * `minimum_node_degree`: Option<NodeT> - Minimum node degree for the topological synonims. By default, 5.
    pub fn get_isomorphic_node_names_groups(
        &self,
        minimum_node_degree: Option<NodeT>,
    ) -> Vec<Vec<String>> {
        self.par_iter_isomorphic_node_names_groups(minimum_node_degree, None)
            .collect()
    }

    /// Returns number of isomorphic node groups.
    ///
    /// # Arguments
    /// * `minimum_node_degree`: Option<NodeT> - Minimum node degree for the topological synonims. By default, 5.
    pub fn get_isomorphic_node_groups_number(&self, minimum_node_degree: Option<NodeT>) -> NodeT {
        self.par_iter_isomorphic_node_ids_groups(minimum_node_degree, None)
            .count() as NodeT
    }

    /// Returns parallel iterator of vectors of isomorphic node types groups names.
    ///
    /// # Arguments
    /// * `k`: Option<NodeTypeT> - Maximum number of groups to retrieve.
    pub fn par_iter_isomorphic_node_type_names_groups(
        &self,
        k: Option<NodeTypeT>,
    ) -> Result<impl ParallelIterator<Item = Vec<String>> + '_> {
        Ok(self
            .par_iter_isomorphic_node_type_ids_groups(k)?
            .map(move |group| {
                group
                    .into_iter()
                    .map(|node_type_id| {
                        self.get_node_type_name_from_node_type_id(node_type_id)
                            .unwrap()
                    })
                    .collect()
            }))
    }

    #[no_numpy_binding]
    /// Returns vector with isomorphic node type groups IDs.
    pub fn get_isomorphic_node_type_ids_groups(&self) -> Result<Vec<Vec<NodeTypeT>>> {
        Ok(self
            .par_iter_isomorphic_node_type_ids_groups(None)?
            .collect())
    }

    #[no_numpy_binding]
    /// Returns vector with isomorphic node type groups names.
    pub fn get_isomorphic_node_type_names_groups(&self) -> Result<Vec<Vec<String>>> {
        Ok(self
            .par_iter_isomorphic_node_type_names_groups(None)?
            .collect())
    }

    /// Returns number of isomorphic node type groups.
    pub fn get_isomorphic_node_type_groups_number(&self) -> Result<NodeTypeT> {
        Ok(self.par_iter_isomorphic_node_type_ids_groups(None)?.count() as NodeTypeT)
    }

    /// Returns parallel iterator of vectors of isomorphic edge types groups names.
    ///
    /// # Arguments
    /// * `k`: Option<NodeTypeT> - Maximum number of groups to retrieve.
    pub fn par_iter_isomorphic_edge_type_names_groups(
        &self,
        k: Option<NodeTypeT>,
    ) -> Result<impl ParallelIterator<Item = Vec<String>> + '_> {
        Ok(self
            .par_iter_isomorphic_edge_type_ids_groups(k)?
            .map(move |group| {
                group
                    .into_iter()
                    .map(|edge_type_id| {
                        self.get_edge_type_name_from_edge_type_id(edge_type_id)
                            .unwrap()
                    })
                    .collect()
            }))
    }

    #[no_numpy_binding]
    /// Returns vector with isomorphic edge type groups IDs.
    pub fn get_isomorphic_edge_type_ids_groups(&self) -> Result<Vec<Vec<EdgeTypeT>>> {
        Ok(self
            .par_iter_isomorphic_edge_type_ids_groups(None)?
            .collect())
    }

    #[no_numpy_binding]
    /// Returns vector with isomorphic edge type groups names.
    pub fn get_isomorphic_edge_type_names_groups(&self) -> Result<Vec<Vec<String>>> {
        Ok(self
            .par_iter_isomorphic_edge_type_names_groups(None)?
            .collect())
    }

    /// Returns number of isomorphic edge type groups.
    pub fn get_isomorphic_edge_type_groups_number(&self) -> Result<EdgeTypeT> {
        Ok(self.par_iter_isomorphic_edge_type_ids_groups(None)?.count() as EdgeTypeT)
    }

    /// Returns whether the current graph has topological synonims.
    ///
    /// # Arguments
    /// * `minimum_node_degree`: Option<NodeT> - Minimum node degree for the topological synonims.
    pub fn has_isomorphic_nodes(&self, minimum_node_degree: Option<NodeT>) -> bool {
        let minimum_node_degree = minimum_node_degree.unwrap_or(5);
        self.par_iter_node_ids().any(move |node_id| unsafe {
            let node_degree = self.get_unchecked_node_degree_from_node_id(node_id);
            if node_degree < minimum_node_degree {
                return false;
            }
            let node_type = self.get_unchecked_node_type_ids_from_node_id(node_id);
            (node_id + 1..self.get_nodes_number())
                .into_par_iter()
                .any(|other_node_id| {
                    self.get_unchecked_node_degree_from_node_id(other_node_id) == node_degree
                        && self.get_unchecked_node_type_ids_from_node_id(other_node_id) == node_type
                        && self
                            .iter_unchecked_neighbour_node_ids_from_source_node_id(node_id)
                            .zip(self.iter_unchecked_neighbour_node_ids_from_source_node_id(
                                other_node_id,
                            ))
                            .all(|(first_node_neighbour_id, second_node_neighbour_id)| {
                                first_node_neighbour_id == second_node_neighbour_id
                            })
                })
        })
    }
}
