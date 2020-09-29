use super::types::*;
use super::Graph;
use indicatif::{ProgressBar, ProgressIterator, ProgressStyle};
use std::collections::HashSet;
use std::iter::FromIterator;

#[macro_export]
/// Macro that computes the maximum between two numbers
macro_rules! max {
    ($a: expr, $b: expr) => {
        if $a >= $b {
            $a
        } else {
            $b
        }
    };
}
#[macro_export]
/// Macro that computes the minimum between two numbers
macro_rules! min {
    ($a: expr, $b: expr) => {
        if $a < $b {
            $a
        } else {
            $b
        }
    };
}

fn find_node_set(sets: &[HashSet<NodeT>], node: NodeT) -> usize {
    sets.iter().position(|set| set.contains(&node)).unwrap()
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
    ) -> HashSet<(NodeT, NodeT, Option<EdgeTypeT>)> {
        let nodes_number = self.get_nodes_number();
        let edges_number = self.get_edges_number();
        // Create vector of sets of the single nodes.
        let mut components: Vec<HashSet<NodeT>> = Vec::new();
        // Create empty vector of inserted values
        let mut inserted_nodes = vec![false; nodes_number];
        // Create the empty tree.
        let mut tree: HashSet<(NodeT, NodeT, Option<EdgeTypeT>)> =
            HashSet::with_capacity(nodes_number);

        let pb = if verbose {
            let pb = ProgressBar::new(edges_number as u64);
            pb.set_draw_delta(edges_number as u64 / 100);
            pb.set_style(ProgressStyle::default_bar().template(
                "Building spanning tree {spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] ({pos}/{len}, ETA {eta})",
            ));
            pb
        } else {
            ProgressBar::hidden()
        };

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
            if !inserted_nodes[src] && !inserted_nodes[dst] {
                inserted_nodes[src] = true;
                inserted_nodes[dst] = true;
                update_tree = true;
                components.push(HashSet::from_iter(vec![src, dst]));
            } else if inserted_nodes[src] ^ inserted_nodes[dst] {
                let (inserted, not_inserted) = if inserted_nodes[src] {
                    (src, dst)
                } else {
                    (dst, src)
                };
                inserted_nodes[not_inserted] = true;
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
                // Here when the user requests to include all the edge types
                // between two nodes, which is very relevant in heterogeneous multi-graphs
                // when the user intends to execute link-prediction on the resulting embedding
                // and not link-type prediction, we include all the edges between the considered nodes.
                let edge_types = if include_all_edge_types {
                    match self.get_unchecked_link_edge_types(src, dst) {
                        Some(ets) => ets.iter().map(|et| Some(*et)).collect(),
                        None => vec![None],
                    }
                } else {
                    // Otherwise we only consider the edge itself
                    vec![if let Some(et) = &self.edge_types {
                        Some(et.ids[edge_id])
                    } else {
                        None
                    }]
                };
                // insert the edges in the tree
                for edge_type in edge_types {
                    tree.insert((src, dst, edge_type));
                }
            }
        }
        tree
    }
}
