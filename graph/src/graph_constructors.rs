use super::*;
use log::info;
use hashbrown::{HashMap, HashSet};
use std::iter::FromIterator;
use rayon::prelude::*;

pub fn validate(
    sources: &[NodeT],
    destinations: &[NodeT],
    nodes_mapping: &HashMap<String, NodeT>,
    nodes_reverse_mapping: &[String],
    node_types: &Option<Vec<NodeTypeT>>,
    edge_types: &Option<Vec<EdgeTypeT>>,
    weights: &Option<Vec<WeightT>>
) -> Result<(), String> {

    info!("Checking that the nodes mappings are of the same length.");
    if nodes_mapping.len() != nodes_reverse_mapping.len() {
        return Err(format!("The size of the node_mapping ({}) does not match the size of the nodes_reverse_mapping ({}).",
            nodes_mapping.len(), nodes_reverse_mapping.len()
        ));
    }

    if let Some(nt) = &node_types {
        info!("Checking that nodes and node types are of the same length.");
        if nt.len() != nodes_reverse_mapping.len() {
            return Err(format!("The number of given nodes ({}) does not match the number of node_types ({}).",
                nt.len(), nodes_reverse_mapping.len()
            ));
        }
    }

    if let Some(nt) = &node_types{
        info!("Checking if every node used by the edges exists.");
        for node in sources.iter().chain(destinations.iter()) {
            if *node >= nt.len() {
                return Err(format!(
                    "A node provided with the edges ('{}') does not exists within given nodes.",
                    node
                ));
            }
        }
    }

    if let Some(w) = weights {
        info!("Checking for length between weights and given edges.");
        if w.len() != sources.len(){
            return Err(format!("Length of given weights ({}) does not match length of given edges ({}).",
            w.len(), sources.len()));
        }
        info!("Checking for non-zero weights.");
        for weight in w.iter() {
            if *weight == 0.0 {
                return Err(format!(
                    "One of the provided weights '{}' is either 0 or within float error to zero.",
                    weight
                ));
            }
            if *weight < 0.0 {
                return Err(format!(
                    "One of the provided weights '{}' is negative.",
                    weight
                ));
            }
            if weight.is_nan(){
                return Err(String::from(
                    "One of the provided weights is NaN."
                ));
            }
            if weight.is_infinite(){
                return Err(String::from(
                    "One of the provided weights is infinite."
                ));
            }
        }
    }
    
    if let Some(et) = edge_types {
        info!("Checking for length between edge types and given edges.");
        if et.len() != sources.len(){
            return Err(format!(
                "The len of edge types ({}) is different than the len of given edges ({}).  ",
                et.len(), sources.len()
            ));
        }
    }

    info!("Checking for unique edges (including edge types).");
    let mut unique_edges: HashSet<(NodeT, NodeT, Option<EdgeTypeT>)> = HashSet::new();
    for i in 0..sources.len(){
        let src = sources[i];
        let dst = destinations[i];
        let edge_type = if let Some(et) = edge_types {
            Some(et[i])
        } else {
            None
        };
        if unique_edges.contains(&(src, dst, edge_type)){
            return Err(format!(
                concat!(
                    "Duplicated edge was found within given edges.\n",
                    "The source node is {src}.\n",
                    "The destination node is {dst}.\n",
                    "{edge_type_message}\n",
                    "This issue is relative to the graph building and not ",
                    "the CSV reader, hence it can not be addressed by passing ",
                    "the parameter ignore_duplicated_edges."
                ),
                src=src,
                dst=dst,
                edge_type_message=if let Some(et) = edge_type {
                    format!("The edge type is {}", et)
                } else {
                    String::from("No edge type was detected.")
                }
            ));
        }
        unique_edges.insert((src, dst, edge_type));
    }

    Ok(())
}

/// Graph Constructors
impl Graph {

    pub fn new_directed(
        sources: Vec<NodeT>,
        destinations: Vec<NodeT>,

        nodes_mapping: HashMap<String, NodeT>,
        nodes_reverse_mapping: Vec<String>,

        node_types: Option<Vec<NodeTypeT>>,
        node_types_mapping: Option<HashMap<String, NodeTypeT>>,
        node_types_reverse_mapping: Option<Vec<String>>,

        edge_types: Option<Vec<EdgeTypeT>>,
        edge_types_mapping: Option<HashMap<String, EdgeTypeT>>,
        edge_types_reverse_mapping: Option<Vec<String>>,

        weights: Option<Vec<WeightT>>,
        validate_input_data: Option<bool>,
    ) -> Result<Graph, String> {
        if validate_input_data.unwrap_or_else(|| true) {
            validate(
                &sources,
                &destinations,
                &nodes_mapping,
                &nodes_reverse_mapping,
                &node_types,
                &edge_types,
                &weights
            )?;
        }

        let nodes_number = nodes_reverse_mapping.len();

        info!("Computing unique edges.");
        let unique_edges: HashMap<(NodeT, NodeT), EdgeT> =
            HashMap::from_iter(
                sources.iter().cloned().zip(
                    destinations.iter().cloned()
                ).enumerate().map(|(i, (src, dst))| ((src, dst), i))
            );

        info!("Computing sorting of given edges based on sources.");
        let mut pairs: Vec<(usize, &NodeT)> = sources.par_iter().enumerate().collect();
        pairs.par_sort_unstable_by_key(|(_, &v)| v);
        let indices: Vec<&usize> = pairs.par_iter().map(|(i, _)| i).collect();
        
        info!("Sorting given sources.");
        let sorted_sources: Vec<NodeT> = indices.par_iter()
            .map(|&&x| sources[x]).collect();
        info!("Sorting given destinations.");
        let sorted_destinations: Vec<NodeT> = indices.par_iter()
            .map(|&&x| destinations[x]).collect();
        info!("Sorting given weights.");
        let sorted_weights: Option<Vec<WeightT>> = weights.map(|w| 
            indices.par_iter()
            .map(|&&x| w[x]).collect()
        ); 
        info!("Sorting given edge types.");
        let sorted_edge_types: Option<Vec<EdgeTypeT>> = edge_types.map(|et| 
            indices.par_iter()
            .map(|&&x| et[x]).collect()
        );

        let outbounds = Graph::compute_outbounds(nodes_number, &sorted_sources);

        let mut graph = Graph {
            nodes_mapping,
            nodes_reverse_mapping,
            unique_edges,
            node_types,
            node_types_mapping,
            node_types_reverse_mapping,
            edge_types_mapping,
            edge_types_reverse_mapping,
            outbounds,
            is_directed: true,
            sources: sorted_sources,
            destinations: sorted_destinations,
            weights: sorted_weights,
            edge_types: sorted_edge_types,
            has_traps: true
        };
        
        // Here we are computing if the graph has any trap nodes.
        // When a graph has no traps, we can use a faster random walk.
        graph.has_traps = (0..graph.get_nodes_number())
            .into_par_iter()
            .any(|node| graph.is_node_trap(node));

        Ok(graph)
    }

