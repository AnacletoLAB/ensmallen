extern crate graph;
use graph::*;
use hashbrown::HashSet;

#[test]
fn test_spanning_tree() {
    for directed in [true, false].iter() {
        let graph = FromCsvBuilder::new(
            "tests/data/edge_file.tsv",
            "subject",
            "object",
            *directed,
            None,
        )
        .unwrap()
        .build()
        .unwrap();
        for node in 0..graph.get_nodes_number() {
            // compute the spanning tree
            let edges: HashSet<(NodeT, NodeT, Option<EdgeTypeT>)> = graph.spanning_tree(node, true);

            // check that the destinations are uniques
            let destinations: HashSet<graph::NodeT> =
                edges.iter().map(|(_, dst, _)| *dst).collect();
            assert_eq!(destinations.len(), edges.len());
        }
    }
}
