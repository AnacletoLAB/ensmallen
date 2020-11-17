use super::*;
use rand::Rng;
use rayon::iter::ParallelIterator;
use std::collections::HashSet;
use std::fs;
use std::path::Path;

// where to save the test files
#[cfg(target_os = "macos")]
static DEFAULT_PATH: &str = "/tmp/";
#[cfg(target_os = "linux")]
static DEFAULT_PATH: &str = "/tmp/";
#[cfg(target_os = "windows")]
static DEFAULT_PATH: &str = "";

const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";

/// Computes a random string,
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
pub fn random_path() -> String {
    Path::new(DEFAULT_PATH)
        .join(random_string(64))
        .to_str()
        .unwrap()
        .to_string()
}

#[allow(clippy::redundant_clone)]
/// Load PPI with given parametrization.
pub fn load_ppi(
    load_nodes: bool,
    load_edge_types: bool,
    load_weights: bool,
    directed: bool,
    verbose: bool,
    skip_self_loops: bool,
) -> Result<Graph, String> {
    let nodes_reader = if load_nodes {
        Some(
            NodeFileReader::new("tests/data/ppi/nodes.tsv".to_string())?
                .set_verbose(Some(false))
                .set_node_types_column_number(Some(1))?
                .set_nodes_column_number(Some(0))?
                .set_node_types_column(Some("category".to_string()))?
                .set_default_node_type(Some("default".to_string()))
                .set_nodes_column(Some("id".to_string()))?
                .set_ignore_duplicates(Some(true))
                .set_separator(Some("\t".to_string()))
                .unwrap()
                .set_header(Some(true))
                .set_max_rows_number(Some(100000))
                .set_rows_to_skip(Some(0))
                .clone(),
        )
    } else {
        None
    };
    let edges_reader = EdgeFileReader::new("tests/data/ppi/edges.tsv".to_string())?
        .set_verbose(Some(verbose))
        .set_ignore_duplicates(Some(true))
        .set_separator(Some("\t".to_string()))
        .unwrap()
        .set_header(Some(true))
        .set_rows_to_skip(Some(0))
        .set_sources_column(Some("subject".to_string()))?
        .set_destinations_column(Some("object".to_string()))?
        .set_weights_column(if load_weights {
            Some("weight".to_string())
        } else {
            None
        })?
        .set_edge_types_column(if load_edge_types {
            Some("edge_label".to_string())
        } else {
            None
        })?
        .set_default_edge_type(Some("Kebab".to_string()))
        .set_max_rows_number(Some(100000))
        .set_default_weight(Some(5.0))
        .set_skip_self_loops(Some(skip_self_loops))
        .clone();

    Graph::from_unsorted_csv(
        edges_reader,
        nodes_reader,
        directed,
        false,
        "Graph".to_owned(),
    )
}

/// Return WalksParameters to execute a first order walk.
pub fn first_order_walker(graph: &Graph) -> Result<WalksParameters, String> {
    Ok(WalksParameters::new(10)?
        .set_iterations(Some(1))?
        .set_random_state(Some(43))
        .set_dense_node_mapping(Some(graph.get_dense_node_mapping())))
}

