#![feature(is_sorted)]

use log::warn;
use shared::*;
use graph::Graph;

mod utils;
pub use utils::*;

mod graph_properties;
pub use graph_properties::*;

mod all_paths;
pub use all_paths::*;

mod bfs;
pub use bfs::*;

mod diameter;
pub use diameter::*;

mod dijkstra;
pub use dijkstra::*;

mod dump_graph;
pub use dump_graph::*;

mod edge_holdout;
pub use edge_holdout::*;

mod holdouts;
pub use holdouts::*;

mod edgelabel_holdouts;
pub use edgelabel_holdouts::*;

mod edgelist_generation;
pub use edgelist_generation::*;

mod embiggen_preprocessing;
pub use embiggen_preprocessing::*;

mod filters;
pub use filters::*;

mod kfold;
pub use kfold::*;

mod negative_edges_generation;
pub use negative_edges_generation::*;

mod node_centralities;
pub use node_centralities::*;

mod nodelabel_holdouts;
pub use nodelabel_holdouts::*;

mod polygons;
pub use polygons::*;

mod random_walks;
pub use random_walks::*;

mod remapping;
pub use remapping::*;

mod remove_components;
pub use remove_components::*;

mod removes;
pub use removes::*;

mod selfloops;
pub use selfloops::*;

mod setters;
pub use setters::*;

mod sorting;
pub use sorting::*;

mod spanning_arborescence;
pub use spanning_arborescence::*;

mod subgraph_generation;
pub use subgraph_generation::*;

mod transitivity;
pub use transitivity::*;

mod vertex_cover;
pub use vertex_cover::*;

/// Executes near-complete test of all functions for the given graph.
fn _default_test_suite(graph: &mut Graph, verbose: Option<bool>) -> Result<()> {
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
    let _ = test_dump_graph(graph, verbose);

    warn!("Testing generic filtering mechanism.");
    let _ = test_graph_filter(graph, verbose);

    warn!("Testing the spanning arborescences.");
    let _ = test_spanning_arborescence_bader(graph, verbose);

    warn!("Testing the graph diameter.");
    let _ = test_graph_diameter(graph, verbose);

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

    warn!("Testing BFS.");
    let _ = test_bfs(graph, verbose);

    warn!("Testing dijkstra.");
    let _ = test_dijkstra(graph, verbose);

    warn!("Testing approximated vertex cover");
    let _ = test_vertex_cover(graph, verbose);

    warn!("Testing node centralities.");
    let _ = test_node_centralities(graph, verbose);

    warn!("Testing polygons.");
    let _ = test_polygons(graph, verbose);

    warn!("Testing transitivity.");
    let _ = test_transitivity(graph, verbose);

    warn!("Testing all paths.");
    let _ = test_all_paths(graph, verbose);

    warn!("Testing generation of selfloops.");
    let _ = test_selfloops(graph, verbose);

    warn!("Testing sorting of the graph.");
    let _ = test_sorting(graph, verbose);

    Ok(())
}

macro_rules! test_mut_graph {
    ($graph:expr, $func:ident, $verbose:expr) => {{
        println!("Testing the graph transformation: {}", stringify!($func));
        let mut transformed_graph = $graph.$func();
        let _ = _default_test_suite(&mut transformed_graph, $verbose);
    }};
    ($graph:expr, $func:ident, $verbose:expr, result) => {{
        println!("Testing the graph transformation: {}", stringify!($func));
        let mut transformed_graph = $graph.$func()?;
        let _ = _default_test_suite(&mut transformed_graph, $verbose);
    }};
}

/// Executes near-complete test of all functions for the given graph.
pub fn default_test_suite(graph: &mut Graph, verbose: Option<bool>) -> Result<()> {
    warn!("Starting default test suite.");
    let _ = _default_test_suite(graph, verbose);
    warn!("Starting default test suite with speedups enabled.");
    graph.enable(Some(true), Some(true), Some(true))?;
    let _ = _default_test_suite(graph, verbose);
    warn!("Starting default test suite on transformed graphs.");

    test_mut_graph!(graph, get_laplacian_transformed_graph, verbose);
    test_mut_graph!(
        graph,
        get_symmetric_normalized_transformed_graph,
        verbose,
        result
    );
    test_mut_graph!(
        graph,
        get_symmetric_normalized_laplacian_transformed_graph,
        verbose,
        result
    );
    test_mut_graph!(
        graph,
        get_random_walk_normalized_laplacian_transformed_graph,
        verbose
    );
    test_mut_graph!(graph, to_upper_triangular, verbose);
    test_mut_graph!(graph, to_lower_triangular, verbose);
    test_mut_graph!(graph, to_main_diagonal, verbose);
    test_mut_graph!(graph, to_anti_diagonal, verbose);
    test_mut_graph!(graph, to_bidiagonal, verbose);
    test_mut_graph!(graph, to_arrowhead, verbose);
    test_mut_graph!(graph, to_transposed, verbose);
    // We skip very heavy operations on graphs with more than 500
    // nodes because it would take way too much time.
    if graph.get_nodes_number() > 20 {
        return Ok(());
    }
    test_mut_graph!(graph, to_complementary, verbose);

    Ok(())
}
