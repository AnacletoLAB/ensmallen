use super::*;
use hashbrown::HashMap as HashBrownMap;
use itertools::Itertools;
use log::info;
use rayon::prelude::*;
use std::collections::{HashMap, HashSet};
use std::iter::FromIterator;

/// Returns outbounds edges ranges for graph.
///
/// # Arguments
///
/// * nodes_number: NodeT - Number of nodes in the graph.
/// * sources: &[NodeT] - source nodes in the graph.
///
pub fn compute_outbounds(nodes_number: NodeT, sources: &[NodeT]) -> Vec<EdgeT> {
    info!("Computing outbound edges ranges from each node.");
    let mut last_src: NodeT = 0;
    // Instead of fixing the last values after the loop, we set directly
    // all values to the length of the sources, which is the sum of all
    // possible neighbors.
    let mut outbounds: Vec<EdgeT> = vec![sources.len(); nodes_number];

    for (i, src) in sources.iter().enumerate() {
        if last_src != *src {
            // Assigning to range instead of single value, so that traps
            // have as delta between previous and next node zero.
            for o in &mut outbounds[last_src..*src] {
                *o = i;
            }
            last_src = *src;
        }
    }

    outbounds
}

pub(crate) fn build_nodes_mapping(
    sources: &[NodeT],
    destinations: &[NodeT],
) -> (Vec<NodeT>, Vec<NodeT>, HashMap<String, NodeT>, Vec<String>) {
    let unique_nodes: Vec<NodeT> = vec![sources, destinations]
        .iter()
        .cloned()
        .flatten()
        .cloned()
        .unique()
        .collect();
    let nodes_mapping: HashMap<String, NodeT> = unique_nodes
        .iter()
        .enumerate()
        .map(|(i, node_id)| (node_id.to_string(), i as NodeT))
        .collect();
    let mut nodes_reverse_mapping: Vec<String> = vec![String::from(""); unique_nodes.len()];
    for (node_name, position) in nodes_mapping.iter() {
        nodes_reverse_mapping[*position] = node_name.clone();
    }

    (
        sources
            .par_iter()
            .map(|node| *nodes_mapping.get(&node.to_string()).unwrap())
            .collect(),
        destinations
            .par_iter()
            .map(|node| *nodes_mapping.get(&node.to_string()).unwrap())
            .collect(),
        nodes_mapping,
        nodes_reverse_mapping,
    )
}

/// # Graph Constructors
impl Graph { 
    pub(crate) fn setup_graph(
        &self,
        sources: Vec<NodeT>,
        destinations: Vec<NodeT>,
        edge_types: Option<Vec<EdgeTypeT>>,
        weights: Option<Vec<WeightT>>,
    ) -> Result<Graph, String> {
        let mut result = Graph::builder(
            sources,
            destinations, 
            self.is_directed,
        ).add_node_mapping(
            self.nodes_mapping.clone(),
            self.nodes_reverse_mapping.clone()
        );

        if self.node_types.is_some() {
            // TODO! this is horrible but I don't know a clean way
            // to extract the content of an option without destroing it like the
            // unwrap does
            if let Some(node_types) = &self.node_types {
                if let Some(node_types_mapping) = &self.node_types_mapping {
                    if let Some(node_types_reverse_mapping) = &self.node_types_reverse_mapping {
                        result = result.add_node_types(
                            node_types.clone(),
                            node_types_mapping.clone(),
                            node_types_reverse_mapping.clone()
                        );
                    }
                }
            }
        }

        if self.edge_types.is_some() {
            // TODO! this is horrible but I don't know a clean way
            // to extract the content of an option without destroing it like the
            // unwrap does
            if let Some(_edge_types) = &edge_types {
                if let Some(edge_types_mapping) = &self.edge_types_mapping {
                    if let Some(edge_types_reverse_mapping) = &self.edge_types_reverse_mapping {
                        result = result.add_node_types(
                            _edge_types.clone(),
                            edge_types_mapping.clone(),
                            edge_types_reverse_mapping.clone()
                        );
                    }
                }
            }
        }

        if self.weights.is_some() {
            if let Some(_weights) = &weights{
                result = result.add_weights(_weights.clone());
            }
        }

        result.build(None)
    }

