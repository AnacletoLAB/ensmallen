extern crate graph;
use graph::*;
use rayon::iter::ParallelIterator;

fn main() {
    let edges_reader = EdgeFileReader::new("/home/zom/complete_string.tsv".to_string()).unwrap()
        .set_separator(Some("\t".to_string())).unwrap()
        .set_sources_column_number(Some(0)).unwrap()
        .set_destinations_column_number(Some(1)).unwrap()
        .set_weights_column_number(Some(2)).unwrap()
        .set_default_weight(Some(1.0))
        .set_verbose(Some(false))
        .set_header(Some(true));
    let mut graph = Graph::from_unsorted_csv(edges_reader, None, false, false, "Graph".to_owned()).unwrap();

    graph.enable(true, true, true, None).unwrap();

    for _ in 0..1_000 {
        let _pred = graph.spanning_arborescence(false);
    }
}   
