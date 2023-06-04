use super::*;
//use crate::hashes::*;
use log::info;
use rayon::prelude::*;

impl Graph {
    /// Returns parallel iterator of vectors of approximated isomorphic node type group IDs.
    ///
    /// # TODO!: this approximation may not be correct and will require some more checks!
    pub fn par_iter_approximated_isomorphic_node_type_ids_groups(
        &self,
    ) -> Result<impl ParallelIterator<Item = Vec<NodeTypeT>> + '_> {
        info!("Computing node type hashes seeds.");
        let mut node_type_hashes = self
            .par_iter_unique_node_type_ids()?
            .map(|node_type_id| unsafe {
                0xDEADBEEFC0FEBABE_u64.wrapping_mul(
                    self.get_unchecked_number_of_nodes_from_node_type_id(node_type_id) as u64,
                )
            })
            .collect::<Vec<u64>>();

        info!("Computing node type hashes.");
        self.iter_node_ids_and_node_type_ids()
            .for_each(|(node_id, node_type_ids)| {
                if let Some(node_type_ids) = node_type_ids {
                    node_type_ids.iter().for_each(|&node_type_id| {
                        node_type_hashes[node_type_id as usize] =
                            (node_type_hashes[node_type_id as usize] ^ node_id as u64)
                                .wrapping_add(0x0A2126967AE81C95_u64);
                    });
                }
            });

        // First we create a vector with the unique node type IDs.
        let mut node_type_ids: Vec<NodeTypeT> = self
            .iter_unique_node_type_ids()?
            .filter(|&node_type_id| unsafe {
                self.get_unchecked_number_of_nodes_from_node_type_id(node_type_id) > 0
            })
            .collect();

        info!("Sorting hashes.");
        // Then we sort it according to their hash and node type ids so that
        // the buckets of common hash are sorted by node type id.
        node_type_ids.par_sort_unstable_by(|&a, &b| {
            match node_type_hashes[a as usize].cmp(&node_type_hashes[b as usize]) {
                std::cmp::Ordering::Equal => a.cmp(&b),
                x => x,
            }
        });

