extern crate graph;
use graph::*;
use rayon::prelude::*;
use hashbrown::HashSet;

#[test]
fn test_spanning_tree() {
    let graph = Graph::from_csv(
        &"tests/data/edge_file.tsv",
        "subject",
        "object",
        false,
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
        None
    ).unwrap();
    for node in 0..graph.get_nodes_number(){
        // compute the spanning tree
        let (tree_src, tree_dst) = graph.spanning_tree(node);

        // THE FOLLOWING CHECK IS NOT VALID
        // because if the graph is composed by multiple components
        // the actual number of edges will be = (|V| - 1 - # components)
        // check that the number of edges is the number of node - 1
        //assert_eq!(tree_dst.len(), graph.get_nodes_number() - 1);

        // check that every edge exists
        assert!(tree_src.par_iter().zip(tree_dst.par_iter()).all(
            |(src, dst)| graph.has_edge(*src, *dst)
        ));
        // check that the destinations are uniques
        let destinations: HashSet<graph::NodeT> = tree_dst.iter().cloned().collect();
        assert_eq!(destinations.len(), tree_dst.len());
    }
}