use super::Graph;
use super::types::*;
use std::cmp::{min};

/// Implementation of Tarjan algorithm
impl Graph {

    fn strong_connection(
        &self,
        src: NodeT,
        indexed_mask:&mut Vec<bool>,
        stacked_mask:&mut Vec<bool>,
        low_indices:&mut Vec<NodeT>,
        indices:&mut Vec<NodeT>,
        stack:&mut Vec<NodeT>,
        common_index:&mut NodeT,
        components:&mut Vec<Vec<NodeT>>
    ){
        // Set the depth index for v to the smallest unused index
        low_indices[src] = *common_index;
        indices[src] = *common_index;
        indexed_mask[src] = true;
        stacked_mask[src] = true;
        *common_index += 1;
        stack.push(src);
      
        let (_min, _max) = self.get_min_max_edge(src);
        // Consider successors of source node
        for dst in (_min.._max).map(|node| self.destinations[node]){
            if !indexed_mask[dst]{
                // Successor w has not yet been visited; recurse on it
                self.strong_connection(
                    dst, 
                    indexed_mask, 
                    stacked_mask,
                    low_indices, 
                    indices, 
                    stack,
                    common_index,
                    components
                );
                low_indices[src] = min(low_indices[src], low_indices[dst]);
            } else if stacked_mask[dst]{
                // Successor w is in stack S and hence in the current SCC
                // If w is not on stack, then (v, w) is an edge pointing to an SCC already found and must be ignored
                // Note: The next line may look odd - but is correct.
                // It says w.index not w.lowlink; that is deliberate and from the original paper
                low_indices[src] = min(low_indices[src], indices[dst]);
            }
        }
      
        // If source is a root node, pop the stack and generate an SCC
        if low_indices[src] == indices[src]{
            // start a new strongly connected component
            let mut new_component:Vec<NodeT> = Vec::new();
            loop {
                let dst = stack.pop().unwrap();
                stacked_mask[dst] = false;
                new_component.push(dst);
                if dst == src{
                    break
                }
            }
            components.push(new_component);
        }
    }

    /// Returns list of nodes of the various strongly connected components.
    /// 
    /// This is an implementation of Tarjan algorithm.
    /// 
    pub fn strongly_connected_components(&self)-> Vec<Vec<NodeT>> {
        let mut indexed_mask:Vec<bool> = vec![false; self.get_nodes_number()];
        let mut stacked_mask:Vec<bool> = vec![false; self.get_nodes_number()];
        let mut low_indices:Vec<NodeT> = vec![0; self.get_nodes_number()];
        let mut indices:Vec<NodeT> = vec![0; self.get_nodes_number()];
        let mut stack:Vec<NodeT> = Vec::new();
        let mut components:Vec<Vec<NodeT>> = Vec::new();
        let mut common_index = 0;
        for src in 0..self.get_nodes_number(){
            if indexed_mask[src]{
                continue;
            }
            self.strong_connection(
                src, 
                &mut indexed_mask, 
                &mut stacked_mask,
                &mut low_indices, 
                &mut indices, 
                &mut stack,
                &mut common_index,
                &mut components
            );
        };
        components
    }
}