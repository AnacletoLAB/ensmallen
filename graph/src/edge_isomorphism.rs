use super::*;
use crate::isomorphism_iter::EqualBucketsParIter;
use rayon::prelude::*;
use rtree::RadixTree;

impl Graph {
    unsafe fn get_mask_from_node_ids(&self, node_ids: &[NodeT]) -> u64 {
        node_ids
            .iter()
            .copied()
            .fold(0_u64, |mut hash, node| hash | (1 << (node % 64)))
    }

    unsafe fn get_hash_from_node_ids(
        &self,
        node_ids: &[NodeT],
        number_of_neighbours_for_hash: usize,
    ) -> u64 {
        iter_set::difference(
            self.iter_unchecked_neighbour_node_ids_union_from_multiple_source_node_ids(node_ids),
            node_ids.iter().copied(),
        )
        .take(number_of_neighbours_for_hash)
        .fold(0_u64, |mut hash, node| hash | (1 << (node % 64)))
    }

    unsafe fn are_unchecked_isomorphic_from_node_id_sets(
        &self,
        first_node_id_set: &[NodeT],
        second_node_id_set: &[NodeT],
    ) -> bool {
        let mut first = self
            .iter_unchecked_neighbour_node_and_edge_ids_union_from_multiple_source_node_ids(
                first_node_id_set,
            )
            .peekable();
        let mut second = self
            .iter_unchecked_neighbour_node_and_edge_ids_union_from_multiple_source_node_ids(
                second_node_id_set,
            )
            .peekable();

        // Counters for edges going FROM the group
        // to the SAME GROUP itself.
        let mut first_selfloops = 0;
        let mut second_selfloops = 0;

        // Counters for edges going FROM a group
        // to the OTHER group. These edges will be
        // certainly equal in undirected graphs,
        // but might now be in the context of directed
        // graphs. It remains relevant to check whether
        // the two groups are connected in order to verify
        // the topological isomorphism were there to be
        // self-loops in either groups.
        let mut first_to_second_connections = 0;
        let mut second_to_first_connections = 0;

        'outer: while let (Some((first_node, first_edge_id)), Some((second_node, second_edge_id))) =
            (first.peek(), second.peek())
        {
            // We start by evaluating whether we are dealing in either
            // the first or second isomorphic candidates with self-loops,
            // that is edges that go from any node in the isomorphic candidate
            // to any node in the SAME isomorphic candidate.
            // If so, we need to increase the relative counter and proceed onward.
            if first_node_id_set.contains(first_node) {
                first_selfloops += 1;
                first.advance_by(1).unwrap();
                continue 'outer;
            }

            if second_node_id_set.contains(second_node) {
                second_selfloops += 1;
                second.advance_by(1).unwrap();
                continue 'outer;
            }

            // Secondarily, we evaluate whether the first group
            // is connected to the second and viceversa.
            if second_node_id_set.contains(first_node) {
                first_to_second_connections += 1;
                first.advance_by(1).unwrap();
                continue 'outer;
            }

            if first_node_id_set.contains(second_node) {
                second_to_first_connections += 1;
                second.advance_by(1).unwrap();
                continue 'outer;
            }

            // Thirdly, and this is the most intuitive check
            // of all others, we need to evaluate whether
            // the two nodes are equal. If the two nodes
            // are not equal, we found a difference between the
            // two neighbourhoods and therefore the two candidate
            // isomorphisms are NOT isomorphic.
            if first_node != second_node {
                return false;
            }

            // We check whether the two edges connecting the neighbouring
            // node to the two candidate isomorphic groups are identical
            if self.get_unchecked_edge_type_id_from_edge_id(*first_edge_id)
                != self.get_unchecked_edge_type_id_from_edge_id(*second_edge_id)
            {
                return false;
            }

            // And finally, we check whether the two edges connecting the neighbouring
            // node to the two candidate isomorphic groups are identical
            if let (Some(first_weight), Some(second_weight)) = (
                self.get_unchecked_edge_weight_from_edge_id(*first_edge_id),
                self.get_unchecked_edge_weight_from_edge_id(*second_edge_id),
            ) {
                if (first_weight - second_weight).abs() > WeightT::EPSILON {
                    return false;
                }
            }

            first.advance_by(1).unwrap();
            second.advance_by(1).unwrap();
        }

        // We need to fully complete consuming both iterators.
        // It may happen that the previous loop finishes with
        // one iterator completed and the other one still with
        // some nodes.
        for (first_node, _first_edge_id) in first {
            // If this is a selfloop.
            if first_node_id_set.contains(&first_node) {
                first_selfloops += 1;
                continue;
            }

            // If this is an edge towards the other loop.
            if second_node_id_set.contains(&first_node) {
                first_to_second_connections += 1;
                continue;
            }

            // Otherwise this is a new node that no longer
            // matches the other iterator, so we can stop.
            return false;
        }

        for (second_node, _second_edge_id) in second {
            // If this is a selfloop.
            if second_node_id_set.contains(&second_node) {
                second_selfloops += 1;
                continue;
            }

            // If this is an edge towards the other loop.
            if first_node_id_set.contains(&second_node) {
                second_to_first_connections += 1;
                continue;
            }

            // Otherwise this is a new node that no longer
            // matches the other iterator, so we can stop.
            return false;
        }

        // We check that is one of the isomorphic groups
        // has self-loops, than the other one either has
        // self-loops or is connected to the first isomorphic group.
        if first_selfloops > 0 && !(second_selfloops > 0 || first_to_second_connections > 0)
            || second_selfloops > 0 && !(first_selfloops > 0 || second_to_first_connections > 0)
        {
            return false;
        }

        true
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
            != self.get_unchecked_edge_type_id_from_edge_id(second_edge_id)
        {
            return false;
        }

        let (first_src, first_dst) = self.get_unchecked_node_ids_from_edge_id(first_edge_id);
        let (second_src, second_dst) = self.get_unchecked_node_ids_from_edge_id(second_edge_id);

        self.are_unchecked_isomorphic_from_node_id_sets(
            &[first_src, first_dst],
            &[second_src, second_dst],
        )
    }

