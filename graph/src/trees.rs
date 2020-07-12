use super::types::*;
use super::Graph;
use hashbrown::HashSet;

/// # Implementation of algorithms relative to trees.
impl Graph {
    /// Returns set of edges composing a spanning tree.
    ///
    /// The spanning tree is NOT minimal.
    /// The given seed is NOT the root of the tree.
    ///
    pub fn spanning_tree(&self, seed: NodeT) -> HashSet<(NodeT, NodeT, Option<EdgeTypeT>)> {
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
                            tree.insert((
                                node_to_explore,
                                dst,
                                if let Some(et) = &self.edge_types {
                                    Some(et[neighbour])
                                } else {
                                    None
                                },
                            ));
                            covered_nodes[src] = true;
                            covered_nodes[dst] = true;
                            stack.push(dst);
                        }
                    }
                }
            }
        }
        tree
    }
}
