//! Test functions used both for testing and fuzzing.

use super::*;
use itertools::Itertools;
use log::warn;
use rand::Rng;
use rayon::iter::ParallelIterator;
use std::collections::HashSet;
use std::fs;
use std::path::Path;

const NONEXISTENT: &str = "Cthulhu is a fictional cosmic entity created by writer H. P. Lovecraft and first introduced in the short story The Call of Cthulhu,[2] published in the American pulp magazine Weird Tales in 1928. Considered a Great Old One within the pantheon of Lovecraftian cosmic entities, the creature has since been featured in numerous popular culture references. Lovecraft depicts it as a gigantic entity worshipped by cultists, in shape like an octopus, a dragon, and a caricature of human form. Its name was given to the Lovecraft-inspired universe where it and its fellow entities existed, the Cthulhu Mythos.";

// where to save the test files
#[cfg(target_os = "macos")]
static DEFAULT_PATH: &str = "/tmp/";
#[cfg(target_os = "linux")]
static DEFAULT_PATH: &str = "/tmp/";
#[cfg(target_os = "windows")]
static DEFAULT_PATH: &str = "";

const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";

/// Computes a random string of the chosen length
pub fn random_string(len: usize) -> String {
    let mut rng = rand::thread_rng();

    (0..len)
        .map(|_| {
            let idx = rng.gen_range(0, CHARSET.len());
            CHARSET[idx] as char
        })
        .collect()
}

/// Computes a random path.
pub fn random_path(path: Option<&str>) -> String {
    Path::new(path.unwrap_or(DEFAULT_PATH))
        .join(random_string(64))
        .to_str()
        .unwrap()
        .to_string()
}

#[allow(clippy::redundant_clone)]
/// Load the Strings Protein Protein Interaction graph with given parametrization.
/// This is our default graph we use on tests.
pub fn load_ppi(
    load_nodes: bool,
    load_edge_types: bool,
    load_weights: bool,
    directed: bool,
    verbose: bool,
    skip_selfloops: bool,
) -> Graph {
    let graph_name = "STRING PPI".to_owned();
    let nodes_reader = if load_nodes {
        Some(
            NodeFileReader::new("tests/data/ppi/nodes.tsv".to_string())
                .unwrap()
                .set_verbose(Some(false))
                .set_node_types_column_number(Some(1))
                .set_nodes_column_number(Some(0))
                .set_node_types_column(Some("category".to_string()))
                .unwrap()
                .set_default_node_type(Some("default".to_string()))
                .set_nodes_column(Some("id".to_string()))
                .unwrap()
                .set_ignore_duplicates(Some(true))
                .set_separator(Some("\t"))
                .unwrap()
                .set_header(Some(true))
                .set_max_rows_number(Some(100000))
                .set_rows_to_skip(Some(0))
                .clone(),
        )
    } else {
        None
    };
    let edges_reader = EdgeFileReader::new("tests/data/ppi/edges.tsv".to_string())
        .unwrap()
        .set_verbose(Some(verbose))
        .set_ignore_duplicates(Some(true))
        .set_separator(Some("\t"))
        .unwrap()
        .set_header(Some(true))
        .set_rows_to_skip(Some(0))
        .set_sources_column(Some("subject".to_string()))
        .unwrap()
        .set_destinations_column(Some("object".to_string()))
        .unwrap()
        .set_weights_column(if load_weights {
            Some("weight".to_string())
        } else {
            None
        })
        .unwrap()
        .set_edge_types_column(if load_edge_types {
            Some("edge_label".to_string())
        } else {
            None
        })
        .unwrap()
        .set_default_edge_type(if load_edge_types {
            Some("Kebab".to_string())
        } else {
            None
        })
        .set_max_rows_number(Some(100000))
        .set_default_weight(if load_weights { Some(5.0) } else { None })
        .set_skip_selfloops(Some(skip_selfloops))
        .clone();

    let ppi = Graph::from_unsorted_csv(
        edges_reader,
        nodes_reader,
        directed,
        false,
        graph_name.clone(),
    )
    .unwrap();
    assert_eq!(ppi.has_node_types(), load_nodes);
    assert_eq!(ppi.has_edge_types(), load_edge_types,);
    assert_eq!(ppi.has_edge_weights(), load_weights);
    assert_eq!(
        ppi.has_selfloops(),
        !skip_selfloops,
        concat!(
            "I was expecting the graph self-loops status to be {} ",
            "since we have given parameter skip_selfloops equal to {}, ",
            "but actually is {}.\n",
            "The graph report is: \n {:?}"
        ),
        !skip_selfloops,
        skip_selfloops,
        ppi.has_selfloops(),
        ppi.textual_report(false)
    );
    ppi
}

/// Load an empty graph instance
pub fn load_empty_graph(directed: bool) -> Graph {
    Graph::build_graph(
        std::iter::empty(),
        0,
        Vocabulary::default(),
        None,
        None,
        directed,
        false,
        "Empty graph",
        false,
        false,
        false,
        false,
        false,
        false,
    )
    .unwrap()
}

