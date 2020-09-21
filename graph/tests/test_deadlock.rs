extern crate graph;
#[test]
// this test used to deadlock the sample negatives
// becasue we computed wrongly the total number of negative edges
// in undirected graphs.
fn test_deadlock1() {

    let edges = vec![
        Ok(("node1".to_string(), "node1".to_string(), None, None)),
        Ok(("node2".to_string(), "node2".to_string(), None, None)),
    ];
    let nodes: Option<Vec<Result<(String, Option<String>), String>>> = None;

    let g = graph::Graph::new(
        edges.iter().cloned(),
        if let Some(dn) = &nodes {
            Some(dn.iter().cloned())
        } else {
            None
        },
        false,
        false,
        false,
        false
    ).unwrap();
    graph::test_utilities::default_test_suite(&g, false);
}

#[test]
// this test used to deadlock the sample negatives
// becasue we erroneously extracted the nodes from the 
// present srcs and dsts instead of random nodes.
fn test_deadlock2() {

    let edges = vec![
        Ok(("node1".to_string(), "node2".to_string(), None, None)),
        Ok(("node2".to_string(), "node3".to_string(), None, None)),
    ];
    let nodes: Option<Vec<Result<(String, Option<String>), String>>> = None;

    let g = graph::Graph::new(
        edges.iter().cloned(),
        if let Some(dn) = &nodes {
            Some(dn.iter().cloned())
        } else {
            None
        },
        true,
        false,
        false,
        false
    ).unwrap();
    graph::test_utilities::default_test_suite(&g, false);
}
