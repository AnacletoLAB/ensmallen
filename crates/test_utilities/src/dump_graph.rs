use graph::{Graph, DumpGraph};
use readers_and_writers::{NodeFileWriter, EdgeFileWriter}; 
use crate::utils::*;
use shared::types::*;
use std::fs;

pub fn test_dump_graph(graph: &mut Graph, verbose: Option<bool>) -> Result<()> {
    let node_file = random_path(None);
    let nodes_writer = NodeFileWriter::new(node_file.clone())
        .set_verbose(verbose)
        .set_separator(Some("\t".to_string()))?
        .set_header(Some(true))
        .set_node_types_column_number(Some(4))
        .set_nodes_column_number(Some(6))
        .set_node_types_column(Some("node_types"))
        .set_nodes_column(Some("node_column".to_string()));
    nodes_writer.dump_graph(&graph)?;
    fs::remove_file(node_file).unwrap();

    let edges_file = random_path(None);
    let edges_writer = EdgeFileWriter::new(edges_file.clone())
        .set_verbose(verbose)
        .set_separator(Some("\t".to_string()))?
        .set_header(Some(true))
        .set_edge_types_column(Some("edge_types".to_owned()))
        .set_destinations_column_number(Some(3))
        .set_weights_column(Some("weight".to_string()))
        .set_weights_column_number(Some(2))
        .set_sources_column(Some("The land of sushi".to_string()))
        .set_sources_column_number(Some(0))
        .set_destinations_column(Some("The land of pizza".to_string()))
        .set_destinations_column_number(Some(1));

    edges_writer.dump_graph(&graph)?;
    fs::remove_file(edges_file).unwrap();

    Ok(())
}