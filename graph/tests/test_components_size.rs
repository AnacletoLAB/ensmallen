extern crate graph;
use graph::{EdgeFileReader, Graph};

#[test]
/// Test that the components number is reasonable, this raised the wierd case in which:
/// singletons: false selfloops: false smallest: 1 biggest: 3, edges: [(0, 1), (2, 3), (4, 5)]
fn test_components_size() {
    let edges_reader = EdgeFileReader::new("tests/data/test_components.csv".to_string())
        .unwrap()
        .set_separator(Some("\c".to_string()))
        .unwrap()
        .set_verbose(Some(false))
        .set_numeric_node_ids(Some(true))
        .set_header(Some(false));

    let mut g =
        Graph::from_sorted_csv(edges_reader, None, false, false, 6108, 242, "Graph".to_owned()).unwrap();

        
    let (components_number, smallest, biggest) = graph.connected_components_number(false);

    assert!(biggest >= smallest, "smallest: {} biggest: {}", smallest, biggest);

    assert_eq!(
        !graph.has_singletons() && !graph.has_selfloops(), smallest > 1, 
        "singletons: {} selfloops: {} smallest: {} biggest: {}, edges: {:?}", 
        graph.has_singletons(), graph.has_selfloops(), smallest, biggest, graph.get_unique_edges_iter(false).collect::<Vec<(NodeT, NodeT)>>()
    );
   
}