#[allow(clippy::redundant_clone)]
/// This is our default graph we use on tests with node types.
pub fn load_cora() -> Result<Graph, String> {
    let graph_name = "Cora".to_owned();
    let edges_reader = EdgeFileReader::new("tests/data/cora/edges.tsv")?
        .set_separator(Some("\t"))?
        .set_verbose(Some(false))
        .set_sources_column(Some("subject"))?
        .set_destinations_column(Some("object"))?
        .set_edge_types_column(Some("edge_type"))?;
    let nodes_reader = Some(
        NodeFileReader::new("tests/data/cora/nodes.tsv")?
            .set_separator(Some("\t"))?
            .set_nodes_column(Some("id"))?
            .set_verbose(Some(false))
            .set_node_types_column(Some("node_type"))?,
    );
    let cora =
        Graph::from_unsorted_csv(edges_reader, nodes_reader, false, false, graph_name.clone())?;
    Ok(cora)
}

/// Return WalksParameters to execute a first order walk.
pub fn first_order_walker(graph: &Graph) -> Result<WalksParameters, String> {
    Ok(WalksParameters::new(8)?
        .set_iterations(Some(1))?
        .set_random_state(Some(43))
        .set_dense_node_mapping(Some(graph.get_dense_nodes_mapping())))
}

/// Return WalksParameters to execute a second order walk.
pub fn second_order_walker(
    graph: &Graph,
    return_weight: WeightT,
    explore_weight: WeightT,
) -> Result<WalksParameters, String> {
    Ok(WalksParameters::new(8)?
        .set_iterations(Some(1))?
        .set_return_weight(Some(return_weight))?
        .set_explore_weight(Some(explore_weight))?
        .set_max_neighbours(Some(3))?
        .set_change_edge_type_weight(Some(2.0))?
        .set_change_node_type_weight(Some(2.0))?
        .set_dense_node_mapping(Some(graph.get_dense_nodes_mapping()))
        .set_random_state(Some(43)))
}

fn validate_vocabularies(graph: &Graph) {
    if let Some(ets) = &graph.edge_types {
        assert_eq!(!ets.ids.is_empty(), graph.has_edge_types());
    }

    if let Some(nts) = &graph.node_types {
        assert_eq!(!nts.ids.is_empty(), graph.has_node_types());
    }

    if let Some(ws) = &graph.weights {
        assert_eq!(
            !ws.is_empty(), graph.has_edge_weights(),
            concat!(
                "We expect the edge weights vector to NOT be empty if the graph says it has weights.\n",
                "The graph report is:\n{:?}"
            ),
            graph.textual_report(false)
        );
    }
}

/// Executes the default test suite for holdouts.
pub fn default_holdout_test_suite(
    graph: &Graph,
    train: &Graph,
    test: &Graph,
) -> Result<(), String> {
    for g in &[graph, train, test] {
        validate_vocabularies(g);
    }
    assert!(
        !train.overlaps(&test)?,
        "Training graph overlaps with test graph!"
    );
    assert!(
        !test.overlaps(&train)?,
        "Test graph overlaps with training graph!"
    );
    assert!(graph.contains(&train)?, "Graph does not training graph.");
    assert!(graph.contains(&test)?, "Graph does not contain test graph.");
    let summed = (train | test)?;
    validate_vocabularies(&summed);
    assert!(
        summed.contains(&graph)?,
        "Composed train and test graph do not contained original graph."
    );
    let subtracted = (graph - test)?;
    validate_vocabularies(&subtracted);
    assert!(
        subtracted.contains(&train)?,
        "Main graph subtracted test does not contain training graph."
    );
    assert!(
        !subtracted.overlaps(&test)?,
        "Main graph subtracted train does not contain test graph."
    );
    let xorred = (graph ^ test)?;
    validate_vocabularies(&xorred);
    assert!(
        xorred.contains(&train)?,
        "Main graph xorred test does not contain training graph."
    );
    assert!(
        !xorred.overlaps(&test)?,
        "Main graph xorred train does not contain testing graph."
    );
    let anded = (graph & test)?;
    validate_vocabularies(&anded);
    assert!(
        anded.contains(&test)?,
        "Main graph anded test does not contain training graph."
    );
    Ok(())
}

/// Test that the spanning arborescence algorithm from bader is working correctly.
pub fn test_spanning_arborescence_bader(graph: &Graph, verbose: bool) {
    let kruskal_tree = graph.spanning_arborescence_kruskal(verbose).0;
    let random_kruskal_tree = graph
        .random_spanning_arborescence_kruskal(42, &None, verbose)
        .0;
    if !graph.directed {
        let spanning_arborescence_bader: Vec<(NodeT, NodeT)> =
            graph.spanning_arborescence(verbose).unwrap().1.collect();
        assert_eq!(
            spanning_arborescence_bader.len(), kruskal_tree.len(),
            "The number of extracted edges forming the spanning arborescence computed by the bader's algorithm does not match the one computed by kruskal. The graph report is:\n{:?}\nThe bader's tree is:\n{:?}\nThe kruskal's tree is:\n{:?}",
            graph.textual_report(false), spanning_arborescence_bader, kruskal_tree,
        );
    } else {
        assert!(graph.spanning_arborescence(verbose).is_err());
    }
    assert_eq!(random_kruskal_tree.len() as usize, kruskal_tree.len());
}

