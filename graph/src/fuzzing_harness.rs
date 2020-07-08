use super::*;

use std::path::Path;
use std::fs::File;
use std::io::prelude::*;
use std::fs::remove_file;

use arbitrary::Arbitrary;

use rand::Rng;
const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                        abcdefghijklmnopqrstuvwxyz\
                        0123456789)(*&^%$#@!~";
    
pub fn random_string(len: usize) -> String{    
    let mut rng = rand::thread_rng();
    
    (0..len)
        .map(|_| {
            let idx = rng.gen_range(0, CHARSET.len());
            CHARSET[idx] as char
        })
        .collect()
}

#[derive(Arbitrary, Debug)]
pub struct FromCsvAgs {
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
pub struct WalkArgs {
    length: u8,
    iterations: Option<u8>,
    start_node: Option<u16>,
    end_node: Option<u16>,
    min_length: Option<u8>,
    return_weight: Option<f32>,
    explore_weight: Option<f32>,
    change_node_type_weight: Option<f32>,
    change_edge_type_weight: Option<f32>
}

#[derive(Arbitrary, Debug)]
pub struct SkipgramsArgs {
    idx: u16,
    batch_size: u8,
    length: u8,
    iterations: Option<u8>,
    window_size: Option<u8>,
    negative_samples: Option<f64>,
    shuffle: Option<bool>,
    min_length: Option<u8>,
    return_weight: Option<f32>,
    explore_weight: Option<f32>,
    change_node_type_weight: Option<f32>,
    change_edge_type_weight: Option<f32>
}

#[derive(Arbitrary, Debug)]
pub struct CooccurrenceArgs {
    length: u8,
    window_size: Option<u8>,
    iterations: Option<u8>,
    min_length: Option<u8>,
    return_weight: Option<f32>,
    explore_weight: Option<f32>,
    change_node_type_weight: Option<f32>,
    change_edge_type_weight: Option<f32>
}

#[derive(Arbitrary, Debug)]
pub struct CbowArgs {
    idx: usize,
    batch_size: u8,
    length: u8,
    iterations: Option<u8>,
    window_size: Option<u8>,
    shuffle: Option<bool>,
    min_length: Option<usize>,
    return_weight: Option<ParamsT>,
    explore_weight: Option<ParamsT>,
    change_node_type_weight: Option<ParamsT>,
    change_edge_type_weight: Option<ParamsT>
}

#[derive(Arbitrary, Debug)]
pub struct LinkPredictionArgs {
    idx: u16,
    batch_size: u8,
    negative_samples: Option<f64>,
    graph_to_avoid: Option<FromCsvAgs>,
    avoid_self_loops: Option<bool>
}

#[derive(Arbitrary, Debug)]
pub struct HoldoutArgs {
    seed: NodeT,
    train_percentage: f32
}

#[derive(Arbitrary, Debug)]
pub struct Metrics {
    one: NodeT,
    two: NodeT
}

#[derive(Arbitrary, Debug)]
pub struct ToFuzz {
    from_csv_args: FromCsvAgs,
    walks_args: WalkArgs,
    skipgrams_args: SkipgramsArgs,
    cooccurence_args: CooccurrenceArgs,
    cbow_args:  CbowArgs,
    link_prediction_args: LinkPredictionArgs,
    holdout_args: HoldoutArgs,
    metrics:Metrics,
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

pub fn harness(data: ToFuzz) {
    println!("{:?}", data);
    let graph = create_graph_from_args_struct(&data.from_csv_args);

    if graph.is_ok(){
        let unwrapped = graph.unwrap();

        // test metrics
        unwrapped.report();
        // to enable once we fix the get_min_max_edge which doesn't check
        // if the nod is inbound and is_edge_trap similarly
        // all these metrics panic if the value is out of bound
        // this is known and was not fixed because the get_min_max_edge function
        // is used in the walk and the checking and error propagation  would slow down
        // the walk
        let one = (data.metrics.one % unwrapped.get_nodes_number()) % unwrapped.get_edges_number();
        let two = (data.metrics.two % unwrapped.get_nodes_number()) % unwrapped.get_edges_number();
        unwrapped.degree(one);
        unwrapped.degree(two);
        unwrapped.degrees_product(one, two);
        unwrapped.jaccard_index(one, two);
        unwrapped.adamic_adar_index(one, two);
        unwrapped.resource_allocation_index(one, two);

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
            Some(false)
        );

        let mut negative_samples = data.skipgrams_args.negative_samples.clone();
        if let Some(ns) = &mut negative_samples{
            if *ns > 100.0{
                *ns = 100.0;
            }
        }
        let _ = unwrapped.skipgrams(
            data.skipgrams_args.idx as usize,
            data.skipgrams_args.batch_size as usize,
            data.skipgrams_args.length as usize,
            data.skipgrams_args.iterations.map(|e| e as usize),
            data.skipgrams_args.window_size.map(|e| e as usize),
            negative_samples,
            data.skipgrams_args.shuffle,
            data.skipgrams_args.min_length.map(|e| e as usize),
            data.skipgrams_args.return_weight.map(|e| e as f64),
            data.skipgrams_args.explore_weight.map(|e| e as f64),
            data.skipgrams_args.change_node_type_weight.map(|e| e as f64),
            data.skipgrams_args.change_edge_type_weight.map(|e| e as f64),
            None
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
            Some(false)
        );

        let _ = unwrapped.cbow(
            data.cbow_args.idx as usize,
            data.cbow_args.batch_size as usize,
            data.cbow_args.length as usize,
            data.cbow_args.iterations.map(|e| e as usize),
            data.cbow_args.window_size.map(|e| e as usize),
            data.cbow_args.shuffle,
            data.cbow_args.min_length.map(|e| e as usize),
            data.cbow_args.return_weight.map(|e| e as f64),
            data.cbow_args.explore_weight.map(|e| e as f64),
            data.cbow_args.change_node_type_weight.map(|e| e as f64),
            data.cbow_args.change_edge_type_weight.map(|e| e as f64)
        );

        let _ = unwrapped.holdout(
            data.holdout_args.seed,
            data.holdout_args.train_percentage as f64
        );

        let mut negative_samples = data.link_prediction_args.negative_samples.clone();
        if let Some(ns) = &mut negative_samples{
            if *ns > 100.0{
                *ns = 100.0;
            }
        }

        if data.link_prediction_args.graph_to_avoid.is_none() {
            
            let _ = unwrapped.link_prediction(
                data.link_prediction_args.idx as u64,
                data.link_prediction_args.batch_size as usize,
                data.link_prediction_args.negative_samples,
                None,
                data.link_prediction_args.avoid_self_loops
            );   
        } else {

            let graph_args_2 = data.link_prediction_args.graph_to_avoid.unwrap();
            let graph2 = create_graph_from_args_struct(&graph_args_2);
            if graph2.is_ok(){
                let _ = unwrapped.link_prediction(
                    data.link_prediction_args.idx as u64,
                    data.link_prediction_args.batch_size as usize,
                    negative_samples,
                    Some(&graph2.unwrap()),
                    None
                );   
            }
        }
    }
}
