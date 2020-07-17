extern crate graph;
use graph::*;

#[test]
fn test_load_ppi() {
    let edge_path = "tests/data/ppi/edges.tsv";
    let node_path = "tests/data/ppi/nodes.tsv";
    for directed in &[true, false] {
        let _graph = FromCsvBuilder::new(edge_path, "subject", "object", *directed, None)
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
    }
}
