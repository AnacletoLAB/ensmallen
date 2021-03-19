extern crate graph;

use graph::{EdgeFileReader, Graph};
use indicatif::{ProgressBar, ProgressIterator, ProgressStyle};

#[test]
/// This is a regression test that has been automatically generated
/// by the fuzzer harness.
/// The test originally caused a panic in the file test_utilities.rs,
/// specifically (at the time) line 379 and column 13.
/// The probability  of this happening is less than 1 in 10_000.
fn test_regression_29() -> Result<(), String> {
    let edges_reader = EdgeFileReader::new("tests/data/regression/29.edges")?;

    let nodes_reader = None;

    let graph = Graph::from_unsorted_csv(
        edges_reader,
        nodes_reader,
        false, // Directed
        false, // Directed edge list
        "", // Name of the graph
    )?;

    let pb = ProgressBar::new(100);
    pb.set_style(ProgressStyle::default_bar().template(&format!(
        "{desc} {{spinner:.green}} [{{elapsed_precise}}] [{{bar:40.cyan/blue}}] ({{pos}}/{{len}}, ETA {{eta}})",
        desc="Executing multiple runs for non-deterministic test"
    )));

    for _ in (0..100).progress_with(pb){
        let _ = graph::test_utilities::test_spanning_arborescence_bader(&graph, false);
    }
    Ok(())
}
