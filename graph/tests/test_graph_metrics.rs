extern crate graph;
use graph::*;

#[test]
fn test_graph_metrics() {
    let edge_path = "tests/data/het_graph_edges.tsv";
    let node_path = "tests/data/het_graph_nodes.tsv";
    for directed in &[true, false] {
        let graph = FromCsvBuilder::new(
            edge_path,
            "subject", 
            "object", 
            *directed, 
            None
        ).unwrap()
        .set_weights("weight", Some(1.0))
        .load_nodes_csv(
            node_path, 
            "id", 
            "category",
            Some("biolink:NamedThing"), 
            None, 
            None
        ).unwrap().build().unwrap();

        format!("{:?}", graph);

        for one in 0..graph.get_nodes_number() {
            for two in 0..graph.get_nodes_number() {
                graph.degrees_product(one, two);
                graph.jaccard_index(one, two);
                graph.adamic_adar_index(one, two);
                graph.resource_allocation_index(one, two);
            }
        }
        graph.traps_rate();
    }
}
