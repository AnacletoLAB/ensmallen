#[macro_use]
extern crate honggfuzz;
extern crate graph_harness;
use graph_harness::*;

fn main() {
    loop {
        fuzz!(|data: FromVecHarnessParams| {
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
                graph.connected_components(false);
            }
        });
    }
}
