extern crate graph;

use indicatif::ParallelProgressIterator;
use rayon::iter::IntoParallelIterator;
use rayon::iter::ParallelIterator;

use graph::{EdgeFileReader, Graph};

//#[test]
/// This is a regression test that has been automatically generated
/// by the fuzzer harness.
/// The test originally caused a panic in the file stack_overflow.rs,
/// specifically (at the time) line 157 and column 13.
/// The provided message was: 'failed to set up alternative stack guard page: Cannot allocate memory (os error 12)'
///
fn test_rayon_matryoshka() -> Result<(), String> {
    let edges_reader = EdgeFileReader::new("tests/data/test_rayon_matriosca.edges")?
        .set_rows_to_skip(Some(0))
        .set_header(Some(false))
        .set_separator(Some(","))?
        .set_verbose(Some(false))
        .set_sources_column_number(Some(0))?
        .set_destinations_column_number(Some(1))?
        .set_ignore_duplicates(Some(true))
        .set_skip_self_loops(Some(false))
        .set_numeric_edge_type_ids(Some(false))
        .set_numeric_node_ids(Some(false))
        .set_skip_weights_if_unavailable(Some(false))
        .set_skip_edge_types_if_unavailable(Some(false))
        .set_edge_types_column_number(Some(2))?;

    let nodes_reader = None;

    let iterations = 1_000_000;
    let pb =
        graph::utils::get_loading_bar(true, "Running non-deterministic component test", iterations);

    let graph = Graph::from_unsorted_csv(
        edges_reader.clone(),
        nodes_reader.clone(),
        false,        // Directed
        false,        // Directed edge list
        "Fuzz Graph", // Name of the graph
    )?;

    (0..iterations)
        .into_par_iter()
        .map(|x| x)
        .progress_with(pb)
        .for_each(move |_| {
            let _ = graph.connected_components(false);
        });
    Ok(())
}
