extern crate graph;
use graph::*;
use rayon::iter::ParallelIterator;

fn main() {
    let graph_name = "Cora".to_owned();
    let edges_reader = EdgeFileReader::new("../graph/tests/data/cora/edges.tsv").unwrap()
        .set_separator(Some("\t")).unwrap()
        .set_verbose(Some(false))
        .set_sources_column(Some("subject")).unwrap()
        .set_destinations_column(Some("object")).unwrap()
        .set_default_weight(Some(1.0))
        .set_edge_types_column(Some("edge_type")).unwrap();
    let nodes_reader = Some(
        NodeFileReader::new("../graph/tests/data/cora/nodes.tsv").unwrap()
            .set_separator(Some("\t")).unwrap()
            .set_nodes_column(Some("id")).unwrap()
            .set_verbose(Some(false))
            .set_node_types_column(Some("node_type")).unwrap(),
    );
    let mut cora = Graph::from_unsorted_csv(edges_reader, nodes_reader, false, false, graph_name.clone()).unwrap();

    cora.enable(true, true, true, None).unwrap();
    
    for _ in 0..1000{
        cora.approximated_vertex_cover(Some(false));
    }
}   
