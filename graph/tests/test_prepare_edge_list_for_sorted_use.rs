extern crate graph;

use graph::{build_optimal_lists_files, EdgeFileReader, Graph, NodeFileReader};

#[test]
fn test_prepare_edge_list_for_sorted_undirected_use() -> Result<(), String> {
    let (_, nodes_number, _, edges_number) = build_optimal_lists_files(
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
        Some("\t".to_string()),
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
        "tests/data/unsorted_macaque.tsv".to_string(),
        None,
        Some(false),
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
        "tests/data/sorted_macaque.tsv".to_string(),
        None,
        Some(true),
        false,
        Some("Macaque".to_string()),
    )?;

    let graph_name = "Macaque".to_owned();
    let edges_reader = EdgeFileReader::new("tests/data/sorted_macaque.tsv")
        .unwrap()
        .set_header(Some(false))
        .unwrap()
        .set_separator(Some("\t".to_string()))
        .unwrap()
        .set_verbose(Some(false))
        .set_numeric_node_ids(Some(true))
        .set_complete(Some(true))
        .set_sorted(Some(true))
        .set_parallel(Some(false))
        .set_sources_column_number(Some(0))?
        .set_destinations_column_number(Some(1))?
        .set_csv_is_correct(Some(true))
        .set_edges_number(Some(edges_number));

    let nodes_reader = NodeFileReader::new(None)
        .unwrap()
        .set_nodes_number(Some(nodes_number));

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
        .set_separator(Some("\t".to_string()))
        .unwrap()
        .set_verbose(Some(false))
        .set_numeric_node_ids(Some(true))
        .set_complete(Some(true))
        .set_sorted(Some(true))
        .set_parallel(Some(true))
        .set_sources_column_number(Some(0))?
        .set_destinations_column_number(Some(1))?
        .set_csv_is_correct(Some(true))
        .set_edges_number(Some(edges_number));

    let nodes_reader = NodeFileReader::new(None)
        .unwrap()
        .set_nodes_number(Some(nodes_number));

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

    let (_, nodes_number, _, edges_number) = build_optimal_lists_files(
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
        Some("\t".to_string()),
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
        "tests/data/unsorted_macaque.tsv".to_string(),
        None,
        Some(false),
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
        "tests/data/sorted_macaque.tsv".to_string(),
        None,
        Some(true),
        true,
        Some("Macaque".to_string()),
    )?;

    let graph_name = "Macaque".to_owned();
    let edges_reader = EdgeFileReader::new("tests/data/sorted_macaque.tsv")
        .unwrap()
        .set_header(Some(false))
        .unwrap()
        .set_separator(Some("\t".to_string()))
        .unwrap()
        .set_verbose(Some(false))
        .set_numeric_node_ids(Some(true))
        .set_complete(Some(true))
        .set_sorted(Some(true))
        .set_parallel(Some(false))
        .set_sources_column_number(Some(0))?
        .set_destinations_column_number(Some(1))?
        .set_csv_is_correct(Some(true))
        .set_edges_number(Some(edges_number));

    let nodes_reader = NodeFileReader::new(None)
        .unwrap()
        .set_nodes_number(Some(nodes_number));

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
        .set_separator(Some("\t".to_string()))
        .unwrap()
        .set_verbose(Some(false))
        .set_numeric_node_ids(Some(true))
        .set_complete(Some(true))
        .set_sorted(Some(true))
        .set_parallel(Some(true))
        .set_sources_column_number(Some(0))?
        .set_destinations_column_number(Some(1))?
        .set_csv_is_correct(Some(true))
        .set_edges_number(Some(edges_number));

    let nodes_reader = NodeFileReader::new(None)
        .unwrap()
        .set_nodes_number(Some(nodes_number));

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
