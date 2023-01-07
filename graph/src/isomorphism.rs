use super::*;
use crate::hashes::Hasher;
use isomorphism_iter::EqualBucketsParIter;
use log::info;
use rayon::prelude::*;

impl Graph {
    /// Returns parallel iterator of vectors of isomorphic node groups IDs.
    ///
    /// # Arguments
    /// * `minimum_node_degree`: Option<NodeT> - Minimum node degree for the topological synonims. By default, 5.
    /// * `hash_strategy`: Option<&str> - The name of the hash strategy to be used. By default, `general` is used.
    /// * `number_of_neighbours_for_hash`: Option<usize> - The number of neighbours to consider for the hash. By default 10.
    /// * `hash_name`: Option<&str> - The name of the hash to be used.
    pub fn par_iter_isomorphic_node_ids_groups(
        &self,
        minimum_node_degree: Option<NodeT>,
        hash_strategy: Option<&str>,
        number_of_neighbours_for_hash: Option<usize>,
        hash_name: Option<&str>,
    ) -> Result<impl ParallelIterator<Item = Vec<NodeT>> + '_> {
        // If no minimum node degree is provided, we use arbitrarily 5.
        let minimum_node_degree = minimum_node_degree.unwrap_or(5);

        let hash_strategy = hash_strategy.unwrap_or("general");
        // TODO! update when we have good experimental results
        let hash_name = hash_name.unwrap_or("xxh3");
        let number_of_neighbours_for_hash = number_of_neighbours_for_hash.unwrap_or(10);

        // Validate the provided hash name
        let _ = Hasher::new(hash_name)?;

        let hash: fn(&Graph, NodeT, usize, &str) -> u32 = match hash_strategy {
            "general" => {
                |graph: &Graph, node_id: NodeT, number_of_neighbours_for_hash: usize, hash_name: &str| {
                    // First, we retrieve the 
                    let node_degree = unsafe{graph.get_unchecked_selfloop_adjusted_node_degree_from_node_id(node_id)};

                    let node_type_ids = unsafe{graph.get_unchecked_node_type_ids_from_node_id(node_id)};

                    let edge_type_ids = graph.edge_types.as_ref().as_ref().map(|ets| {
                        let (min_edge_id, max_edge_id) =
                        unsafe{graph.get_unchecked_minmax_edge_ids_from_source_node_id(node_id)};
                        &ets.ids[min_edge_id as usize..max_edge_id as usize]
                    });

                    let hasher = Hasher::new(hash_name).unwrap();

                    unsafe{graph.iter_unchecked_neighbour_node_ids_from_source_node_id(node_id)}
                        .enumerate()
                        .filter_map(|(_, dst)|{
                            let dst_node_degree = unsafe{graph.get_unchecked_selfloop_adjusted_node_degree_from_node_id(node_id)};
                            // We remove self-loops or nodes with the same node degree
                            // as these may be connected isomorphic nodes.
                            if dst == node_id || dst_node_degree == node_degree {
                                None
                            } else {
                                Some((dst, dst_node_degree, edge_type_ids.as_ref().map(|ids| ids[dst])))
                            }
                        }).take(number_of_neighbours_for_hash).for_each(|(node, node_degree, edge_type_id)|{
                            hasher.update((node, node_degree, edge_type_id));
                        });
                        hasher.digest()
                }
            },
            hash_strategy => {
                return Err(format!(
                    concat!(
                        "The provided hash strategy `{hash_strategy}` is not supported. ",
                        "The supported hash strategys are:\n",
                        "* `general`, which supports isomorphic connected nodes with self-loops.",
                        "* `unconnected`, which does not supports isomorphic connected nodes with self-loops."
                    ),
                    hash_strategy = hash_strategy
                ))
            }
        };

        // We collect the node IDs that have degree higher than the provided one.
        // TODO! Explore other possible hash!
        let mut degree_bounded_hash_and_node_ids = self
            .par_iter_node_ids()
            .zip(self.par_iter_node_degrees())
            .filter_map(|(node_id, node_degree)| {
                if node_degree < minimum_node_degree {
                    None
                } else {
                    Some((hash(&self, node_id, number_of_neighbours_for_hash, hash_name), node_id))
                }
            })
            .collect::<Vec<(u32, NodeT)>>();

        // Then we sort the nodes, according to the score.
        // TODO! This sorting operation is implemented using quicksort
        // and is general porpose, including support for swapping
        // large complex structs. This is overkill for our use
        // case, since we only need to sort u32s, and it is likely
        // we could re-implement this in an ad-hoc manner that
        // is sensibly faster.
        degree_bounded_hash_and_node_ids.par_sort_unstable();

