use super::Graph;
use super::types::*;
use hashbrown::{HashSet};
use rand::SeedableRng;
use rand::rngs::SmallRng;
use rand::seq::SliceRandom;

/// Implementation of algorithms relative to trees.
impl Graph {

    fn copy_from_index(
        &self,
        index:EdgeT,
        sources:& mut Vec<NodeT>,
        destinations:& mut Vec<NodeT>,
        weights:& mut Vec<WeightT>,
        edge_types:& mut Vec<EdgeTypeT>
    ) {
        let src = self.sources[index];
        let dst = self.destinations[index];
        sources.push(src);
        destinations.push(dst);
        if let Some(w) = &self.weights {
            weights.push(w[index]);
        }
        if let Some(et) = &self.edge_types {
            edge_types.push(et[index]);
        }
    }

    pub fn holdout(
        &self,
        seed:NodeT,
        train_percentage:f64
    )-> Result<(Graph, Graph), String>{
        let tree:HashSet<(NodeT, NodeT, Option<EdgeTypeT>)> = self.spanning_tree(seed);
        let mut used_edges:HashSet<(NodeT, NodeT, Option<EdgeTypeT>)> = HashSet::new();
        
        // generate and shuffle the indices of the edges
        let mut rng = SmallRng::seed_from_u64((seed ^ 0xBAD5eedBAD5eed11) as u64);
        let mut edge_indices: Vec<NodeT> = (0..self.get_edges_number()).collect();
        edge_indices.shuffle(&mut rng);

        let valid_edges_number = (self.get_edges_number() as f64*(1.0-train_percentage)) as usize;
        let train_edges_number = (self.get_edges_number() as f64*train_percentage) as usize;
        let mut valid_edges_number_total = 0;

        let mut valid_sources: Vec<NodeT> = Vec::with_capacity(valid_edges_number);
        let mut valid_destinations: Vec<NodeT> = Vec::with_capacity(valid_edges_number);
        let mut valid_weights: Vec<WeightT> = Vec::with_capacity(valid_edges_number);
        let mut valid_edge_types: Vec<EdgeTypeT> = Vec::with_capacity(valid_edges_number);

        let mut train_sources: Vec<NodeT> = Vec::with_capacity(train_edges_number);
        let mut train_destinations: Vec<NodeT> = Vec::with_capacity(train_edges_number);
        let mut train_weights: Vec<WeightT> = Vec::with_capacity(train_edges_number);
        let mut train_edge_types: Vec<EdgeTypeT> = Vec::with_capacity(train_edges_number);

        for edge in edge_indices.iter() {
            let src = self.sources[*edge];
            let dst = self.destinations[*edge];
            let edge_type = if let Some(et) = &self.edge_types {
                Some(et[*edge])
            } else {
                None
            };
            // If the spanning tree does not include the current edge
            // and, if we are in an undirected graph, does not include neither
            // the graph in the opposite direction:
            if ! (tree.contains(&(src, dst, edge_type)) || ! self.is_directed && tree.contains(&(dst, src, edge_type))) {
                // We stop adding edges when we have reached the minimum amount.
                if valid_edges_number_total < valid_edges_number && (self.is_directed || !used_edges.contains(&(dst, src, edge_type))) {
                    // add the edge
                    //println!("Validation {}, {}", self.nodes_reverse_mapping[src], self.nodes_reverse_mapping[dst]);
                    self.copy_from_index(
                        *edge,
                        &mut valid_sources,
                        &mut valid_destinations,
                        &mut valid_weights,
                        &mut valid_edge_types
                    );
                    used_edges.insert((src, dst, edge_type));
                    valid_edges_number_total += 1;
                    if ! self.is_directed {
                        valid_edges_number_total += 1;
                    }
                    continue;
                }   
            }
            // Otherwise we add the edges to the training set.
            // 
            // When the graph is directed we need to check that the edge
            // in the opposite direction was not already inserted.
            if self.is_directed || !used_edges.contains(&(dst, src, edge_type)) {
                used_edges.insert((src, dst, edge_type));
                //println!("Training {}, {}", self.nodes_reverse_mapping[src], self.nodes_reverse_mapping[dst]);
                self.copy_from_index(
                    *edge,
                    &mut train_sources,
                    &mut train_destinations,
                    &mut train_weights,
                    &mut train_edge_types
                );
            }
        }

        Ok(if self.is_directed {
            (
                Graph::new_directed(
                    train_sources,
                    train_destinations,
                    Some(self.nodes_mapping.clone()),
                    Some(self.nodes_reverse_mapping.clone()),
                    self.node_types.clone(),
                    self.node_types_mapping.clone(),
                    self.node_types_reverse_mapping.clone(),
                    if self.edge_types.is_some() {
                        Some(train_edge_types)
                    } else {
                        None
                    },
                    self.edge_types_mapping.clone(),
                    self.edge_types_reverse_mapping.clone(),
                    if self.weights.is_some() {
                        Some(train_weights)
                    } else {
                        None
                    },
                    Some(true)
                )?,
                Graph::new_directed(
                    valid_sources,
                    valid_destinations,
                    Some(self.nodes_mapping.clone()),
                    Some(self.nodes_reverse_mapping.clone()),
                    self.node_types.clone(),
                    self.node_types_mapping.clone(),
                    self.node_types_reverse_mapping.clone(),
                    if self.edge_types.is_some() {
                        Some(valid_edge_types)
                    } else {
                        None
                    },
                    self.edge_types_mapping.clone(),
                    self.edge_types_reverse_mapping.clone(),
                    if self.weights.is_some() {
                        Some(valid_weights)
                    } else {
                        None
                    },
                    Some(true)
                )?
            )
        } else {
            let g1 = Graph::new_undirected(
                train_sources,
                train_destinations,
                Some(self.nodes_mapping.clone()),
                Some(self.nodes_reverse_mapping.clone()),
                self.node_types.clone(),
                self.node_types_mapping.clone(),
                self.node_types_reverse_mapping.clone(),
                if self.edge_types.is_some() {
                    Some(train_edge_types)
                } else {
                    None
                },
                self.edge_types_mapping.clone(),
                self.edge_types_reverse_mapping.clone(),
                if self.weights.is_some() {
                    Some(train_weights)
                } else {
                    None
                },
                Some(true),
                None
            ).unwrap();
            let g2 = Graph::new_undirected(
                valid_sources,
                valid_destinations,
                Some(self.nodes_mapping.clone()),
                Some(self.nodes_reverse_mapping.clone()),
                self.node_types.clone(),
                self.node_types_mapping.clone(),
                self.node_types_reverse_mapping.clone(),
                if self.edge_types.is_some() {
                    Some(valid_edge_types)
                } else {
                    None
                },
                self.edge_types_mapping.clone(),
                self.edge_types_reverse_mapping.clone(),
                if self.weights.is_some() {
                    Some(valid_weights)
                } else {
                    None
                },
                Some(true),
                None
            ).unwrap();
            (g1, g2)
        })
    }
}