    pub fn builder(sources: Vec<NodeT>, destinations: Vec<NodeT>,  is_directed: bool) -> Graph {
        Graph {
            sources: sources,
            destinations: destinations,
            is_directed: is_directed,
            is_builded: false,
            has_traps: true,
            unique_edges: HashBrownMap::new(),
            not_trap_nodes: Vec::new(),
            outbounds: Vec::new(),
            nodes_mapping: HashMap::new(),
            nodes_reverse_mapping: Vec::new(),
            weights: None,
            edge_types: None,
            node_types: None,
            node_types_mapping: None,
            node_types_reverse_mapping: None,
            edge_types_mapping: None,
            edge_types_reverse_mapping: None,
        }
    }

    pub fn add_node_mapping(
        mut self,
        nodes_mapping: HashMap<String, NodeT>,
        nodes_reverse_mapping: Vec<String>,
    ) -> Graph {
        self.nodes_mapping = nodes_mapping;
        self.nodes_reverse_mapping = nodes_reverse_mapping;
        self.is_builded = false;
        self
    }

    pub fn add_node_types(
        mut self,
        node_types: Vec<NodeTypeT>,
        node_types_mapping: HashMap<String, NodeTypeT>,
        node_types_reverse_mapping: Vec<String>,
    ) -> Graph {
        self.node_types = Some(node_types);
        self.node_types_mapping = Some(node_types_mapping);
        self.node_types_reverse_mapping = Some(node_types_reverse_mapping);
        self.is_builded = false;
        self
    }

    pub fn add_edge_types(
        mut self,
        edge_types: Vec<EdgeTypeT>,
        edge_types_mapping: HashMap<String, EdgeTypeT>,
        edge_types_reverse_mapping: Vec<String>,
    ) -> Graph {
        self.edge_types = Some(edge_types);
        self.edge_types_mapping = Some(edge_types_mapping);
        self.edge_types_reverse_mapping = Some(edge_types_reverse_mapping);
        self.is_builded = false;
        self
    }

    pub fn add_weights(
        mut self,
        weights: Vec<WeightT>
    ) -> Graph {
        self.weights = Some(weights);
        self.is_builded = false;
        self
    }

