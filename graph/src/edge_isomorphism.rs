use super::*;
use crate::hashes::*;
use crate::isomorphism_iter::EqualBucketsParIter;
use rayon::prelude::*;

impl Graph {
    unsafe fn are_unchecked_isomorphic_from_node_id_sets(&self, first_node_id_set: &[NodeT], second_node_id_set: &[NodeT]) -> bool {
        let mut first = iter_set::difference_by(
            self.iter_unchecked_neighbour_node_and_edge_ids_union_from_multiple_source_node_ids(first_node_id_set),
            first_node_id_set.iter().copied().enumerate().map(|(padding, node_id)| (node_id, padding as EdgeT)),
            |(left, _), (right, _)| {
                left.cmp(&right)
            }
        );
        let mut second = iter_set::difference_by(
            self.iter_unchecked_neighbour_node_and_edge_ids_union_from_multiple_source_node_ids(second_node_id_set),
            second_node_id_set.iter().copied().enumerate().map(|(padding, node_id)| (node_id, padding as EdgeT)),
            |(left, _), (right, _)| {
                left.cmp(&right)
            }
        );
        
    }

    /// Returns whether two provided edge IDs are isomorphic to one another.
    ///
    /// # Arguments
    /// * `first_edge_id`: EdgeT - The first edge to check for.
    /// * `second_edge_id`: EdgeT - The first edge to check for.
    ///
    /// # Safety
    /// This method assumes that the two provided edge IDs are effectively within
    /// the set of edges in the graph. Out of bound errors might be raised with
    /// improper parametrization of the method.
    unsafe fn are_unchecked_isomorphic_from_edge_ids(
        &self,
        first_edge_id: EdgeT,
        second_edge_id: EdgeT,
    ) -> bool {
        if self.get_unchecked_edge_type_id_from_edge_id(first_edge_id)
            != self.get_unchecked_edge_type_id_from_edge_id(first_edge_id)
        {
            return false;
        }

        let (first_src, first_dst) = self.get_unchecked_node_ids_from_edge_id(first_edge_id);
        let (second_src, second_dst) = self.get_unchecked_node_ids_from_edge_id(second_edge_id);

        self.iter_unchecked_neighbour_node_and_edge_ids_union_from_multiple_source_node_ids(&[first_src, first_dst])

        true
    }

    unsafe fn get_unchecked_selfloop_adjusted_edge_degree_from_node_ids(
        &self,
        src: NodeT,
        dst: NodeT,
    ) -> NodeT {
        let src_degree =
            unsafe { self.get_unchecked_selfloop_adjusted_node_degree_from_node_id(src) };
        let dst_degree =
            unsafe { self.get_unchecked_selfloop_adjusted_node_degree_from_node_id(dst) };

        let edge_degree = src_degree + dst_degree - 2;

        edge_degree
    }

    /// Returns the hash associated to the provided edge.
    ///
    /// # Arguments
    /// * `src`: NodeT - The source node of the edge to be considered.
    /// * `dst`: NodeT - The destination node of the edge to be considered.
    /// * `edge_type_id`: Option<EdgeTypeT> - The edge type of the edge to be considered.
    /// * `number_of_neighbours_for_hash`: usize - the number of neighbours of the edge to be considered.
    ///
    /// # Safety
    /// It is assumed that the nodes provided exist in the graph and are connected.
    unsafe fn get_unchecked_edge_hash(
        &self,
        src: NodeT,
        dst: NodeT,
        edge_type_id: Option<EdgeTypeT>,
        number_of_neighbours_for_hash: usize,
    ) -> u32 {
        // First, we compute the self-loop corrected edge degree.
        let edge_degree = self.get_unchecked_selfloop_adjusted_edge_degree_from_node_ids(src, dst);

        // !TODO! Add support for node type IDS
        // let node_type_ids = unsafe { self.get_unchecked_node_type_ids_from_node_id(node_id) };
        // hasher.update(&node_type_ids);

        let src_edge_type_ids = self.edge_types.as_ref().as_ref().map(|ets| {
            let (min_edge_id, max_edge_id) =
                unsafe { self.get_unchecked_minmax_edge_ids_from_source_node_id(src) };
            &ets.ids[min_edge_id as usize..max_edge_id as usize]
        });

        let dst_edge_type_ids = self.edge_types.as_ref().as_ref().map(|ets| {
            let (min_edge_id, max_edge_id) =
                unsafe { self.get_unchecked_minmax_edge_ids_from_source_node_id(dst) };
            &ets.ids[min_edge_id as usize..max_edge_id as usize]
        });

        let mut hasher = Hasher::new("simple").unwrap();

        hasher.update(&edge_degree);
        hasher.update(&edge_type_id);

        let mut src_index = 0;
        let mut dst_index = 0;
        let mut considered_neighbours = 0;
        let src_neighbours = self
            .edges
            .get_unchecked_neighbours_node_ids_from_src_node_id(src);
        let dst_neighbours = self
            .edges
            .get_unchecked_neighbours_node_ids_from_src_node_id(dst);

        while (src_index < src_neighbours.len() || dst_index < dst_neighbours.len())
            && considered_neighbours < number_of_neighbours_for_hash
        {
            // We alternate adding edges from the source
            // and from the destination node to form the hash.
            for (node, index, neighbours, edge_types) in [
                (src, &mut src_index, src_neighbours, src_edge_type_ids),
                (dst, &mut dst_index, dst_neighbours, dst_edge_type_ids),
            ] {
                if *index < neighbours.len() {
                    let neighbour = neighbours[*index];
                    // If we have encountered an edge going to the other
                    // node, we can skip.
                    if node == src || node == dst {
                        *index += 1;
                        continue;
                    }
                    let this_edge_degree = self
                        .get_unchecked_selfloop_adjusted_edge_degree_from_node_ids(node, neighbour);
                    // This cannot be for sure an isomorphic edge
                    // of the same group as the current one
                    if edge_degree != this_edge_degree {
                        hasher.update(&(
                            neighbour,
                            this_edge_degree,
                            edge_types.as_ref().and_then(|ids| ids[*index]),
                        ));
                    }
                    *index += 1;
                }
            }
        }

        hasher.digest()
    }

