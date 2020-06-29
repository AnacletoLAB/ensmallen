extern crate graph;
use graph::graph::Graph;
use std::fs::File;
use linecount::count_lines;

#[test]
fn test_graph_from_csv_edge_only() {
    let path = "tests/data/edge_file.tsv";
    for directed in &[true, false]{
        let graph = Graph::from_csv(
            &path,
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
            None
        ).unwrap();
        assert_eq!(graph.get_edge_types_number(), 0);
        assert_eq!(graph.get_node_types_number(), 0);
        let lines: usize = count_lines(File::open(path).unwrap()).unwrap();
        if *directed{
            assert_eq!(lines, graph.get_edges_number());
        }
        assert!(graph.get_node_type_id(0).is_err());
        let _cooccurrence = graph.cooccurence_matrix(10, None, None, None, Some(0.5), Some(2.0), Some(3.0), Some(4.0), Some(false));
        let _skipgrams = graph.skipgrams(0, 128, 80, None, None, None, None, None, None, None, None, None);
    }
}

#[test]
fn test_graph_from_csv_edge_types() {
    let path = "tests/data/edge_file.tsv";
    for directed in &[true, false]{
        let graph = Graph::from_csv(
            &path,
            "subject",
            "object",
            *directed,
            Some("edge_label"),
            Some("biolink:Association"),
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
        let lines: usize = count_lines(File::open(path).unwrap()).unwrap();
        if *directed{
            assert_eq!(lines, graph.get_edges_number());
        }
        assert_eq!(graph.get_edge_types_number(), 3);
        assert_eq!(graph.get_node_types_number(), 0);
        graph.walk(10, Some(10), None, None, Some(0), Some(0.5), Some(2.0), Some(3.0), Some(4.0), Some(true)).unwrap();
    }
}


#[test]
#[should_panic]
fn test_walk_wrong_return_weights_parameter() {
    let path = "tests/data/edge_file.tsv";
    Graph::from_csv(
        &path,
        "subject",
        "object",
        true,
        Some("edge_label"),
        Some("biolink:Association"),
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
    ).unwrap().walk(10, None, None, None, None, Some(0.0), Some(2.0), Some(3.0), Some(4.0), None).unwrap();
}

#[test]
#[should_panic]
fn test_walk_wrong_explore_weight_parameter() {
    let path = "tests/data/edge_file.tsv";
    Graph::from_csv(
        &path,
        "subject",
        "object",
        true,
        Some("edge_label"),
        Some("biolink:Association"),
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
    ).unwrap()
    .walk(10, None, None, None, None, Some(1.0), Some(0.0), Some(3.0), Some(4.0), None).unwrap();
}

#[test]
#[should_panic]
fn test_walk_wrong_change_node_type_weight_parameter() {
    let path = "tests/data/edge_file.tsv";
    Graph::from_csv(
        &path,
        "subject",
        "object",
        true,
        Some("edge_label"),
        Some("biolink:Association"),
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
    ).unwrap()
    .walk(10, None, None, None, None, Some(1.0), Some(1.0), Some(0.0), Some(4.0), None).unwrap();
}

#[test]
#[should_panic]
fn test_walk_wrong_change_edge_type_weight_parameter() {
    let path = "tests/data/edge_file.tsv";
    Graph::from_csv(
        &path,
        "subject",
        "object",
        true,
        Some("edge_label"),
        Some("biolink:Association"),
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
    ).unwrap()
    .walk(10, None, None, None, None, Some(1.0), Some(1.0), Some(1.0), Some(0.0), None).unwrap();
}

#[test]
#[should_panic]
fn test_graph_from_csv_weights_panic() {
    let path = "tests/data/zero_weights.tsv";
    Graph::from_csv(
        path,
        "subject",
        "object",
        true,
        Some("edge_label"),
        Some("biolink:Association"),
        Some("weight"),
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
}

#[test]
#[should_panic]
fn test_graph_from_csv_duplicated_edges_panic() {
    let path = "tests/data/duplicated_edge.tsv";
    Graph::from_csv(
        path,
        "subject",
        "object",
        true,
        Some("edge_label"),
        Some("biolink:Association"),
        Some("weight"),
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
}

#[test]
fn test_graph_from_csv_duplicated_edges_not_panic() {
    let path = "tests/data/duplicated_edge.tsv";
    Graph::from_csv(
        path,
        "subject",
        "object",
        true,
        Some("edge_label"),
        Some("biolink:Association"),
        Some("weight"),
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        Some(true),
        None
    ).unwrap();
}

#[test]
#[should_panic]
fn test_graph_from_csv_duplicated_edges_without_label_panic() {
    let path = "tests/data/duplicated_edge.tsv";
    Graph::from_csv(
        path,
        "subject",
        "object",
        true,
        None,
        Some("weight"),
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
}

#[test]
#[should_panic]
fn test_graph_from_csv_duplicated_nodes_panic() {
    let edge_path = "tests/data/edge_file.tsv";
    let node_path = "tests/data/duplicated_node.tsv";
    Graph::from_csv(
        edge_path,
        "subject",
        "object",
        true,
        Some("edge_label"),
        Some("biolink:Association"),
        Some("weight"),
        Some(1.0),
        Some(node_path),
        Some("id"),
        Some("category"),
        Some("biolink:NamedThing"),
        None,
        None,
        None,
        None,
        None
    ).unwrap();
}

#[test]
#[should_panic]
fn test_graph_from_csv_no_nodes_column_panic() {
    let edge_path = "tests/data/edge_file.tsv";
    let node_path = "tests/data/node_file.tsv";
    Graph::from_csv(
        edge_path,
        "subject",
        "object",
        true,
        Some("edge_label"),
        Some("biolink:Association"),
        Some("weight"),
        Some(1.0),
        Some(node_path),
        None,
        Some("category"),
        Some("biolink:NamedThing"),
        None,
        None,
        None,
        None,
        None
    ).unwrap();
}


#[test]
#[should_panic]
fn test_graph_from_csv_no_node_types_column_panic() {
    let edge_path = "tests/data/edge_file.tsv";
    let node_path = "tests/data/node_file.tsv";
    Graph::from_csv(
        edge_path,
        "subject",
        "object",
        true,
        Some("edge_label"),
        Some("biolink:Association"),
        Some("weight"),
        Some(1.0),
        Some(node_path),
        Some("id"),
        None,
        Some("biolink:NamedThing"),
        None,
        None,
        None,
        None,
        None
    ).unwrap();
}


#[test]
#[should_panic]
fn test_graph_from_csv_no_default_node_types_panic() {
    let edge_path = "tests/data/edge_file.tsv";
    let node_path = "tests/data/node_file.tsv";
    Graph::from_csv(
        edge_path,
        "subject",
        "object",
        true,
        Some("edge_label"),
        Some("biolink:Association"),
        Some("weight"),
        Some(1.0),
        Some(node_path),
        Some("id"),
        Some("category"),
        None,
        None,
        None,
        None,
        None,
        None
    ).unwrap();
}



#[test]
#[should_panic]
fn test_graph_from_csv_weird_edge_nodes() {
    let edge_path = "tests/data/edge_file_with_weird_nodes.tsv";
    let node_path = "tests/data/node_file.tsv";
    Graph::from_csv(
        edge_path,
        "subject",
        "object",
        true,
        Some("edge_label"),
        Some("biolink:Association"),
        Some("weight"),
        Some(1.0),
        Some(node_path),
        Some("id"),
        Some("category"),
        Some("biolink:NamedThing"),
        None,
        None,
        None,
        None,
        None
    ).unwrap();
}

#[test]
fn test_graph_from_csv_with_edge_and_nodes() {
    let edge_path = "tests/data/edge_file.tsv";
    let node_path = "tests/data/node_file.tsv";
    for directed in &[true, false]{
        let graph = Graph::from_csv(
            edge_path,
            "subject",
            "object",
            *directed,
            Some("edge_label"),
            Some("biolink:Association"),
            Some("weight"),
            Some(1.0),
            Some(node_path),
            Some("id"),
            Some("category"),
            Some("biolink:NamedThing"),
            None,
            None,
            None,
            None,
            None
        ).unwrap();
        let edge_lines: usize = count_lines(File::open(edge_path).unwrap()).unwrap();
        if *directed{
            assert_eq!(edge_lines, graph.get_edges_number());
        }
        assert_eq!(graph.get_edge_types_number(), 3);
        graph.walk(10, Some(10), None, None, Some(0), Some(0.5), Some(2.0), Some(3.0), Some(4.0), Some(false)).unwrap();
    };
}

#[test]
fn test_graph_negative_edge_weights() {
    let edge_path = "tests/data/negative_edge_weights.tsv";
    for directed in &[true, false]{
        let graph = Graph::from_csv(
            edge_path,
            "subject",
            "object",
            *directed,
            Some("edge_label"),
            Some("biolink:Association"),
            Some("weight"),
            Some(1.0),
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None
        );
        assert!(graph.is_err());
        println!("{:?}", graph);
    };
}

#[test]
fn test_graph_invalid_edge_weights() {
    let edge_path = "tests/data/invalid_edge_weights.tsv";
    for directed in &[true, false]{
        let graph = Graph::from_csv(
            edge_path,
            "subject",
            "object",
            *directed,
            Some("edge_label"),
            Some("biolink:Association"),
            Some("weight"),
            Some(1.0),
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None
        );
        assert!(graph.is_err());
        println!("{:?}", graph);
    };
}

#[test]
fn test_graph_nan_edge_weights() {
    let edge_path = "tests/data/nan_edge_weights.tsv";
    for directed in &[true, false]{
        let graph = Graph::from_csv(
            edge_path,
            "subject",
            "object",
            *directed,
            Some("edge_label"),
            Some("biolink:Association"),
            Some("weight"),
            Some(1.0),
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None
        );
        assert!(graph.is_err());
        println!("{:?}", graph);
    };
}

#[test]
fn test_graph_inf_edge_weights() {
    let edge_path = "tests/data/infinite_edge_weights.tsv";
    for directed in &[true, false]{
        let graph :Result<Graph, String> = Graph::from_csv(
            edge_path,
            "subject",
            "object",
            *directed,
            Some("edge_label"),
            Some("biolink:Association"),
            Some("weight"),
            Some(1.0),
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None
        );
        assert!(graph.is_err());
        println!("{:?}", graph);
    };
}

#[test]
fn test_graph_from_csv_with_edge_and_nodes_types() {
    let edge_path = "tests/data/edge_file.tsv";
    let node_path = "tests/data/node_file.tsv";
    for directed in &[true, false]{
        let graph = Graph::from_csv(
            edge_path,
            "subject",
            "object",
            *directed,
            Some("edge_label"),
            Some("biolink:Association"),
            Some("weight"),
            Some(1.0),
            Some(node_path),
            Some("id"),
            Some("category"),
            Some("biolink:NamedThing"),
            None,
            None,
            None,
            None,
            None
        ).unwrap();
        let edge_lines: usize = count_lines(File::open(edge_path).unwrap()).unwrap();
        if *directed{
            assert_eq!(edge_lines, graph.get_edges_number());
        }
        assert_eq!(graph.get_edge_types_number(), 3);
        graph.walk(10, Some(10), None, None, Some(0), Some(0.5), Some(2.0), Some(3.0), Some(4.0), Some(false)).unwrap();
        for one in 0..graph.get_nodes_number(){
            graph.get_node_type_id(one).unwrap();
            for two in 0..graph.get_nodes_number(){
                if graph.has_edge(one, two){
                    graph.get_edge_type_id(
                        graph.get_edge_id(one, two).unwrap()
                    ).unwrap();    
                } else {
                    assert!(graph.get_edge_id(one, two).is_err());
                }
            }
        }
        assert!(graph.get_edge_type_id(100000000000).is_err());
    };
}

#[test]
fn test_graph_from_csv_het() {
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
            None,
            None
        ).unwrap();
        let edge_lines: usize = count_lines(File::open(edge_path).unwrap()).unwrap();
        if *directed{
            assert_eq!(edge_lines, graph.get_edges_number());
        }
        assert_eq!(4, graph.get_node_types_number());
        graph.walk(10, Some(10), None, None, Some(0), Some(0.5), Some(2.0), Some(3.0), Some(4.0), Some(false)).unwrap();
        assert!(graph.get_node_type_id(100000000000).is_err());
        assert!(graph.get_edge_type_id(100000000000).is_err());
        let _ = graph.report();
    };
}

