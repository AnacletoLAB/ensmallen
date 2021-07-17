extern crate graph;
use graph::*;
use rayon::iter::ParallelIterator;

fn main() {
    let edges_reader = EdgeFileReader::new("../graph/tests/data/ppi/edges.tsv".to_string()).unwrap()
        .set_separator(Some("\t".to_string())).unwrap()
        .set_sources_column(Some("subject".to_string())).unwrap()
        .set_destinations_column(Some("object".to_string())).unwrap()
        .set_weights_column(Some("weight".to_string())).unwrap()
        .set_default_weight(Some(1.0))
        .set_verbose(Some(false))
        .set_header(Some(true));
        
    let mut graph = Graph::from_unsorted_csv(edges_reader, None, false, false, "Graph".to_owned()).unwrap();
}   