pub fn test_graph_properties(graph: &mut Graph, verbose: bool) -> Result<(), String> {
    // Testing that vocabularies are properly loaded
    validate_vocabularies(graph);

    // Test get_edge_id_from_node_names_and_edge_type_name()
    assert!(
        graph
            .get_edge_id_from_node_names_and_edge_type_name(NONEXISTENT, NONEXISTENT, None)
            .is_err(),
        "Graph contains non-existing edge."
    );

    // Test has_node_from_name
    assert!(
        !(graph.has_node_from_node_name_and_node_type_name(NONEXISTENT, None)),
        "The graph seems to have a non-existing node."
    );
    assert!(
        !(graph.has_node_from_node_name(NONEXISTENT)),
        "The graph seems to have a non-existing node."
    );

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
        graph.get_not_singleton_nodes_number() + graph.get_singleton_nodes_number(),
        graph.get_nodes_number(),
        "Sum of singleton and non singleton nodes number does not match."
    );

    warn!("Running connected components tests.");
    let (_components_number, smallest, biggest) = graph.get_connected_components_number(false);
    assert!(
        biggest >= smallest,
        "smallest: {} biggest: {}",
        smallest,
        biggest
    );

    if smallest == 1 {
        assert!(
            graph.has_singletons() || graph.has_singletons_with_selfloops(),
            "When the smallest component is one the graph must have singletons! Graph report: \n{:?}",
            graph.textual_report(false)
        );
    }

    if smallest == 0 {
        assert!(
            !graph.has_nodes(),
            "When the smallest component is zero the graph must be empty! Graph report: \n{:?}",
            graph.textual_report(false)
        );
    }

    // Get one edge from the graph if there are any presents
    if let Some(edge) = graph.iter_unique_edge_node_ids(true).next() {
        let src_string = graph.get_unchecked_node_name_from_node_id(edge.0);
        let dst_string = graph.get_unchecked_node_name_from_node_id(edge.1);
        let edge_id = graph.get_edge_id_from_node_names(&src_string, &dst_string)?;
        if graph.has_edge_types() {
            let edge_type = graph.get_edge_type_name_from_edge_id(edge_id)?;
            assert!(
                graph.has_edge_from_node_names_and_edge_type_name(&src_string, &dst_string, edge_type.as_ref()),
                "I was expecting for the edge ({}, {}, {:?}) to exist, but it seems to not exist in graph {:?}",
                src_string,
                dst_string,
                edge_type,
                graph.textual_report(false)
            );
        } else {
            assert!(
                graph.has_edge_from_node_names(&src_string, &dst_string),
                "I was expecting for the edge ({}, {}) without type to exist, but it seems to not exist in graph {:?}",
                src_string,
                dst_string,
                graph.textual_report(false)
            );
        }
        assert!(
            graph.has_node_from_node_name(&src_string)
                && graph.has_node_from_node_name(&dst_string)
        );
        if graph.has_node_types() {
            assert!(
                graph.has_node_from_node_name_and_node_type_name(
                    &src_string,
                    graph.get_node_type_name_from_node_name(&src_string)?
                ) && graph.has_node_from_node_name_and_node_type_name(
                    &dst_string,
                    graph.get_node_type_name_from_node_name(&dst_string)?
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
                graph.get_node_type_name_from_node_name(&src_string),
                graph.get_node_type_name_from_node_name(&dst_string),
                graph.has_node_from_node_name_and_node_type_name(
                    &src_string,
                    graph.get_node_type_name_from_node_name(&src_string)?
                ),
                graph.has_node_from_node_name_and_node_type_name(
                    &dst_string,
                    graph.get_node_type_name_from_node_name(&dst_string)?
                ),
                graph.textual_report(false)
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
    graph.textual_report(verbose)?;

    // Compute degrees metrics
    for src in 0..5 {
        for dst in 0..5 {
            let _ = graph.degrees_product(src, dst);
            let _ = graph.jaccard_index(src, dst);
            let _ = graph.adamic_adar_index(src, dst);
            let _ = graph.resource_allocation_index(src, dst);
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

    // Evaluate get_node_type_counts
    assert_eq!(
        graph.get_node_type_counter().is_ok(),
        graph.has_node_types()
    );

    // Evaluate get_edge_type_counts
    assert_eq!(
        graph.get_edge_type_counter().is_ok(),
        graph.has_edge_types()
    );

    // Evaluate get_edge_type_counts_hashmap
    assert_eq!(
        graph.get_edge_type_counts_hashmap().is_ok(),
        graph.has_edge_types()
    );

    graph.set_name(graph.get_name());
    graph.strongly_connected_components();

    // Checking that the connected components are a dense range.
    let (_, connected_components, total_connected_components, _, _) =
        graph.random_spanning_arborescence_kruskal(42, &None, verbose);
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
            graph.textual_report(false)
        );
    }
    if !graph.is_directed() {
        // Checking that the connected components are a dense range.
        let (connected_components, total_connected_components, _, _) =
            graph.connected_components(verbose)?;
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
                graph.textual_report(false)
            );
        }
    }
    Ok(())
}

pub fn test_random_walks(graph: &mut Graph, _verbose: bool) -> Result<(), String> {
    // Testing principal random walk algorithms
    let walker = first_order_walker(&graph)?;
    assert_eq!(walker.clone(), walker);
    let walker2 = second_order_walker(&graph, 2.0, 2.0)?;
    assert_eq!(walker2.clone(), walker2);

    if !graph.directed {
        warn!("Executing random walks tests.");
        for mode in 0..3 {
            if mode == 1 {
                graph.enable(false, true, true, None)?;
                if let Some(cumulative_node_degrees) = &graph.cumulative_node_degrees {
                    assert_eq!(
                        cumulative_node_degrees.len(),
                        graph.get_nodes_number() as usize,
                        "Length of cumulative_node_degrees does not match number of nodes in the graph."
                    );
                }
                if let Some(destinations) = &graph.destinations {
                    assert_eq!(
                        destinations.len(),
                        graph.get_directed_edges_number() as usize,
                        "Length of destinations does not match number of edges in the graph."
                    );
                }
            }
            if mode == 2 {
                graph.enable(false, false, false, Some(0.05))?;
                assert!(
                    graph.cached_destinations.is_some(),
                    "Cached destinations are not None when cache is enabled."
                );
            }
            assert_eq!(
                graph
                    .iter_random_walks(1, &walker)
                    .map(|iter| iter.collect::<Vec<Vec<NodeT>>>()),
                graph
                    .iter_random_walks(1, &walker)
                    .map(|iter| iter.collect::<Vec<Vec<NodeT>>>()),
                "Walks of first order are not reproducible!"
            );

            assert_eq!(
                graph
                    .iter_random_walks(1, &second_order_walker(&graph, 2.0, 2.0)?)
                    .map(|iter| iter.collect::<Vec<Vec<NodeT>>>()),
                graph
                    .iter_random_walks(1, &second_order_walker(&graph, 2.0, 2.0)?)
                    .map(|iter| iter.collect::<Vec<Vec<NodeT>>>()),
                "Walks of second order are not reproducible!"
            );

            assert_eq!(
                graph
                    .iter_complete_walks(&walker)
                    .map(|iter| iter.collect::<Vec<Vec<NodeT>>>()),
                graph
                    .iter_complete_walks(&walker)
                    .map(|iter| iter.collect::<Vec<Vec<NodeT>>>()),
                "Complete first order walks are not reproducible!"
            );

            assert_eq!(
                graph
                    .iter_complete_walks(&second_order_walker(&graph, 2.0, 2.0)?)
                    .map(|iter| iter.collect::<Vec<Vec<NodeT>>>()),
                graph
                    .iter_complete_walks(&second_order_walker(&graph, 2.0, 2.0)?)
                    .map(|iter| iter.collect::<Vec<Vec<NodeT>>>()),
                "Complete second order walks are not reproducible!"
            );

            assert_eq!(
                graph
                    .iter_complete_walks(&second_order_walker(&graph, 2.0, 1.0)?)
                    .map(|iter| iter.collect::<Vec<Vec<NodeT>>>()),
                graph
                    .iter_complete_walks(&second_order_walker(&graph, 2.0, 1.0)?)
                    .map(|iter| iter.collect::<Vec<Vec<NodeT>>>()),
                "Complete second order walks are not reproducible!"
            );

            assert_eq!(
                graph
                    .iter_complete_walks(&second_order_walker(&graph, 1.0, 2.0)?)
                    .map(|iter| iter.collect::<Vec<Vec<NodeT>>>()),
                graph
                    .iter_complete_walks(&second_order_walker(&graph, 1.0, 2.0)?)
                    .map(|iter| iter.collect::<Vec<Vec<NodeT>>>()),
                "Complete second order walks are not reproducible!"
            );
        }
    } else {
        assert!(graph.iter_complete_walks(&walker).is_err());
    }
    Ok(())
}

pub fn test_edge_holdouts(graph: &mut Graph, verbose: bool) -> Result<(), String> {
    for include_all_edge_types in &[false, true] {
        let (train, test) =
            graph.random_holdout(4, 0.6, *include_all_edge_types, None, None, verbose)?;
        default_holdout_test_suite(graph, &train, &test)?;
        let (train, test) =
            graph.connected_holdout(4, 0.8, None, *include_all_edge_types, verbose)?;
        let (total, min_comp, max_comp) = graph.get_connected_components_number(verbose);
        assert_eq!(
            graph.get_connected_components_number(verbose),
            train.get_connected_components_number(verbose),
            "The number of components of the original graph and the connected training set does not match. Particularly, the number of nodes in the graph is {nodes_number}.",
            nodes_number=graph.get_nodes_number().to_string()
        );
        if total == 1 {
            assert_eq!(
                min_comp,
                graph.get_nodes_number(),
                concat!(
                    "We expect for the minimum size of connected components ",
                    "in a graph with a single connected component to ",
                    "match the number of nodes of the graph, but we got ",
                    "the minimum component with size {} and the number ",
                    "of nodes in the graph equal to {}.\n",
                    "The graph report is: \n {:?}",
                ),
                min_comp,
                graph.get_nodes_number(),
                graph.textual_report(false)
            );
            assert_eq!(max_comp, graph.get_nodes_number());
            assert_eq!(min_comp, test.get_nodes_number());
            assert_eq!(max_comp, test.get_nodes_number());
        }
        if total == 2 {
            assert_eq!(
                max_comp + min_comp, graph.get_nodes_number(),
                "We expected that the number of the minimum component ({}) plus the maximum component ({}), when the components are two, made up the graph nodes ({}).\nThe graph report is:\n {:?}",
                min_comp, max_comp, graph.get_nodes_number(),
                graph.textual_report(false)
            );
            assert_eq!(max_comp + min_comp, test.get_nodes_number());
        }
        default_holdout_test_suite(graph, &train, &test)?;
    }
    Ok(())
}

pub fn test_remove_components(graph: &mut Graph, verbose: bool) -> Result<(), String> {
    if graph.get_connected_components_number(verbose).0 > 1 {
        let without_selfloops = graph.remove(
            None, None, None, None, None, None, None, None, false, false, false, false, true,
            verbose,
        )?;

        assert_eq!(
            graph.get_connected_components_number(verbose),
            without_selfloops.get_connected_components_number(verbose),
            concat!(
                "We expected the graph to have the same components once we remove the selfloops.\n",
                "The report of the original graph is {:?}\n",
                "The report of the filtered graph is {:?}\n",
            ),
            graph.textual_report(false),
            without_selfloops.textual_report(false),
        );

        let single_component = graph.remove_components(None, None, None, None, Some(1), verbose);
        assert!(
            single_component.is_ok(),
            concat!(
                "Removing all the components except the first one returned an error.\n",
                "The error is:\n{:?}\nand the graph report is:\n{:?}"
            ),
            single_component,
            graph.textual_report(false)
        );
        let single_component_number = single_component
            .unwrap()
            .get_connected_components_number(verbose)
            .0;
        assert_eq!(
            single_component_number,
            1,
            concat!(
                "Removing all the components except the first one returned a graph ",
                "with {} components, which is not one.\nThe report of the graph is:{:?}\n"
            ),
            single_component_number,
            graph.textual_report(false)
        );

        let test = graph.remove_components(
            Some(vec![graph.nodes.unchecked_translate(0)]),
            None,
            None,
            None,
            None,
            verbose,
        )?;
        let no_selfloops = test.remove(
            None, None, None, None, None, None, None, None, false, false, false, false, true,
            verbose,
        )?;
        assert_eq!(
            no_selfloops.get_connected_components_number(verbose).0,
            1,
            concat!(
                "Expected number of components (1) is not matched!\n",
                "The report of the original graph is {:?}\n",
                "The report of the graph with only one component is {:?}\n",
                "The report of the graph without selfloops is {:?}\n",
            ),
            graph.textual_report(false),
            test.textual_report(false),
            no_selfloops.textual_report(false)
        );
        if let Ok(node_type_name) = graph.get_node_type_name_from_node_type_id(0) {
            assert!(graph
                .remove_components(
                    None,
                    Some(vec![Some(node_type_name)]),
                    None,
                    None,
                    None,
                    verbose
                )
                .is_ok());
        }
        if graph.has_unknown_node_types() {
            let without_unknowns =
                graph.remove_components(None, Some(vec![None]), None, None, None, verbose);
            assert!(
                without_unknowns.is_ok(),
                "Could not remove components without node type None.\nThe error is {:?}\nThe graph report is {:?}",
                without_unknowns, graph.textual_report(false)
            );
        }
        if let Ok(edge_type_name) = graph.get_edge_type_name_from_edge_type_id(0) {
            assert!(graph
                .remove_components(
                    None,
                    None,
                    Some(vec![Some(edge_type_name)]),
                    None,
                    None,
                    verbose
                )
                .is_ok());
        }
        if graph.has_unknown_edge_types() {
            assert!(graph
                .remove_components(None, None, Some(vec![None]), None, None, verbose)
                .is_ok());
        }
    } else {
        assert!(
            graph
                .remove_components(None, None, None, None, None, verbose)
                .is_ok(),
            "We expect it to be possible, now, to create empty graphs."
        );
    }

    Ok(())
}

pub fn test_kfold(graph: &mut Graph, _verbose: bool) -> Result<(), String> {
    let k = 3;
    for i in 0..k {
        let (train, test) = graph.kfold(k, i, None, 42, false)?;
        assert!(
            test.get_edges_number() <= (graph.get_edges_number() / k) + 1,
            concat!(
                "Check that test kfolds respect size bound has failed!\n",
                "The value of k is {}.\n",
                "The report of the original graph is:\n{:?}\n",
                "The report of the train graph is:\n{:?}\n",
                "The report of the test graph is:\n{:?}\n",
                "We expect that the test graph has at most {} edges but it has {}.\n",
                "The holdout index is {}.\n",
            ),
            k,
            graph.textual_report(false),
            train.textual_report(false),
            test.textual_report(false),
            (graph.get_edges_number() / k) + 1,
            test.get_edges_number(),
            i
        );
        default_holdout_test_suite(graph, &train, &test)?;
    }

    if let Ok(edge_t) = graph.get_edge_type_name_from_edge_type_id(0) {
        for i in 0..k {
            let (train, test) = graph.kfold(k, i, Some(vec![Some(edge_t.clone())]), 1337, false)?;
            default_holdout_test_suite(graph, &train, &test)?;
        }
    }

    Ok(())
}

pub fn test_negative_edges_generation(graph: &mut Graph, verbose: bool) -> Result<(), String> {
    for only_from_same_component in &[true, false] {
        let negatives = graph.sample_negatives(
            4,
            graph.get_edges_number(),
            None,
            *only_from_same_component,
            verbose,
        )?;
        assert_eq!(
            graph.get_edges_number(),
            negatives.get_edges_number(),
            "We expect the graph and its negative graph to have the same number of edges but we got {} and {}.",
            graph.get_edges_number(),
            negatives.get_edges_number()
        );
        validate_vocabularies(&negatives);
        if !graph.has_edge_types() {
            assert!(!graph.overlaps(&negatives)?);
            assert!(!negatives.overlaps(&graph)?);
        }
        // Testing holdouts executed on negative edges.
        let (neg_train, neg_test) =
            negatives.random_holdout(32, 0.8, false, None, None, verbose)?;

        neg_test.get_trap_nodes_number();

        default_holdout_test_suite(&negatives, &neg_train, &neg_test)?;
    }

    Ok(())
}

pub fn test_subgraph_generation(graph: &mut Graph, verbose: bool) -> Result<(), String> {
    let expected_nodes = graph.get_not_singleton_nodes_number() / 10;
    let subgraph = graph.random_subgraph(6, expected_nodes, verbose)?;
    assert!(subgraph.overlaps(&graph)?);
    assert!(subgraph.get_not_singleton_nodes_number() <= expected_nodes + 1);
    Ok(())
}

pub fn test_dump_graph(graph: &mut Graph, verbose: bool) -> Result<(), String> {
    let node_file = random_path(None);
    let nodes_writer = NodeFileWriter::new(node_file.clone())
        .set_verbose(Some(verbose))
        .set_separator(Some("\t"))
        .set_header(Some(true))
        .set_node_types_column_number(Some(4))
        .set_nodes_column_number(Some(6))
        .set_node_types_column(Some("node_types"))
        .set_nodes_column(Some("node_column".to_string()));
    nodes_writer.dump(&graph)?;
    fs::remove_file(node_file).unwrap();

    let edges_file = random_path(None);
    let edges_writer = EdgeFileWriter::new(edges_file.clone())
        .set_verbose(Some(verbose))
        .set_separator(Some("\t"))
        .set_header(Some(true))
        .set_edge_types_column(Some("edge_types"))
        .set_destinations_column_number(Some(3))
        .set_weights_column(Some("weight".to_string()))
        .set_weights_column_number(Some(2))
        .set_sources_column(Some("The land of sushi".to_string()))
        .set_sources_column_number(Some(0))
        .set_destinations_column(Some("The land of pizza".to_string()))
        .set_destinations_column_number(Some(1));

    edges_writer.dump(&graph)?;
    fs::remove_file(edges_file).unwrap();

    Ok(())
}

pub fn test_embiggen_preprocessing(graph: &mut Graph, verbose: bool) -> Result<(), String> {
    let walker = first_order_walker(&graph)?;
    if !graph.directed {
        let (terms_number, iterator) = graph.cooccurence_matrix(&walker, 3, verbose)?;
        assert_eq!(terms_number, iterator.count());

        let window_size = 3;
        let batch_size = 256;
        let data = graph
            .node2vec(&walker, batch_size, window_size)?
            .collect::<Vec<_>>();
        assert_eq!(
            data.len(),
            batch_size as usize
                * walker.iterations as usize
                * (walker.single_walk_parameters.walk_length as usize - window_size * 2)
        );
        for (context, _) in data.iter() {
            assert_eq!(context.len(), window_size * 2);
        }
    }
    if graph.has_edges() {
        graph
            .link_prediction_degrees(0, 256, true, 10.0, false, 10, &None)
            .unwrap()
            .collect::<Vec<_>>();
        graph
            .link_prediction_ids(0, 256, 10.0, false, 10, &None)
            .unwrap()
            .collect::<Vec<_>>();
    }

    Ok(())
}

pub fn test_graph_filter(graph: &mut Graph, verbose: bool) -> Result<(), String> {
    assert!(graph
        .filter(
            Some(graph.get_node_names()),
            graph
                .get_node_type_names()
                .ok()
                .map(|ntn| ntn.into_iter().map(Option::Some).collect()),
            graph
                .get_edge_type_names()
                .map(|etn| etn.into_iter().map(Option::Some).collect()),
            Some(1000.0),
            Some(10.0),
            verbose,
        )
        .is_err());
    let _ = graph.filter(
        Some(graph.get_node_names()),
        graph
            .get_node_type_names()
            .ok()
            .map(|ntn| ntn.into_iter().map(Option::Some).collect()),
        graph
            .get_edge_type_names()
            .map(|etn| etn.into_iter().map(Option::Some).collect()),
        graph.get_min_edge_weight().ok(),
        graph.get_max_edge_weight().ok(),
        verbose,
    );
    Ok(())
}

pub fn test_edgelist_generation(graph: &mut Graph, _verbose: bool) -> Result<(), String> {
    let _clique = graph.get_clique_edge_names(
        None,
        None,
        Some(false),
        None,
        // limit to compute the clique for at most the first 3 nodes
        // because it's really expensive computationally.
        Some(
            graph
                .get_node_names()
                .iter()
                .take(3)
                .cloned()
                .collect::<HashSet<String>>(),
        ),
    );
    warn!("Running edge lists generator tests.");
    if graph.get_nodes_number() > 1 {
        let _bipartite = graph.get_bipartite_edge_names(
            None,
            Some(
                [graph.get_unchecked_node_name_from_node_id(0)]
                    .iter()
                    .cloned()
                    .collect::<HashSet<String>>(),
            ),
            Some(
                [graph.get_unchecked_node_name_from_node_id(1)]
                    .iter()
                    .cloned()
                    .collect::<HashSet<String>>(),
            ),
            None,
            None,
        )?;
        let _star = graph.get_star_edges(
            graph.get_unchecked_node_name_from_node_id(0),
            Some(false),
            Some(
                [graph.get_unchecked_node_name_from_node_id(1)]
                    .iter()
                    .cloned()
                    .collect::<HashSet<String>>(),
            ),
            None,
        )?;
        let _star = graph.get_star_edge_names(
            graph.get_unchecked_node_name_from_node_id(0),
            Some(false),
            Some(
                [graph.get_unchecked_node_name_from_node_id(1)]
                    .iter()
                    .cloned()
                    .collect::<HashSet<String>>(),
            ),
            None,
        )?;
    }
    Ok(())
}

pub fn test_nodelabel_holdouts(graph: &mut Graph, _verbose: bool) -> Result<(), String> {
    for use_stratification in [true, false].iter() {
        if *use_stratification
            && (graph.has_multilabel_node_types() || graph.get_minimum_node_types_number() < 2)
            || graph.get_nodes_number() - graph.get_unknown_node_types_number() < 2
            || !graph.has_node_types()
        {
            assert!(graph
                .node_label_holdout(0.8, *use_stratification, 42)
                .is_err());
            continue;
        }

        let (train, test) = graph.node_label_holdout(0.8, *use_stratification, 42)?;
        let remerged = &mut (&train | &test)?;
        assert_eq!(remerged.node_types, graph.node_types);
        assert!(
            remerged.contains(graph)?,
            "The re-merged holdouts does not contain the original graph."
        );
        assert!(
            graph.contains(remerged)?,
            "The re-merged holdouts does not contain the original graph."
        );
        assert!(
            train.node_types.as_ref().map_or(false, |train_nts| {
                test.node_types.as_ref().map_or(false, |test_nts| {
                    train_nts.ids.iter().zip(test_nts.ids.iter()).all(
                        |(train_node_type, test_node_type)| {
                            !(train_node_type.is_some() && test_node_type.is_some())
                        },
                    )
                })
            }),
            "The train and test node-label graphs are overlapping!"
        );
    }
    Ok(())
}

pub fn test_edgelabel_holdouts(graph: &mut Graph, _verbose: bool) -> Result<(), String> {
    for use_stratification in [true, false].iter() {
        if *use_stratification && graph.get_minimum_edge_types_number() < 2
            || graph.get_directed_edges_number() - graph.get_unknown_edge_types_number() < 2
            || !graph.has_edge_types()
        {
            assert!(graph
                .edge_label_holdout(0.8, *use_stratification, 42)
                .is_err());
            continue;
        }
        let (train, test) = graph.edge_label_holdout(0.8, *use_stratification, 42)?;
        assert!(
            train.edge_types.as_ref().map_or(false, |train_nts| {
                test.edge_types.as_ref().map_or(false, |test_nts| {
                    train_nts.ids.iter().zip(test_nts.ids.iter()).all(
                        |(train_edge_type, test_edge_type)| {
                            !(train_edge_type.is_some() && test_edge_type.is_some())
                        },
                    )
                })
            }),
            "The train and test edge-label graphs are overlapping!"
        );
    }
    Ok(())
}

pub fn test_graph_removes(graph: &mut Graph, verbose: bool) -> Result<(), String> {
    {
        let without_edge_types = graph.remove(
            None, None, None, None, None, None, None, None, false, false, true, false, false,
            verbose,
        );
        if let Some(we) = &without_edge_types.ok() {
            validate_vocabularies(we);
            assert_eq!(we.has_edge_types(), false);
            assert_eq!(we.has_edge_weights(), graph.has_edge_weights());
            assert_eq!(we.node_types, graph.node_types);
            assert_eq!(
                we.get_unique_edges_number(),
                graph.get_unique_edges_number(),
                concat!(
                    "Number of unique edges does not match in graph without edge types.\n",
                    "The report of the original graph is \n{:?}\n",
                    "The report of the graph without edge types is \n{:?}",
                ),
                graph.textual_report(false),
                we.textual_report(false),
            );
            assert_eq!(
                we.get_unique_selfloop_number(),
                graph.get_unique_selfloop_number(),
                "Number of unique self loops does not match in graph without edge types."
            );
            assert_eq!(we.nodes, graph.nodes);
        }
    }
    {
        let without_node_types = graph.remove(
            None, None, None, None, None, None, None, None, false, true, false, false, false,
            verbose,
        );
        if let Some(wn) = &without_node_types.ok() {
            validate_vocabularies(wn);
            assert_eq!(wn.has_node_types(), false);
            assert_eq!(
                wn.weights,
                graph.weights,
                concat!(
                    "We expected the weights not to change when removig node types.",
                    "\nThe report of the original graph is {:?}.",
                    "\nThe report of the filtered graph is {:?}."
                ),
                graph.textual_report(false),
                wn.textual_report(false)
            );
            assert_eq!(wn.has_selfloops(), graph.has_selfloops());
            assert_eq!(wn.nodes, graph.nodes);
        }
    }
    {
        let without_weights = graph.remove(
            None, None, None, None, None, None, None, None, true, false, false, false, false,
            verbose,
        );
        if let Some(ww) = &without_weights.ok() {
            validate_vocabularies(ww);
            assert_eq!(ww.has_edge_weights(), false);
            assert_eq!(ww.node_types, graph.node_types);
            assert_eq!(ww.has_selfloops(), graph.has_selfloops());
            assert_eq!(ww.nodes, graph.nodes);
        }
    }

    Ok(())
}

pub fn test_clone_and_setters(graph: &mut Graph, _verbose: bool) -> Result<(), String> {
    let mut clone = graph.clone();
    clone = clone.set_all_edge_types("TEST_SET_ALL_EDGE_TYPES")?;
    clone = clone.set_all_node_types("TEST_SET_ALL_NODE_TYPES")?;

    assert_eq!(
        clone.get_edge_types_number(),
        1,
        "Number of edge types of the graph is not 1."
    );
    assert_eq!(
        clone.get_unchecked_edge_count_from_edge_type_id(Some(0)),
        graph.get_directed_edges_number(),
        "Number of edges with the unique edge type does not match number of edges in the graph."
    );

    assert_eq!(
        clone.get_node_types_number(),
        1,
        "Number of node types of the graph is not 1."
    );
    assert_eq!(
        clone.get_unchecked_node_count_from_node_type_id(Some(0)),
        graph.get_nodes_number(),
        "Number of nodes with the unique node type does not match number of nodes in the graph."
    );

    Ok(())
}

pub fn test_graph_remapping(graph: &mut Graph, verbose: bool) -> Result<(), String> {
    assert!(
        graph.are_nodes_remappable(&graph),
        "Graph always should be remappable to itself."
    );
    assert!(
        graph.remap(&graph, verbose).is_ok(),
        "Graph always should be remappable to itself."
    );
    Ok(())
}

/// Executes near-complete test of all functions for the given graph.
fn _default_test_suite(graph: &mut Graph, verbose: bool) -> Result<(), String> {
    warn!("Starting default test suite.");
    let _ = test_graph_properties(graph, verbose);

    warn!("Testing SkipGram / CBOW / GloVe preprocessing.");
    let _ = test_embiggen_preprocessing(graph, verbose);

    warn!("Testing subgraph generation.");
    let _ = test_subgraph_generation(graph, verbose);

    warn!("Testing clone and setters.");
    let _ = test_clone_and_setters(graph, verbose);

    warn!("Testing edge-label holdouts tests.");
    let _ = test_edgelabel_holdouts(graph, verbose);

    warn!("Testing writing out graph to file.");
    //let _ = test_dump_graph(graph, verbose);

    warn!("Testing generic filtering mechanism.");
    let _ = test_graph_filter(graph, verbose);

    warn!("Testing the spanning arborescences.");
    let _ = test_spanning_arborescence_bader(graph, verbose);

    warn!("Running node-label holdouts tests.");
    let _ = test_nodelabel_holdouts(graph, verbose);

    warn!("Running remove components tests.");
    let _ = test_remove_components(graph, verbose);

    warn!("Testing removes.");
    let _ = test_graph_removes(graph, verbose);

    warn!("Testing negative edges generation.");
    let _ = test_negative_edges_generation(graph, verbose);

    warn!("Executing edge holdouts tests.");
    let _ = test_edge_holdouts(graph, verbose);

    warn!("Testing k-fold holdouts.");
    let _ = test_kfold(graph, verbose);

    warn!("Testing edge lists generation.");
    let _ = test_edgelist_generation(graph, verbose);

    warn!("Testing graph remapping.");
    let _ = test_graph_remapping(graph, verbose);

    warn!("Testing random walks.");
    let _ = test_random_walks(graph, verbose);

    Ok(())
}

/// Executes near-complete test of all functions for the given graph.
pub fn default_test_suite(graph: &mut Graph, verbose: bool) -> Result<(), String> {
    warn!("Starting default test suite.");
    let _ = _default_test_suite(graph, verbose);
    warn!("Starting default test suite with speedups enabled.");
    graph.enable(true, true, true, None)?;
    let _ = _default_test_suite(graph, verbose);
    Ok(())
}
