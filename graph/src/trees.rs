use super::*;
use indicatif::ProgressIterator;
use roaring::{RoaringBitmap, RoaringTreemap};
use std::collections::HashSet;
use std::iter::FromIterator;
use vec_rand::xorshift::xorshift as rand_u64;

/// # Implementation of algorithms relative to trees.
impl Graph {
    fn iter_edges_from_random_state(
        &self,
        random_state: u64,
    ) -> impl Iterator<Item = (EdgeT, NodeT, NodeT)> + '_ {
        let edges_number = self.get_edges_number();
        // We execute two times the xorshift to improve the randomness of the seed.
        let updated_random_state = rand_u64(rand_u64(random_state ^ SEED_XOR as u64));
        (updated_random_state..edges_number + updated_random_state).filter_map(move |i| {
            let edge_id = i % edges_number;
            let (src, dst) = self.get_edge_from_edge_id(edge_id);
            match src == dst || !self.directed && src > dst {
                true => None,
                false => Some((edge_id, src, dst)),
            }
        })
    }

    fn iter_on_edges_with_preference<'a>(
        &'a self,
        random_state: u64,
        verbose: bool,
        unwanted_edge_types: &'a Option<HashSet<EdgeTypeT>>,
    ) -> impl Iterator<Item = (EdgeT, NodeT, NodeT)> + 'a {
        // TODO! FIX THIS CRASH if called with unwanted_edge_types and the graph does not have edge types.
        let result: Box<dyn Iterator<Item = (EdgeT, NodeT, NodeT)>> =
            if let Some(uet) = unwanted_edge_types {
                Box::new(
                    self.iter_edges_from_random_state(random_state)
                        .filter(move |(edge_id, _, _)| {
                            !uet.contains(&self.get_unchecked_edge_type(*edge_id).unwrap())
                        })
                        .chain(self.iter_edges_from_random_state(random_state).filter(
                            move |(edge_id, _, _)| {
                                uet.contains(&self.get_unchecked_edge_type(*edge_id).unwrap())
                            },
                        )),
                )
            } else {
                Box::new(self.iter_edges_from_random_state(random_state))
            };

        let pb = get_loading_bar(
            verbose,
            format!("Building spanning tree for {}", self.name).as_ref(),
            self.get_edges_number() as usize,
        );
        result.progress_with(pb)
    }

    /// Returns set of edges composing a spanning tree and connected components.
    ///
    /// The spanning tree is NOT minimal.
    /// The given random_state is NOT the root of the tree.
    ///
    /// # Arguments
    ///
    /// * `random_state`:NodeT - The random_state to use for the holdout,
    /// * `include_all_edge_types`: bool - Wethever to include all the edges between two nodes.
    /// * `unwanted_edge_types`: &Option<HashSet<EdgeTypeT>> - Which edge types id to try to avoid.
    /// * `verbose`: bool - Wethever to show a loading bar or not.
    ///
    pub fn spanning_tree(
        &self,
        random_state: EdgeT,
        include_all_edge_types: bool,
        unwanted_edge_types: &Option<HashSet<EdgeTypeT>>,
        verbose: bool,
    ) -> (RoaringTreemap, Vec<RoaringBitmap>) {
        // Create vector of sets of the single nodes.
        let mut components: Vec<RoaringBitmap> = Vec::new();
        // Create vector of nodes component numbers.
        let mut nodes_components: Vec<Option<usize>> = vec![None; self.get_nodes_number() as usize];
        // Create the empty tree (this will be sparse on most graphs so roaring can save memory).
        let mut tree = RoaringTreemap::new();

        // Iterate over all the edges and add and edge to the mst
        // iff the edge create, expand or merge components.
        for (edge_id, src, dst) in
            self.iter_on_edges_with_preference(random_state, verbose, unwanted_edge_types)
        {
            let mut update_tree = false;
            // if both nodes are not covered then the edge is isolated
            // and must start its own component
            if nodes_components[src as usize].is_none() && nodes_components[dst as usize].is_none()
            {
                update_tree = true;
                nodes_components[src as usize] = Some(components.len());
                nodes_components[dst as usize] = Some(components.len());
                components.push(RoaringBitmap::from_iter(vec![src, dst]));
            // if one of the nodes is covered then we are extending one componet.
            } else if nodes_components[src as usize].is_some()
                ^ nodes_components[dst as usize].is_some()
            {
                let (inserted, not_inserted) = if nodes_components[src as usize].is_some() {
                    (src, dst)
                } else {
                    (dst, src)
                };
                let inserted_component = nodes_components[inserted as usize].unwrap();
                components[inserted_component].insert(not_inserted);
                nodes_components[not_inserted as usize] = Some(inserted_component);

                update_tree = true;
            // if both are covered then we will insert the edge iff
            // its nodes are form different components, this way the edge will merge them
            // creating a single component
            } else {
                // if the components are different then we add it because it will merge them
                let src_component = nodes_components[src as usize].unwrap();
                let dst_component = nodes_components[dst as usize].unwrap();
                if src_component != dst_component {
                    let removed_component = components.remove(src_component);
                    nodes_components.iter_mut().for_each(|component_number| {
                        if let Some(cn) = component_number {
                            if *cn == src_component {
                                *cn = dst_component;
                            }
                            if *cn > src_component {
                                *cn -= 1;
                            }
                        }
                    });
                    components[nodes_components[dst as usize].unwrap()]
                        .union_with(&removed_component);
                    update_tree = true;
                    // else the edge is already covered
                }
            }

            if update_tree {
                tree.extend(self.compute_edge_ids_vector(edge_id, src, dst, include_all_edge_types))
            }
        }
        (tree, components)
    }
}