    #[no_numpy_binding]
    /// Returns parallel iterator of vectors of isomorphic edges groups IDs.
    ///
    /// # Arguments
    /// * `minimum_node_degree`: Option<NodeT> - Minimum node degree for each of the two nodes involved in the edge isomorphism. By default, 10.
    /// * `number_of_neighbours_for_hash`: Option<usize> - The number of neighbours to consider for the hash. By default 10.
    pub fn get_isomorphic_edge_ids_groups(
        &self,
        minimum_node_degree: Option<NodeT>,
        number_of_neighbours_for_hash: Option<usize>,
    ) -> Result<Vec<Vec<EdgeT>>> {
        // If the graph does not have edges, it is pointless.
        self.must_have_edges()?;

        // If no minimum node degree is provided, we use arbitrarily 10.
        let minimum_node_degree =
            minimum_node_degree.unwrap_or(10.min(self.get_maximum_node_degree().unwrap_or(0)));

        let number_of_neighbours_for_hash = number_of_neighbours_for_hash.unwrap_or(10);

        let mut tree: RadixTree<(EdgeT, u64, u64)> = RadixTree::new();

        // We collect the node IDs that have degree higher than the provided one.
        // TODO! MAKE PARALLEL! CURRENTLY PARALLEL NOT SUPPORTED BY RADIX TREE!
        self.iter_node_ids()
            .zip(self.iter_node_degrees())
            .filter(|(_, node_degree)| *node_degree > minimum_node_degree)
            .for_each(|(src, _src_node_degree)| {
                let (min_edge_id, max_edge_id) =
                    unsafe { self.get_unchecked_minmax_edge_ids_from_source_node_id(src) };
                unsafe { self.iter_unchecked_neighbour_node_ids_from_source_node_id(src) }
                    .zip(min_edge_id..max_edge_id)
                    .filter(move |(dst, _edge_id)| {
                        (self.is_directed() || src < *dst)
                            && unsafe { self.get_unchecked_node_degree_from_node_id(*dst) }
                                > minimum_node_degree
                    })
                    .for_each(|(dst, edge_id)| {
                        let mask = unsafe { self.get_mask_from_node_ids(&[src, dst]) };
                        let hash = unsafe {
                            self.get_hash_from_node_ids(&[src, dst], number_of_neighbours_for_hash)
                        };
                        tree.insert((edge_id, hash, mask), hash);
                    });
            });

        if tree.is_empty() {
            return Err(format!(
                concat!(
                    "The provided parametrization in the current graph, ",
                    "including specifically minimum_node_degree=`{minimum_node_degree}`, ",
                    "has caused the list of degree-bounded candidates to be empty. ",
                    "Consider relaxing the constraints."
                ),
                minimum_node_degree = minimum_node_degree
            ));
        }

        Ok(tree
            .iter()
            .copied()
            .flat_map(|(edge_id, hash, mask): (EdgeT, u64, u64)| {
                // First, we proceed assuming for the best case scenario which
                // would also be the fastest: if the `candidate_isomorphic_group_slice` is
                // indeed an isomorphic group of nodes.
                let candidate_isomorphic_group = tree.get_masked(hash, mask);
                let first_edge_id: EdgeT = edge_id;

                // TODO! FIND BETTER WAY TO SKIP CHECKING MULTIPLE TIMES!
                if candidate_isomorphic_group.iter().any(|slice| {
                    slice
                        .iter()
                        .any(|(other_edge_id, _, _)| *other_edge_id < edge_id)
                }) {
                    return Vec::new();
                }

                let mut candidate_isomorphic_groups: Vec<Vec<EdgeT>> = vec![vec![edge_id]];

                // We set a flag that determines whether we will need to filter out isomorphic groups with
                // only a single element in them.
                let mut number_of_isomorphic_groups_with_size_one = 1;
                // We start to iterate to the nodes that immediately follow the last node that
                // we have already checked previously, and we keep all of the subsequent nodes that have indeed the same local hash.
                candidate_isomorphic_group
                    .iter()
                    .flat_map(|slice| slice.iter().map(|(other_edge_id, _, _)| *other_edge_id))
                    .for_each(|other_edge_id: EdgeT| {
                        // TODO! HANDLE SELFLOOPS BETTTER!
                        if other_edge_id == edge_id {
                            return;
                        }

                        // Then, since within the same hash there might be multiple isomorphic node groups in collision
                        // we need to identify which one of these groups is actually isomorphic with the current node.
                        if let Some(isomorphic_group) =
                            //
                            candidate_isomorphic_groups
                                .iter_mut()
                                .find(|candidate_isomorphic_group| unsafe {
                                    self.are_unchecked_isomorphic_from_edge_ids(
                                        candidate_isomorphic_group[0],
                                        other_edge_id,
                                    )
                                })
                        {
                            if isomorphic_group.len() == 1 {
                                number_of_isomorphic_groups_with_size_one -= 1;
                            }
                            isomorphic_group.push(other_edge_id);
                        } else {
                            // We may have found another isomorphic group, or, possibly, a single node
                            // with a colliding hash. As such, we will need to verify whether this group
                            // will effectively grow or not.
                            number_of_isomorphic_groups_with_size_one += 1;
                            candidate_isomorphic_groups.push(vec![other_edge_id]);
                        }
                    });
                // We check whether there may be groups with a single node,
                // which of course do not count as isomorphic groups
                if number_of_isomorphic_groups_with_size_one > 0 {
                    candidate_isomorphic_groups
                        .retain(|candidate_isomorphic_group| candidate_isomorphic_group.len() > 1);
                }

                candidate_isomorphic_groups
            })
            .collect::<Vec<Vec<EdgeT>>>())
    }
}
