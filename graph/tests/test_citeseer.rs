extern crate graph;

use graph::{EdgeFileReader, Graph, NodeFileReader};

#[test]
fn test_citeseer() -> Result<(), String> {
    env_logger::init();
    let edges_reader = EdgeFileReader::new("tests/data/citeseer/edges.tsv")?
        .set_separator(Some("\t"))?
        .set_verbose(Some(true))
        .set_sources_column(Some("subject"))?
        .set_destinations_column(Some("object"))?
        .set_edge_types_column(Some("edge_type"))?;
    let nodes_reader = Some(
        NodeFileReader::new("tests/data/citeseer/nodes.tsv")?
            .set_separator(Some("\t"))?
            .set_nodes_column(Some("id"))?
            .set_node_types_column(Some("node_type"))?,
    );
    let mut citeseer = Graph::from_unsorted_csv(
        edges_reader,
        nodes_reader,
        false,
        false,
        "CiteSeer".to_owned(),
    )?;
    let _ = graph::test_utilities::default_test_suite(&mut citeseer, false);
    Ok(())
}
