use super::types::*;
use super::Graph;
use std::cmp::min;
use std::collections::HashSet;

/// # Tarjan algorithm
impl Graph {
    /// Returns list of nodes of the various strongly connected components.
    ///
    /// This is an implementation of Tarjan algorithm.
    ///
    pub fn strongly_connected_components(&self) -> Vec<HashSet<NodeT>> {
        let mut indexed_mask: Vec<bool> = vec![false; self.get_nodes_number() as usize];
        let mut stacked_mask: Vec<bool> = vec![false; self.get_nodes_number() as usize];
        let mut low_indices: Vec<NodeT> = vec![0; self.get_nodes_number() as usize];
        let mut indices: Vec<NodeT> = vec![0; self.get_nodes_number() as usize];
        let mut components_stack: Vec<NodeT> = Vec::new();
        let mut components: Vec<HashSet<NodeT>> = Vec::new();
        let mut common_index = 0;
        let mut recurse: bool;
        for node in self.iter_node_ids() {
            if !indexed_mask[node as usize] {
                let mut to_visit: Vec<(NodeT, usize)> = vec![(node, 0)];
                while !to_visit.is_empty() {
                    let (src, i) = to_visit.pop().unwrap();
                    if !indexed_mask[src as usize] {
                        low_indices[src as usize] = common_index;
                        indices[src as usize] = common_index;
                        indexed_mask[src as usize] = true;
                        stacked_mask[src as usize] = true;
                        common_index += 1;
                        components_stack.push(src);
                    }
                    recurse = false;
                    let (_min, _max) =
                        unsafe { self.get_unchecked_minmax_edge_ids_from_source_node_id(src) };
                    // Consider successors of source node
                    for (j, dst) in ((_min + i as EdgeT).._max)
                        .map(|edge_id| unsafe {
                            self.get_unchecked_destination_node_id_from_edge_id(edge_id)
                        })
                        .enumerate()
                    {
                        if !indexed_mask[dst as usize] {
                            // Successor w has not yet been visited; recurse on it
                            to_visit.push((src, i + j + 1));
                            to_visit.push((dst, 0));
                            recurse = true;
                            break;
                        } else if stacked_mask[dst as usize] {
                            // Successor w is in stack S and hence in the current SCC
                            // If w is not on stack, then (v, w) is an edge pointing to an SCC already found and must be ignored
                            // Note: The next line may look odd - but is correct.
                            // It says w.index not w.lowlink; that is deliberate and from the original paper
                            low_indices[src as usize] =
                                min(low_indices[src as usize], indices[dst as usize]);
                        }
                    }

                    if recurse {
                        continue;
                    }

                    // If source is a root node, pop the stack and generate an SCC
                    if low_indices[src as usize] == indices[src as usize] {
                        // start a new strongly connected component
                        let mut new_component: HashSet<NodeT> = HashSet::new();
                        loop {
                            let dst = components_stack.pop().unwrap();
                            stacked_mask[dst as usize] = false;
                            new_component.insert(dst);
                            if dst == src {
                                break;
                            }
                        }
                        components.push(new_component);
                    }

                    if !to_visit.is_empty() {
                        let (root, _) = to_visit.last().unwrap();
                        low_indices[*root as usize] =
                            min(low_indices[*root as usize], low_indices[src as usize]);
                    }
                }
            }
        }
        components
    }
}
