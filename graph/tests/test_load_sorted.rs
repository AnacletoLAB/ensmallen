extern crate graph;
use graph::{EdgeFileReader, Graph};

#[test]
/// this test used to deadlock the sample negatives
/// becasue we computed wrongly the total number of negative edges
/// in undirected graphs.
fn test_load_sorted() {
    let graph_name = "Macaque".to_owned();
    let edges_reader = EdgeFileReader::new("tests/data/macaque.tsv")
        .unwrap()
        .set_separator(Some("\t"))
        .unwrap()
        .set_verbose(Some(false))
        .set_numeric_node_ids(Some(true))
        .set_header(Some(false));

    let mut g = Graph::from_sorted_csv(
        edges_reader,
        None,
        false,
        false,
        6108,
        242,
        graph_name.clone(),
    )
    .unwrap();

    let _ = graph::test_utilities::default_test_suite(&mut g, Some(true));
}
