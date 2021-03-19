extern crate graph;

use graph::{EdgeFileReader, Graph, NodeFileReader};

#[test]
fn test_cora() -> Result<(), String> {
    env_logger::init();
    let edges_reader = EdgeFileReader::new("tests/data/cora/edges.tsv")?
        .set_separator(Some("\t"))?
        .set_verbose(Some(true))
        .set_max_rows_number(Some(10000))
        .set_sources_column(Some("subject"))?
        .set_destinations_column(Some("object"))?
        .set_edge_types_column(Some("edge_type"))?;
    let nodes_reader = Some(
        NodeFileReader::new("tests/data/cora/nodes.tsv")?
            .set_separator(Some("\t"))?
            .set_nodes_column(Some("id"))?
            .set_node_types_column(Some("node_type"))?,
    );
    let mut cora =
        Graph::from_unsorted_csv(edges_reader, nodes_reader, false, false, "Cora".to_owned())?
            .remove(
                None,
                None,
                None,
                Some(["Word".to_string()].iter().map(|nt| nt.clone()).collect()),
                None,
                None,
                None,
                None,
                false,
                false,
                false,
                false,
                false,
                false,
            )?;
    let _ = graph::test_utilities::default_test_suite(&mut cora, false);
    Ok(())
}
