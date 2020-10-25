use super::*;
use log::info;
use rand::Rng;
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
                .set_nodes_column_number(Some(1))?
                .set_node_types_column(Some("category".to_string()))?
                .set_default_node_type(Some("default".to_string()))
                .set_nodes_column(Some("id".to_string()))?
                .set_ignore_duplicates(Some(true))
                .set_separator(Some("\t".to_string()))
                .unwrap()
                .set_header(Some(true))
                .set_max_rows_number(Some(100000))
                .set_rows_to_skip(Some(0)),
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
        .set_sources_column_number(Some(1))?
        .set_sources_column(Some("subject".to_string()))?
        .set_destinations_column_number(Some(1))?
        .set_destinations_column(Some("object".to_string()))?
        .set_weights_column_number(if load_weights { Some(1) } else { None })?
        .set_weights_column(if load_weights {
            Some("weight".to_string())
        } else {
            None
        })?
        .set_edge_types_column_number(if load_edge_types { Some(1) } else { None })?
        .set_edge_types_column(if load_edge_types {
            Some("edge_label".to_string())
        } else {
            None
        })?
        .set_default_edge_type(Some("Kebab".to_string()))
        .set_max_rows_number(Some(100000))
        .set_default_weight(Some(5.0))
        .set_skip_self_loops(Some(skip_self_loops));

    Graph::from_unsorted_csv(edges_reader, nodes_reader, directed, "Graph".to_owned())
}

/// Return WalksParameters to execute a first order walk.
pub fn first_order_walker(graph: &Graph, verbose: bool) -> Result<WalksParameters, String> {
    Ok(WalksParameters::new(10)?
        .set_iterations(Some(1))?
        .set_min_length(Some(1))?
        .set_verbose(Some(verbose))
        .set_random_state(Some(43))
        .set_dense_node_mapping(Some(graph.get_dense_node_mapping())))
}