        info!("Computing isomorphic node types.");
        let considered_node_type_ids_number = node_type_ids.len();
        Ok((0..(considered_node_type_ids_number - 1))
            .into_par_iter()
            .filter_map(move |i| {
                let node_type_id = node_type_ids[i];
                // We only explore the group starters.
                let node_type_hash = node_type_hashes[node_type_id as usize];
                if i != 0 && node_type_hash == node_type_hashes[node_type_ids[i - 1] as usize]
                    || node_type_hash != node_type_hashes[node_type_ids[i + 1] as usize]
                {
                    return None;
                }
                Some(
                    (i..considered_node_type_ids_number)
                        .map(|j| node_type_ids[j])
                        .take_while(|&node_type_id| {
                            node_type_hash == node_type_hashes[node_type_id as usize]
                        })
                        .collect::<Vec<_>>(),
                )
            }))
    }

    /// Returns parallel iterator of vectors of isomorphic node type groups IDs.
    pub fn par_iter_isomorphic_node_type_ids_groups(
        &self,
    ) -> Result<impl ParallelIterator<Item = Vec<NodeTypeT>> + '_> {
        Ok(self
            .par_iter_approximated_isomorphic_node_type_ids_groups()?
            .filter_map(move |candidate_isomorphic_group| {
                let mut candidate_isomorphic_groups = vec![candidate_isomorphic_group];
                self.iter_node_ids_and_node_type_ids()
                    .for_each(|(_, node_type_ids)| {
                        if node_type_ids.is_none() {
                            return;
                        }

                        let node_type_ids = node_type_ids.unwrap();

                        let number_of_groups = candidate_isomorphic_groups.len();
                        let mut remove_empty_groups = false;

                        for index in 0..number_of_groups {
                            let candidate_isomorphic_group =
                                &mut candidate_isomorphic_groups[index];
                            let number_of_shared_elements = iter_set::intersection(
                                candidate_isomorphic_group.iter().copied(),
                                node_type_ids.iter().copied(),
                            )
                            .count();
                            if number_of_shared_elements == 0
                                || number_of_shared_elements == candidate_isomorphic_group.len()
                            {
                                // The group of node type IDs is still a valid candidate
                                continue;
                            }

                            // If the current isomorphic candidate group was composed of two node types
                            // and exclusively one of these was present in the node type IDs of the current
                            // node, then this is not an isomorphic group altogheter and must be removed.
                            if number_of_shared_elements == 1
                                && candidate_isomorphic_group.len() == 2
                            {
                                remove_empty_groups = true;
                                candidate_isomorphic_group.clear();
                                continue;
                            }

                            // If the number of shared elements is exactly one, we just need to remove this entity
                            // from the current isomorphic group.
                            if number_of_shared_elements == 1 {
                                let single_shared_node_type = iter_set::intersection(
                                    candidate_isomorphic_group.iter().copied(),
                                    node_type_ids.iter().copied(),
                                )
                                .nth(0)
                                .unwrap();
                                candidate_isomorphic_group.retain(|&node_type_id| {
                                    node_type_id != single_shared_node_type
                                });
                                continue;
                            }

                            // If the number of non shared elements is exactly one,
                            // we just need to remove this entity
                            // from the current isomorphic group.
                            if candidate_isomorphic_group.len() - number_of_shared_elements == 1 {
                                let single_non_shared_node_type = iter_set::difference(
                                    candidate_isomorphic_group.iter().copied(),
                                    node_type_ids.iter().copied(),
                                )
                                .nth(0)
                                .unwrap();
                                candidate_isomorphic_group.retain(|&node_type_id| {
                                    node_type_id != single_non_shared_node_type
                                });
                                continue;
                            }

                            let shared_node_type = iter_set::intersection(
                                candidate_isomorphic_group.iter().copied(),
                                node_type_ids.iter().copied(),
                            )
                            .collect::<Vec<_>>();

                            let different_node_type = iter_set::difference(
                                candidate_isomorphic_group.iter().copied(),
                                node_type_ids.iter().copied(),
                            )
                            .collect::<Vec<_>>();

                            *candidate_isomorphic_group = shared_node_type;
                            candidate_isomorphic_groups.push(different_node_type);
                        }

                        if remove_empty_groups {
                            candidate_isomorphic_groups.retain(|x| !x.is_empty());
                        }
                    });

                if candidate_isomorphic_groups.is_empty() {
                    None
                } else {
                    Some(candidate_isomorphic_groups)
                }
            })
            .flat_map(|candidate_isomorphic_groups| candidate_isomorphic_groups))
    }

    /// Returns parallel iterator of vectors of isomorphic edge type groups IDs.
    ///
    /// # Arguments
    /// * `minimum_number_of_edges`: Option<EdgeT> - Minimum number of edges to detect edge types topological synonims. By default, 5.
    pub fn par_iter_isomorphic_edge_type_ids_groups(
        &self,
        minimum_number_of_edges: Option<EdgeT>,
    ) -> Result<impl ParallelIterator<Item = Vec<EdgeTypeT>> + '_> {
        let minimum_number_of_edges = minimum_number_of_edges.unwrap_or(5);
        let edge_type_hashes = self
            .par_iter_unique_edge_type_ids()?
            .map(|edge_type_id| unsafe {
                let number_of_edges =
                    self.get_unchecked_number_of_edges_from_edge_type_id(edge_type_id);
                if number_of_edges < minimum_number_of_edges {
                    return 0;
                }
                let seed: u64 = 0xDEADBEEFC0FEBABE_u64.wrapping_mul(number_of_edges as u64);
                self.iter_edge_node_ids_from_edge_type_id(Some(edge_type_id), true)
                    .unwrap()
                    .take(50)
                    .map(|(src, dst)| (src as u64 ^ dst as u64).wrapping_add(0x0A2126967AE81C95))
                    .fold(seed, |a: u64, b: u64| {
                        (a ^ b).wrapping_add(0x0A2126967AE81C95)
                    })
            })
            .collect::<Vec<u64>>();
        // First we create a vector with the unique edge type IDs.
        let mut edge_type_ids: Vec<EdgeTypeT> = self
            .iter_unique_edge_type_ids()?
            .filter(|&edge_type_id| unsafe {
                self.get_unchecked_number_of_edges_from_edge_type_id(edge_type_id)
                    > minimum_number_of_edges
            })
            .collect();
        // Then we sort it according to the number of edges with this edge type.
        edge_type_ids.par_sort_unstable_by(|&a, &b| {
            edge_type_hashes[a as usize].cmp(&edge_type_hashes[b as usize])
        });
        let considered_edge_type_ids_number = edge_type_ids.len();
        Ok((0..(considered_edge_type_ids_number - 1))
            .into_par_iter()
            .filter_map(move |i| unsafe {
                let edge_type_id = edge_type_ids[i];
                // We only explore the group starters.
                let edge_type_hash = edge_type_hashes[edge_type_id as usize];
                if i != 0 && edge_type_hash == edge_type_hashes[edge_type_ids[i - 1] as usize]
                    || edge_type_hash != edge_type_hashes[edge_type_ids[i + 1] as usize]
                {
                    return None;
                }
                let mut candidate_isomorphic_groups = vec![vec![edge_type_id]];
                let mut filtering_is_necessary = false;
                for other_edge_type_id in ((i + 1)..considered_edge_type_ids_number)
                    .map(|j| edge_type_ids[j])
                    .take_while(|&edge_type_id| {
                        edge_type_hash == edge_type_hashes[edge_type_id as usize]
                    })
                {
                    if let Some(isomorphic_group) =
                        candidate_isomorphic_groups
                            .iter_mut()
                            .find(|candidate_isomorphic_group| {
                                let edge_type_id = candidate_isomorphic_group[0];
                                !self.par_iter_directed_edge_ids_with_known_edge_types().unwrap().any(
                                    |edge_id| {
                                        let edge_type_ids = self
                                            .iter_unchecked_edge_type_ids_from_edge_id(edge_id)
                                            .map(|edge_type_id| edge_type_id.unwrap())
                                            .collect::<Vec<EdgeTypeT>>();
                                        edge_type_ids.contains(&other_edge_type_id)
                                            ^ edge_type_ids.contains(&edge_type_id)
                                    },
                                )
                            })
                    {
                        isomorphic_group.push(other_edge_type_id);
                    } else {
                        filtering_is_necessary = true;
                        candidate_isomorphic_groups.push(vec![other_edge_type_id]);
                    }
                }
                if filtering_is_necessary {
                    candidate_isomorphic_groups = candidate_isomorphic_groups
                        .into_iter()
                        .filter(|candidate_isomorphic_group| candidate_isomorphic_group.len() > 1)
                        .collect();
                }
                if candidate_isomorphic_groups.is_empty() {
                    None
                } else {
                    Some(candidate_isomorphic_groups)
                }
            })
            .flat_map(|candidate_isomorphic_groups| candidate_isomorphic_groups))
    }

    /// Returns parallel iterator of vectors of isomorphic node types groups names.
    pub fn par_iter_isomorphic_node_type_names_groups(
        &self,
    ) -> Result<impl ParallelIterator<Item = Vec<String>> + '_> {
        Ok(self
            .par_iter_isomorphic_node_type_ids_groups()?
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
        Ok(self.par_iter_isomorphic_node_type_ids_groups()?.collect())
    }

    #[no_numpy_binding]
    /// Returns vector with isomorphic node type groups names.
    pub fn get_isomorphic_node_type_names_groups(&self) -> Result<Vec<Vec<String>>> {
        Ok(self.par_iter_isomorphic_node_type_names_groups()?.collect())
    }

    /// Returns number of isomorphic node type groups.
    pub fn get_number_of_isomorphic_node_type_groups(&self) -> Result<NodeTypeT> {
        Ok(self.par_iter_isomorphic_node_type_ids_groups()?.count() as NodeTypeT)
    }

    /// Returns parallel iterator of vectors of isomorphic node types groups names.
    pub fn par_iter_approximated_isomorphic_node_type_names_groups(
        &self,
    ) -> Result<impl ParallelIterator<Item = Vec<String>> + '_> {
        Ok(self
            .par_iter_approximated_isomorphic_node_type_ids_groups()?
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
    pub fn get_approximated_isomorphic_node_type_ids_groups(&self) -> Result<Vec<Vec<NodeTypeT>>> {
        Ok(self
            .par_iter_approximated_isomorphic_node_type_ids_groups()?
            .collect())
    }

    #[no_numpy_binding]
    /// Returns vector with isomorphic node type groups names.
    pub fn get_approximated_isomorphic_node_type_names_groups(&self) -> Result<Vec<Vec<String>>> {
        Ok(self
            .par_iter_approximated_isomorphic_node_type_names_groups()?
            .collect())
    }

    /// Returns number of isomorphic node type groups.
    pub fn get_number_of_approximated_isomorphic_node_type_groups(&self) -> Result<NodeTypeT> {
        Ok(self
            .par_iter_approximated_isomorphic_node_type_ids_groups()?
            .count() as NodeTypeT)
    }

    /// Returns parallel iterator of vectors of isomorphic edge types groups names.
    ///
    /// # Arguments
    /// * `minimum_number_of_edges`: Option<EdgeT> - Minimum number of edges to detect edge types topological synonims. By default, 5.
    pub fn par_iter_isomorphic_edge_type_names_groups(
        &self,
        minimum_number_of_edges: Option<EdgeT>,
    ) -> Result<impl ParallelIterator<Item = Vec<String>> + '_> {
        Ok(self
            .par_iter_isomorphic_edge_type_ids_groups(minimum_number_of_edges)?
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
    ///
    /// # Arguments
    /// * `minimum_number_of_edges`: Option<EdgeT> - Minimum number of edges to detect edge types topological synonims. By default, 5.
    pub fn get_isomorphic_edge_type_ids_groups(
        &self,
        minimum_number_of_edges: Option<EdgeT>,
    ) -> Result<Vec<Vec<EdgeTypeT>>> {
        Ok(self
            .par_iter_isomorphic_edge_type_ids_groups(minimum_number_of_edges)?
            .collect())
    }

    #[no_numpy_binding]
    /// Returns vector with isomorphic edge type groups names.
    ///
    /// # Arguments
    /// * `minimum_number_of_edges`: Option<EdgeT> - Minimum number of edges to detect edge types topological synonims. By default, 5.
    pub fn get_isomorphic_edge_type_names_groups(
        &self,
        minimum_number_of_edges: Option<EdgeT>,
    ) -> Result<Vec<Vec<String>>> {
        Ok(self
            .par_iter_isomorphic_edge_type_names_groups(minimum_number_of_edges)?
            .collect())
    }

    /// Returns number of isomorphic edge type groups.
    ///
    /// # Arguments
    /// * `minimum_number_of_edges`: Option<EdgeT> - Minimum number of edges to detect edge types topological synonims. By default, 5.
    pub fn get_number_of_isomorphic_edge_type_groups(
        &self,
        minimum_number_of_edges: Option<EdgeT>,
    ) -> Result<EdgeTypeT> {
        Ok(self
            .par_iter_isomorphic_edge_type_ids_groups(minimum_number_of_edges)?
            .count() as EdgeTypeT)
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
            (node_id + 1..self.get_number_of_nodes())
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

    /// Returns whether the set of provided node IDs have isomorphic node types.
    ///
    /// # Arguments
    /// * `node_ids`: &[NodeT] - Node IDs to check for.
    pub unsafe fn has_unchecked_isomorphic_node_types_from_node_ids(
        &self,
        node_ids: &[NodeT],
    ) -> bool {
        let node_type_ids = self.get_unchecked_node_type_ids_from_node_id(node_ids[0]);
        node_ids[1..]
            .par_iter()
            .all(|&node_id| node_type_ids == self.get_unchecked_node_type_ids_from_node_id(node_id))
    }

    /// Returns whether the set of provided node IDs have isomorphic node types.
    ///
    /// # Arguments
    /// * `node_ids`: &[NodeT] - Node IDs to check for.
    pub fn has_isomorphic_node_types_from_node_ids(&self, node_ids: &[NodeT]) -> Result<bool> {
        self.must_have_node_types()?;
        if node_ids.is_empty() {
            return Err("The provided list of node IDs is empty.".to_string());
        }
        if node_ids
            .par_iter()
            .any(|&node_id| self.validate_node_id(node_id).is_err())
        {
            return Err(
                "One of the provided node IDs is higher than the number of nodes in the graph."
                    .to_string(),
            );
        }
        Ok(unsafe { self.has_unchecked_isomorphic_node_types_from_node_ids(node_ids) })
    }
}
