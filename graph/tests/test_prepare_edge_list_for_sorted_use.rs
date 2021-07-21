extern crate graph;

use graph::{
    add_edge_id_to_edge_list, convert_undirected_edge_list_to_directed,
    densify_sparse_numeric_edge_list, sort_numeric_edge_list, EdgeFileReader, Graph,
    NodeFileReader,
};

#[test]
fn test_prepare_edge_list_for_sorted_use() -> Result<(), String> {
    densify_sparse_numeric_edge_list(
        None,
        "tests/data/sparse_numeric_macaque.tsv",
        Some("\t".to_string()),
        Some(false),
        None,
        Some(0),
        None,
        Some(1),
        None,
        None,
        None,
        None,
        "tests/data/dense_macaque.tsv",
        Some("\t".to_string()),
        Some(true),
        Some("subject".to_string()),
        None,
        Some("object".to_string()),
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
    )?;
    convert_undirected_edge_list_to_directed(
        "tests/data/dense_macaque.tsv",
        Some("\t".to_string()),
        Some(true),
        Some("subject".to_string()),
        None,
        Some("object".to_string()),
        None,
        None,
        None,
        None,
        None,
        "tests/data/undirected_macaque.tsv",
        Some("\t".to_string()),
        Some(false),
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
    )?;
    sort_numeric_edge_list(
        "tests/data/undirected_macaque.tsv",
        "tests/data/sorted_undirected_macaque.tsv",
        Some("\t".to_string()),
        Some(false),
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
    )?;

    add_edge_id_to_edge_list(
        "tests/data/sorted_undirected_macaque.tsv",
        Some("\t".to_string()),
        Some(false),
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        "tests/data/sorted_undirected_macaque_with_edge_ids.tsv",
        Some("\t".to_string()),
        Some(false),
        None,
        Some(1),
        None,
        Some(2),
        None,
        None,
        None,
        None,
        None,
        Some(0),
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
    )?;

    let graph_name = "Macaque".to_owned();
    let edges_reader =
        EdgeFileReader::new("tests/data/sorted_undirected_macaque.tsv")
            .unwrap()
            .set_separator(Some("\t"))
            .unwrap()
            .set_verbose(Some(false))
            .set_numeric_node_ids(Some(true))
            .set_complete(Some(true))
            .set_sorted(Some(true))
            .set_parallel(Some(false))
            .set_csv_is_correct(Some(true))
            .set_edges_number(Some(2598))
            .set_header(Some(false));

    let nodes_reader = NodeFileReader::new(None)
        .unwrap()
        .set_nodes_number(Some(193));

    let mut g = Graph::from_file_readers(
        Some(edges_reader),
        Some(nodes_reader),
        None,
        None,
        false,
        graph_name.clone(),
    )
    .unwrap();

    let _ = graph::test_utilities::default_test_suite(&mut g, Some(true));

    let graph_name = "Macaque".to_owned();
    let edges_reader = EdgeFileReader::new("tests/data/sorted_undirected_macaque_with_edge_ids.tsv")
        .unwrap()
        .set_separator(Some("\t"))
        .unwrap()
        .set_verbose(Some(false))
        .set_numeric_node_ids(Some(true))
        .set_complete(Some(true))
        .set_sorted(Some(true))
        .set_parallel(Some(true))
        .set_edge_ids_column_number(Some(0))?
        .set_sources_column_number(Some(1))?
        .set_destinations_column_number(Some(2))?
        .set_csv_is_correct(Some(true))
        .set_edges_number(Some(2598))
        .set_header(Some(false));

    let nodes_reader = NodeFileReader::new(None)
        .unwrap()
        .set_nodes_number(Some(193));

    let mut g = Graph::from_file_readers(
        Some(edges_reader),
        Some(nodes_reader),
        None,
        None,
        false,
        graph_name.clone(),
    )
    .unwrap();

    let _ = graph::test_utilities::default_test_suite(&mut g, Some(true));

    Ok(())
}
