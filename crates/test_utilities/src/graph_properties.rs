use graph::Graph;
use shared::*;
use std::collections::HashSet;
use crate::utils::*;
use itertools::Itertools;
use log::warn;
use num_traits::Zero;

pub fn test_graph_properties(graph: &Graph, verbose: Option<bool>) -> Result<()> {
    // Testing that vocabularies are properly loaded
    validate_vocabularies(graph);

    // Collect set of connected nodes, INCLUDING singleton with selfloops.
    let not_singleton_nodes = graph
        .get_edge_node_ids(true)
        .into_iter()
        .flatten()
        .unique()
        .collect::<HashSet<NodeT>>();
    // Collect the set of singleton nodes, i.e. nodes not in the previous set.
    let singleton_nodes = graph
        .iter_node_ids()
        .filter(|node_id| !not_singleton_nodes.contains(node_id))
        .collect::<HashSet<NodeT>>();

    if graph.has_nodes() && !graph.has_edges() {
        assert!(
            graph.has_singleton_nodes(),
            concat!(
                "This graph has nodes (nodes number: {}) but ",
                "has no edges (edges number: {}), therefore it ",
                "should have singletons, but this does not seem ",
                "to be the case (singletons number {})."
            ),
            graph.get_nodes_number(),
            graph.get_edges_number(),
            graph.get_singleton_nodes_number()
        );
    }

    // Check properties relative to singletons.
    assert_eq!(
        !singleton_nodes.is_empty(),
        graph.has_singleton_nodes(),
        concat!(
            "If singleton nodes are detected, the has_singleton_nodes ",
            "method of the graph should return true.\n",
            "The singleton nodes detected are: {:?}.\n",
            "The has_singleton_nodes method returned: {:?}."
        ),
        singleton_nodes,
        graph.has_singleton_nodes()
    );
    assert_eq!(
        singleton_nodes.len(),
        graph.get_singleton_nodes_number() as usize
    );

    assert!(unsafe {
        singleton_nodes
            .iter()
            .all(|node_id| graph.is_unchecked_singleton_from_node_id(*node_id))
    });
    assert!(unsafe {
        singleton_nodes
            .iter()
            .all(|node_id| graph.get_unchecked_node_degree_from_node_id(*node_id) == 0)
    });

    // For now we limit this test to undirected graphs
    // to avoid the complexity of computing the indegree.
    if !graph.is_directed() {
        let singleton_nodes_with_selfloops = graph
            .iter_node_ids()
            .filter(|node_id| unsafe {
                graph.get_unchecked_node_degree_from_node_id(*node_id) > 0
                    && graph
                        .iter_unchecked_neighbour_node_ids_from_source_node_id(*node_id)
                        .all(|dst| dst == *node_id)
            })
            .collect::<HashSet<NodeT>>();

        assert_eq!(
            !singleton_nodes_with_selfloops.is_empty(),
            graph.has_singleton_nodes_with_selfloops(),
            concat!(
                "Singleton nodes with selfloops were found within ",
                "the provided, but the graph would not seem to ",
                "contain any.\n",
                "The graph edge list is:\n{:?}."
            ),
            graph.get_edge_node_ids(true)
        );
        assert_eq!(
            singleton_nodes_with_selfloops.len(),
            graph.get_singleton_nodes_with_selfloops_number() as usize
        );
        assert!(
            singleton_nodes_with_selfloops.iter().all(|node_id| unsafe {
                graph.is_unchecked_singleton_with_selfloops_from_node_id(*node_id)
            }),
            concat!(
                "The singleton with self-loops are defined as the set of nodes that ",
                "exclusively have self-loop edges.\n",
                "We have found the following list of singleton nodes with selfloops: {:?}\n"
            ),
            singleton_nodes_with_selfloops
        );
    }

    // If the graph is undirected, all the edges must have their symmetrical one
    if !graph.is_directed() {
        graph
            .iter_edge_node_ids(true)
            .for_each(|(_, src_node_id, dst_node_id)| {
                assert!(
                    graph.has_edge_from_node_ids(dst_node_id, src_node_id),
                    concat!(
                        "In an undirected graph, for every edge there must ",
                        "have its own symmetric edge.\n",
                        "In the provided graph instance, for the edge from ",
                        "the source node ID {} to the destination node ID {} ",
                        "the symmetric edge does not exist.\n",
                        "This error is likely caused by some mis-parametrization ",
                        "in a method that is expected to produce a simmetric graph.\n",
                        "The complete set of edges in the graph is:\n{:?}"
                    ),
                    src_node_id,
                    dst_node_id,
                    graph.get_edge_node_ids(true)
                );
            });
    }

    assert_eq!(
        graph.iter_node_degrees().is_sorted(),
        graph.has_nodes_sorted_by_increasing_outbound_node_degree(),
        concat!(
            "The cached value for the method ",
            "has_nodes_sorted_by_increasing_outbound_node_degree ",
            "does not match the computed method.\n",
            "The degrees of this graph are:\n {:?}\n",
            "The reported order was: {}"
        ),
        graph.get_node_degrees(),
        graph.has_nodes_sorted_by_increasing_outbound_node_degree(),
    );
    let mut degrees = graph.get_node_degrees();
    degrees.reverse();
    assert_eq!(
        degrees.is_sorted(),
        graph.has_nodes_sorted_by_decreasing_outbound_node_degree(),
        concat!(
            "The cached value for the method ",
            "has_nodes_sorted_by_decreasing_outbound_node_degree ",
            "does not match the computed method."
        )
    );

    if graph.has_nodes() && (graph.has_singleton_nodes() || graph.has_trap_nodes()) {
        assert!(
            graph.get_minimum_node_degree().unwrap() == 0,
            concat!(
                "When the graph either contains singleton nodes or trap nodes ",
                "we expect for the minimum node degree to be zero, but is {}."
            ),
            graph.get_minimum_node_degree().unwrap()
        );
        assert!(graph.iter_node_degrees().min().unwrap() == 0);
    }

    if let (Ok(min_degree), Ok(max_degree)) = (
        graph.get_minimum_node_degree(),
        graph.get_maximum_node_degree(),
    ) {
        assert_eq!(
            graph.has_nodes_sorted_by_decreasing_outbound_node_degree()
                && graph.has_nodes_sorted_by_increasing_outbound_node_degree(),
            min_degree == max_degree,
            concat!(
                "When the the nodes are sorted both by decreasing and increasing node degree ",
                "the minimum and maximum node degrees must be equal, and viceversa.\n",
                "The computed minimum node degree is {}.\n",
                "The computed maximum node degree is {}.\n",
                "The result of has_nodes_sorted_by_decreasing_outbound_node_degree is {}.\n",
                "The result of has_nodes_sorted_by_increasing_outbound_node_degree is {}.\n",
                "The node degrees are:\n{:?}."
            ),
            min_degree,
            max_degree,
            graph.has_nodes_sorted_by_decreasing_outbound_node_degree(),
            graph.has_nodes_sorted_by_increasing_outbound_node_degree(),
            graph.get_node_degrees()
        );
    }

    // Test that the weights do not contain zeros.
    if graph.has_edge_weights() {
        graph.iter_edge_weights().unwrap().for_each(|w| {
            assert!(!w.is_zero(), "The graph cannot contain a zero weight. ");
            assert!(
                !w.is_infinite(),
                "The graph cannot contain an infinite weight. "
            );
            assert!(!w.is_nan(), "The graph cannot contain a nan weight. ");
        });
        // If the graph is undirected, the edge weights must be symmetrical
        if !graph.is_directed() {
            graph
                .iter_edge_node_ids(false)
                .for_each(|(_, src_node_id, dst_node_id)| unsafe {
                    assert!(
                        (graph.get_unchecked_edge_weight_from_node_ids(src_node_id, dst_node_id)
                            - graph
                                .get_unchecked_edge_weight_from_node_ids(dst_node_id, src_node_id))
                        .abs()
                            < WeightT::EPSILON,
                        concat!(
                            "In an undirected graph, we expect for the edge weights to be symmetrical ",
                            "but in the provided graph there has been found a case where the edge ",
                            "from {} to {} has weight {}, while the edge from {} to {} has ",
                            "weight {}, creating an asymetrical case."
                        ),
                        src_node_id,
                        dst_node_id,
                        graph.get_unchecked_edge_weight_from_node_ids(src_node_id, dst_node_id),
                        dst_node_id,
                        src_node_id,
                        graph.get_unchecked_edge_weight_from_node_ids(dst_node_id, src_node_id),

                    );
                });
        }
    }

    // Testing that the degrees computation is correct
    assert_eq!(
        graph.get_maximum_node_degree()?,
        graph.iter_node_degrees().max().unwrap(),
        "The cached maximum degree does not match the one computed from the node degrees."
    );

    if !graph.is_directed() && !graph.has_singleton_nodes() {
        assert!(graph.get_minimum_node_degree()? > 0);
        assert!(graph.iter_node_degrees().min().unwrap() > 0);
    }

    if !graph.is_directed() && graph.get_minimum_node_degree()? == 0 {
        assert!(graph.has_singleton_nodes());
    }

    if !graph.has_disconnected_nodes() && !graph.has_trap_nodes() {
        assert!(graph.get_minimum_node_degree()? > 0);
        assert!(
            graph.iter_node_degrees().min().unwrap() > 0,
            concat!(
                "Since the graph does not contain disconnected nodes nor it ",
                "contains trap nodes, the minimum outbound node degree must be ",
                "greater than zero.\n\n",
                "The graph edges are: {:?}"
            ),
            graph.get_edge_node_ids(true)
        );
    }

    if graph.has_node_types() {
        assert!(graph.has_nodes());
        assert_eq!(
            graph.get_known_node_types_number().unwrap(),
            graph
                .iter_node_ids_and_node_type_ids()
                .map(|(_, node_type)| node_type.is_some() as NodeT)
                .sum::<NodeT>()
        );
        assert_eq!(
            graph.get_unknown_node_types_number().unwrap(),
            graph
                .iter_node_ids_and_node_type_ids()
                .map(|(_, node_type)| node_type.is_none() as NodeT)
                .sum::<NodeT>()
        );
        if graph.has_unknown_node_types().unwrap() {
            assert!(graph
                .iter_node_ids_and_node_type_ids()
                .any(|(_, node_type)| node_type.is_none()));
            assert!(graph.get_node_ids_with_unknown_node_types().unwrap().len() > 0);
        }
        if graph.has_known_node_types().unwrap() {
            assert!(graph
                .iter_node_ids_and_node_type_ids()
                .any(|(_, node_type)| node_type.is_some()));
            assert!(graph.get_node_ids_with_known_node_types().unwrap().len() > 0);
        }
    }

    if graph.has_edge_types() {
        assert!(graph.has_edges());
        assert_eq!(
            graph.get_known_edge_types_number().unwrap(),
            graph
                .iter_edge_node_ids_and_edge_type_id(true)
                .map(|(_, _, _, edge_type)| edge_type.is_some() as EdgeT)
                .sum::<EdgeT>()
        );
        assert_eq!(
            graph.get_unknown_edge_types_number().unwrap(),
            graph
                .iter_edge_node_ids_and_edge_type_id(true)
                .map(|(_, _, _, edge_type)| edge_type.is_none() as EdgeT)
                .sum::<EdgeT>()
        );
        if graph.has_unknown_edge_types().unwrap() {
            assert!(graph
                .iter_edge_node_ids_and_edge_type_id(true)
                .any(|(_, _, _, edge_type)| edge_type.is_none()));
            assert!(graph.get_edge_ids_with_unknown_edge_types().unwrap().len() > 0);
        }
        if graph.has_known_edge_types().unwrap() {
            assert!(
                graph
                    .iter_edge_node_ids_and_edge_type_id(true)
                    .any(|(_, _, _, edge_type)| edge_type.is_some()),
                concat!(
                    "We expected for the graph to contain at least one edge ",
                    "with a known edge type, but apparently it does not contain ",
                    "any. The graph contains {} edges and would have seemed to contain ",
                    "{} edges with known edge types."
                ),
                graph.get_directed_edges_number(),
                graph.get_known_edge_types_number().unwrap()
            );
            assert!(graph.get_edge_ids_with_known_edge_types().unwrap().len() > 0);
        }
    }

    assert_eq!(
        graph.get_minimum_node_degree()?,
        graph.iter_node_degrees().min().unwrap(),
        concat!(
            "The cached minimum degree does not match the one computed from the node degrees.\n",
            "The outbound node degrees are: {:?}"
        ),
        graph.get_node_degrees()
    );

    if graph.has_edge_weights() {
        assert!(
            ((graph.get_weighted_maximum_node_degree().clone())?
                - graph
                    .iter_weighted_node_degrees()?
                    .max_by(|a, b| a.partial_cmp(b).unwrap())
                    .unwrap())
            .abs()
                < f64::EPSILON,
            concat!(
                "The cached weighted maximum degree ({}) ",
                "does not match the one computed from the node degrees ({}), ",
                "where the node degrees list is {:?}.\n",
                "Additionally the number of weighted singleton nodes is {:?}."
            ),
            (graph.get_weighted_maximum_node_degree().clone())?,
            graph
                .iter_weighted_node_degrees()?
                .max_by(|a, b| a.partial_cmp(b).unwrap())
                .unwrap(),
            graph.get_weighted_node_degrees(),
            graph.get_weighted_singleton_nodes_number()
        );
        assert!(
            ((graph.get_weighted_minimum_node_degree().clone())?
                - graph
                    .iter_weighted_node_degrees()?
                    .min_by(|a, b| a.partial_cmp(b).unwrap())
                    .unwrap())
            .abs()
                < f64::EPSILON,
            "The cached weighted minimum degree ({:?}) does not match the one computed from the node degrees ({:?}).",
            (graph.get_weighted_minimum_node_degree().clone())?,
            graph
                    .iter_weighted_node_degrees()?
                    .min_by(|a, b| a.partial_cmp(b).unwrap())
                    .unwrap()
        );
    }

    for singleton_node_id in graph.iter_singleton_node_ids() {
        assert!(unsafe { graph.get_unchecked_node_degree_from_node_id(singleton_node_id) } == 0);
        assert!(unsafe { graph.is_unchecked_singleton_from_node_id(singleton_node_id) });
    }

    if !graph.is_directed() {
        for node_id in graph.iter_node_ids() {
            unsafe {
                assert_eq!(
                    graph.is_unchecked_singleton_from_node_id(node_id),
                    graph.get_unchecked_node_degree_from_node_id(node_id) == 0
                )
            };
        }
    }

    // Test get_edge_id_from_node_names_and_edge_type_name()
    assert!(
        graph
            .get_edge_id_from_node_names_and_edge_type_name(NONEXISTENT, NONEXISTENT, None)
            .is_err(),
        "Graph contains non-existing edge."
    );

    // Test has_node_from_name
    assert!(
        !(graph.has_node_name_and_node_type_name(NONEXISTENT, None)),
        "The graph seems to have a non-existing node."
    );
    assert!(
        !(graph.has_node_name(NONEXISTENT)),
        "The graph seems to have a non-existing node."
    );

    // Singletons with selfloops can only exist if the graph has nodes AND selfloops
    if graph.has_singleton_nodes() {
        assert!(graph.has_nodes());
    }
    if graph.has_singleton_nodes_with_selfloops() {
        assert!(graph.has_nodes());
        assert!(graph.has_selfloops());
    }

    // Test translate_edge|node_types()
    assert!(
        graph
            .get_edge_type_ids_from_edge_type_names(vec![Some(NONEXISTENT.to_string())])
            .is_err(),
        "The graph seems to have a non-existing edge type."
    );

    assert!(
        graph
            .get_node_type_ids_from_node_type_names(vec![Some(NONEXISTENT.to_string())])
            .is_err(),
        "The graph seems to have a non-existing node type."
    );

    assert_eq!(
        graph.get_connected_nodes_number()
            + graph.get_singleton_nodes_number()
            + graph.get_singleton_nodes_with_selfloops_number(),
        graph.get_nodes_number(),
        "Sum of singleton and non singleton nodes number does not match."
    );

    assert_eq!(
        graph.get_disconnected_nodes_number(), graph.get_singleton_nodes_number() + graph.get_singleton_nodes_with_selfloops_number(),
        "Sum of singleton and singleton with selfloops does not match the number of disconnected nodes."
    );

    warn!("Running connected components tests.");
    let (_components_number, smallest, biggest) = graph.get_connected_components_number(None);
    assert!(
        biggest >= smallest,
        "smallest: {} biggest: {}",
        smallest,
        biggest
    );

    if smallest == 1 {
        assert!(
            graph.has_singleton_nodes() || graph.has_singleton_nodes_with_selfloops(),
            "When the smallest component is one the graph must have singletons! Graph report: \n{:?}",
            graph.textual_report()
        );
    }

    assert_eq!(
        !graph.has_nodes(),
        smallest == 0,
        "When the smallest component is zero the graph must be empty! Graph report: \n{:?}",
        graph.textual_report()
    );

    // Get one edge from the graph if there are any presents
    if let Some(edge) = graph.iter_unique_edge_node_ids(true).next() {
        let src_string = unsafe { graph.get_unchecked_node_name_from_node_id(edge.0) };
        let dst_string = unsafe { graph.get_unchecked_node_name_from_node_id(edge.1) };
        let edge_id = graph.get_edge_id_from_node_names(&src_string, &dst_string)?;
        if graph.has_edge_types() {
            let edge_type = graph.get_edge_type_name_from_edge_id(edge_id)?;
            let clone_edge_type = edge_type.clone();
            assert!(
                graph.has_edge_from_node_names_and_edge_type_name(&src_string, &dst_string, clone_edge_type.as_deref()),
                "I was expecting for the edge ({}, {}, {:?}) to exist, but it seems to not exist in graph {:?}",
                src_string,
                dst_string,
                edge_type,
                graph.textual_report()
            );
        } else {
            assert!(
                graph.has_edge_from_node_names(&src_string, &dst_string),
                "I was expecting for the edge ({}, {}) without type to exist, but it seems to not exist in graph {:?}",
                src_string,
                dst_string,
                graph.textual_report()
            );
        }
        assert!(graph.has_node_name(&src_string) && graph.has_node_name(&dst_string));
        if graph.has_node_types() {
            assert!(
                graph.has_node_name_and_node_type_name(
                    &src_string,
                    graph.get_node_type_names_from_node_name(&src_string)?
                ) && graph.has_node_name_and_node_type_name(
                    &dst_string,
                    graph.get_node_type_names_from_node_name(&dst_string)?
                ),
                concat!(
                    "The nodes {:?} and {:?} with node types are not present in the graph.\n",
                    "The node types are {:?} and {:?}.\n",
                    "The first node existance is {}\n",
                    "The second node existance is {}\n",
                    "The graph report is {:?}"
                ),
                src_string,
                dst_string,
                graph.get_node_type_names_from_node_name(&src_string),
                graph.get_node_type_names_from_node_name(&dst_string),
                graph.has_node_name_and_node_type_name(
                    &src_string,
                    graph.get_node_type_names_from_node_name(&src_string)?
                ),
                graph.has_node_name_and_node_type_name(
                    &dst_string,
                    graph.get_node_type_names_from_node_name(&dst_string)?
                ),
                graph.textual_report()
            );
        }
        assert_eq!(
            graph.get_edge_id_from_node_names(&src_string, &dst_string)?,
            graph.get_edge_id_from_node_ids(edge.0, edge.1).unwrap(),
            "Check of given edge ID does not match."
        );
    }

    // Test the generation of the textual report, this includes the connected components algorithm.
    graph.report();
    graph.textual_report();
    graph.overlap_textual_report(&graph, verbose)?;

    // Compute degrees metrics
    for src in 0..5 {
        for dst in 0..5 {
            let _ = graph.get_preferential_attachment_from_node_ids(src, dst, true);
            let _ = graph.get_preferential_attachment_from_node_ids(src, dst, false);
            let _ = graph.get_jaccard_coefficient_from_node_ids(src, dst);
            let _ = graph.get_adamic_adar_index_from_node_ids(src, dst);
            let _ = graph.get_resource_allocation_index_from_node_ids(src, dst);
            if graph.has_edge_weights() {
                let _ = graph.get_weighted_preferential_attachment_from_node_ids(src, dst, true);
                let _ = graph.get_weighted_preferential_attachment_from_node_ids(src, dst, false);
                let _ = graph.get_weighted_resource_allocation_index_from_node_ids(src, dst);
            }
        }
    }

    assert_eq!(
        graph.has_node_types(),
        graph.get_node_type_id_from_node_id(0).is_ok()
    );

    assert!(
        graph.get_node_type_id_from_node_id(graph.get_nodes_number() + 1).is_err(),
        "Given graph does not raise an exception when a node's node type greater than the number of available nodes is requested."
    );

    assert_eq!(
        graph.has_edge_types(),
        graph.get_edge_type_id_from_edge_id(0).is_ok()
    );

    assert!(
        graph.get_edge_type_id_from_edge_id(graph.get_directed_edges_number() + 1).is_err(),
        "Given graph does not raise an exception when a edge's edge type greater than the number of available edges is requested."
    );

    // Evaluate get_node_type
    assert_eq!(
        graph.get_node_type_id_from_node_id(0).is_ok(),
        graph.has_node_types()
    );

    // Evaluate get_edge_type
    assert_eq!(
        graph.get_edge_type_id_from_edge_id(0).is_ok(),
        graph.has_edge_types()
    );

    // Evaluate get_edge_type_counts
    assert_eq!(
        graph.get_edge_type_id_counts_hashmap().is_ok(),
        graph.has_edge_types()
    );

    // Evaluate get_edge_type_id_counts_hashmap
    assert_eq!(
        graph.get_edge_type_id_counts_hashmap().is_ok(),
        graph.has_edge_types()
    );

    graph.strongly_connected_components();

    // Checking that the connected components are a dense range.
    let (_, connected_components, total_connected_components, _, _) =
        graph.random_spanning_arborescence_kruskal(Some(42), None, verbose);
    let actual_components_number = connected_components.iter().unique().count() as NodeT;

    assert_eq!(
        actual_components_number, total_connected_components,
        concat!(
            "The measured number of connected components ({}) ",
            "does not match the computed number of connected components ({}).\n",
            "That is, the components are not a dense set.\n",
            "This is likely caused by a problem with the remapping of the ",
            "components."
        ),
        actual_components_number, total_connected_components,
    );

    let max_component_id = connected_components.iter().max();
    if let Some(mci) = max_component_id {
        assert_eq!(
            *mci as usize,
            total_connected_components as usize - 1,
            "We expected the connected components to be a dense set.\n The obtained components are: \n{:?}\n The graph report is:\n{:?}",
            connected_components,
            graph.textual_report()
        );
    }
    if !graph.is_directed() {
        // Checking that the connected components are a dense range.
        let (connected_components, total_connected_components, _, _) =
            graph.connected_components(verbose).unwrap();
        let actual_components_number = connected_components.iter().unique().count() as NodeT;
        assert_eq!(
            actual_components_number,
            total_connected_components,
            "The measured number of connected components ({}) does not match the computed number of connected components ({}).",
            actual_components_number,
            total_connected_components
        );
        let max_component_id = connected_components.iter().max();
        if let Some(mci) = max_component_id {
            assert_eq!(
                *mci as usize,
                total_connected_components as usize - 1,
                "We expected the connected components to be a dense set.\n The obtained components are: \n{:?}\n The graph report is:\n{:?}",
                connected_components,
                graph.textual_report()
            );
        }
    }

    let _total_memory_used = graph.memory_stats().total();

    Ok(())
}