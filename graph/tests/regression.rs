extern crate graph;
use graph::{EdgeFileReader, Graph};

#[test]
/// this test used to deadlock the sample negatives
/// becasue we computed wrongly the total number of negative edges
/// in undirected graphs.
fn test_deadlock1() -> Result<(), String> {
    let edges_reader = EdgeFileReader::new("tests/data/regression/1.tsv".to_string())?
        .set_separator(Some(",".to_string()))?
        .set_verbose(Some(false))
        .set_header(Some(false));
    let mut g = Graph::from_unsorted_csv(edges_reader, None, false, false, "Graph".to_owned())?;
    let _ = graph::test_utilities::default_test_suite(&mut g, false);
    Ok(())
}

#[test]
/// this test used to deadlock the sample negatives
/// becasue we erroneously extracted the nodes from the
/// present srcs and dsts instead of random nodes.
fn test_deadlock2() -> Result<(), String> {
    let edges_reader = EdgeFileReader::new("tests/data/regression/2.tsv".to_string())?
        .set_separator(Some(",".to_string()))?
        .set_verbose(Some(false))
        .set_header(Some(false));
    let mut g = Graph::from_unsorted_csv(edges_reader, None, true, false, "Graph".to_owned())?;
    let _ = graph::test_utilities::default_test_suite(&mut g, false);
    Ok(())
}

#[test]
/// this test used to deadlock the sample negatives
/// because the condition was unique_edges_tree.len() <= negatives_number
/// instead of unique_edges_tree.len() < negatives_number
/// therefore it used to return one edge more than the needed
/// and if the graph had EXACTLY the number of negative edges as the wanted
fn test_deadlock3() -> Result<(), String> {
    let edges_reader = EdgeFileReader::new("tests/data/regression/3.tsv".to_string())?
        .set_separator(Some(",".to_string()))?
        .set_verbose(Some(false))
        .set_header(Some(false))
        .set_edge_types_column_number(Some(2))?;
    let mut g = Graph::from_unsorted_csv(edges_reader, None, true, false, "Graph".to_owned())?;
    let _ = graph::test_utilities::default_test_suite(&mut g, false);
    Ok(())
}

#[test]
/// this test used to deadlock the sample negatives
/// This was caused because the insertion of the current node was done in the wrong
/// place that made impossible to add some self-loops.
fn test_deadlock4() -> Result<(), String> {
    let edges_reader = EdgeFileReader::new("tests/data/regression/4.tsv".to_string())?
        .set_separator(Some(",".to_string()))?
        .set_verbose(Some(false))
        .set_header(Some(false));
    let mut g = Graph::from_unsorted_csv(edges_reader, None, false, false, "Graph".to_owned())?;
    let _ = graph::test_utilities::default_test_suite(&mut g, false);
    Ok(())
}

#[test]
/// this test used to panic subgraph
/// the graph is a star
/// This used to crash because the algorithm would insert the center of the star
/// and then it couldn't add any other node because of a bad check that did not
/// add nodes already present.
fn test_regression5() -> Result<(), String> {
    let edges_reader = EdgeFileReader::new("tests/data/regression/5.tsv".to_string())?
        .set_separator(Some(",".to_string()))?
        .set_verbose(Some(false))
        .set_header(Some(false));
    let mut g = Graph::from_unsorted_csv(edges_reader, None, true, false, "Graph".to_owned())?;
    let _ = graph::test_utilities::default_test_suite(&mut g, false);
    Ok(())
}

#[test]
/// this test used to panic subgraph
/// the graph is a star
/// To make this problem computable (not NP) we might add at most ONE extra node
/// than the required ones. Because If we want 4 nodes and we take a component with
/// 3 nodes. There is no way to add another not-singleton node.
/// Therefore it could became a knapsack problem.
fn test_regression6() -> Result<(), String> {
    let edges_reader = EdgeFileReader::new("tests/data/regression/6.tsv".to_string())?
        .set_separator(Some(",".to_string()))?
        .set_verbose(Some(false))
        .set_header(Some(false));
    let mut g = Graph::from_unsorted_csv(edges_reader, None, true, false, "Graph".to_owned())?;
    let _ = graph::test_utilities::default_test_suite(&mut g, false);
    Ok(())
}

