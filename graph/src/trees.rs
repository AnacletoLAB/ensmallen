use super::*;
use indicatif::ProgressIterator;
use std::iter::FromIterator;
use roaring::{RoaringBitmap, RoaringTreemap};

fn find_node_set(sets: &[RoaringBitmap], node: NodeT) -> usize {
    sets.iter().position(|set| set.contains(node)).unwrap()
}

/// # Implementation of algorithms relative to trees.
impl Graph {
    /// Returns set of edges composing a spanning tree.
    ///
    /// The spanning tree is NOT minimal.
    /// The given seed is NOT the root of the tree.
    ///
    /// # Arguments
    ///
    /// * `seed`:NodeT - The seed to use for the holdout,
    /// * `include_all_edge_types`: bool - Wethever to include all the edges between two nodes.
    /// * `verbose`: bool - Wethever to show a loading bar or not.
    ///
    pub fn spanning_tree(
        &self,
        seed: EdgeT,
        include_all_edge_types: bool,
        verbose: bool,
    ) -> RoaringTreemap {
        let edges_number = self.get_edges_number();
        // Create vector of sets of the single nodes.
        let mut components: Vec<RoaringBitmap> = Vec::new();
        // Create empty vector of inserted values (this will be dense so its a normal BitVec)
        let mut inserted_nodes = RoaringBitmap::new();
        // Create the empty tree (this will be sparse on most graphs so roaring can save memory).
        let mut tree = RoaringTreemap::new();

        let pb = get_loading_bar(verbose, "Building spanning tree", edges_number as usize);

        for (edge_id, src, dst) in (seed..edges_number + seed)
            .progress_with(pb)
            .filter_map(|i| {
                let edge_id = i % edges_number;
                let (src, dst) = self.get_edge_from_edge_id(edge_id);
                match src == dst || !self.directed && src > dst {
                    true => None,
                    false => Some((edge_id, src, dst)),
                }
            })
        {
            let mut update_tree = false;
            if !inserted_nodes.contains(src) && !inserted_nodes.contains(dst) {
                inserted_nodes.insert(src);
                inserted_nodes.insert(dst);
                update_tree = true;
                components.push(RoaringBitmap::from_iter(vec![src, dst]));
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
            } else {
                let src_set_index = find_node_set(&components, src);
                let mut dst_set_index = find_node_set(&components, dst);
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
                }
            }

            if update_tree {
                tree.extend(self.compute_edge_ids_vector(
                    edge_id, src, dst, include_all_edge_types
                ))
            }
        }
        tree
    }
}
