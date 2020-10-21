use super::*;
use indicatif::ProgressIterator;
use roaring::{RoaringBitmap, RoaringTreemap};
use std::collections::HashSet;
use std::iter::FromIterator;

fn find_node_set(sets: &[RoaringBitmap], node: NodeT) -> usize {
    sets.iter().position(|set| set.contains(node)).unwrap()
}

/// # Implementation of algorithms relative to trees.
impl Graph {
    fn iter_edges_from_random_state(&self, random_state: u64) -> impl Iterator<Item = (EdgeT, NodeT, NodeT)> + '_ {
        let edges_number = self.get_edges_number();
        (random_state..edges_number + random_state).filter_map(move |i| {
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
        let result: Box<dyn Iterator<Item = (EdgeT, NodeT, NodeT)>> = if let Some(uet) = unwanted_edge_types {
            Box::new(self.iter_edges_from_random_state(random_state)
                .filter(move |(edge_id, _, _)| {
                    uet.contains(&self.get_unchecked_edge_type(*edge_id).unwrap())
                })
                .chain(self.iter_edges_from_random_state(random_state).filter(move |(edge_id, _, _)| {
                    !uet.contains(&self.get_unchecked_edge_type(*edge_id).unwrap())
                })))
        } else {
            Box::new(self.iter_edges_from_random_state(random_state))
        };

        let pb = get_loading_bar(
            verbose,
            "Building spanning tree",
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
        // Create empty vector of inserted values (this will be dense so its a normal BitVec)
        let mut inserted_nodes = RoaringBitmap::new();
        // Create the empty tree (this will be sparse on most graphs so roaring can save memory).
        let mut tree = RoaringTreemap::new();

        // Iterate over all the edges and add and edge to the mst
        // iff the edge create, expand or merge components.
        for (edge_id, src, dst) in
            self.iter_on_edges_with_preference(random_state, verbose,unwanted_edge_types)
        {
            let mut update_tree = false;
            // if both nodes are not covered then the edge is isolated
            // and must start its own component
            if !inserted_nodes.contains(src) && !inserted_nodes.contains(dst) {
                inserted_nodes.insert(src);
                inserted_nodes.insert(dst);
                update_tree = true;
                components.push(RoaringBitmap::from_iter(vec![src, dst]));
            // if one of the nodes is covered then we are extending one componet.
            } else if inserted_nodes.contains(src) ^ inserted_nodes.contains(dst) {
                let (inserted, not_inserted) = if inserted_nodes.contains(src) {
                    (src, dst)
                } else {
                    (dst, src)
                };
                inserted_nodes.insert(not_inserted);
                let inserted_index = find_node_set(&components, inserted);
                components
                    .get_mut(inserted_index)
                    .unwrap()
                    .insert(not_inserted);
                update_tree = true;
            // if both are covered then we will insert the edge iff
            // its nodes are form different components, this way the edge will merge them
            // creating a single component
            } else {
                // get the components of the nodes
                let src_set_index = find_node_set(&components, src);
                let mut dst_set_index = find_node_set(&components, dst);
                // if the components are different then we add it because it will merge them
                if src_set_index != dst_set_index {
                    let src_set = components.remove(src_set_index);
                    if dst_set_index > src_set_index {
                        dst_set_index -= 1;
                    }
                    components
                        .get_mut(dst_set_index)
                        .unwrap()
                        .extend(src_set.iter());
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
