extern crate graph;

use graph::{build_optimal_lists_files, EdgeFileReader, Graph, NodeFileReader};

#[test]
fn test_prepare_edge_list_for_sorted_undirected_use() -> Result<(), String> {
    let (_, number_of_nodes, _, number_of_edges) = build_optimal_lists_files(
        "tests/data/unsorted_macaque.tsv".to_string(),
        "tests/data/sorted_macaque.tsv".to_string(),
        false,
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
        None,
        None,
        None,
        None,
        Some("tests/data/macaque_node_list.tsv".to_string()),
        Some('\t'),
        Some(true),
        Some("node_name".to_string()),
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
        Some(false),
        None,
        Some(0),
        None,
        Some(1),
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
        Some(true),
        None,
        None,
        None,
        None,
        None,
        None,
        Some(true),
        Some("Macaque".to_string()),
    )?;

    let graph_name = "Macaque".to_owned();
    let edges_reader = EdgeFileReader::new("tests/data/sorted_macaque.tsv")
        .unwrap()
        .set_header(Some(false))
        .unwrap()
        .set_separator(Some('\t'))
        .unwrap()
        .set_verbose(Some(false))
        .set_numeric_node_ids(Some(true))
        .set_complete(Some(true))
        .set_sorted(Some(true))
        .set_parallel(Some(false))
        .set_sources_column_number(Some(0))?
        .set_destinations_column_number(Some(1))?
        .set_csv_is_correct(Some(true))
        .set_number_of_edges(Some(number_of_edges));

    let nodes_reader = NodeFileReader::new(None)
        .unwrap()
        .set_number_of_nodes(Some(number_of_nodes));

    let mut g = Graph::from_file_readers(
        Some(edges_reader),
        Some(nodes_reader),
        None,
        None,
        true,
        true,
        false,
        graph_name.clone(),
    )
    .unwrap();

    let _ = graph::test_utilities::default_test_suite(&mut g, Some(false));

    let graph_name = "Macaque".to_owned();
    let edges_reader = EdgeFileReader::new("tests/data/sorted_macaque.tsv")
        .unwrap()
        .set_header(Some(false))
        .unwrap()
        .set_separator(Some('\t'))
        .unwrap()
        .set_verbose(Some(false))
        .set_numeric_node_ids(Some(true))
        .set_complete(Some(true))
        .set_sorted(Some(true))
        .set_parallel(Some(true))
        .set_sources_column_number(Some(0))?
        .set_destinations_column_number(Some(1))?
        .set_csv_is_correct(Some(true))
        .set_number_of_edges(Some(number_of_edges));

    let nodes_reader = NodeFileReader::new(None)
        .unwrap()
        .set_number_of_nodes(Some(number_of_nodes));

    let mut g = Graph::from_file_readers(
        Some(edges_reader),
        Some(nodes_reader),
        None,
        None,
        true,
        true,
        false,
        graph_name.clone(),
    )
    .unwrap();

    let _ = graph::test_utilities::default_test_suite(&mut g, Some(false));

    let (_, number_of_nodes, _, number_of_edges) = build_optimal_lists_files(
        "tests/data/unsorted_macaque.tsv".to_string(),
        "tests/data/sorted_macaque.tsv".to_string(),
        true,
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
        None,
        None,
        None,
        None,
        Some("tests/data/macaque_node_list.tsv".to_string()),
        Some('\t'),
        Some(true),
        Some("node_name".to_string()),
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
        Some(false),
        None,
        Some(0),
        None,
        Some(1),
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
        Some(true),
        None,
        None,
        None,
        None,
        None,
        None,
        Some(true),
        Some("Macaque".to_string()),
    )?;

    let graph_name = "Macaque".to_owned();
    let edges_reader = EdgeFileReader::new("tests/data/sorted_macaque.tsv")
        .unwrap()
        .set_header(Some(false))
        .unwrap()
        .set_separator(Some('\t'))
        .unwrap()
        .set_verbose(Some(false))
        .set_numeric_node_ids(Some(true))
        .set_complete(Some(true))
        .set_sorted(Some(true))
        .set_parallel(Some(false))
        .set_sources_column_number(Some(0))?
        .set_destinations_column_number(Some(1))?
        .set_csv_is_correct(Some(true))
        .set_number_of_edges(Some(number_of_edges));

    let nodes_reader = NodeFileReader::new(None)
        .unwrap()
        .set_number_of_nodes(Some(number_of_nodes));

    let mut g = Graph::from_file_readers(
        Some(edges_reader),
        Some(nodes_reader),
        None,
        None,
        true,
        true,
        true,
        graph_name.clone(),
    )
    .unwrap();

    let _ = graph::test_utilities::default_test_suite(&mut g, Some(false));

    let graph_name = "Macaque".to_owned();
    let edges_reader = EdgeFileReader::new("tests/data/sorted_macaque.tsv")
        .unwrap()
        .set_header(Some(false))
        .unwrap()
        .set_separator(Some('\t'))
        .unwrap()
        .set_verbose(Some(false))
        .set_numeric_node_ids(Some(true))
        .set_complete(Some(true))
        .set_sorted(Some(true))
        .set_parallel(Some(true))
        .set_sources_column_number(Some(0))?
        .set_destinations_column_number(Some(1))?
        .set_csv_is_correct(Some(true))
        .set_number_of_edges(Some(number_of_edges));

    let nodes_reader = NodeFileReader::new(None)
        .unwrap()
        .set_number_of_nodes(Some(number_of_nodes));

    let mut g = Graph::from_file_readers(
        Some(edges_reader),
        Some(nodes_reader),
        None,
        None,
        true,
        true,
        true,
        graph_name.clone(),
    )
    .unwrap();

    let _ = graph::test_utilities::default_test_suite(&mut g, Some(false));

    Ok(())
}
