
use graph::{Graph, WalksParameters};
use shared::*;
use readers_and_writers::*;

use rand::Rng;
use std::path::Path;

// where to save the test files
#[cfg(any(target_os = "linux", target_os = "macos"))]
pub static DEFAULT_PATH: &str = "/tmp/";
#[cfg(not(any(target_os = "linux", target_os = "macos")))]
pub static DEFAULT_PATH: &str = "";

pub const NONEXISTENT: &str = "Cthulhu is a fictional cosmic entity created by writer H. P. Lovecraft and first introduced in the short story The Call of Cthulhu,[2] published in the American pulp magazine Weird Tales in 1928. Considered a Great Old One within the pantheon of Lovecraftian cosmic entities, the creature has since been featured in numerous popular culture references. Lovecraft depicts it as a gigantic entity worshipped by cultists, in shape like an octopus, a dragon, and a caricature of human form. Its name was given to the Lovecraft-inspired universe where it and its fellow entities existed, the Cthulhu Mythos.";

pub const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";

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
    parallel: bool,
) -> Graph {
    let graph_name = "STRING PPI".to_owned();
    let nodes_reader = if load_nodes {
        Some(
            NodeFileReader::new(Some("tests/data/ppi/nodes.tsv".to_string()))
                .unwrap()
                .set_verbose(Some(false))
                .set_rows_to_skip(Some(0))
                .unwrap()
                .set_header(Some(true))
                .unwrap()
                .set_max_rows_number(Some(100000))
                .unwrap()
                .set_default_node_type(Some("default".to_string()))
                .set_ignore_duplicates(Some(true))
                .unwrap()
                .set_separator(Some("\t".to_string()))
                .unwrap()
                .set_nodes_column(Some("id".to_string()))
                .unwrap()
                .set_node_types_column_number(Some(1))
                .unwrap()
                .set_nodes_column_number(Some(0))
                .unwrap()
                .set_node_types_column(Some("category".to_string()))
                .unwrap()
                .set_csv_is_correct(Some(true))
                .unwrap()
                .set_nodes_number(Some(37163))
                .set_parallel(Some(parallel))
                .unwrap()
                .clone(),
        )
    } else {
        None
    };
    let edges_reader = EdgeFileReader::new("tests/data/ppi/edges.tsv".to_string())
        .unwrap()
        .set_verbose(Some(verbose))
        .set_ignore_duplicates(Some(true))
        .set_header(Some(true))
        .unwrap()
        .set_max_rows_number(Some(100000))
        .unwrap()
        .set_rows_to_skip(Some(0))
        .unwrap()
        .set_separator(None::<String>)
        .unwrap()
        .set_sources_column(Some("subject".to_string()))
        .unwrap()
        .set_destinations_column(Some("object".to_string()))
        .unwrap()
        .set_parallel(Some(parallel))
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
        .set_csv_is_correct(Some(true))
        .set_default_edge_type(if load_edge_types {
            Some("Kebab".to_string())
        } else {
            None
        })
        .set_default_weight(if load_weights { Some(5.0) } else { None })
        .unwrap()
        .clone();

    let ppi = Graph::from_file_readers(
        Some(edges_reader),
        nodes_reader,
        None,
        None,
        true,
        true,
        directed,
        graph_name.clone(),
    )
    .unwrap();
    assert_eq!(ppi.has_node_types(), load_nodes);
    assert_eq!(
        ppi.has_edge_types(),
        load_edge_types,
        concat!(
            "Both the `has_edge_types` method and the `load_edge_types`\n",
            "flag shoud have the same value but were:\n",
            "* has_edge_types: {}\n",
            "* load_edge_types: {}\n",
        ),
        ppi.has_edge_types(),
        load_edge_types,
    );
    assert_eq!(ppi.has_edge_weights(), load_weights);
    ppi
}

#[allow(clippy::redundant_clone)]
/// This is our default graph we use on tests with node types.
pub fn load_cora() -> Graph {
    let graph_name = "Cora".to_owned();
    let edges_reader = EdgeFileReader::new("tests/data/cora/edges.tsv")
        .unwrap()
        .set_separator(Some("\t".to_string()))
        .unwrap()
        .set_verbose(Some(false))
        .set_sources_column(Some("subject"))
        .unwrap()
        .set_destinations_column(Some("object"))
        .unwrap()
        .set_edge_types_column(Some("edge_type"))
        .unwrap();
    let nodes_reader = NodeFileReader::new(Some("tests/data/cora/nodes.tsv".to_owned()))
        .unwrap()
        .set_separator(Some("\t".to_string()))
        .unwrap()
        .set_nodes_column(Some("id"))
        .unwrap()
        .set_verbose(Some(false))
        .set_node_types_column(Some("node_type"))
        .unwrap();
    Graph::from_file_readers(
        Some(edges_reader),
        Some(nodes_reader),
        None,
        None,
        true,
        true,
        false,
        graph_name.clone(),
    )
    .unwrap()
}

/// Return WalksParameters to execute a first order walk.
pub fn first_order_walker(graph: &Graph) -> Result<WalksParameters> {
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
) -> Result<WalksParameters> {
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

pub fn validate_vocabularies(graph: &Graph) {
    if let Some(ets) = &graph.edge_types {
        assert_eq!(!ets.ids.is_empty(), graph.has_edge_types(),
            "We expected that if the graph has edge types then it cannot be empty. The report of the graph is:\n{:?}",
            graph.textual_report()
        );
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
            graph.textual_report()
        );
    }
}
