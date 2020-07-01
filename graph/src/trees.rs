use super::Graph;
use super::types::*;
use hashbrown::HashSet;

/// Implementation of algorithms relative to trees.
impl Graph {
    /// Returns set of edges composing a spanning tree.
    /// 
    /// The spanning tree is NOT minimal.
    /// The given seed is NOT the root of the tree.
    /// 
    pub fn spanning_tree(&self, seed:NodeT)-> HashSet<(NodeT, NodeT, Option<EdgeTypeT>)> {
        let edges_number = self.get_nodes_number();
        let mut bitmap:Vec<bool> = vec![false; self.get_nodes_number()];
        let mut tree:HashSet<(NodeT, NodeT, Option<EdgeTypeT>)> = HashSet::with_capacity(edges_number);
        
        for node in seed..self.get_nodes_number()+seed{
            let src = node % self.get_nodes_number();
            let (_min, _max) = self.get_min_max_edge(src);
            for neighbour in _min.._max {
                let dst = self.destinations[neighbour];
                if ! bitmap[dst] {
                    tree.insert((
                        src,
                        dst,
                        if let Some(et) = &self.edge_types {
                            Some(et[neighbour])
                        } else {
                            None
                        }
                    ));
                    bitmap[dst] = true;
                    bitmap[src] = true;
                }
            }
        }

        tree
    }
}