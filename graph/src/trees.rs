use super::Graph;
use super::types::*;


/// Implementation of algorithms relative to trees.
impl Graph {
    pub fn spanning_tree(&self, seed:NodeT)-> (Vec<NodeT>, Vec<NodeT>) {
        let nodes_number = self.get_nodes_number();
        let mut bitmap:Vec<bool> = vec![false; nodes_number];
        let mut sources:Vec<NodeT> = Vec::with_capacity(nodes_number);
        let mut destinations:Vec<NodeT> = Vec::with_capacity(nodes_number);
        
        for node in seed..nodes_number+seed{
            let _node = node % nodes_number;
            let (_min, _max) = self.get_min_max_edge(_node);
            bitmap[_node] = true;
            for neighbour in _min.._max {
                let dst = self.destinations[neighbour];
                if ! bitmap[dst] {
                    sources.push(_node);
                    destinations.push(dst);
                    bitmap[dst] = true;
                }
            }
        }

        (sources, destinations)
    }
}