        Ok(
            unsafe { EqualBucketsParIter::new(degree_bounded_hash_and_node_ids) }.flat_map(
                move |candidate_isomorphic_group_slice| {
                    // First, we proceed assuming for the best case scenario which
                    // would also be the fastest: if the `candidate_isomorphic_group_slice` is
                    // indeed an isomorphic group of nodes.
                    let first = candidate_isomorphic_group_slice[0].1;
                    // We proceed to count how many of these nodes are effectively isomorphic
                    // to the first one.
                    let number_of_initial_isomorphic_nodes = 1 + candidate_isomorphic_group_slice
                        [1..]
                        .iter()
                        .copied()
                        .take_while(|&(_, second)| unsafe {
                            self.are_unchecked_isomorphic_from_node_ids(first, second)
                        })
                        .count();

                    // If all of the nodes are isomorphic to the first node,
                    // then we have finished.
                    if number_of_initial_isomorphic_nodes == candidate_isomorphic_group_slice.len()
                    {
                        return vec![candidate_isomorphic_group_slice
                            .iter()
                            .map(|&(_, node_id)| node_id)
                            .collect::<Vec<NodeT>>()];
                    }

                    // We can do the same thing also for the case where we are only off by
                    // one node, since that is surely an hash singleton.
                    // Of course, we need to check that we would not be left with only
                    // a single node in the case of an slice of two candidate isomorphic nodes.
                    if number_of_initial_isomorphic_nodes > 1
                        && number_of_initial_isomorphic_nodes
                            == candidate_isomorphic_group_slice.len() - 1
                    {
                        return vec![candidate_isomorphic_group_slice
                            [..number_of_initial_isomorphic_nodes]
                            .iter()
                            .map(|&(_, node_id)| node_id)
                            .collect::<Vec<NodeT>>()];
                    }

                    // Otherwise, we are in a situation where either we have multiple
                    // isomorphic groups that were smashed togheter by an hash collision,
                    // or we have hash singletons, that is nodes that do not actually share
                    // the neighbours with these nodes but have the same hash.

                    // The two initial isomorphic groups are composed by
                    let mut candidate_isomorphic_groups: Vec<Vec<NodeT>> = vec![
                        // The nodes that we have checked as being isomorphic
                        candidate_isomorphic_group_slice[..number_of_initial_isomorphic_nodes]
                            .iter()
                            .map(|&(_, node_id)| node_id)
                            .collect::<Vec<NodeT>>(),
                        // The first node that appeared to be not isomorphic to the previous ones
                        vec![
                            candidate_isomorphic_group_slice[number_of_initial_isomorphic_nodes].1,
                        ],
                    ];

                    // We set a flag that determines whether we will need to filter out isomorphic groups with
                    // only a single element in them.
                    let mut number_of_isomorphic_groups_with_size_one =
                        if number_of_initial_isomorphic_nodes == 1 {
                            // If the number of isomorphic nodes we have managed to validate
                            // is nada, i.e. only the first one, we currently have two potentially hash singletons
                            // in the array `candidate_isomorphic_groups`.
                            2
                        } else {
                            // Otherwise, we have only one potential hash singleton in the array.
                            1
                        };
                    // We start to iterate to the nodes that immediately follow the last node that
                    // we have already checked previously, and we keep all of the subsequent nodes that have indeed the same local hash.
                    for (_, other_node_id) in candidate_isomorphic_group_slice
                        [(number_of_initial_isomorphic_nodes + 1)..]
                        .iter()
                        .copied()
                    {
                        // Then, since within the same hash there might be multiple isomorphic node groups in collision
                        // we need to identify which one of these groups is actually isomorphic with the current node.
                        if let Some(isomorphic_group) =
                            //
                            candidate_isomorphic_groups
                                .iter_mut()
                                .find(|candidate_isomorphic_group| unsafe {
                                    self.are_unchecked_isomorphic_from_node_ids(
                                        candidate_isomorphic_group[0],
                                        other_node_id,
                                    )
                                })
                        {
                            if isomorphic_group.len() == 1 {
                                number_of_isomorphic_groups_with_size_one -= 1;
                            }
                            isomorphic_group.push(other_node_id);
                        } else {
                            // We may have found another isomorphic group, or, possibly, a single node
                            // with a colliding hash. As such, we will need to verify whether this group
                            // will effectively grow or not.
                            number_of_isomorphic_groups_with_size_one += 1;
                            candidate_isomorphic_groups.push(vec![other_node_id]);
                        }
                    }
                    // We check whether there may be groups with a single node,
                    // which of course do not count as isomorphic groups
                    if number_of_isomorphic_groups_with_size_one > 0 {
                        candidate_isomorphic_groups
                            .drain_filter(|candidate_isomorphic_group| {
                                candidate_isomorphic_group.len() < 2
                            })
                            .for_each(|_| {});
                    }

                    candidate_isomorphic_groups
                },
            ),
        )
    }

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
                                !self.par_iter_edge_ids_with_known_edge_types().unwrap().any(
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

    /// Returns parallel iterator of vectors of isomorphic node groups names.
    ///
    /// # Arguments
    /// * `minimum_node_degree`: Option<NodeT> - Minimum node degree for the topological synonims. By default, 5.
    /// * `hash_strategy`: Option<&str> - The name of the hash to be used. By default, `general` is used.
    /// * `number_of_neighbours_for_hash`: Option<usize> - The number of neighbours to consider for the hash. By default 10.
    /// * `hash_name`: Option<&str> - The name of the hash to be used.
    pub fn par_iter_isomorphic_node_names_groups(
        &self,
        minimum_node_degree: Option<NodeT>,
        hash_strategy: Option<&str>,
        number_of_neighbours_for_hash: Option<usize>,
        hash_name: Option<&str>
    ) -> Result<impl ParallelIterator<Item = Vec<String>> + '_> {
        Ok(self
            .par_iter_isomorphic_node_ids_groups(
                minimum_node_degree,
                hash_strategy,
                number_of_neighbours_for_hash,
                hash_name
            )?
            .map(move |group| {
                group
                    .into_iter()
                    .map(|node_id| unsafe { self.get_unchecked_node_name_from_node_id(node_id) })
                    .collect()
            }))
    }

    #[no_numpy_binding]
    /// Returns vector with isomorphic node groups IDs.
    ///
    /// # Arguments
    /// * `minimum_node_degree`: Option<NodeT> - Minimum node degree for the topological synonims. By default, 5.
    /// * `hash_strategy`: Option<&str> - The name of the hash to be used. By default, `general` is used.
    /// * `number_of_neighbours_for_hash`: Option<usize> - The number of neighbours to consider for the hash. By default 10.
    /// * `hash_name`: Option<&str> - The name of the hash to be used.
    pub fn get_isomorphic_node_ids_groups(
        &self,
        minimum_node_degree: Option<NodeT>,
        hash_strategy: Option<&str>,
        number_of_neighbours_for_hash: Option<usize>,
        hash_name: Option<&str>
    ) -> Result<Vec<Vec<NodeT>>> {
        Ok(self
            .par_iter_isomorphic_node_ids_groups(
                minimum_node_degree,
                hash_strategy,
                number_of_neighbours_for_hash,
                hash_name
            )?
            .collect())
    }

    #[no_numpy_binding]
    /// Returns vector with isomorphic node groups names.
    ///
    /// # Arguments
    /// * `minimum_node_degree`: Option<NodeT> - Minimum node degree for the topological synonims. By default, 5.
    /// * `hash_strategy`: Option<&str> - The name of the hash to be used. By default, `general` is used.
    /// * `number_of_neighbours_for_hash`: Option<usize> - The number of neighbours to consider for the hash. By default 10.
    /// * `hash_name`: Option<&str> - The name of the hash to be used.
    pub fn get_isomorphic_node_names_groups(
        &self,
        minimum_node_degree: Option<NodeT>,
        hash_strategy: Option<&str>,
        number_of_neighbours_for_hash: Option<usize>,
        hash_name: Option<&str>,
    ) -> Result<Vec<Vec<String>>> {
        Ok(self
            .par_iter_isomorphic_node_names_groups(
                minimum_node_degree,
                hash_strategy,
                number_of_neighbours_for_hash,
                hash_name
            )?
            .collect())
    }

    /// Returns number of isomorphic node groups.
    ///
    /// # Arguments
    /// * `minimum_node_degree`: Option<NodeT> - Minimum node degree for the topological synonims. By default, 5.
    /// * `hash_strategy`: Option<&str> - The name of the hash strategy to be used. By default, `general` is used.
    /// * `number_of_neighbours_for_hash`: Option<usize> - The number of neighbours to consider for the hash. By default 10.
    /// * `hash_name`: Option<&str> - The name of the hash to be used.
    pub fn get_number_of_isomorphic_node_groups(
        &self,
        minimum_node_degree: Option<NodeT>,
        hash_strategy: Option<&str>,
        number_of_neighbours_for_hash: Option<usize>,
        hash_name: Option<&str>,
    ) -> Result<NodeT> {
        Ok(self.par_iter_isomorphic_node_ids_groups(minimum_node_degree, hash_strategy, number_of_neighbours_for_hash, hash_name)?
            .count() as NodeT)
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
