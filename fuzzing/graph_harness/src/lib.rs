extern crate graph;
use graph::*;
use graph::{
    WalksParameters,
    WalkWeights,
    SingleWalkParameters
};
use std::path::Path;
use std::fs::File;
use std::io::prelude::*;
use std::fs::remove_file;
use std::collections::HashMap;
use arbitrary::Arbitrary;

use rand::Rng;
const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                        abcdefghijklmnopqrstuvwxyz\
                        0123456789()*&^%$#@!~";
    
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
pub struct ToFuzz {
    from_csv_args: FromCsvArgs,
    walks_args: _WalksParameters,
    skipgrams_args: SkipgramsArgs,
    cooccurence_args: CooccurrenceArgs,
    node2vec_args:  Node2VecArgs,
    link_prediction_args: LinkPredictionArgs,
    holdout_args: HoldoutArgs,
    metrics:Metrics,
}

#[derive(Arbitrary, Debug)]
pub struct _SingleWalkParameters {
    pub(crate) length: u16,
    pub(crate) weights: _WalkWeights,
}

#[derive(Arbitrary, Debug)]
pub struct _WalksParameters {
    pub(crate) single_walk_parameters: _SingleWalkParameters,
    pub(crate) iterations: u16,
    pub(crate) min_length: u16,
    pub(crate) verbose: bool,
    pub(crate) start_node: NodeT,
    pub(crate) end_node: NodeT,
    pub(crate) dense_nodes_mapping: Option<HashMap<NodeT, NodeT>>,
}

#[derive(Arbitrary, Debug)]
pub struct _WalkWeights {
    pub(crate) return_weight: ParamsT,
    pub(crate) explore_weight: ParamsT,
    pub(crate) change_node_type_weight: ParamsT,
    pub(crate) change_edge_type_weight: ParamsT,
}

#[derive(Arbitrary, Debug)]
pub struct FromCsvArgs {
    edges_content: String,
    sources_column: String,
    destinations_column: String,
    directed: bool,
    edge_sep: Option<String>,
    ignore_duplicated_edges: bool,
    force_conversion_to_undirected: Option<bool>,
    edge_types: Option<FromCsvArgsEdgeTypes>,
    weights: Option<FromCsvArgsWeight>,
    nodes: Option<FromCsvArgsNodes>,
}
#[derive(Arbitrary, Debug)]
pub struct FromCsvArgsEdgeTypes {
    edge_types_column: String,
    default_edge_type: Option<String>,
}
#[derive(Arbitrary, Debug)]
pub struct FromCsvArgsWeight {
    weights_column: String,
    default_weight: Option<WeightT>,
}
#[derive(Arbitrary, Debug)]
pub struct FromCsvArgsNodes {
    nodes_content: String,
    nodes_column: String,
    node_types_column: String,
    default_node_type: Option<String>,
    ignore_duplicated_nodes: Option<bool>,
    node_sep: Option<String>,
}

#[derive(Arbitrary, Debug)]
pub struct SkipgramsArgs {
    seed: usize,
    walk_parameters: _WalksParameters,
    window_size: Option<usize>,
    negative_samples: Option<f64>,
    shuffle: Option<bool>
}

#[derive(Arbitrary, Debug)]
pub struct CooccurrenceArgs {
    walks_parameters: _WalksParameters,
    window_size: Option<usize>,
    verbose: Option<bool>,
}

#[derive(Arbitrary, Debug)]
pub struct Node2VecArgs {
    walk_parameters: _WalksParameters,
    window_size: Option<usize>,
    shuffle: Option<bool>,
}

