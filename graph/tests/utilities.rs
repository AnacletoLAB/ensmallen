extern crate graph;
use graph::*;

#[cfg(test)]
pub(crate) fn load_ppi(load_nodes: bool) -> Result<Graph, String> {
    let edges_csv_reader =
        CSVFileReader::new("tests/data/expected_to_pass/ppi/edges.tsv".to_string())?
            .set_verbose(Some(false));
    let nodes_csv_reader =
        CSVFileReader::new("tests/data/expected_to_pass/ppi/nodes.tsv".to_string())?
            .set_verbose(Some(false));
    let nodes_reader = if load_nodes {
        Some(
            NodeFileReader::new(&nodes_csv_reader)
                .set_node_types_column(Some("category".to_string()))?,
        )
    } else {
        None
    };
    let edges_reader = EdgeFileReader::new(&edges_csv_reader)
        .set_sources_column(Some("subject".to_string()))?
        .set_destinations_column(Some("object".to_string()))?
        .set_weights_column(Some("weight".to_string()))?;

    Graph::from_csv(edges_reader, nodes_reader, false, false, false, false)
}

#[cfg(test)]
pub(crate) fn first_order_walker(graph: &Graph) -> WalksParameters {
    WalksParameters::new(
        SingleWalkParameters::new(50, WalkWeights::default()).unwrap(),
        0,
        graph.not_trap_nodes().len(),
    )
    .unwrap()
}

#[cfg(test)]
pub(crate) fn second_order_walker(graph: &Graph) -> WalksParameters {
    WalksParameters::new(
        SingleWalkParameters::new(
            50,
            WalkWeights::default()
                .set_explore_weight(Some(2.0))
                .unwrap(),
        )
        .unwrap(),
        0,
        graph.not_trap_nodes().len(),
    )
    .unwrap()
}

#[cfg(test)]
fn default_holdout_test_suite(graph: &Graph, train: &Graph, test: &Graph) {
    assert!(!train.overlaps(&test).unwrap());
    assert!(!test.overlaps(&train).unwrap());
    assert!(graph.contains(&train).unwrap());
    assert!(graph.contains(&test).unwrap());
}

#[cfg(test)]
pub(crate) fn default_test_suite(graph: &Graph) {
    graph.walk(&first_order_walker(&graph)).unwrap();
    graph.walk(&second_order_walker(&graph)).unwrap();
    let (train, test) = graph.random_holdout(4, 0.6, true).unwrap();
    default_holdout_test_suite(graph, &train, &test);
    let (train, test) = graph.connected_holdout(4, 0.6, true).unwrap();
    default_holdout_test_suite(graph, &train, &test);
}