#[test]
/// "red" hardcoded in the default test suite
/// this is not really a problem but since we already wrote it
/// we will keep it because it's an extra test for free.
fn test_regression7() -> Result<(), String> {
    let edges_reader = EdgeFileReader::new("tests/data/regression/7.tsv".to_string())?
        .set_separator(Some(",".to_string()))?
        .set_verbose(Some(false))
        .set_skip_self_loops(Some(true))
        .set_ignore_duplicates(Some(true))
        .set_edge_types_column_number(Some(2))?
        .set_weights_column_number(Some(3))?
        .set_header(Some(false));
    let mut g = Graph::from_unsorted_csv(edges_reader, None, true, false, "Graph".to_owned())?;
    let _ = graph::test_utilities::default_test_suite(&mut g, false);
    Ok(())
}

#[test]
/// Some issue is causing the walk to mis-behave when the graph is composite of two self loops of different types on the same node.
fn test_regression8() -> Result<(), String> {
    let edges_reader = EdgeFileReader::new("tests/data/regression/8.tsv".to_string())?
        .set_separator(Some(",".to_string()))?
        .set_verbose(Some(false))
        .set_ignore_duplicates(Some(true))
        .set_edge_types_column_number(Some(2))?
        .set_weights_column_number(Some(3))?
        .set_header(Some(false));
    let mut g = Graph::from_unsorted_csv(edges_reader, None, false, false, "Graph".to_owned())?;
    let _ = graph::test_utilities::default_test_suite(&mut g, false);
    Ok(())
}

#[test]
/// An unknown cause that does no longer exist used to make the library crash on this file.
fn test_regression9() -> Result<(), String> {
    let edges_reader = EdgeFileReader::new("tests/data/regression/9.tsv".to_string())?
        .set_separator(Some(",".to_string()))?
        .set_verbose(Some(false))
        .set_skip_self_loops(Some(true))
        .set_ignore_duplicates(Some(true))
        .set_header(Some(false));
    let mut g = Graph::from_unsorted_csv(edges_reader, None, true, false, "Graph".to_owned())?;
    let _ = graph::test_utilities::default_test_suite(&mut g, false);
    Ok(())
}

#[test]
/// An unknown cause is making the library crash on this file.
fn test_regression10() -> Result<(), String> {
    let edges_reader = EdgeFileReader::new("tests/data/regression/10.tsv".to_string())?
        .set_separator(Some(" ".to_string()))?
        .set_verbose(Some(false))
        .set_sources_column_number(Some(0))?
        .set_destinations_column_number(Some(1))?
        .set_weights_column_number(Some(2))?
        .set_header(Some(false));
    let mut g = Graph::from_unsorted_csv(edges_reader, None, true, false, "Graph".to_owned())?;
    let _ = graph::test_utilities::default_test_suite(&mut g, false);
    Ok(())
}

#[test]
fn test_regression11() -> Result<(), String> {
    let edges_reader = EdgeFileReader::new("tests/data/regression/11.tsv".to_string())?
        .set_separator(Some(",".to_string()))?
        .set_verbose(Some(false))
        .set_skip_self_loops(Some(true))
        .set_sources_column_number(Some(0))?
        .set_destinations_column_number(Some(1))?
        .set_header(Some(false));
    let mut g = Graph::from_unsorted_csv(edges_reader, None, true, false, "Graph".to_owned())?;
    let _ = graph::test_utilities::default_test_suite(&mut g, false);
    Ok(())
}

#[test]
fn test_regression12() -> Result<(), String> {
    let edges_reader = EdgeFileReader::new("tests/data/regression/12.tsv".to_string())?
        .set_separator(Some(",".to_string()))?
        .set_verbose(Some(false))
        .set_sources_column_number(Some(0))?
        .set_destinations_column_number(Some(1))?
        .set_edge_types_column_number(Some(2))?
        .set_header(Some(false));
    let mut g = Graph::from_unsorted_csv(edges_reader, None, false, false, "Graph".to_owned())?;
    let _ = graph::test_utilities::default_test_suite(&mut g, false);
    Ok(())
}

#[test]
fn test_regression13() -> Result<(), String> {
    let edges_reader = EdgeFileReader::new("tests/data/regression/13.tsv".to_string())?
        .set_separator(Some(",".to_string()))?
        .set_verbose(Some(false))
        .set_sources_column_number(Some(0))?
        .set_destinations_column_number(Some(1))?
        .set_edge_types_column_number(Some(2))?
        .set_header(Some(false));
    let mut g = Graph::from_unsorted_csv(edges_reader, None, false, false, "Graph".to_owned())?;
    let _ = graph::test_utilities::default_test_suite(&mut g, false);
    Ok(())
}