#[derive(Arbitrary, Debug)]
pub struct LinkPredictionArgs {
    idx: u16,
    batch_size: u8,
    negative_samples: Option<f64>,
    graph_to_avoid: Option<FromCsvArgs>,
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

fn convert_walk_parameters(args: _WalksParameters) -> Result<WalksParameters, String> {
    let walk_parameters = WalksParameters::new(
        SingleWalkParameters::new(
            args.single_walk_parameters.length as usize, 
            WalkWeights::default().set_change_edge_type_weight(
                Some(args.single_walk_parameters.weights.change_edge_type_weight)
            )?.set_change_node_type_weight(
                Some(args.single_walk_parameters.weights.change_node_type_weight)
            )?.set_explore_weight(
                Some(args.single_walk_parameters.weights.explore_weight)
            )?.set_return_weight(
                Some(args.single_walk_parameters.weights.return_weight)
            )?
        )?,
        args.start_node,
        args.end_node,
    )?.set_iterations(Some(args.iterations as usize))?
    .set_min_length(Some(args.min_length as usize))?
    .set_verbose(Some(args.verbose))
    .set_dense_nodes_mapping(args.dense_nodes_mapping);
    Ok(walk_parameters)
}

fn create_graph_from_args_struct(args: &FromCsvArgs) -> Result<Graph, String>{
    
    // Create the edges file
    let edges_fname = Path::new("/tmp").join(random_string(64));
    let edges_filename = edges_fname.to_str().unwrap();
    let mut edges_file = File::create(&edges_filename).unwrap();
    edges_file.write_all(&args.edges_content.as_bytes()).unwrap();

    let nodes_fname = Path::new("/tmp").join(random_string(64));
    let nodes_filename = nodes_fname.to_str().unwrap();
    let node_file = if let Some(ns) = &args.nodes {
        let mut nodes_file = File::create(&nodes_filename).unwrap();
        nodes_file.write_all(ns.nodes_content.as_bytes()).unwrap();
        Some(nodes_filename)
    } else {
        None
    };

    let mut result = FromCsvBuilder::new(
        edges_filename,
        &args.sources_column, 
        &args.destinations_column, 
        args.directed, 
        None
    )?;

    if let Some(w) = &args.weights {
        result = result.set_weights(
            &w.weights_column, 
            w.default_weight
        );
    }

    if let Some(n) = &args.nodes {
        result = result.load_nodes_csv(
            node_file.unwrap(),
            &n.nodes_column, 
            &n.node_types_column, 
            match &n.default_node_type{
                Some(e) => Some(&e),
                None => None
            }, 
            match &n.node_sep{
                Some(e) => Some(&e),
                None => None
            },
            n.ignore_duplicated_nodes
        )?;
    }
    
    if let Some(e) = &args.edge_types {
        result = result.set_edge_types(
            &e.edge_types_column, 
            match &e.default_edge_type {
                Some(g) => Some(&g),
                None => None
            }, 
        );
    }
    
    let graph = result.build();
    
    //clean up
    let _ = remove_file(&edges_filename).unwrap();
    if args.nodes.is_some() {
        let _ = remove_file(&nodes_filename);
    }
    
    graph
}

pub fn harness(data: ToFuzz){
    let maybe_graph = create_graph_from_args_struct(&data.from_csv_args);

    if maybe_graph.is_err() {
        return;
    }
    let graph = maybe_graph.unwrap();
    // test metrics
    graph.report();
    // to enable once we fix the get_min_max_edge which doesn't check
    // if the nod is inbound and is_edge_trap similarly
    // all these metrics panic if the value is out of bound
    // this is known and was not fixed because the get_min_max_edge function
    // is used in the walk and the checking and error propagation  would slow down
    // the walk
    let one = (data.metrics.one % graph.get_nodes_number()) % graph.get_edges_number();
    let two = (data.metrics.two % graph.get_nodes_number()) % graph.get_edges_number();
    graph.degree(one);
    graph.degree(two);
    graph.degrees_product(one, two);
    graph.jaccard_index(one, two);
    graph.adamic_adar_index(one, two);
    graph.resource_allocation_index(one, two);

    match convert_walk_parameters(data.walks_args) {
        Err(_) => {},
        Ok(param) => {
            graph.walk(&param);
        }
    }

    let mut negative_samples = data.skipgrams_args.negative_samples;
    if let Some(ns) = &mut negative_samples{
        if *ns > 100.0{
            *ns = 100.0;
        }
    }

    match convert_walk_parameters(data.skipgrams_args.walk_parameters) {
        Err(_) => {},
        Ok(param) => {
            graph.binary_skipgrams(
            data.skipgrams_args.seed,
            &param,
            data.skipgrams_args.window_size,
            negative_samples,
            data.skipgrams_args.shuffle
        );
        }
    }

    match convert_walk_parameters(data.cooccurence_args.walks_parameters) {
        Err(_) => {},
        Ok(param) => {
            graph.cooccurence_matrix(
            &param,
            data.cooccurence_args.window_size,
            data.cooccurence_args.verbose,
        );
        }
    }

    match convert_walk_parameters(data.node2vec_args.walk_parameters) {
        Err(_) => {},
        Ok(param) => {
            graph.node2vec(
            &param,
            data.node2vec_args.window_size,
            data.node2vec_args.shuffle
        );
        }
    }

    let _ = graph.connected_holdout(
        data.holdout_args.seed,
        data.holdout_args.train_percentage as f64
    );

    let _ = graph.random_holdout(
        data.holdout_args.seed,
        data.holdout_args.train_percentage as f64
    );

    let mut negative_samples = data.link_prediction_args.negative_samples;
    if let Some(ns) = &mut negative_samples{
        if *ns > 100.0{
            *ns = 100.0;
        }
    }

    if data.link_prediction_args.graph_to_avoid.is_none() {
        
        let _ = graph.link_prediction(
            data.link_prediction_args.idx as u64,
            data.link_prediction_args.batch_size as usize,
            data.link_prediction_args.negative_samples,
            None,
            data.link_prediction_args.avoid_self_loops
        );   
    } else {

        let graph_args_2 = data.link_prediction_args.graph_to_avoid.unwrap();
        let _ = match create_graph_from_args_struct(&graph_args_2){
            Ok(g) => Some(graph.link_prediction(
                data.link_prediction_args.idx as u64,
                data.link_prediction_args.batch_size as usize,
                negative_samples,
                Some(&g),
                None
            )),
            Err(_) => None
        };

    }
}
