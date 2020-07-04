#![no_main]
use libfuzzer_sys::fuzz_target;
extern crate graph;

use graph::*;

use std::path::Path;
use std::fs::File;
use std::io::prelude::*;
use std::fs::remove_file;

mod utils;
use utils::*;

//use libfuzzer_sys::arbitrary::Arbitrary;
use arbitrary::Arbitrary;

#[derive(Arbitrary, Debug)]
struct FromCsvAgs {
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
    ignore_duplicated_edges: Option<bool>,
    ignore_duplicated_nodes: Option<bool>,
    force_conversion_to_undirected: Option<bool>,
}

#[derive(Arbitrary, Debug)]
struct WalkArgs {
    length: u8,
    iterations: Option<u8>,
    start_node: Option<u16>,
    end_node: Option<u16>,
    min_length: Option<u8>,
    return_weight: Option<f32>,
    explore_weight: Option<f32>,
    change_node_type_weight: Option<f32>,
    change_edge_type_weight: Option<f32>,
    verbose: Option<bool>,
}

#[derive(Arbitrary, Debug)]
struct SkipgramsArgs {
    idx: u16,
    batch_size: u16,
    length: u8,
    iterations: Option<u8>,
    window_size: Option<u8>,
    negative_samples: Option<f32>,
    shuffle: Option<bool>,
    min_length: Option<u8>,
    return_weight: Option<f32>,
    explore_weight: Option<f32>,
    change_node_type_weight: Option<f32>,
    change_edge_type_weight: Option<f32>
}

#[derive(Arbitrary, Debug)]
struct CooccurrenceArgs {
    length: u8,
    window_size: Option<u8>,
    iterations: Option<u8>,
    min_length: Option<u8>,
    return_weight: Option<f32>,
    explore_weight: Option<f32>,
    change_node_type_weight: Option<f32>,
    change_edge_type_weight: Option<f32>,
    verbose: Option<bool>
}

#[derive(Arbitrary, Debug)]
struct LinkPredictionArgs {
    idx: u16,
    batch_size: u16,
    negative_samples: Option<f32>,
    graph_to_avoid: Option<FromCsvAgs>,
    shuffle: Option<bool>
}

#[derive(Arbitrary, Debug)]
struct HoldoutArgs {
    seed: NodeT,
    train_percentage: f32
}

#[derive(Arbitrary, Debug)]
struct ToFuzz {
    from_csv_args: FromCsvAgs,
    walks_args: WalkArgs,
    skipgrams_args: SkipgramsArgs,
    cooccurence_args: CooccurrenceArgs,
    link_prediction_args: LinkPredictionArgs,
    holdout_args: HoldoutArgs
}

fn create_graph_from_args_struct(args: &FromCsvAgs) -> Result<Graph, String>{
    
    // Create the edges file
    let edges_fname = Path::new("/tmp").join(random_string(64));
    let edges_filename = edges_fname.to_str().unwrap();
    let mut edges_file = File::create(&edges_filename).unwrap();
    edges_file.write_all(&args.edges_content.as_bytes()).unwrap();

    let nodes_fname = Path::new("/tmp").join(random_string(64));
    let nodes_filename = nodes_fname.to_str().unwrap();
    let node_file = if let Some(ns) = &args.nodes_content {
        let mut nodes_file = File::create(&nodes_filename).unwrap();
        nodes_file.write_all(ns.as_bytes()).unwrap();
        Some(nodes_filename)
    } else {
        None
    };

    let edge_types_column = if let Some(v) = &args.edge_types_column {
        Some(v.as_str())
    } else {
        None
    };

    let default_edge_type = if let Some(v) = &args.default_edge_type {
        Some(v.as_str())
    } else {
        None
    };

    let weights_column = if let Some(v) = &args.weights_column {
        Some(v.as_str())
    } else {
        None
    };

    let nodes_column = if let Some(v) = &args.nodes_column {
        Some(v.as_str())
    } else {
        None
    };
    
    let node_types_column = if let Some(v) = &args.node_types_column {
        Some(v.as_str())
    } else {
        None
    };

    let default_node_type = if let Some(v) = &args.default_node_type {
        Some(v.as_str())
    } else {
        None
    };

    let edge_sep = if let Some(v) = &args.edge_sep {
        Some(v.as_str())
    } else {
        None
    };

    let node_sep = if let Some(v) = &args.node_sep {
        Some(v.as_str())
    } else {
        None
    };

    let graph = graph::Graph::from_csv(
        &edges_filename,
        &args.sources_column,
        &args.destinations_column,
        args.directed,
        edge_types_column,
        default_edge_type,
        weights_column,
        args.default_weight.map(|e| e as f64),
        node_file,
        nodes_column,
        node_types_column,
        default_node_type,
        edge_sep,
        node_sep,
        args.ignore_duplicated_edges,
        args.ignore_duplicated_nodes,
        args.force_conversion_to_undirected
    );
    
    //clean up
    let _ = remove_file(&edges_filename).unwrap();
    if args.nodes_content.is_some() {
        let _ = remove_file(&nodes_filename);
    }
    
    graph
}

