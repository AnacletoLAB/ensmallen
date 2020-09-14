use super::types::*;
use super::Graph;
use std::collections::HashSet;

#[macro_export]
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
macro_rules! min {
    ($a: expr, $b: expr) => {
        if $a < $b {
            $a
        } else {
            $b
        }
    };
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
        seed: NodeT,
        include_all_edge_types: bool,
    ) -> HashSet<(NodeT, NodeT, Option<EdgeTypeT>)> {
        let edges_number = self.get_nodes_number();
        let mut covered_nodes: Vec<bool> = vec![false; self.get_nodes_number()];
        let mut tree: HashSet<(NodeT, NodeT, Option<EdgeTypeT>)> =
            HashSet::with_capacity(edges_number);

        let n = self.get_nodes_number();
        for node in 0..n {
            // this is just (node + seed) % n but computed this way
            // we minimize the possibility of overflowing
            let src = ((node % n) + (seed % n)) % n;

            if !covered_nodes[src] {
                let mut stack: Vec<NodeT> = vec![src];
                while !stack.is_empty() {
                    let node_to_explore = stack.pop().unwrap();
                    let (_min, _max) = self.get_min_max_edge(node_to_explore);
                    for neighbour in _min.._max {
                        let dst = self.destinations[neighbour];
                        // If the destination node is not already covered and it
                        // does not match with the source node, hence the edge
                        // that is currently considered would be a self-loop
                        // we proceed to push the destination node and mark the
                        // nodes as covered.
                        if !covered_nodes[dst] && src != dst {
                            let first = if self.is_directed {
                                node_to_explore
                            } else {
                                min!(node_to_explore, dst)
                            };
                            let second = if self.is_directed {
                                dst
                            } else {
                                max!(node_to_explore, dst)
                            };
                            // Here when the user requests to include all the edge types
                            // between two nodes, which is very relevant in heterogeneous multi-graphs
                            // when the user intends to execute link-prediction on the resulting embedding
                            // and not link-type prediction, we include all the edges between the considered nodes.
                            let edge_types = if include_all_edge_types {
                                match &self.unique_edges.get(&(src, dst)).unwrap().edge_types {
                                    Some(ets) => {
                                        ets.iter().map(|et| Some(*et)).collect()
                                    },
                                    None => vec![None]
                                }
                            } else {
                                // Otherwise we only consider the one.
                                vec![if let Some(et) = &self.edge_types {
                                    Some(et.ids[neighbour])
                                } else {
                                    None
                                }]
                            };
                            // insert the edges in the tree
                            for edge_type in edge_types {
                                tree.insert((first, second, edge_type)); 
                            }           
                            // sign both src and dst as covered aka reachable        
                            covered_nodes[src] = true;
                            covered_nodes[dst] = true;
                            // push dst as new node that will be explored later
                            stack.push(dst);
                        }
                    }
                }
            }
        }
        tree
    }
}
