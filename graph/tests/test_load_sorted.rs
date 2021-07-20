extern crate graph;
use graph::{EdgeFileReader, Graph, NodeFileReader};

#[test]
fn test_load_sorted_parallel() {
    let graph_name = "Macaque".to_owned();
    let edges_reader = EdgeFileReader::new("tests/data/macaque.tsv")
        .unwrap()
        .set_separator(Some("\t"))
        .unwrap()
        .set_verbose(Some(false))
        .set_numeric_node_ids(Some(true))
        .set_complete(Some(true))
        .set_sorted(Some(true))
        .set_parallel(Some(true))
        .set_csv_is_correct(Some(true))
        .set_edges_number(Some(3054))
        .set_header(Some(false));

    let nodes_reader = NodeFileReader::new(None)
        .unwrap()
        .set_nodes_number(Some(242));

    let mut g = Graph::from_file_readers(
        Some(edges_reader),
        Some(nodes_reader),
        None,
        None,
        true,
        graph_name.clone(),
    )
    .unwrap();

    let _ = graph::test_utilities::default_test_suite(&mut g, Some(true));
}

#[test]
fn test_load_sorted_sequential() {
    let graph_name = "Macaque".to_owned();
    let edges_reader = EdgeFileReader::new("tests/data/macaque.tsv")
        .unwrap()
        .set_separator(Some("\t"))
        .unwrap()
        .set_verbose(Some(false))
        .set_numeric_node_ids(Some(true))
        .set_complete(Some(true))
        .set_sorted(Some(true))
        .set_parallel(Some(false))
        .set_csv_is_correct(Some(true))
        .set_edges_number(Some(3054))
        .set_header(Some(false));

    let nodes_reader = NodeFileReader::new(None)
        .unwrap()
        .set_nodes_number(Some(242));

    let mut g = Graph::from_file_readers(
        Some(edges_reader),
        Some(nodes_reader),
        None,
        None,
        true,
        graph_name.clone(),
    )
    .unwrap();

    let _ = graph::test_utilities::default_test_suite(&mut g, Some(true));
}
