extern crate graph;

use graph::Graph;

#[test]
fn test_regression_38() -> Result<(), String> {
    let mut graph = Graph::from_string_unsorted(
        Vec::new().into_iter(),
        Some(
            vec![Ok((
                "0".to_string(),
                None,
            ))]
            .into_iter(),
        ),
        false,        // Directed
        true,         // Directed edge list
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
    let _ = graph::test_utilities::default_test_suite(&mut graph, None);
    Ok(())
}
