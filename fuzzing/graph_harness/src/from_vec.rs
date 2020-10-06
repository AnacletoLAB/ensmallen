use super::*;
use arbitrary::Arbitrary;

#[derive(Arbitrary, Debug)]
pub struct FromVecHarnessParams {
    edges: Vec<Result<StringQuadruple, String>>,
    nodes: Option<Vec<Result<(String, Option<String>), String>>>,
    directed: bool,
    ignore_duplicated_nodes: bool,
    ignore_duplicated_edges: bool,
    numeric_edge_types_ids: bool,
    numeric_node_ids: bool,
    numeric_node_types_ids: bool
}

pub fn from_vec_harness(data: FromVecHarnessParams) -> Result<(), String> {
    let g = graph::Graph::from_string_unsorted(
        data.edges.iter().cloned(),
        match &data.nodes {
            Some(ns) => Some(ns.iter().cloned()),
            None => None,
        },
        data.directed,
        data.ignore_duplicated_nodes,
        data.ignore_duplicated_edges,
        false,
        data.numeric_edge_types_ids,
        data.numeric_node_ids,
        data.numeric_node_types_ids
    )?;
    // We ignore this error because we execute only the fuzzing to find
    // the panic situations that are NOT just errors, but unhandled errors.
    let _ = graph::test_utilities::default_test_suite(&g, false);

    Ok(())
}
