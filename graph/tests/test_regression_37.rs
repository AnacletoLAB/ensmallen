extern crate graph;

use graph::{Graph, EdgeFileReader, NodeFileReader};

#[test]
fn test_regression_37() -> Result<(), String> {
    let mut graph = Graph::from_string_unsorted(
        Vec::new().into_iter(),
        Some(vec![
            Ok(("0".to_string(), Some(vec!["0".to_string()]))),
            Ok(("1".to_string(), Some(vec!["1".to_string()]))),
        ].into_iter()),
        false, // Directed
        true, // Directed edge list
        "Fuzz Graph", // Name of the graph
        true,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        true,
        true,
        false,
        true,
        true,
        true,
        false,
    )?;
    graph.remove_inplace_singleton_node_types().unwrap();
    let _ = graph::test_utilities::default_test_suite(&mut graph, false);
    Ok(())
}
