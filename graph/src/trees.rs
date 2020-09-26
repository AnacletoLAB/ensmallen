use super::types::*;
use super::Graph;
use std::collections::HashSet;

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
    ///
    pub fn spanning_tree(
        &self,
        seed: EdgeT,
        include_all_edge_types: bool,
    ) -> HashSet<(NodeT, NodeT, Option<EdgeTypeT>)> {
        let nodes_number = self.get_nodes_number();
        let edges_number = self.get_edges_number();
        // Create vector of sets of the single nodes.
        let mut components: Vec<HashSet<NodeT>> = (0..nodes_number)
            .map(|node_id| {
                let mut set = HashSet::new();
                set.insert(node_id);
                set
            })
            .collect();
        // Create the empty tree.
        let mut tree: HashSet<(NodeT, NodeT, Option<EdgeTypeT>)> =
            HashSet::with_capacity(nodes_number);

        for (edge_id, src, dst) in (seed..edges_number + seed).filter_map(|i| {
            let edge_id = i % edges_number;
            let (src, dst) = (self.sources[edge_id], self.destinations[edge_id]);
            match src == dst || !self.is_directed && src > dst {
                true => None,
                false => Some((edge_id, src, dst)),
            }
        }) {
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
                // Here when the user requests to include all the edge types
                // between two nodes, which is very relevant in heterogeneous multi-graphs
                // when the user intends to execute link-prediction on the resulting embedding
                // and not link-type prediction, we include all the edges between the considered nodes.
                let edge_types = if include_all_edge_types {
                    match self.get_link_edge_types(src, dst) {
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
            // If we have completed the spanning tree we can stop early.
            if components.len() == 1 {
                break;
            }
        }
        tree
    }
}