#[test]
fn test_regression14() -> Result<(), String> {
    let edges_reader = EdgeFileReader::new("tests/data/regression/14.tsv".to_string())?
        .set_separator(Some(",".to_string()))?
        .set_verbose(Some(false))
        .set_sources_column_number(Some(0))?
        .set_destinations_column_number(Some(1))?
        .set_edge_types_column_number(Some(2))?
        .set_header(Some(false));
    let mut g = Graph::from_unsorted_csv(edges_reader, None, true, false, "Graph".to_owned())?;
    let _ = graph::test_utilities::default_test_suite(&mut g, false);
    Ok(())
}

#[test]
fn test_regression15() -> Result<(), String> {
    let edges_reader = EdgeFileReader::new("tests/data/regression/15.tsv".to_string())?
        .set_separator(Some(",".to_string()))?
        .set_verbose(Some(false))
        .set_sources_column_number(Some(0))?
        .set_destinations_column_number(Some(1))?
        .set_edge_types_column_number(Some(2))?
        .set_header(Some(false));
    let mut g = Graph::from_unsorted_csv(edges_reader, None, false, false, "Graph".to_owned())?;
    let _ = graph::test_utilities::default_test_suite(&mut g, false);
    Ok(())
}

#[test]
fn test_regression16() -> Result<(), String> {
    let edges_reader = EdgeFileReader::new("tests/data/regression/16.tsv".to_string())?
        .set_separator(Some(",".to_string()))?
        .set_verbose(Some(false))
        .set_sources_column_number(Some(0))?
        .set_destinations_column_number(Some(1))?
        .set_edge_types_column_number(Some(2))?
        .set_weights_column_number(Some(3))?
        .set_header(Some(false));
    let mut g = Graph::from_unsorted_csv(edges_reader, None, true, false, "Graph".to_owned())?;
    let _ = graph::test_utilities::default_test_suite(&mut g, false);
    Ok(())
}

#[test]
fn test_regression17() -> Result<(), String> {
    let edges_reader = EdgeFileReader::new("tests/data/regression/17.tsv".to_string())?
        .set_separator(Some(",".to_string()))?
        .set_verbose(Some(false))
        .set_sources_column_number(Some(0))?
        .set_destinations_column_number(Some(1))?
        .set_edge_types_column_number(Some(2))?
        .set_weights_column_number(Some(3))?
        .set_header(Some(false));
    let mut g = Graph::from_unsorted_csv(edges_reader, None, true, false, "Graph".to_owned())?;
    let _ = graph::test_utilities::default_test_suite(&mut g, false);
    Ok(())
}

#[test]
fn test_regression18() -> Result<(), String> {
    let edges_reader = EdgeFileReader::new("tests/data/regression/18.tsv".to_string())?
        .set_separator(Some(",".to_string()))?
        .set_verbose(Some(false))
        .set_sources_column_number(Some(0))?
        .set_destinations_column_number(Some(1))?
        .set_edge_types_column_number(Some(2))?
        .set_weights_column_number(Some(3))?
        .set_header(Some(false));
    let mut g = Graph::from_unsorted_csv(edges_reader, None, true, false, "Graph".to_owned())?;
    let _ = graph::test_utilities::default_test_suite(&mut g, false);
    Ok(())
}

#[test]
fn test_regression19() -> Result<(), String> {
    let edges_reader = EdgeFileReader::new("tests/data/regression/19.tsv".to_string())?
        .set_separator(Some(",".to_string()))?
        .set_verbose(Some(false))
        .set_sources_column(Some("subject".to_owned()))?
        .set_destinations_column(Some("object".to_owned()))?
        .set_edge_types_column(Some("edge_type".to_owned()))?
        .set_weights_column(Some("weight".to_owned()))?
        .set_header(Some(true));
    let mut g = Graph::from_unsorted_csv(edges_reader, None, true, false, "Graph".to_owned())?;
    let _ = graph::test_utilities::default_test_suite(&mut g, false);
    Ok(())
}

