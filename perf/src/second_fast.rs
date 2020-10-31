extern crate graph;
use graph::*;
use rayon::iter::ParallelIterator;

/// Return WalksParameters to execute a second order walk.
pub fn second_order_walker(graph: &Graph, verbose: bool) -> Result<WalksParameters, String> {
    Ok(WalksParameters::new(100)?
        .set_iterations(Some(1))?
        .set_return_weight(Some(2.0))?
        .set_explore_weight(Some(2.0))?
        .set_change_edge_type_weight(Some(2.0))?
        .set_change_node_type_weight(Some(2.0))?
        .set_dense_node_mapping(Some(graph.get_dense_node_mapping()))
        .set_random_state(Some(43)))
}

fn main() {
    let edges_reader = EdgeFileReader::new("/home/zom/complete_string.tsv".to_string()).unwrap()
        .set_separator(Some("\t".to_string())).unwrap()
        .set_sources_column_number(Some(0)).unwrap()
        .set_destinations_column_number(Some(1)).unwrap()
        .set_weights_column_number(Some(2)).unwrap()
        .set_default_weight(Some(1.0f64))
        .set_verbose(Some(false))
        .set_header(Some(false));
    let mut graph = Graph::from_unsorted_csv(edges_reader, None, false, "Graph".to_owned()).unwrap();

    graph.enable_fast_walk(true, true);

    let walker = second_order_walker(&graph, false).unwrap();
    let walks = graph.complete_walks_iter( &walker).unwrap().collect::<Vec<Vec<NodeT>>>();
    println!("{:?}", walks[walks.len() - 1]);
}   