    pub fn build(mut self, force_conversion_to_undirected: Option<bool>) -> Result<Graph, String> {
        if self.is_builded {
            return Ok(self);
        }
        if self.nodes_mapping.is_empty() || self.nodes_reverse_mapping.is_empty() {
            let (_sources, _destinations, _nodes_mapping, _nodes_reverse_mapping) = build_nodes_mapping(&self.sources, &self.destinations);
            self.sources = _sources;
            self.destinations = _destinations;
            self.nodes_mapping = _nodes_mapping;
            self.nodes_reverse_mapping = _nodes_reverse_mapping;
        } 

        validate(
            &self.sources,
            &self.destinations,
            &self.nodes_mapping,
            &self.nodes_reverse_mapping,
            &self.node_types,
            &self.edge_types,
            &self.weights,
        )?;

        if ! self.is_directed {
            let _force_conversion_to_undirected = force_conversion_to_undirected.unwrap_or(false);
            let mut full_sources: Vec<NodeT> = Vec::new();
            let mut full_destinations: Vec<NodeT> = Vec::new();
            let mut full_edge_types: Vec<NodeTypeT> = Vec::new();
            let mut full_weights: Vec<WeightT> = Vec::new();
            let mut unique_edges: HashSet<(NodeT, NodeT, Option<EdgeTypeT>)> = HashSet::new();

            for index in 0..self.sources.len() {
                let src = self.sources[index];
                let dst = self.destinations[index];
                let edge_type = if let Some(et) = &self.edge_types {
                    Some(et[index])
                } else {
                    None
                };
                if !unique_edges.contains(&(src, dst, edge_type)) {
                    full_sources.push(src);
                    full_destinations.push(dst);
                    if let Some(w) = &self.weights {
                        full_weights.push(w[index]);
                    }
                    let edge_type = if let Some(et) = &self.edge_types {
                        full_edge_types.push(et[index]);
                        Some(et[index])
                    } else {
                        None
                    };

                    unique_edges.insert((src, dst, edge_type));
                    // If the two current nodes are not the same, hence this is
                    // not a self-loop, we also add the opposite direction.
                    if src != dst {
                        full_sources.push(dst);
                        full_destinations.push(src);
                        if let Some(w) = &self.weights {
                            full_weights.push(w[index]);
                        }

                        if let Some(et) = &edge_type {
                            full_edge_types.push(*et);
                        }

                        unique_edges.insert((dst, src, edge_type));
                    }
                } else if !_force_conversion_to_undirected {
                    return Err(format!(
                        concat!(
                            "Within given edges there are birectional directed edges.\n",
                            "The source node is {src_name} ({src})\n",
                            "The destination node is {dst_name} ({dst})\n",
                            "{edge_type_message}\n",
                            "This means you are forcibly converting a directed ",
                            "graph into an undirected graph.\n",
                            "You can enforce the conversion by passing the flag ",
                            "force_conversion_to_undirected as true.\n",
                            "The conversion will ignore edges that are ",
                            "directed between two nodes, have the same edge type ",
                            "but different weights.\n",
                            "For example, an edge from A to B of type 1 ",
                            "with weight 10 would be inserted alongside ",
                            "the simmetric counter part B to A of type 1 ",
                            "but a following edge from B to A of type 1 ",
                            "with weight 5 would be ignored."
                        ),
                        src_name = self.nodes_reverse_mapping[src],
                        src = src,
                        dst_name = self.nodes_reverse_mapping[dst],
                        dst = dst,
                        edge_type_message = if let Some(et) = edge_type {
                            format!("The edge type is {}", et)
                        } else {
                            String::from("No edge type was provided for the edge.")
                        }
                    ));
                }
            }
            self.sources = full_sources;
            self.destinations = full_destinations;
            self.edge_types = 
                if !full_edge_types.is_empty() {
                    Some(full_edge_types)
                } else {
                    None
                };
            self.weights = 
                if !full_weights.is_empty() {
                    Some(full_weights)
                } else {
                    None
                }; 
        }

        let nodes_number = self.nodes_reverse_mapping.len();

        info!("Computing unique edges.");
        self.unique_edges = HashBrownMap::from_iter(
            self.sources
                .iter()
                .cloned()
                .zip(self.destinations.iter().cloned())
                .enumerate()
                .map(|(i, (src, dst))| ((src, dst), i)),
        );

        info!("Computing sorting of given edges based on sources.");
        let mut pairs: Vec<(usize, NodeT)> = self.sources.par_iter().cloned().enumerate().collect();
        pairs.par_sort_unstable_by_key(|(_, v)| *v);
        let indices: Vec<usize> = pairs.par_iter().map(|(i, _)| *i).collect();

        info!("Sorting given sources.");
        self.sources = indices.par_iter().map(|&x| self.sources[x]).collect();
        info!("Sorting given destinations.");
        self.destinations = 
            indices.par_iter().map(|&x| self.destinations[x]).collect();
        info!("Sorting given weights.");
        self.weights =
            self.weights.map(|w| indices.par_iter().map(|&x| w[x]).collect());
        info!("Sorting given edge types.");
        self.edge_types =
            self.edge_types.map(|et| indices.par_iter().map(|&x| et[x]).collect());

        self.outbounds = compute_outbounds(nodes_number, &self.sources);

        self.not_trap_nodes = self.sources.iter().cloned().unique().collect::<Vec<NodeT>>();

        // Here we are computing if the graph has any trap nodes.
        // When a graph has no traps, we can use a faster random walk.
        self.has_traps = (0..self.get_nodes_number())
            .into_par_iter()
            .any(|node| self.is_node_trap(node));

        self.is_builded = true;
        Ok(self)
    }

}
