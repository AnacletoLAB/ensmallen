extern crate graph;
use graph::*;
use hashbrown::HashSet;

#[test]
fn test_spanning_tree() {
    for directed in [true, false].iter() {
        let graph = Graph::from_csv(
            &"tests/data/edge_file.tsv",
            "subject",
            "object",
            *directed,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
        )
        .unwrap();
        for node in 0..graph.get_nodes_number() {
            // compute the spanning tree
            let edges: HashSet<(NodeT, NodeT, Option<EdgeTypeT>)> = graph.spanning_tree(node);

            // check that the destinations are uniques
            let destinations: HashSet<graph::NodeT> =
                edges.iter().map(|(_, dst, _)| *dst).collect();
            assert_eq!(destinations.len(), edges.len());
        }
    }
}
