use super::*;
use arbitrary::Arbitrary;

#[derive(Arbitrary, Debug, Clone)]
pub struct FuzzFeaturesHarnessParams {
    from_vec: FromVecHarnessParams,
    features: Vec<Vec<f64>>,
}

pub fn features_harness(data: FuzzFeaturesHarnessParams) -> Result<(), String> {
    // let data_for_panic_handling1 = data.clone();
    // let data_for_panic_handling2 = data.clone();
    // let data_for_signal_handling = data.clone();
    // std::panic::set_hook(Box::new(move |info| {
    //     handle_panics_from_vec(Some(info), data_for_panic_handling1.clone(), None);
    // }));


    // register_handler(libc::SIGABRT, abrt_handler, data_for_signal_handling);

    let mut graph = graph::Graph::from_string_unsorted(
        data.from_vec.edges.into_iter(),
        data.from_vec.nodes.map(|ns| ns.into_iter()),
        data.from_vec.directed,
        data.from_vec.directed_edge_list,
        "Fuzz Graph",
        data.from_vec.ignore_duplicated_nodes,
        false,
        data.from_vec.ignore_duplicated_edges,
        false,
        data.from_vec.numeric_edge_types_ids,
        data.from_vec.numeric_node_ids,
        data.from_vec.numeric_edge_node_ids,
        data.from_vec.numeric_node_types_ids,
        data.from_vec.has_node_types,
        data.from_vec.has_edge_types,
        data.from_vec.has_edge_weights,
        true,
        true,
        true,
        data.from_vec.verbose,
    )?;

    // let graph_copy_for_panic_handling = graph.clone();
    // std::panic::set_hook(Box::new(move |info| {
    //     handle_panics_from_vec_once_loaded(
    //         Some(info),
    //         data_for_panic_handling2.clone(),
    //         graph_copy_for_panic_handling.clone()
    //     );
    // }));

    // thicc-en the graph
    let mut graph = graph.generate_new_edges_from_node_features(
        data.features,
        None,
        None,
        Some(false),
    )?;

    println!("{:#4?}", graph);

    // We ignore this error because we execute only the fuzzing to find
    // the panic situations that are NOT just errors, but unhandled errors.
    let _ = graph::test_utilities::default_test_suite(&mut graph, false);
    
    Ok(())
}
