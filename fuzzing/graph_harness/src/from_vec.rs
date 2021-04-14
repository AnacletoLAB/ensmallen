use super::*;
use arbitrary::Arbitrary;

#[derive(Arbitrary, Debug, Clone)]
pub struct FromVecHarnessParams {
    pub directed: bool,
    pub directed_edge_list: bool,
    pub ignore_duplicated_nodes: bool,
    pub ignore_duplicated_edges: bool,
    pub verbose: bool,
    pub numeric_edge_types_ids: bool,
    pub numeric_node_ids: bool,
    pub numeric_edge_node_ids: bool,
    pub numeric_node_types_ids: bool,
    pub has_node_types: bool,
    pub has_edge_types: bool,
    pub has_weights: bool,
    pub edges: Vec<Result<StringQuadruple, String>>,
    pub nodes: Option<Vec<Result<(String, Option<Vec<String>>), String>>>,
}

pub fn from_vec_harness(data: FromVecHarnessParams) -> Result<(), String> {
    let data_copy = data.clone();
    let data_copy2 = data.clone();
    std::panic::set_hook(Box::new(move |info| {
        handle_panics_from_vec(info, data_copy.clone());
    }));

    let mut g = graph::Graph::from_string_unsorted(
        data.edges.into_iter(),
        data.nodes.map(|ns| ns.into_iter()),
        data.directed,
        data.directed_edge_list,
        "Fuzz Graph",
        data.ignore_duplicated_nodes,
        false,
        data.ignore_duplicated_edges,
        false,
        data.verbose,
        data.numeric_edge_types_ids,
        data.numeric_node_ids,
        data.numeric_edge_node_ids,
        data.numeric_node_types_ids,
        data.has_node_types,
        data.has_edge_types,
        data.has_weights,
        true,
        true,
        true,
    )?;

    let g_copy = g.clone();
    handle_panics_from_vec_once_loaded(None, data_copy2.clone(), g_copy.clone());
    std::panic::set_hook(Box::new(move |info| {
        handle_panics_from_vec_once_loaded(Some(info), data_copy2.clone(), g_copy.clone());
    }));

    // We ignore this error because we execute only the fuzzing to find
    // the panic situations that are NOT just errors, but unhandled errors.
    let _ = graph::test_utilities::default_test_suite(&mut g, false);

    Ok(())
}