#[test]
fn test_regression20() -> Result<(), String> {
    let edges_reader = EdgeFileReader::new("tests/data/regression/20.tsv".to_string())?
        .set_separator(Some(",".to_string()))?
        .set_verbose(Some(false))
        .set_sources_column(Some("subject".to_owned()))?
        .set_destinations_column(Some("object".to_owned()))?
        .set_edge_types_column(Some("edge_type".to_owned()))?
        .set_header(Some(true));
    let mut g = Graph::from_unsorted_csv(edges_reader, None, true, false, "Graph".to_owned())?;
    let _ = graph::test_utilities::default_test_suite(&mut g, false);
    Ok(())
}

#[test]
fn test_regression21() -> Result<(), String> {
    let edges_reader = EdgeFileReader::new("tests/data/regression/21.tsv".to_string())?
        .set_separator(Some(",".to_string()))?
        .set_verbose(Some(false))
        .set_sources_column(Some("subject".to_owned()))?
        .set_destinations_column(Some("object".to_owned()))?
        .set_edge_types_column(Some("edge_type".to_owned()))?
        .set_weights_column(Some("weight".to_owned()))?
        .set_header(Some(true));
    let mut g = Graph::from_unsorted_csv(edges_reader, None, false, false, "Graph".to_owned())?;
    let _ = graph::test_utilities::default_test_suite(&mut g, false);
    Ok(())
}

#[test]
fn test_regression22() -> Result<(), String> {
    let edges_reader = EdgeFileReader::new("tests/data/regression/22.tsv".to_string())?
        .set_separator(Some(",".to_string()))?;
    let mut g = Graph::from_unsorted_csv(edges_reader, None, false, false, "Graph".to_owned())?;
    let _ = graph::test_utilities::default_test_suite(&mut g, false);
    Ok(())
}

/// This used to cause a subtract with overflow during the samplig of negativ edges,
/// the solution was to have two separate counters instead of a single one.
/// Of course this does not impact on the result, but it's cleaner and more informative.
#[test]
fn test_regression23() -> Result<(), String> {
    let edges_reader = EdgeFileReader::new("tests/data/regression/23.csv".to_string())?
        .set_separator(Some(",".to_string()))?;
    let mut g = Graph::from_unsorted_csv(edges_reader, None, false, false, "Graph".to_owned())?;
    let _ = graph::test_utilities::default_test_suite(&mut g, false);
    Ok(())
}

#[test]
fn test_regression24() -> Result<(), String> {
    let edges_reader = EdgeFileReader::new("tests/data/regression/24.csv".to_string())?
        .set_separator(Some(",".to_string()))?;
    assert!(
        Graph::from_unsorted_csv(edges_reader, None, false, false, "Graph".to_owned()).is_err()
    );
    Ok(())
}

#[test]
fn test_regression25() -> Result<(), String> {
    let edges_reader = EdgeFileReader::new("tests/data/regression/25.csv".to_string())?
        .set_separator(Some(",".to_string()))?;
    assert!(
        Graph::from_unsorted_csv(edges_reader, None, false, false, "Graph".to_owned()).is_err()
    );
    Ok(())
}

#[test]
fn test_regression26()->Result<(), String>{
    let edges_reader = EdgeFileReader::new("tests/data/regression/26.csv".to_string())?
        .set_separator(Some(",".to_string()))?;
    let mut g = Graph::from_unsorted_csv(edges_reader, None, false, false, "Graph".to_owned())?;
    let _ = graph::test_utilities::default_test_suite(&mut g, false);
    Ok(())
}

#[test]
fn test_regression27()->Result<(), String>{
    let edges_reader = EdgeFileReader::new("tests/data/regression/27.csv".to_string())?
        .set_separator(Some(",".to_string()))?;
    let mut g = Graph::from_unsorted_csv(edges_reader, None, false, false, "Graph".to_owned())?;
    let _ = graph::test_utilities::default_test_suite(&mut g, false);
    Ok(())
}

#[test]
fn test_regression28()->Result<(), String>{
    let edges_reader = EdgeFileReader::new("tests/data/regression/28.csv".to_string())?
        .set_separator(Some(",".to_string()))?;
    let mut g = Graph::from_unsorted_csv(edges_reader, None, false, false, "Graph".to_owned())?;
    let _ = graph::test_utilities::default_test_suite(&mut g, false);
    Ok(())
}
