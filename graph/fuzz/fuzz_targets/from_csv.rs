#![no_main]
use libfuzzer_sys::fuzz_target;
extern crate graph;

use graph::WeightT;

use std::path::Path;
use std::fs::File;
use std::io::prelude::*;
use std::fs::remove_file;

mod utils;
use utils::*;

//use libfuzzer_sys::arbitrary::Arbitrary;
use arbitrary::Arbitrary;

#[derive(Arbitrary, Debug)]
struct ToFuzz {
    edges_content: String,
    nodes_content: Option<String>,

    sources_column: String,
    destinations_column: String,
    directed: bool,
    edge_types_column: Option<String>,
    default_edge_type: Option<String>,
    weights_column: Option<String>,
    default_weight: Option<WeightT>,
    nodes_column: Option<String>,
    node_types_column: Option<String>,
    default_node_type: Option<String>,
    edge_sep: Option<String>,
    node_sep: Option<String>,
    validate_input_data: Option<bool>,
    ignore_duplicated_edges: Option<bool>,
    ignore_duplicated_nodes: Option<bool>,
    force_conversion_to_undirected: Option<bool>
}

fuzz_target!(|data: ToFuzz| {
    // Create the edges file
    let edges_fname = Path::new("/tmp").join(random_string(64));
    let edges_filename = edges_fname.to_str().unwrap();
    let mut edges_file = File::create(&edges_filename).unwrap();
    edges_file.write_all(&data.edges_content.as_bytes()).unwrap();

    let nodes_fname = Path::new("/tmp").join(random_string(64));
    let nodes_filename = nodes_fname.to_str().unwrap();
    let node_file = if let Some(ns) = &data.nodes_content {
        let mut nodes_file = File::create(&nodes_filename).unwrap();
        nodes_file.write_all(ns.as_bytes()).unwrap();
        Some(nodes_filename)
    } else {
        None
    };

    let edge_types_column = if let Some(v) = &data.edge_types_column {
        Some(v.as_str())
    } else {
        None
    };

    let default_edge_type = if let Some(v) = &data.default_edge_type {
        Some(v.as_str())
    } else {
        None
    };

    let weights_column = if let Some(v) = &data.weights_column {
        Some(v.as_str())
    } else {
        None
    };

    let nodes_column = if let Some(v) = &data.nodes_column {
        Some(v.as_str())
    } else {
        None
    };
    
    let node_types_column = if let Some(v) = &data.node_types_column {
        Some(v.as_str())
    } else {
        None
    };

    let default_edge_type = if let Some(v) = &data.default_edge_type {
        Some(v.as_str())
    } else {
        None
    };

    let edge_sep = if let Some(v) = &data.edge_sep {
        Some(v.as_str())
    } else {
        None
    };

    let node_sep = if let Some(v) = &data.node_sep {
        Some(v.as_str())
    } else {
        None
    };

    let graph = graph::Graph::from_csv(
        &edges_filename,
        &data.sources_column,
        &data.destinations_column,
        data.directed,
        edge_types_column,
        default_edge_type,
        weights_column,
        data.default_weight,
        node_file,
        nodes_column,
        node_types_column,
        default_edge_type,
        edge_sep,
        node_sep,
        data.validate_input_data,
        data.ignore_duplicated_edges,
        data.ignore_duplicated_nodes,
        data.force_conversion_to_undirected
    );

    if graph.is_ok(){
        let _ = graph.unwrap().walk(10, Some(10), None, None, Some(0), Some(0.5), Some(2.0), Some(3.0), Some(4.0), Some(false));
    }
    
    let _ = remove_file(&edges_filename).unwrap();

    if let Some(ns) = &data.nodes_content {
        let _ = remove_file(&ns);
    }
});