    /// Returns parallel iterator of vectors of isomorphic edges groups IDs.
    ///
    /// # Arguments
    /// * `minimum_node_degree`: Option<NodeT> - Minimum node degree for each of the two nodes involved in the edge isomorphism. By default, 10.
    /// * `minimum_edge_degree`: Option<NodeT> - Minimum edge degree for the isomorphic edges, obtained as SUM of the source and destination node degrees. By default, 10.
    /// * `number_of_neighbours_for_hash`: Option<usize> - The number of neighbours to consider for the hash. By default 10.
    pub fn par_iter_isomorphic_edge_ids_groups(
        &self,
        minimum_node_degree: Option<NodeT>,
        minimum_edge_degree: Option<NodeT>,
        number_of_neighbours_for_hash: Option<usize>,
    ) -> Result<impl ParallelIterator<Item = Vec<EdgeT>> + '_> {
        // If the graph does not have edges, it is pointless.
        self.must_have_edges()?;

        // If no minimum node degree is provided, we use arbitrarily 10.
        let minimum_node_degree =
            minimum_node_degree.unwrap_or(10.min(self.get_maximum_node_degree().unwrap_or(0)));
        let number_of_neighbours_for_hash = number_of_neighbours_for_hash.unwrap_or(10);

        // We collect the node IDs that have degree higher than the provided one.
        let mut degree_bounded_hash_and_edge_ids: Vec<(u32, usize)> = self
            .par_iter_node_ids()
            .zip(self.par_iter_node_degrees())
            .filter(|(_, node_degree)| *node_degree > minimum_node_degree)
            .flat_map(|(src, src_node_degree)| {
                let (min_edge_id, max_edge_id) =
                    unsafe { self.get_unchecked_minmax_edge_ids_from_source_node_id(src) };
                let min_edge_id = min_edge_id as usize;
                let max_edge_id = max_edge_id as usize;
                let src_edge_type_ids = self
                    .edge_types
                    .as_ref()
                    .as_ref()
                    .map(|ets| &ets.ids[min_edge_id..max_edge_id]);
                unsafe { self.par_iter_unchecked_neighbour_node_ids_from_source_node_id(src) }
                    .zip(min_edge_id..max_edge_id)
                    .enumerate()
                    .filter(|(i, (dst, edge_id))| {
                        (self.is_directed() || src < *dst)
                            && unsafe { self.get_unchecked_node_degree_from_node_id(*dst) }
                                > minimum_node_degree
                    })
                    .map(|(i, (dst, edge_id))| {
                        (
                            unsafe {
                                self.get_unchecked_edge_hash(
                                    src,
                                    dst,
                                    src_edge_type_ids.as_ref().and_then(|ids| ids[i]),
                                    number_of_neighbours_for_hash,
                                )
                            },
                            edge_id,
                        )
                    })
            })
            .collect::<Vec<(u32, usize)>>();

        if degree_bounded_hash_and_edge_ids.len() <= 1 {
            return Err(format!(
                concat!(
                    "The provided parametrization in the current graph, ",
                    "including specifically minimum_node_degree=`{minimum_node_degree}`, ",
                    "has caused the list of degree-bounded nodes to be empty. ",
                    "Consider relaxing the constraints."
                ),
                minimum_node_degree = minimum_node_degree
            ));
        }

        // Then we sort the nodes, according to the score.
        // TODO! This sorting operation is implemented using quicksort
        // and is general porpose, including support for swapping
        // large complex structs. This is overkill for our use
        // case, since we only need to sort u32s, and it is likely
        // we could re-implement this in an ad-hoc manner that
        // is sensibly faster.
        degree_bounded_hash_and_edge_ids.par_sort_unstable();

        Ok(
            unsafe { EqualBucketsParIter::new(degree_bounded_hash_and_edge_ids) }.flat_map(
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
                        candidate_isomorphic_groups.retain(|candidate_isomorphic_group| {
                            candidate_isomorphic_group.len() > 1
                        });
                    }

                    candidate_isomorphic_groups
                },
            ),
        )
    }
}
