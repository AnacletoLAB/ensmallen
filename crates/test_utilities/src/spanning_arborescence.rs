use graph::Graph;
use shared::types::*;

/// Test that the spanning arborescence algorithm from bader is working correctly.
pub fn test_spanning_arborescence_bader(graph: &Graph, verbose: Option<bool>) {
    let kruskal_tree = graph.spanning_arborescence_kruskal(verbose).0;
    let random_kruskal_tree = graph
        .random_spanning_arborescence_kruskal(Some(42), None, verbose)
        .0;
    if !graph.directed {
        let spanning_arborescence_bader: Vec<(NodeT, NodeT)> =
            graph.spanning_arborescence(verbose).unwrap().1.collect();
        assert_eq!(
                spanning_arborescence_bader.len(), kruskal_tree.len(),
                "The number of extracted edges forming the spanning arborescence computed by the bader's algorithm does not match the one computed by kruskal. The graph report is:\n{:?}\nThe bader's tree is:\n{:?}\nThe kruskal's tree is:\n{:?}",
                graph.textual_report(), spanning_arborescence_bader, kruskal_tree,
            );
    } else {
        assert!(graph.spanning_arborescence(verbose).is_err());
    }
    assert_eq!(random_kruskal_tree.len() as usize, kruskal_tree.len());
}