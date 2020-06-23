extern crate graph;
use graph::graph::Graph;

#[test]
fn test_graph_metrics() {
    let edge_path = "tests/data/het_graph_edges.tsv";
    let node_path = "tests/data/het_graph_nodes.tsv";
    for directed in &[true, false]{
        let graph = Graph::from_csv(
            edge_path,
            "subject",
            "object",
            *directed,
            None,
            None,
            Some("weight"),
            Some(1.0),
            Some(node_path),
            Some("id"),
            Some("category"),
            Some("biolink:NamedThing"),
            None,
            None,
            None,
        ).unwrap();

        format!("{:?}", graph);
        
        for one in 0..graph.get_nodes_number(){
            for two in 0..graph.get_nodes_number(){
                graph.degrees_product(one, two);
                graph.jaccard_index(one, two);
                graph.adamic_adar_index(one, two);
                graph.resource_allocation_index(one, two);
            }
        }
        graph.traps_rate();
    };
}