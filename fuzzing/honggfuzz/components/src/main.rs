#[macro_use]
extern crate honggfuzz;
extern crate graph_harness;
use graph_harness::*;
use arbitrary::Arbitrary;

#[derive(Arbitrary, Debug, Clone)]
struct Params {
    pub data: FromVecHarnessParams,
    pub verbose: bool,
}

fn main() {
    loop {
        fuzz!(|params: Params| {
            let data = params.data;
            let data_for_signal_handling = data.clone();
            let data_for_panic_handling = data.clone();
            // We ignore this error because we execute only the fuzzing to find
            // the panic situations that are NOT just errors, but unhandled errors.
            let maybe_graph = graph::Graph::from_string_unsorted(
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
                data.has_edge_weights,
                true,
                true,
                true,
            );

            if let Ok(graph) = maybe_graph {
                register_handler(libc::SIGABRT, abrt_handler, data_for_signal_handling);

                let graph_copy_for_panic_handling = graph.clone();
                std::panic::set_hook(Box::new(move |info| {
                    handle_panics_from_vec_once_loaded(Some(info), data_for_panic_handling.clone(), graph_copy_for_panic_handling.clone());
                }));
                
                let _ = graph.spanning_arborescence(params.verbose);
            }
        });
    }
}