fuzz_target!(|data: ToFuzz| {

    let graph = create_graph_from_args_struct(&data.from_csv_args);

    if graph.is_ok(){
        let unwrapped = graph.unwrap();
        let _ = unwrapped.walk(
            data.walks_args.length as usize,
            data.walks_args.iterations.map(|e| e as usize),
            data.walks_args.start_node.map(|e| e as usize),
            data.walks_args.end_node.map(|e| e as usize),
            data.walks_args.min_length.map(|e| e as usize),
            data.walks_args.return_weight.map(|e| e as f64),
            data.walks_args.explore_weight.map(|e| e as f64),
            data.walks_args.change_node_type_weight.map(|e| e as f64),
            data.walks_args.change_edge_type_weight.map(|e| e as f64),
            data.walks_args.verbose
        );

        let _ = unwrapped.skipgrams(
            data.skipgrams_args.idx as usize,
            data.skipgrams_args.batch_size as usize,
            data.skipgrams_args.length as usize,
            data.skipgrams_args.iterations.map(|e| e as usize),
            data.skipgrams_args.window_size.map(|e| e as usize),
            data.skipgrams_args.negative_samples.map(|e| e as f64),
            data.skipgrams_args.shuffle,
            data.skipgrams_args.min_length.map(|e| e as usize),
            data.skipgrams_args.return_weight.map(|e| e as f64),
            data.skipgrams_args.explore_weight.map(|e| e as f64),
            data.skipgrams_args.change_node_type_weight.map(|e| e as f64),
            data.skipgrams_args.change_edge_type_weight.map(|e| e as f64),
        );

        let _ = unwrapped.cooccurence_matrix(
            data.cooccurence_args.length as usize,
            data.cooccurence_args.window_size.map(|e| e as usize),
            data.cooccurence_args.iterations.map(|e| e as usize),
            data.cooccurence_args.min_length.map(|e| e as usize),
            data.cooccurence_args.return_weight.map(|e| e as f64),
            data.cooccurence_args.explore_weight.map(|e| e as f64),
            data.cooccurence_args.change_node_type_weight.map(|e| e as f64),
            data.cooccurence_args.change_edge_type_weight.map(|e| e as f64),
            data.cooccurence_args.verbose,
        );

        let _ = unwrapped.holdout(
            data.holdout_args.seed,
            data.holdout_args.train_percentage as f64
        );

        if data.link_prediction_args.graph_to_avoid.is_none() {
            let _ = unwrapped.link_prediction(
                data.link_prediction_args.idx as u64,
                data.link_prediction_args.batch_size as usize,
                data.link_prediction_args.negative_samples.map(|e| e as f64),
                None,
                data.link_prediction_args.shuffle,
            );   
        } else {

            let graph_args_2 = data.link_prediction_args.graph_to_avoid.unwrap();
            let graph2 = create_graph_from_args_struct(&graph_args_2);
            if graph2.is_ok(){
                let _ = unwrapped.link_prediction(
                    data.link_prediction_args.idx as u64,
                    data.link_prediction_args.batch_size as usize,
                    data.link_prediction_args.negative_samples.map(|e| e as f64),
                    Some(&graph2.unwrap()),
                    data.link_prediction_args.shuffle,
                );   
            }
        }
    }
});
