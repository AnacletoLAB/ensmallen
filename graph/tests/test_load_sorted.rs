extern crate graph;
use graph::{EdgeFileReader, EdgeFileWriter, Graph};

#[test]
/// this test used to deadlock the sample negatives
/// becasue we computed wrongly the total number of negative edges
/// in undirected graphs.
fn test_load_sorted() {
    let edges_reader = EdgeFileReader::new("tests/data/macaque3.tsv".to_string())
        .unwrap()
        .set_separator(Some("\t".to_string()))
        .set_verbose(Some(false))
        //.set_numeric_node_ids(Some(true))
        .set_header(Some(false));

    // let edges_writer = EdgeFileWriter::new("tests/data/macaque3.tsv".to_string())
    //     .set_separator(Some("\t".to_string()))
    //     .set_verbose(Some(false))
    //     .set_header(Some(false));

    let g = Graph::from_unsorted_csv(edges_reader.clone(), None, false).unwrap();

    //edges_writer.dump(&g).unwrap();

    let g2 = Graph::from_sorted_csv(
        edges_reader,
        None,
        false,
        g.get_edges_number(),
        g.get_nodes_number(),
    )
    .unwrap();

    let _ = graph::test_utilities::default_test_suite(&g2, true).unwrap();
}
