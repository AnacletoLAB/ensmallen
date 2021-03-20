use super::*;
use arbitrary::Arbitrary;

#[derive(Arbitrary, Debug, Clone)]
pub struct FromVecHarnessParams {
    directed: bool,
    directed_edge_list: bool,
    ignore_duplicated_nodes: bool,
    ignore_duplicated_edges: bool,
    verbose: bool,
    numeric_edge_types_ids: bool,
    numeric_node_ids: bool,
    numeric_edge_node_ids: bool,
    numeric_node_types_ids: bool,
    has_node_types: bool,
    has_edge_types: bool,
    has_weights: bool,
    name: String,
    edges: Vec<Result<StringQuadruple, String>>,
    nodes: Option<Vec<Result<(String, Option<Vec<String>>), String>>>,
}

pub fn from_vec_harness(data: FromVecHarnessParams) -> Result<(), String> {

    let data_copy = data.clone();
    std::panic::set_hook(Box::new(move |info| {
        handle_panics_vec(info, data_copy.clone());
    }));

    let mut g = graph::Graph::from_string_unsorted(
        data.edges.into_iter(),
        match data.nodes {
            Some(ns) => Some(ns.into_iter()),
            None => None,
        },
        data.directed,
        data.directed_edge_list,
        data.name,
        data.ignore_duplicated_nodes,
        data.ignore_duplicated_edges,
        data.verbose,
        data.numeric_edge_types_ids,
        data.numeric_node_ids,
        data.numeric_edge_node_ids,
        data.numeric_node_types_ids,
        data.has_node_types,
        data.has_edge_types,
        data.has_weights,
    )?;
    // We ignore this error because we execute only the fuzzing to find
    // the panic situations that are NOT just errors, but unhandled errors.
    let _ = graph::test_utilities::default_test_suite(&mut g, false);

    Ok(())
}