    pub fn new_undirected(
        sources: Vec<NodeT>,
        destinations: Vec<NodeT>,
        nodes_mapping: HashMap<String, NodeT>,
        nodes_reverse_mapping: Vec<String>,
        node_types: Option<Vec<NodeTypeT>>,
        node_types_mapping: Option<HashMap<String, NodeTypeT>>,
        node_types_reverse_mapping: Option<Vec<String>>,
        edge_types: Option<Vec<EdgeTypeT>>,
        edge_types_mapping: Option<HashMap<String, EdgeTypeT>>,
        edge_types_reverse_mapping: Option<Vec<String>>,
        weights: Option<Vec<WeightT>>,
        validate_input_data: Option<bool>,
        force_conversion_to_undirected: Option<bool>
    ) -> Result<Graph, String> {

        if validate_input_data.unwrap_or_else(|| true) {
            validate(
                &sources,
                &destinations,
                &nodes_mapping,
                &nodes_reverse_mapping,
                &node_types,
                &edge_types,
                &weights
            )?;
        }

        let _force_conversion_to_undirected = force_conversion_to_undirected.unwrap_or(false);
        let mut full_sources: Vec<NodeT> = Vec::new();
        let mut full_destinations: Vec<NodeT> = Vec::new();
        let mut full_edge_types: Vec<NodeTypeT> = Vec::new();
        let mut full_weights: Vec<WeightT> = Vec::new();
        let mut unique_edges: HashSet<(NodeT, NodeT, Option<EdgeTypeT>)> = HashSet::new();
        
        for index in 0..sources.len(){
            let src = sources[index];
            let dst = destinations[index];
            let edge_type = if let Some(et) = &edge_types {
                Some(et[index])
            } else {
                None
            };
            if ! unique_edges.contains(&(src, dst, edge_type)){
                full_sources.push(src);
                full_destinations.push(dst);
                if let Some(w) = &weights {
                    full_weights.push(w[index]);
                }
                let edge_type = if let Some(et) = &edge_types {
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
                    if let Some(w) = &weights {
                        full_weights.push(w[index]);
                    }
                    
                    if let Some(et) = edge_type {
                        full_edge_types.push(et);
                    }   

                    unique_edges.insert((dst, src, edge_type));
                }
            } else if ! _force_conversion_to_undirected {
                return Err(format!(
                    concat!(
                        "Within given edges there are directed edges.\n",
                        "The source node is {src}\n",
                        "The destination node is {dst}\n",
                        "{edge_type_message}\n",
                        "This means you are forcibly converting a directed ",
                        "graph into an undirected graph.\n",
                        "You can enforce the conversion by passing the flag ",
                        "force_conversion_to_undirected as true.\n",
                        "The conversion will ignore edges that are",
                        "directed between two nodes, have the same edge type ",
                        "but different weights.\n",
                        "For example, an edge from A to B of type 1 ",
                        "with weight 10 would be inserted alongside ",
                        "the simmetric counter part B to A of type 1 ",
                        "but a following edge from B to A of type 1 ",
                        "with weight 5 would be ignored."
                    ),
                    src=src,
                    dst=dst,
                    edge_type_message= if let Some(et) = edge_type {
                        format!("The edge type is {}", et)
                    } else {
                        String::from("No edge type was provided for the edge.")
                    }
                ));
            }
        }
        
        let mut result = Graph::new_directed(
            full_sources,
            full_destinations,
            nodes_mapping,
            nodes_reverse_mapping,
            node_types,
            node_types_mapping,
            node_types_reverse_mapping,
            if edge_types.is_some() {
                Some(full_edge_types)
            } else {
                None
            },
            edge_types_mapping,
            edge_types_reverse_mapping,
            if weights.is_some() {
                Some(full_weights)
            } else {
                None
            },
            Some(false),
        )?;
        result.is_directed = false;
        Ok(result)
    }
}