/// Return WalksParameters to execute a second order walk.
pub fn second_order_walker(graph: &Graph) -> Result<WalksParameters, String> {
    Ok(WalksParameters::new(10)?
        .set_iterations(Some(1))?
        .set_return_weight(Some(2.0))?
        .set_explore_weight(Some(2.0))?
        .set_max_neighbours(Some(20))?
        .set_change_edge_type_weight(Some(2.0))?
        .set_change_node_type_weight(Some(2.0))?
        .set_dense_node_mapping(Some(graph.get_dense_node_mapping()))
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
        assert_eq!(!ws.is_empty(), graph.has_weights());
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

/// Executes near-complete test of all functions for the given graph.
pub fn default_test_suite(graph: &mut Graph, verbose: bool) -> Result<(), String> {
    // Testing that vocabularies are properly loaded
    validate_vocabularies(graph);
    // Testing principal random walk algorithms
    let walker = first_order_walker(&graph)?;
    if !graph.directed {
        for mode in 0..3 {
            if mode == 1 {
                graph.enable(false, true, true, None)?;
                if let Some(outbounds) = &graph.outbounds {
                    assert_eq!(
                        outbounds.len(),
                        graph.get_nodes_number() as usize,
                        "Length of outbounds does not match number of nodes in the graph."
                    );
                }
                if let Some(destinations) = &graph.destinations {
                    assert_eq!(
                        destinations.len(),
                        graph.get_edges_number() as usize,
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
                    .random_walks_iter(1, &walker)
                    .map(|iter| iter.collect::<Vec<Vec<NodeT>>>()),
                graph
                    .random_walks_iter(1, &walker)
                    .map(|iter| iter.collect::<Vec<Vec<NodeT>>>()),
                "Walks of first order are not reproducible!"
            );

            assert_eq!(
                graph
                    .random_walks_iter(1, &second_order_walker(&graph)?)
                    .map(|iter| iter.collect::<Vec<Vec<NodeT>>>()),
                graph
                    .random_walks_iter(1, &second_order_walker(&graph)?)
                    .map(|iter| iter.collect::<Vec<Vec<NodeT>>>()),
                "Walks of second order are not reproducible!"
            );

            assert_eq!(
                graph
                    .complete_walks_iter(&walker)
                    .map(|iter| iter.collect::<Vec<Vec<NodeT>>>()),
                graph
                    .complete_walks_iter(&walker)
                    .map(|iter| iter.collect::<Vec<Vec<NodeT>>>()),
                "Complete first order walks are not reproducible!"
            );

            assert_eq!(
                graph
                    .complete_walks_iter(&second_order_walker(&graph)?)
                    .map(|iter| iter.collect::<Vec<Vec<NodeT>>>()),
                graph
                    .complete_walks_iter(&second_order_walker(&graph)?)
                    .map(|iter| iter.collect::<Vec<Vec<NodeT>>>()),
                "Complete second order walks are not reproducible!"
            );
        }
    }

    // Test get_edge_id_string()
    assert_eq!(
        graph.get_edge_id_string("NONEXISTENT", "NONEXISTENT", None),
        None,
        "Graph contains non-existing edge."
    );

    assert!(
        graph.get_singleton_nodes_with_self_loops_number() <= graph.get_singleton_nodes_number(),
        "Graph singleton nodes with selfloops is bigger than number of singleton nodes."
    );

    assert_eq!(
        graph.get_not_singleton_nodes_number() + graph.get_singleton_nodes_number(),
        graph.get_nodes_number(),
        "Sum of singleton and non singleton nodes number does not match."
    );

    if !graph.directed {
        let has_singletons = graph.get_node_degrees().iter().any(|degree| *degree == 0);
        assert_eq!(has_singletons, graph.has_singletons());
    }

    if let Some(edge) = graph.get_unique_edges_iter(true).next() {
        let src_string = graph.get_node_name(edge.0).unwrap();
        let dst_string = graph.get_node_name(edge.1).unwrap();
        assert!(graph.has_edge_string(&src_string, &dst_string, None));
        assert!(
            graph.has_node_string(&src_string, None) && graph.has_node_string(&dst_string, None)
        );
        assert_eq!(
            graph.get_edge_id_string(
                &src_string,
                &dst_string,
                Some(&"NONEXISTENT_EDGE_TYPE".to_string())
            ),
            None
        );
        if !graph.has_edge_types() {
            assert_eq!(
                graph.get_edge_id_string(&src_string, &dst_string, None),
                graph.get_edge_id(edge.0, edge.1, None),
                "Check of given edge ID does not match."
            );
        }
    }
    // Test has_node_string
    assert!(
        !(graph.has_node_string("NONEXISTENT", None)),
        "The graph seems to have a non-existing node."
    );

    // Test translate_edge|node_types()
    assert!(
        graph
            .translate_edge_types(vec!["NONEXISTENT_EDGE_TYPE".to_string()])
            .is_err(),
        "The graph seems to have a non-existing edge type."
    );
    assert!(
        graph
            .translate_node_types(vec!["NONEXISTENT_NODE_TYPE".to_string()])
            .is_err(),
        "The graph seems to have a non-existing node type."
    );

    // Testing main holdout mechanisms
    for include_all_edge_types in &[false, true] {
        let (train, test) =
            graph.random_holdout(4, 0.6, *include_all_edge_types, None, None, verbose)?;
        default_holdout_test_suite(graph, &train, &test)?;
        let (train, test) =
            graph.connected_holdout(4, 0.8, None, *include_all_edge_types, verbose)?;
        let kruskal_tree = graph.spanning_arborescence_kruskal(verbose).0;
        let random_kruskal_tree = graph
            .random_spanning_arborescence_kruskal(42, &None, verbose)
            .0;
        if !graph.directed {
            let spanning_arborescence_bader = graph
                .spanning_arborescence(verbose)
                .unwrap()
                .1
                .collect::<Vec<(NodeT, NodeT)>>();
            assert_eq!(
                spanning_arborescence_bader.len() as usize,
                kruskal_tree.len()
            );
        }
        assert_eq!(random_kruskal_tree.len() as usize, kruskal_tree.len());
        let (total, min_comp, max_comp) = graph.connected_components_number(verbose);
        assert_eq!(
            graph.connected_components_number(verbose),
            train.connected_components_number(verbose),
            "The number of components of the original graph and the connected training set does not match."
        );
        if total == 1 {
            assert_eq!(min_comp, graph.get_nodes_number());
            assert_eq!(max_comp, graph.get_nodes_number());
            assert_eq!(min_comp, test.get_nodes_number());
            assert_eq!(max_comp, test.get_nodes_number());
        }
        if total == 2 {
            assert_eq!(max_comp + min_comp, graph.get_nodes_number());
            assert_eq!(max_comp + min_comp, test.get_nodes_number());
        }
        default_holdout_test_suite(graph, &train, &test)?;
    }

    // test remove components
    if graph.connected_components_number(verbose).0 > 1 {
        let test = graph.remove_components(
            Some(vec![graph.nodes.translate(0).to_string()]),
            None,
            None,
            None,
            None,
            verbose,
        )?;
        assert_eq!(
            test.remove(
                None, None, None, None, None, None, None, None, false, false, false, true, verbose
            )?
            .connected_components_number(verbose)
            .0,
            1,
            "Expected number of components (1) is not matched!"
        );

        if let Some(nts) = &graph.node_types {
            let test = graph.remove_components(
                None,
                Some(vec![nts.translate(0).to_string()]),
                None,
                None,
                None,
                verbose,
            )?;
            assert_eq!(
                test.remove(
                    None, None, None, None, None, None, None, None, false, false, false, true,
                    verbose
                )?
                .connected_components_number(verbose)
                .0,
                1,
                "Expected number of components (1) is not matched!"
            );
        }

        if let Some(ets) = &graph.edge_types {
            let test = graph.remove_components(
                None,
                None,
                Some(vec![ets.translate(0).to_string()]),
                None,
                None,
                verbose,
            )?;
            assert_eq!(
                test.remove(
                    None, None, None, None, None, None, None, None, false, false, false, true,
                    verbose
                )?
                .connected_components_number(verbose)
                .0,
                1,
                "Expected number of components (1) is not matched!"
            );
        }
    }

    // test the kfold
    let k = 10;
    for i in 0..k {
        let (train, test) = graph.kfold(k, i, None, 42, false)?;
        assert!(
            test.get_edges_number() <= graph.get_edges_number() / k + 1,
            "Check that test kfolds respect size bound has failed!"
        );
        default_holdout_test_suite(graph, &train, &test)?;
    }
    if let Some(edge_t) = graph.get_edge_type_string(0) {
        for i in 0..k {
            let (train, test) = graph.kfold(k, i, Some(vec![edge_t.clone()]), 1337, false)?;
            default_holdout_test_suite(graph, &train, &test)?;
        }
    }

    // Testing negative edges generation
    for only_from_same_component in &[true, false] {
        let negatives = graph.sample_negatives(
            4,
            graph.get_edges_number(),
            None,
            *only_from_same_component,
            verbose,
        )?;
        validate_vocabularies(&negatives);
        if !graph.has_edge_types() {
            assert!(!graph.overlaps(&negatives)?);
            assert!(!negatives.overlaps(&graph)?);
        }
        // Testing holdouts executed on negative edges.
        let (neg_train, neg_test) =
            negatives.random_holdout(32, 0.8, false, None, None, verbose)?;
        default_holdout_test_suite(&negatives, &neg_train, &neg_test)?;
    }
    // Testing subgraph generation
    let expected_nodes = graph.get_not_singleton_nodes_number() / 10;
    let subgraph = graph.random_subgraph(6, expected_nodes, verbose)?;
    assert!(subgraph.overlaps(&graph)?);
    assert!(subgraph.get_not_singleton_nodes_number() <= expected_nodes + 1);

    // Testing writing out graph to file
    let node_file = random_path();
    let nodes_writer = NodeFileWriter::new(node_file.clone())
        .set_verbose(Some(verbose))
        .set_separator(Some("\t".to_string()))
        .set_header(Some(true))
        .set_node_types_column_number(Some(4))
        .set_nodes_column_number(Some(6))
        .set_node_types_column(Some("node_types".to_string()))
        .set_nodes_column(Some("node_column".to_string()));
    nodes_writer.dump(&graph)?;
    fs::remove_file(node_file).unwrap();

    let edges_file = random_path();
    let edges_writer = EdgeFileWriter::new(edges_file.clone())
        .set_verbose(Some(verbose))
        .set_separator(Some("\t".to_string()))
        .set_header(Some(true))
        .set_edge_types_column(Some("edge_types".to_string()))
        .set_destinations_column_number(Some(3))
        .set_weights_column(Some("weight".to_string()))
        .set_weights_column_number(Some(2))
        .set_sources_column(Some("The land of sushi".to_string()))
        .set_sources_column_number(Some(0))
        .set_destinations_column(Some("The land of pizza".to_string()))
        .set_destinations_column_number(Some(1));

    edges_writer.dump(&graph)?;
    fs::remove_file(edges_file).unwrap();

    if !graph.directed {
        // Testing SkipGram / CBOW / GloVe preprocessing
        graph.cooccurence_matrix(&walker, 3, verbose)?;
        graph.node2vec(&walker, 1, 3)?;
        // Testing link prediction pre-processing
        graph.link_prediction(0, 1, 1.0, true, None)?;
        graph.link_prediction(0, 1, 1.0, false, None)?;
    }
    // Compute metrics of the graph
    graph.report();
    graph.textual_report(verbose)?;
    // Compute degrees metrics
    for src in 0..10 {
        for dst in 0..10 {
            graph.degrees_product(src, dst)?;
            graph.jaccard_index(src, dst)?;
            graph.adamic_adar_index(src, dst)?;
            graph.resource_allocation_index(src, dst)?;
        }
    }

    // Testing generic filtering mechanisms
    let _filtered = graph
        .filter(
            Some(graph.get_node_names()),
            graph.get_node_type_names(),
            graph.get_edge_type_names(),
            graph.get_edge_weight(0),
            graph.get_edge_weight(graph.get_edges_number() - 1),
            verbose,
        )
        .unwrap();

    // Tetsing edge lists generation
    let _clique = graph.get_clique_edge_names(
        None,
        None,
        Some(false),
        None,
        Some(
            graph
                .get_node_names()
                .iter()
                .cloned()
                .collect::<HashSet<String>>(),
        ),
    );
    if graph.get_nodes_number() > 1 {
        let _bipartite = graph.get_bipartite_edge_names(
            None,
            Some(
                [graph.get_node_name(0).unwrap()]
                    .iter()
                    .cloned()
                    .collect::<HashSet<String>>(),
            ),
            Some(
                [graph.get_node_name(1).unwrap()]
                    .iter()
                    .cloned()
                    .collect::<HashSet<String>>(),
            ),
            None,
            None,
        )?;
        let _star = graph.get_star_edges(
            graph.get_node_name(0).unwrap(),
            Some(false),
            Some(
                [graph.get_node_name(1).unwrap()]
                    .iter()
                    .cloned()
                    .collect::<HashSet<String>>(),
            ),
            None,
        )?;
        let _star = graph.get_star_edge_names(
            graph.get_node_name(0).unwrap(),
            Some(false),
            Some(
                [graph.get_node_name(1).unwrap()]
                    .iter()
                    .cloned()
                    .collect::<HashSet<String>>(),
            ),
            None,
        )?;
    }

    // Testing the top Ks
    if graph.has_node_types() {
        graph.get_node_type(0)?;

        assert!(
            graph.get_node_type(graph.get_nodes_number() + 1).is_err(),
            "Given graph does not raise an exception when a node's node type greater than the number of available nodes is requested."
        );
    }
    if graph.has_edge_types() {
        graph.get_edge_type(0)?;

        assert!(
            graph.get_edge_type(graph.get_edges_number() + 1).is_err(),
            "Given graph does not raise an exception when a edge's edge type greater than the number of available edges is requested."
        );
    }
    // Evaluate get_node_type
    assert_eq!(graph.get_node_type(0).is_ok(), graph.has_node_types());

    // Evaluate get_edge_type
    assert_eq!(graph.get_edge_type(0).is_ok(), graph.has_edge_types());

    // Evaluate get_node_type_counts
    assert_eq!(graph.get_node_type_counts().is_ok(), graph.has_node_types());

    // Evaluate get_edge_type_counts
    assert_eq!(graph.get_edge_type_counts().is_ok(), graph.has_edge_types());

    // Evaluate get_edge_type_counts_hashmap
    assert_eq!(
        graph.get_edge_type_counts_hashmap().is_ok(),
        graph.has_edge_types()
    );

    graph.set_name("Non c'è l'agavazzzz, c'è la manetta".to_owned());
    graph.strongly_connected_components();

    //test removes
    {
        let without_edge_types = graph.remove(
            None, None, None, None, None, None, None, None, false, false, true, false, verbose,
        );
        if let Some(we) = &without_edge_types.ok() {
            validate_vocabularies(we);
            assert_eq!(we.has_edge_types(), false);
            assert_eq!(we.has_weights(), graph.has_weights());
            assert_eq!(we.node_types, graph.node_types);
            assert_eq!(
                we.get_unique_edges_number(),
                graph.get_unique_edges_number(),
                "Number of unique edges does not match in graph without edge types."
            );
            assert_eq!(
                we.get_unique_self_loop_number(),
                graph.get_unique_self_loop_number(),
                "Number of unique self loops does not match in graph without edge types."
            );
            assert_eq!(we.nodes, graph.nodes);
            assert_eq!(
                graph.has_edge_types(),
                graph.validate_operator_terms(&we).is_err()
            );
        }
    }
    {
        let without_node_types = graph.remove(
            None, None, None, None, None, None, None, None, false, true, false, false, verbose,
        );
        if let Some(wn) = &without_node_types.ok() {
            validate_vocabularies(wn);
            assert_eq!(wn.has_node_types(), false);
            assert_eq!(wn.weights, graph.weights);
            assert_eq!(wn.has_selfloops(), graph.has_selfloops());
            assert_eq!(wn.nodes, graph.nodes);
            //assert_eq!(wn.edges, graph.edges);
            assert_eq!(
                graph.has_node_types(),
                graph.validate_operator_terms(&wn).is_err()
            );
        }
    }
    {
        let without_weights = graph.remove(
            None, None, None, None, None, None, None, None, true, false, false, false, verbose,
        );
        if let Some(ww) = &without_weights.ok() {
            validate_vocabularies(ww);
            assert_eq!(ww.has_weights(), false);
            assert_eq!(ww.node_types, graph.node_types);
            assert_eq!(ww.has_selfloops(), graph.has_selfloops());
            assert_eq!(ww.nodes, graph.nodes);
            //assert_eq!(ww.edges, graph.edges);
            assert_eq!(
                graph.has_weights(),
                graph.validate_operator_terms(&ww).is_err()
            );
        }
    }

    // Testing cloning
    let mut clone = graph.clone();
    clone = clone.set_all_edge_types("TEST_SET_ALL_EDGE_TYPES".to_string());
    clone = clone.set_all_node_types("TEST_SET_ALL_NODE_TYPES".to_string());

    assert_eq!(
        clone.get_edge_types_number(),
        1,
        "Number of edge types of the graph is not 1."
    );
    assert_eq!(
        clone.get_unchecked_edge_count_by_edge_type(0),
        graph.get_edges_number(),
        "Number of edges with the unique edge type does not match number of edges in the graph."
    );

    assert_eq!(
        clone.get_node_types_number(),
        1,
        "Number of node types of the graph is not 1."
    );
    assert_eq!(
        clone.get_unchecked_node_count_by_node_type(0),
        graph.get_nodes_number(),
        "Number of nodes with the unique node type does not match number of nodes in the graph."
    );

    Ok(())
}
