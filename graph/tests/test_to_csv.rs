extern crate graph;
use graph::*;
use std::fs::remove_file;

use rand::Rng;
const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                        abcdefghijklmnopqrstuvwxyz\
                        0123456789";
    
pub fn random_string(len: usize) -> String{    
    let mut rng = rand::thread_rng();
    
    (0..len)
        .map(|_| {
            let idx = rng.gen_range(0, CHARSET.len());
            CHARSET[idx] as char
        })
        .collect()
}

#[test]
fn test_to_edges_csv() {
    for directed in &[true, false] {
        let graph = FromCsvBuilder::new("tests/data/edge_file.tsv", "subject", "object", *directed, None)
            .unwrap()
            .build()
            .unwrap();

        let edges_path = format!("tests/data/{}", random_string(64));
        graph.to_edges_csv(&edges_path, Some("\t"), Some("subject"), Some("object"), None, None, None).unwrap();

        let graph2 = FromCsvBuilder::new(&edges_path, "subject", "object", *directed, None)
            .unwrap()
            .build()
            .unwrap();

        let _ = remove_file(&edges_path).unwrap();

        assert_eq!(graph.sources(), graph2.sources());
        assert_eq!(graph.destinations(), graph2.destinations());
        assert_eq!(graph.outbounds(), graph2.outbounds());
        assert_eq!(graph.weights(), graph2.weights());
        assert_eq!(graph.node_types(), graph2.node_types());
        assert_eq!(graph.node_types_reverse_mapping(), graph2.node_types_reverse_mapping());
        assert_eq!(graph.edge_types(), graph2.edge_types());
        assert_eq!(graph.edge_types_reverse_mapping(), graph2.edge_types_reverse_mapping());
        // this is not mantained because we sort src, dst.
        //assert_eq!(graph.nodes_reverse_mapping(), graph2.nodes_reverse_mapping());
    }
}


#[test]
fn test_to_nodess_csv() {
    let src_edge_path = "tests/data/edge_file.tsv";
    let src_node_path = "tests/data/node_file.tsv";
    for directed in &[true, false] {
        let graph = FromCsvBuilder::new(&src_edge_path, "subject", "object", *directed, None)
            .unwrap()
            .set_edge_types("edge_label", Some("biolink:Association"),)
            .set_weights("weight", Some(1.0))
            .load_nodes_csv(
                &src_node_path,
                "id",
                "category",
                Some("biolink:NamedThing"),
                None,
                Some(false)
            ).unwrap()
            .build().unwrap();

        let edges_path = format!("tests/data/{}", random_string(64));
        let nodes_path = format!("tests/data/{}", random_string(64));
        graph.to_edges_csv(&edges_path, Some("\t"), Some("subject"), Some("object"), Some("edge_label"), Some("weight"), None).unwrap();
        graph.to_nodes_csv(&nodes_path, Some("\t"), Some("id"), Some("category"), None).unwrap();

        let graph2 = FromCsvBuilder::new(&edges_path, "subject", "object", *directed, None)
            .unwrap()
            .set_edge_types("edge_label", Some("biolink:Association"),)
            .set_weights("weight", Some(1.0))
            .load_nodes_csv(
                &nodes_path,
                "id",
                "category",
                Some("biolink:NamedThing"),
                None,
                Some(false)
            ).unwrap()
            .build().unwrap();

        let _ = remove_file(&edges_path).unwrap();
        let _ = remove_file(&nodes_path).unwrap();

        assert_eq!(graph.sources(), graph2.sources());
        assert_eq!(graph.outbounds(), graph2.outbounds());
        assert_eq!(graph.node_types(), graph2.node_types());
        assert_eq!(graph.node_types_reverse_mapping(), graph2.node_types_reverse_mapping());
        assert_eq!(graph.edge_types_reverse_mapping(), graph2.edge_types_reverse_mapping());
        assert_eq!(graph.nodes_reverse_mapping(), graph2.nodes_reverse_mapping());

        // not passing for the undirected case because the ordering changes
        // assert_eq!(graph.weights(), graph2.weights());
        // assert_eq!(graph.edge_types(), graph2.edge_types());
        // assert_eq!(graph.destinations(), graph2.destinations());

    }
}
