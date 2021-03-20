use super::*;
use std::fs::File;
use std::io::prelude::*;

/// Simple macro to dump in a standard way all the pairs key:value
macro_rules! dump {
    ($file:expr, $key:expr, $val:expr) => {
        write!($file, "{},{:?}\n", $key, $val).expect("Cannot write to file.");
    };
}

/// This function takes the data used for the current fuzz case and dump it.
/// this is needed for the automatic generation of unit tests from fuzzing.
pub(crate) fn handle_panics_from_csv(info: &std::panic::PanicInfo, data: FromCsvHarnessParams) {
    // Find the root of the repository
    let mut currdir = get_path("ensmallen_graph");
    // Build the path to the folder for the tests
    currdir.push("fuzzing");
    currdir.push("unit_tests");
    // Create a random directory
    let path = graph::test_utilities::random_path(currdir.to_str());
    std::fs::create_dir_all(&path).unwrap();
    
    // Dump the informations
    dump_panic_info(format!("{}/panic.csv", path), info);
    dump_graph_metadata(format!("{}/graph_metadata.csv", path), &data);
    dump_edges(format!("{}/edges.csv", path), &data.edge_reader.file);
    dump_edges_metadata(format!("{}/edges_metadata.csv", path), &data.edge_reader);

    // If there is a node files
    if let Some(nodes_reader) = data.nodes_reader{
        dump_nodes(format!("{}/nodes.csv", path), &nodes_reader.file);
        dump_nodes_metadata(format!("{}/nodes_metadata.csv", path), &nodes_reader);
    }
}

/// This function takes the data used for the current fuzz case and dump it.
/// this is needed for the automatic generation of unit tests from fuzzing.
pub(crate) fn handle_panics_from_vec(info: &std::panic::PanicInfo, data: FromVecHarnessParams) {
    // Find the root of the repository
    let mut currdir = get_path("ensmallen_graph");
    // Build the path to the folder for the tests
    currdir.push("fuzzing");
    currdir.push("unit_tests");
    // Create a random directory
    let path = graph::test_utilities::random_path(currdir.to_str());
    std::fs::create_dir_all(&path).unwrap();
    
    // Dump the informations
    dump_panic_info(format!("{}/panic.csv", path), info);
    dump_graph_metadata(format!("{}/graph_metadata.csv", path), &data);
    dump_edges(format!("{}/edges.csv", path), &data.edge_reader.file);
    dump_edges_metadata(format!("{}/edges_metadata.csv", path), &data.edge_reader);

    // If there is a node files
    if let Some(nodes) = data.nodes{
        dump_nodes(format!("{}/nodes.csv", path), &nodes_reader.file);
        dump_nodes_metadata(format!("{}/nodes_metadata.csv", path), &nodes_reader);
    }
}

/// Return a path stopping at the first occurence of wanted_folder.
fn get_path(wanted_folder: &str) -> std::path::PathBuf {
    let curr_dir = std::env::current_dir().unwrap().canonicalize().unwrap();

    let mut new_path = std::path::PathBuf::new();

    for part in curr_dir.iter() {
        new_path.push(part);
        if part == wanted_folder{
            break
        }
    }

    new_path
}

/// Dump the informations about the panic
fn dump_panic_info(path: String, info: &std::panic::PanicInfo){
    let mut file = File::create(path).unwrap();
    if let Some(s) = info.location() {
        dump!(file, "file", s.file());
        dump!(file, "line", s.line());
        dump!(file, "col",  s.column());
    }
}

/// Dump the metadata specific for the graphs
fn dump_graph_metadata(path: String, data: &FromCsvHarnessParams){
    let mut file = File::create(path).unwrap();
    dump!(file, "directed", data.directed);
    dump!(file, "directed_edge_list", data.directed_edge_list);
    dump!(file, "name", data.name);
}

/// Dump the edges file
fn dump_edges(path: String, edges: &str){
    std::fs::write(path, edges).expect("Cannot write the edge file");
}

/// Dump the parameters used to load the edges file
fn dump_edges_metadata(path: String, data: &EdgeFileReaderParams){
    let mut file = File::create(path).unwrap();
    // Csv default
    dump!(file, "verbose", data.reader.verbose);
    dump!(file, "separator", data.reader.separator);
    dump!(file, "header", data.reader.header);
    dump!(file, "rows_to_skip", data.reader.rows_to_skip);
    dump!(file, "ignore_duplicates", data.reader.ignore_duplicates);
    dump!(file, "max_rows_number", data.reader.max_rows_number);
    // edge specific
    dump!(file, "sources_column_number", data.sources_column_number);
    dump!(file, "sources_column", data.sources_column);
    dump!(file, "destinations_column_number", data.destinations_column_number);
    dump!(file, "destinations_column", data.destinations_column);
    dump!(file, "edge_types_column_number", data.edge_types_column_number);
    dump!(file, "edge_types_column", data.edge_types_column);
    dump!(file, "weights_column_number", data.weights_column_number);
    dump!(file, "weights_column", data.weights_column);
    dump!(file, "default_weight", data.default_weight);
    dump!(file, "default_edge_type", data.sources_column_number);
    dump!(file, "skip_self_loops", data.skip_self_loops);
    dump!(file, "numeric_edge_type_ids", data.numeric_edge_type_ids);
    dump!(file, "numeric_node_ids", data.numeric_node_ids);
    dump!(file, "skip_weights_if_unavailable", data.skip_weights_if_unavailable);
    dump!(file, "skip_edge_types_if_unavailable", data.skip_edge_types_if_unavailable);
}

/// Dump the nodes file
fn dump_nodes(path: String, nodes: &str){
    std::fs::write(path, nodes).expect("Cannot write the edge file");
}

/// Dump the parameters used to load the node files
fn dump_nodes_metadata(path: String, data: &NodeFileReaderParams){
    let mut file = File::create(path).unwrap();
    // Csv default
    dump!(file, "verbose", data.reader.verbose);
    dump!(file, "separator", data.reader.separator);
    dump!(file, "header", data.reader.header);
    dump!(file, "rows_to_skip", data.reader.rows_to_skip);
    dump!(file, "ignore_duplicates", data.reader.ignore_duplicates);
    dump!(file, "max_rows_number", data.reader.max_rows_number);
    // nodes specific
    dump!(file, "default_node_type", data.default_node_type);
    dump!(file, "nodes_column_number", data.nodes_column_number);
    dump!(file, "node_types_separator", data.node_types_separator);
    dump!(file, "node_types_column", data.node_types_column);
    dump!(file, "node_types_column_number", data.node_types_column_number);
    dump!(file, "numeric_node_ids", data.numeric_node_ids);
    dump!(file, "numeric_node_type_ids", data.numeric_node_type_ids);
    dump!(file, "skip_node_types_if_unavailable", data.skip_node_types_if_unavailable);
    dump!(file, "nodes_column", data.nodes_column);
    dump!(file, "node_types_column", data.node_types_column);
}