/// Return WalksParameters to execute a second order walk.
pub fn second_order_walker(graph: &Graph, verbose: bool) -> Result<WalksParameters, String> {
    Ok(WalksParameters::new(10)?
        .set_iterations(Some(1))?
        .set_min_length(Some(1))?
        .set_verbose(Some(verbose))
        .set_return_weight(Some(2.0))?
        .set_explore_weight(Some(2.0))?
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
    assert!(!train.overlaps(&test)?);
    assert!(!test.overlaps(&train)?);
    assert!(graph.contains(&train)?);
    assert!(graph.contains(&test)?);
    let summed = (train | test)?;
    validate_vocabularies(&summed);
    assert!(summed.contains(&graph)?);
    let subtracted = (graph - test)?;
    validate_vocabularies(&subtracted);
    assert!(subtracted.contains(&train)?);
    assert!(!subtracted.overlaps(&test)?);
    let xorred = (graph ^ test)?;
    validate_vocabularies(&xorred);
    assert!(xorred.contains(&train)?);
    assert!(!xorred.overlaps(&test)?);
    let anded = (graph & test)?;
    validate_vocabularies(&anded);
    assert!(anded.contains(&test)?);
    Ok(())
}

/// Executes near-complete test of all functions for the given graph.
pub fn default_test_suite(graph: &mut Graph, verbose: bool) -> Result<(), String> {
    // Testing that vocabularies are properly loaded
    validate_vocabularies(graph);
    // Testing principal random walk algorithms
    let walker = first_order_walker(&graph, verbose)?;
    if !graph.directed {
        for i in 0..2 {
            if i == 1 {
                graph.enable_fast_walk(true, true);
                if let Some(outbounds) = &graph.outbounds {
                    assert_eq!(outbounds.len(), graph.get_nodes_number() as usize);
                }
                if let Some(destinations) = &graph.destinations {
                    assert_eq!(destinations.len(), graph.get_edges_number() as usize);
                }
            }
            info!("Executing random walks.");
            assert_eq!(
                graph.random_walks(1, &walker)?,
                graph.random_walks(1, &walker)?
            );

            assert_eq!(
                graph.random_walks(1, &second_order_walker(&graph, verbose)?)?,
                graph.random_walks(1, &second_order_walker(&graph, verbose)?)?
            );

            assert_eq!(
                graph.complete_walks(&walker)?,
                graph.complete_walks(&walker)?
            );

            assert_eq!(
                graph.complete_walks(&second_order_walker(&graph, verbose)?)?,
                graph.complete_walks(&second_order_walker(&graph, verbose)?)?
            );
        }
    }

    // Test get_edge_id_string()
    assert_eq!(graph.get_edge_id_string("NONEXISTENT", "NONEXISTENT", None), None);
    if let Some(edge) = graph.get_unique_edges_iter().next() {
        let src_string = graph.get_node_name(edge.0).unwrap();
        let dst_string = graph.get_node_name(edge.1).unwrap();
        assert!(graph.has_edge_string(&src_string, &dst_string, None));
        assert!(graph.has_node_string(&src_string, None) && graph.has_node_string(&dst_string, None));
        assert_eq!(graph.get_edge_id_string(&src_string, &dst_string,
                                            Some(&"NONEXISTENT_EDGE_TYPE".to_string())),
                   None);
        if ! graph.has_edge_types(){
            assert_eq!(graph.get_edge_id_string(&src_string, &dst_string, None),
                       graph.get_edge_id(edge.0, edge.1, None));
        }
    }
    // Test has_node_string
    assert!(!(graph.has_node_string("NONEXISTENT", None)));

    // Test translate_edge|node_types()
    assert!(graph.translate_edge_types(vec!["NONEXISTENT_EDGE_TYPE".to_string()]).is_err());
    assert!(graph.translate_node_types(vec!["NONEXISTENT_NODE_TYPE".to_string()]).is_err());

    // Testing main holdout mechanisms
    for include_all_edge_types in &[false, true] {
        let (train, test) =
            graph.random_holdout(4, 0.6, *include_all_edge_types, None, None, verbose)?;
        default_holdout_test_suite(graph, &train, &test)?;
        let (train, test) =
            graph.connected_holdout(4, 0.8, None, *include_all_edge_types, verbose)?;
        assert_eq!(
            graph.connected_components_number(false),
            train.connected_components_number(false)
        );

        default_holdout_test_suite(graph, &train, &test)?;
    }

    // test drop components
    if graph.connected_components_number(false).0 > 1 {
        let test = graph.drop_components(
            Some(vec![graph.nodes.translate(0).to_string()]),
            None,
            None,
            None,
            None,
            verbose,
        )?;
        assert_eq!(
            test.drop(None, None, None, None, false, false, false, true, verbose)?
                .connected_components_number(false)
                .0,
            1
        );

        if let Some(nts) = &graph.node_types {
            let test = graph.drop_components(
                None,
                Some(vec![nts.translate(0).to_string()]),
                None,
                None,
                None,
                verbose,
            )?;
            assert_eq!(
                test.drop(None, None, None, None, false, false, false, true, verbose)?
                    .connected_components_number(false)
                    .0,
                1
            );
        }

        if let Some(ets) = &graph.edge_types {
            let test = graph.drop_components(
                None,
                None,
                Some(vec![ets.translate(0).to_string()]),
                None,
                None,
                verbose,
            )?;
            assert_eq!(
                test.drop(None, None, None, None, false, false, false, true, verbose)?
                    .connected_components_number(false)
                    .0,
                1
            );
        }
    }

    // test the kfold
    let k = 10;
    for i in 0..k {
        let (train, test) = graph.kfold(k, i, None, 42, false)?;
        assert!(test.get_edges_number() <= graph.get_edges_number() / k + 1);
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
        graph.link_prediction(0, 1, 1.0, None)?;
    }
    // Compute metrics of the graph
    graph.report();
    graph.textual_report();
    // Compute degrees metrics
    for src in 0..10 {
        for dst in 0..10 {
            graph.degrees_product(src, dst)?;
            graph.jaccard_index(src, dst)?;
            graph.adamic_adar_index(src, dst)?;
            graph.resource_allocation_index(src, dst)?;
        }
    }
    // Testing the top Ks
    if graph.has_node_types() {
        graph.get_node_type(0)?;

        assert!(graph.get_node_type(graph.get_nodes_number() + 1).is_err());
    }
    if graph.has_edge_types() {
        graph.get_edge_type(0)?;

        assert!(graph.get_edge_type(graph.get_edges_number() + 1).is_err());
    }
    // Evaluate get_node_type
    assert_eq!(graph.get_node_type(0).is_ok(), graph.has_node_types());

    // Evaluate get_edge_type
    assert_eq!(graph.get_edge_type(0).is_ok(), graph.has_edge_types());

    // Evaluate get_node_type_counts
    assert_eq!(graph.get_node_type_counts().is_ok(), graph.has_node_types());

    // Evaluate get_edge_type_counts
    assert_eq!(graph.get_edge_type_counts().is_ok(), graph.has_edge_types());

    //test drops
    {
        let without_edges =
            graph.drop(None, None, None, None, false, false, true, false, verbose);
        assert_eq!(without_edges.is_ok(), graph.has_edge_types());
        if let Some(we) = &without_edges.ok() {
            validate_vocabularies(we);
            assert_eq!(we.has_edge_types(), false);
            assert_eq!(we.has_weights(), graph.has_weights());
            assert!(we.node_types == graph.node_types);
            assert_eq!(
                we.get_unique_edges_number(),
                graph.get_unique_edges_number()
            );
            assert_eq!(
                we.get_unique_self_loop_number(),
                graph.get_unique_self_loop_number()
            );
            assert_eq!(we.has_traps(), graph.has_traps());
            assert_eq!(we.nodes, graph.nodes);

            // expect errors for undefined behavior in overlap() and contains()
            assert!(!graph.overlaps(&we)?);
            assert!(!graph.contains(&we)?);
        }
    }
    {
        let without_nodes = graph.drop(None, None, None, None, false, true, false, false, verbose);
        assert_eq!(without_nodes.is_ok(), graph.has_node_types());
        if let Some(wn) = &without_nodes.ok() {
            validate_vocabularies(wn);
            assert_eq!(wn.has_node_types(), false);
            assert!(wn.edge_types == graph.edge_types);
            assert_eq!(wn.weights, graph.weights);
            assert_eq!(wn.has_selfloops(), graph.has_selfloops());
            assert_eq!(wn.has_traps(), graph.has_traps());
            assert_eq!(wn.nodes, graph.nodes);
            assert_eq!(wn.edges, graph.edges);
        }
    }
    {
        let without_weights = graph.drop(None, None, None, None, true, false, false, false, verbose);
        assert_eq!(without_weights.is_ok(), graph.has_weights());
        if let Some(ww) = &without_weights.ok() {
            validate_vocabularies(ww);
            assert_eq!(ww.has_weights(), false);
            assert!(ww.node_types == graph.node_types);
            assert!(ww.edge_types == graph.edge_types);
            assert_eq!(ww.has_selfloops(), graph.has_selfloops());
            assert_eq!(ww.has_traps(), graph.has_traps());
            assert_eq!(ww.nodes, graph.nodes);
            assert_eq!(ww.edges, graph.edges);
        }
    }

    // Testing cloning
    let mut clone = graph.clone();
    clone = clone.set_all_edge_types("TEST_SET_ALL_EDGE_TYPES".to_string());
    clone = clone.set_all_node_types("TEST_SET_ALL_NODE_TYPES".to_string());

    assert_eq!(clone.get_edge_types_number(), 1);
    assert_eq!(clone.get_edge_type_number(0), graph.get_edges_number());

    assert_eq!(clone.get_node_types_number(), 1);
    assert_eq!(clone.get_node_type_number(0), graph.get_nodes_number());

    Ok(())
}
