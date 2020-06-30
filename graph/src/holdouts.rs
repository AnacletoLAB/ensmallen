use super::Graph;
use super::types::*;
use hashbrown::{HashSet};
use rand::thread_rng;
use rand::prngs::SmallRng;
use rand::seq::SliceRandom;

/// Implementation of algorithms relative to trees.
impl Graph {

    fn copy_from_index(
        &self,
        index:EdgeT,
        sources:& mut Vec<NodeT>,
        destinations:& mut Vec<NodeT>,
        weights:& mut Vec<WeightT>,
        edge_types:& mut Vec<WeightT>
    ) {
        let src = self.sources[index];
        let dst = self.destinations[index];
        sources.push(src);
        destinations.push(dst);
        if let Some(w) = self.weights {
            weights.push(w[index]);
        }
        if let Some(et) = self.edge_types {
            edge_types.push(et[index]);
        }
    }

    pub fn holdout(
        &self,
        seed:NodeT,
        train_percentage:f64
    )-> (Graph, Graph){
        let (tree_src, tree_dst) = self.spanning_tree(seed);
        let tree:HashSet<(NodeT, NodeT)> = tree_src.iter().zip(tree_dst.iter()).collect();
        let edges_for_valid_set:Vec<bool> = vec![false, self.get_edges_number()];
        
        // generate and shuffle the indices of the edges
        let mut rng = SmallRng::seed_from_u64(seed ^ 0xBAD5eedBAD5eed11);
        let mut edge_indices: Vec<NodeT> = (0..self.get_edges_number()).collect();
        edge_indices.shuffle(&mut rng);

        let valid_edges_number = (self.get_edges_number() as f64*(1.0-train_percentage)) as usize;
        let train_edges_number = (self.get_edges_number() as f64*train_percentage) as usize;
        let valid_edges_number_total = 0;

        for index in edge_indices {
            let src = self.sources[index];
            let dst = self.destinations[index];
            if ! tree.contains(&(src, dst)){
                // add the edge
                edges_for_valid_set[index] = true;
                valid_edges_number_total += 1;
                // if undirected
                if !self.is_directed {
                    // add the reverse edg
                    let reverse_edge_id = self.get_edge_id(dst, src);
                    edges_for_valid_set[reverse_edge_id] = true;
                    valid_edges_number_total += 1;
                }
                // We stop adding edges when we have reached the minimum amount.
                if valid_edges_number_total >= valid_edges_number - 1 {
                    break;
                }
            }
        }

        let valid_sources: Vec<NodeT> = Vec::with_capacity(valid_edges_number);
        let valid_destinations: Vec<NodeT> = Vec::with_capacity(valid_edges_number);
        let valid_weights: Vec<WeightT> = Vec::with_capacity(valid_edges_number);
        let valid_edge_types: Vec<EdgeTypeT> = Vec::with_capacity(valid_edge_types);

        let train_sources: Vec<NodeT> = Vec::with_capacity(train_edges_number);
        let train_destinations: Vec<NodeT> = Vec::with_capacity(train_edges_number);
        let train_weights: Vec<WeightT> = Vec::with_capacity(train_edges_number);
        let train_edge_types: Vec<EdgeTypeT> = Vec::with_capacity(train_edge_types);

        for (index, for_valid) in edge_indices.iter().zip(edges_for_valid_set.iter()) {
            if for_valid {
                self.copy_from_index(
                    index,
                    valid_sources,
                    valid_destinations,
                    valid_weights,
                    valid_edge_types
                );
            } else {
                self.copy_from_index(
                    index,
                    train_sources,
                    train_destinations,
                    train_weights,
                    train_edge_types
                );
            }
        }

        if self.is_directed {
            (
                Graph::new_directed(
                    train_sources,
                    train_destinations,
                    self.nodes_mapping.clone(),
                    self.nodes_reverse_mapping.clone(),
                    self.node_types.clone(),
                    self.node_types_mapping.clone(),
                    self.node_types_reverse_mapping.clone(),
                    train_edge_types,
                    self.edge_types_mapping.clone(),
                    self.edge_types_reverse_mapping.clone(),
                    train_weights
                    Some(false)
                ),
                Graph::new_directed(
                    valid_sources,
                    valid_destinations,
                    self.nodes_mapping.clone(),
                    self.nodes_reverse_mapping.clone(),
                    self.node_types.clone(),
                    self.node_types_mapping.clone(),
                    self.node_types_reverse_mapping.clone(),
                    valid_edge_types,
                    self.edge_types_mapping.clone(),
                    self.edge_types_reverse_mapping.clone(),
                    valid_weights,
                    Some(false)
                )
            )
        } else {
            (
                Graph::new_undirected(
                    train_sources,
                    train_destinations,
                    self.nodes_mapping.clone(),
                    self.nodes_reverse_mapping.clone(),
                    self.node_types.clone(),
                    self.node_types_mapping.clone(),
                    self.node_types_reverse_mapping.clone(),
                    train_edge_types,
                    self.edge_types_mapping.clone(),
                    self.edge_types_reverse_mapping.clone(),
                    train_weights
                    Some(false)
                ),
                Graph::new_undirected(
                    valid_sources,
                    valid_destinations,
                    self.nodes_mapping.clone(),
                    self.nodes_reverse_mapping.clone(),
                    self.node_types.clone(),
                    self.node_types_mapping.clone(),
                    self.node_types_reverse_mapping.clone(),
                    valid_edge_types,
                    self.edge_types_mapping.clone(),
                    self.edge_types_reverse_mapping.clone(),
                    valid_weights,
                    Some(false)
                )
            )
        }
    }
}