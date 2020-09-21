extern crate graph;
#[test]
/// this test used to deadlock the sample negatives
/// becasue we computed wrongly the total number of negative edges
/// in undirected graphs.
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
/// this test used to deadlock the sample negatives
/// becasue we erroneously extracted the nodes from the 
/// present srcs and dsts instead of random nodes.
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

#[test]
/// this test used to deadlock the sample negatives
/// because the condition was unique_edges_tree.len() <= negatives_number
/// instead of unique_edges_tree.len() < negatives_number
/// therefore it used to return one edge more than the needed
/// and if the graph had EXACTLY the number of negative edges as the wanted
fn test_deadlock3() {

    let edges = vec![
        Ok(("node1".to_string(), "node2".to_string(), Some("type1".to_string()), None)),
        Ok(("node2".to_string(), "node2".to_string(), Some("type2".to_string()), None)),
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

#[test]
/// this test used to deadlock the sample negatives
fn test_deadlock4() {

    let edges = vec![
        Ok(("node1".to_string(), "node1".to_string(),None, None)),
        Ok(("node1".to_string(), "node2".to_string(),None, None)),
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
/// this test used to deadlock the sample negatives
fn test_regression5() {

    let edges = vec![
        Ok(("18".to_string(), "0".to_string(), None, None)),
        Ok(("3".to_string(), "0".to_string(), None, None)),
        Ok(("8".to_string(), "0".to_string(), None, None)),
        Ok(("14".to_string(), "0".to_string(), None, None)),
        Ok(("19".to_string(), "0".to_string(), None, None)),
        Ok(("24".to_string(), "0".to_string(), None, None)),
        Ok(("4".to_string(), "0".to_string(), None, None)),
        Ok(("9".to_string(), "0".to_string(), None, None)),
        Ok(("15".to_string(), "0".to_string(), None, None)),
        Ok(("20".to_string(), "0".to_string(), None, None)),
        Ok(("25".to_string(), "0".to_string(), None, None)),
        Ok(("5".to_string(), "0".to_string(), None, None)),
        Ok(("10".to_string(), "0".to_string(), None, None)),
        Ok(("0".to_string(), "0".to_string(), None, None)),
        Ok(("21".to_string(), "0".to_string(), None, None)),
        Ok(("26".to_string(), "0".to_string(), None, None)),
        Ok(("6".to_string(), "0".to_string(), None, None)),
        Ok(("11".to_string(), "0".to_string(), None, None)),
        Ok(("16".to_string(), "0".to_string(), None, None)),
        Ok(("1".to_string(), "0".to_string(), None, None)),
        Ok(("22".to_string(), "0".to_string(), None, None)),
        Ok(("27".to_string(), "0".to_string(), None, None)),
        Ok(("7".to_string(), "0".to_string(), None, None)),
        Ok(("12".to_string(), "0".to_string(), None, None)),
        Ok(("17".to_string(), "0".to_string(), None, None)),
        Ok(("2".to_string(), "0".to_string(), None, None)),
        Ok(("23".to_string(), "0".to_string(), None, None)),
        Ok(("28".to_string(), "0".to_string(), None, None)),
        Ok(("13".to_string(), "0".to_string(), None, None))
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
