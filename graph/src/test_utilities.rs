use super::*;
use std::fs;


pub fn load_ppi(
    load_nodes: bool,
    load_edge_types: bool,
    load_weights: bool,
    directed: bool,
    verbose: bool,
) -> Result<Graph, String> {
    let edges_csv_reader = CSVFileReader::new("tests/data/ppi/edges.tsv".to_string())?
        .set_verbose(Some(verbose))
        .set_ignore_duplicates(Some(true))
        .set_separator(Some("\t".to_string()))
        .set_header(Some(true))
        .set_rows_to_skip(Some(0));
    let nodes_csv_reader =
        CSVFileReader::new("tests/data/ppi/nodes.tsv".to_string())?.set_verbose(Some(false));
    let nodes_reader = if load_nodes {
        Some(
            NodeFileReader::new(&nodes_csv_reader)
                .set_node_types_column_number(Some(56))
                .set_nodes_column_number(Some(56))
                .set_node_types_column(Some("category".to_string()))?
                .set_default_node_type(Some("default".to_string()))
                .set_nodes_column(Some("id".to_string()))?
                .set_ignore_duplicated_nodes(Some(true)),
        )
    } else {
        None
    };
    let edges_reader = EdgeFileReader::new(&edges_csv_reader)
        .set_sources_column(Some("subject".to_string()))?
        .set_destinations_column(Some("object".to_string()))?
        .set_weights_column(if load_weights {
            Some("weight".to_string())
        } else {
            None
        })?
        .set_edge_types_column(if load_edge_types {
            Some("edge_label".to_string())
        } else {
            None
        })?
        .set_default_edge_type(Some("Kebab".to_string()))
        .set_default_weight(Some(5.0))
        .set_ignore_duplicated_edges(Some(true))
        .set_skip_self_loops(Some(false));

    Graph::from_csv(edges_reader, nodes_reader, directed)
}


pub fn first_order_walker(graph: &Graph) -> WalksParameters {
    WalksParameters::new(
        SingleWalkParameters::new(50, WalkWeights::default()).unwrap(),
        0,
        graph.get_not_trap_nodes_number(),
    )
    .unwrap()
    .set_iterations(Some(1))
    .unwrap()
    .set_min_length(Some(1))
    .unwrap()
    .set_verbose(Some(false))
    .set_seed(Some(43))
    .set_dense_nodes_mapping(Some(graph.get_dense_nodes_mapping()))
}


pub fn second_order_walker(graph: &Graph) -> WalksParameters {
    WalksParameters::new(
        SingleWalkParameters::new(
            50,
            WalkWeights::default()
                .set_explore_weight(Some(2.0))
                .unwrap()
                .set_return_weight(Some(2.0))
                .unwrap()
                .set_change_node_type_weight(Some(2.0))
                .unwrap()
                .set_change_edge_type_weight(Some(2.0))
                .unwrap(),
        )
        .unwrap(),
        0,
        graph.get_not_trap_nodes_number(),
    )
    .unwrap()
    .set_iterations(Some(1))
    .unwrap()
    .set_min_length(Some(1))
    .unwrap()
    .set_verbose(Some(false))
    .set_seed(Some(43))
}


pub fn default_holdout_test_suite(graph: &Graph, train: &Graph, test: &Graph) {
    assert!(!train.overlaps(&test).unwrap());
    assert!(!test.overlaps(&train).unwrap());
    assert!(graph.contains(&train).unwrap());
    assert!(graph.contains(&test).unwrap());
    let summed = (train + test).unwrap();
    assert!(summed.contains(&graph).unwrap());
}


pub fn default_test_suite(graph: &Graph, verbose: bool) {
    // Testing principal random walk algorithms
    let walker = first_order_walker(&graph);
    graph.walk(&walker).unwrap();
    graph.walk(&second_order_walker(&graph)).unwrap();
    // Testing main holdout mechanisms
    for include_all_edge_types in &[true, false] {
        let (train, test) = graph
            .random_holdout(4, 0.6, *include_all_edge_types, verbose)
            .unwrap();
        default_holdout_test_suite(graph, &train, &test);
        let (train, test) = graph
            .connected_holdout(4, 0.6, *include_all_edge_types, verbose)
            .unwrap();
        default_holdout_test_suite(graph, &train, &test);
        assert!(train != test);
    }
    // Testing cloning
    let _ = graph.clone();
    // Testing negative edges generation
    let negatives = graph
        .sample_negatives(4, graph.get_edges_number(), true, verbose)
        .unwrap();
    assert!(!graph.overlaps(&negatives).unwrap());
    assert!(!negatives.overlaps(&graph).unwrap());
    // Testing subgraph generation
    let subgraph = graph
        .random_subgraph(6, graph.get_nodes_number() / 10, verbose)
        .unwrap();
    assert!(subgraph.overlaps(&graph).unwrap());
    // Testing writing out graph to file
    let nodes_writer = NodeFileWriter::new(
        CSVFileWriter::new("tmp_node_file.tsv".to_string()).set_verbose(Some(verbose)),
    )
    .set_node_types_column_number(Some(4))
    .set_nodes_column_number(Some(6))
    .set_node_types_column(Some("node_types".to_string()))
    .set_nodes_column(Some("node_column".to_string()));
    nodes_writer.dump(&graph).unwrap();
    fs::remove_file("tmp_node_file.tsv").unwrap();
    let edges_writer = EdgeFileWriter::new(
        CSVFileWriter::new("tmp_edge_file.tsv".to_string())
            .set_verbose(Some(verbose))
            .set_separator(Some("\t".to_string()))
            .set_header(Some(true)),
    )
    .set_edge_types_column(Some("edge_types".to_string()))
    .set_destinations_column_number(Some(3))
    .set_weights_column(Some("weight".to_string()))
    .set_weights_column_number(Some(2))
    .set_sources_column(Some("The land of sushi".to_string()))
    .set_sources_column_number(Some(0))
    .set_destinations_column(Some("The land of pizza".to_string()))
    .set_destinations_column_number(Some(1));
    edges_writer.dump(&graph).unwrap();
    fs::remove_file("tmp_edge_file.tsv").unwrap();
    // Testing SkipGram / CBOW / GloVe preprocessing
    graph
        .binary_skipgrams(56, &walker, Some(3), Some(2.0), Some(true))
        .unwrap();
    graph
        .cooccurence_matrix(&walker, Some(3), Some(verbose))
        .unwrap();
    graph.node2vec(&walker, Some(3), Some(true), 56).unwrap();
    // Testing link prediction pre-processing
    graph.link_prediction(0, 16, Some(1.0), None, None).unwrap();
    // Compute metrics of the graph
    graph.report();
    // Compute degrees metrics
    for src in 0..10 {
        for dst in 0..10 {
            graph.degrees_product(src, dst).unwrap();
            graph.jaccard_index(src, dst).unwrap();
            graph.adamic_adar_index(src, dst).unwrap();
            graph.resource_allocation_index(src, dst).unwrap();
        }
    }
    // Testing the top Ks
    if graph.has_node_types(){
        graph.get_top_k_nodes_by_node_type(10).unwrap();
        graph.get_node_type_id(0).unwrap();
    }
    if graph.has_edge_types(){
        graph.get_top_k_edges_by_edge_type(10).unwrap();
        graph.get_edge_type_id(0).unwrap();
    }

    // Evaluate get_node_type_counts
    assert_eq!(graph.get_node_type_counts().is_ok(), graph.has_node_types());

    // Evaluate get_edge_type_counts
    assert_eq!(graph.get_edge_type_counts().is_ok(), graph.has_edge_types());

}
