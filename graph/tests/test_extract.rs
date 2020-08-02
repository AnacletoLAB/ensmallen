extern crate graph;
use graph::FromCsvBuilder;

#[test]
fn test_extract_nodes() {
    let edge_path = "tests/data/het_graph_edges.tsv";
    let node_path = "tests/data/het_graph_nodes.tsv";
    let graph = FromCsvBuilder::new(edge_path, "subject", "object", false, None)
    .unwrap()
    .set_weights("weight", Some(1.0))
    .load_nodes_csv(
        node_path,
        "id",
        "category",
        Some("biolink:NamedThing"),
        None,
        None,
    )
    .unwrap()
    .build()
    .unwrap();

    for i in 0..12 {
        let size = (1 << i) - 1;
        let nodes1 = graph.extract_random_nodes(size, 0xbad53ed);
        assert_eq!(size, nodes1.len());
        let nodes2 = graph.extract_random_nodes_par(size, 0xbad53ed, None);
        assert_eq!(size, nodes2.len());
    }
}

#[test]
fn test_extract_edges() {
    let edge_path = "tests/data/het_graph_edges.tsv";
    let node_path = "tests/data/het_graph_nodes.tsv";
    let graph = FromCsvBuilder::new(edge_path, "subject", "object", false, None)
    .unwrap()
    .set_weights("weight", Some(1.0))
    .load_nodes_csv(
        node_path,
        "id",
        "category",
        Some("biolink:NamedThing"),
        None,
        None,
    )
    .unwrap()
    .build()
    .unwrap();

    for i in 0..12 {
        let size = (1 << i) - 1;
        let nodes1 = graph.extract_random_edges(size, 0xbad53ed);
        assert_eq!(size, nodes1.len());
        let nodes2 = graph.extract_random_edges_par(size, 0xbad53ed, None);
        assert_eq!(size, nodes2.len());
    }
}
