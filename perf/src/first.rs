extern crate graph;
use graph::*;
use rayon::iter::ParallelIterator;

/// Return WalksParameters to execute a first order walk.
pub fn first_order_walker(graph: &Graph) -> Result<WalksParameters, String> {
    Ok(WalksParameters::new(1_000)?
        .set_iterations(Some(50_000))?
        .set_random_state(Some(43))
        .set_dense_node_mapping(Some(graph.get_dense_node_mapping())))
}

fn main() {
    let edges_reader = EdgeFileReader::new("../graph/tests/data/ppi/edges.tsv".to_string()).unwrap()
        .set_separator(Some("\t".to_string())).unwrap()
        .set_sources_column(Some("subject".to_string())).unwrap()
        .set_destinations_column(Some("object".to_string())).unwrap()
        .set_weights_column(Some("weight".to_string())).unwrap()
        .set_default_weight(Some(1.0))
        .set_verbose(Some(false))
        .set_header(Some(true));
    let mut graph = Graph::from_unsorted_csv(edges_reader, None, false, "Graph".to_owned()).unwrap();

    graph.enable_fast_walk(true, true);

    let walker = first_order_walker(&graph).unwrap();
    let walks = graph.random_walks_iter(1, &walker).unwrap().collect::<Vec<Vec<NodeT>>>();
    println!("{:?}", walks[walks.len() - 1]);
}   
