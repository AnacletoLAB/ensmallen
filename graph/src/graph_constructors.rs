use super::*;
use log::debug;
use std::collections::{HashMap, HashSet};
use std::iter::FromIterator;
use itertools::Itertools;
use rayon::prelude::*;

pub fn validate(
    sources: &Vec<NodeT>,
    destinations: &Vec<NodeT>,
    nodes_mapping: &HashMap<String, NodeT>,
    nodes_reverse_mapping: &Vec<String>,
    node_types: &Option<Vec<NodeTypeT>>,
    edge_types: &Option<Vec<EdgeTypeT>>,
    weights: &Option<Vec<WeightT>>,
) {
    
    debug!("Checking that the nodes mappings are of the same length.");
    if nodes_mapping.len() != nodes_reverse_mapping.len() {
        panic!("The size of the node_mapping ({}) does not match the size of the nodes_reverse_mapping ({}).",
            nodes_mapping.len(), nodes_reverse_mapping.len()
        );
    }

    if let Some(nt) = &node_types {
        debug!("Checking that nodes and node types are of the same length.");
        if nt.len() != nodes_reverse_mapping.len() {
            panic!("The number of given nodes ({}) does not match the number of node_types ({}).",
                nt.len(), nodes_reverse_mapping.len()
            );
        }
    }

    if let Some(nt) = &node_types{
        debug!("Checking if every node used by the edges exists.");
        sources
            .iter()
            .chain(destinations.iter())
            .for_each(|node| {
            if *node >= nt.len() {
                panic!(
                    "A node provided with the edges ('{}') does not exists within given nodes.",
                    node
                );
            }
        });
    }

    debug!("Checking that nodes must be uniques.");
    if nodes_reverse_mapping.len() != nodes_reverse_mapping.iter().unique().count() {
        panic!("The nodes must be uniques. Duplicates were found in the data.")
    }

    if let Some(w) = weights {
        debug!("Checking for length between weights and given edges.");
        if w.len() != sources.len(){
            panic!("Length of given weights ({}) does not match length of given edges ({}).",
            w.len(), sources.len())
        }
        debug!("Checking for non-zero weights.");
        w.par_iter().for_each(|weight| {
            if *weight == 0.0 {
                panic!(
                    "One of the provided weights '{}' is either 0 or within float error to zero.",
                    weight
                );
            }
        });
    }
    
    if let Some(et) = edge_types {
        debug!("Checking for length between edge types and given edges.");
        if et.len() != sources.len(){
            panic!(
                "The len of edge types ({}) is different than the len of given edges ({}).  ",
                et.len(), sources.len()
            );
        }
    }
}

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
    ) -> Graph {
        if validate_input_data.unwrap_or_else(|| true) {
            validate(
                &sources,
                &destinations,
                &nodes_mapping,
                &nodes_reverse_mapping,
                &node_types,
                &edge_types,
                &weights
            );
        }

        let nodes_number = nodes_reverse_mapping.len();

        debug!("Computing unique edges.");
        let unique_edges: HashSet<(NodeT, NodeT)> =
            HashSet::from_iter(sources.iter().cloned().zip(destinations.iter().cloned()));

        debug!("Computing sorting of given edges based on sources.");
        let mut pairs: Vec<(usize, &NodeT)> = sources.par_iter().enumerate().collect();
        pairs.sort_unstable_by_key(|(_, &v)| v);
        let indices: Vec<&usize> = pairs.par_iter().map(|(i, _)| i).collect();
        
        debug!("Sorting given sources.");
        let sorted_sources: Vec<NodeT> = indices.par_iter()
            .map(|&&x| sources[x]).collect();
        debug!("Sorting given destinations.");
        let sorted_destinations: Vec<NodeT> = indices.par_iter()
            .map(|&&x| destinations[x]).collect();
        debug!("Sorting given weights.");
        let sorted_weights: Option<Vec<WeightT>> = weights.map(|w| 
            indices.par_iter()
            .map(|&&x| w[x]).collect()
        ); 
        debug!("Sorting given edge types.");
        let sorted_edge_types: Option<Vec<EdgeTypeT>> = edge_types.map(|et| 
            indices.par_iter()
            .map(|&&x| et[x]).collect()
        );

        let outbounds = Graph::compute_outbounds(nodes_number, &sorted_sources);

        Graph {
            nodes_mapping,
            nodes_reverse_mapping,
            unique_edges,
            node_types,
            node_types_mapping,
            node_types_reverse_mapping,
            edge_types_mapping,
            edge_types_reverse_mapping,
            sources: sorted_sources,
            destinations: sorted_destinations,
            outbounds: outbounds,
            weights: sorted_weights,
            edge_types: sorted_edge_types,
        }
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
    ) -> Graph {

        if validate_input_data.unwrap_or_else(|| true) {
            validate(
                &sources,
                &destinations,
                &nodes_mapping,
                &nodes_reverse_mapping,
                &node_types,
                &edge_types,
                &weights
            );
        }

        debug!("Identifying self-loops present in given graph.");
        let loops_mask: Vec<bool> = sources
            .par_iter()
            .zip(destinations.par_iter())
            .map(|(a, b)| a == b)
            .collect();

        debug!("Building undirected graph sources.");
        let mut full_sources: Vec<NodeT> = sources.clone();
        full_sources.extend(
            destinations
                .par_iter()
                .zip(loops_mask.par_iter())
                .filter(|&(_, &mask)| !mask)
                .map(|(value, _)| value.clone())
                .collect::<Vec<NodeT>>(),
        );

        debug!("Building undirected graph destinations.");
        let mut full_destinations: Vec<NodeT> = destinations.clone();
        full_destinations.extend(
            sources
                .par_iter()
                .zip(loops_mask.par_iter())
                .filter(|&(_, &mask)| !mask)
                .map(|(value, _)| value.clone())
                .collect::<Vec<NodeT>>(),
        );

        let mut full_edge_types = edge_types;
        if let Some(e) = &mut full_edge_types {
            debug!("Building undirected graph edge types.");
            e.extend(
                e.par_iter()
                    .zip(loops_mask.par_iter())
                    .filter(|&(_, &mask)| !mask)
                    .map(|(value, _)| value.clone())
                    .collect::<Vec<NodeTypeT>>(),
            );
        };

        let mut full_weights = weights;
        if let Some(w) = &mut full_weights {
            debug!("Building undirected graph weights.");
            w.extend(
                w.par_iter()
                    .zip(loops_mask.par_iter())
                    .filter(|&(_, &mask)| !mask)
                    .map(|(value, _)| *value)
                    .collect::<Vec<WeightT>>(),
            );
        };

        Graph::new_directed(
            full_sources,
            full_destinations,
            nodes_mapping,
            nodes_reverse_mapping,
            node_types,
            node_types_mapping,
            node_types_reverse_mapping,
            full_edge_types,
            edge_types_mapping,
            edge_types_reverse_mapping,
            full_weights,
            Some(false),
        )
    }
}