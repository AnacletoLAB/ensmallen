extern crate graph;
use graph::*;

pub(crate) fn load_ppi() -> Result<Graph, String> {
    let edges_csv_reader =
        CSVFileReader::new("tests/data/expected_to_pass/ppi/edges.tsv".to_string())?
            .set_verbose(Some(false));
    let nodes_csv_reader =
        CSVFileReader::new("tests/data/expected_to_pass/ppi/nodes.tsv".to_string())?
            .set_verbose(Some(false));
    let edges_reader = EdgeFileReader::new(&edges_csv_reader)
        .set_sources_column(Some("subject".to_string()))?
        .set_destinations_column(Some("object".to_string()))?
        .set_weights_column(Some("weight".to_string()))?;
    let nodes_reader = NodeFileReader::new(&nodes_csv_reader)
        .set_node_types_column(Some("category".to_string()))?;
    Graph::from_csv(edges_reader, Some(nodes_reader), false, false, false, false)
}

pub(crate) fn first_order_walker(graph: &Graph) -> WalksParameters {
    WalksParameters::new(
        SingleWalkParameters::new(
            50, 
            WalkWeights::default()
        ).unwrap(), 
        0, 
        graph.not_trap_nodes().len()
    ).unwrap()
}

pub(crate) fn second_order_walker(graph: &Graph) -> WalksParameters {
    WalksParameters::new(
        SingleWalkParameters::new(
            50, 
            WalkWeights::default().set_explore_weight(Some(2.0)).unwrap()
        ).unwrap(), 
        0, 
        graph.not_trap_nodes().len()
    ).unwrap()
}
