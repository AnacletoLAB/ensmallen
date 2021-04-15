use super::*;
use std::fs::File;
use std::io::prelude::*;use backtrace::Backtrace;

/// Simple macro to dump in a standard way all the pairs key:value
macro_rules! dump {
    ($file:expr, $key:expr, $val:expr) => {
        write!($file, "{},{:?}\n", $key, $val).expect("Cannot write to file.");
    };
}

fn get_folder() -> String {
    // Find the root of the repository
    let mut currdir = get_path("ensmallen_graph");
    // Build the path to the folder for the tests
    currdir.push("fuzzing");
    currdir.push("unit_tests");
    // Create a random directory
    let path = graph::test_utilities::random_path(currdir.to_str());
    std::fs::create_dir_all(&path).unwrap();
    return path;
}

/// This function takes the data used for the current fuzz case and dump it.
/// this is needed for the automatic generation of unit tests from fuzzing.
pub(crate) fn handle_panics_from_csv(info: Option<&std::panic::PanicInfo>, data: FromCsvHarnessParams) {
    println!("Panic: {:?}", info);
    // Find the root of the repository
    let path = get_folder();
    dump_backtrace(&path);
    // Dump the informations
    std::fs::write(format!("{}/data.txt", &path), format!("{:#4?}", &data))
        .expect("Cannot write the edge file");
    if let Some(info) = info {
        dump_panic_info(format!("{}/panic.csv", path), info);
    }
    dump_graph_metadata(format!("{}/graph_metadata.csv", path), &data);
    dump_edges(format!("{}/edges.edges", path), &data.edge_reader.file);
    dump_edges_metadata(format!("{}/edges_metadata.csv", path), &data.edge_reader);

    // If there is a node files
    if let Some(nodes_reader) = data.nodes_reader {
        dump_nodes(format!("{}/nodes.nodes", path), &nodes_reader.file);
        dump_nodes_metadata(format!("{}/nodes_metadata.csv", path), &nodes_reader);
    }
}

/// This function takes the data used for the current fuzz case and dump it.
/// this is needed for the automatic generation of unit tests from fuzzing.
pub(crate) fn handle_panics_from_csv_once_loaded(
    info: Option<&std::panic::PanicInfo>,
    data: FromCsvHarnessParams,
    graph: Graph,
) {
    println!("Panic: {:?}", info);
    // Find the root of the repository
    let path = get_folder();
    dump_backtrace(&path);
    // Dump the informations
    std::fs::write(format!("{}/data.txt", &path), format!("{:#4?}", &data))
        .expect("Cannot write the edge file");
    if let Some(info) = info {
        dump_panic_info(format!("{}/panic.csv", path), info);
    }
    dump_graph_metadata(format!("{}/graph_metadata.csv", path), &data);
    dump_edges(format!("{}/edges.edges", path), &data.edge_reader.file);
    dump_edges_metadata(format!("{}/edges_metadata.csv", path), &data.edge_reader);

    // If there is a node files
    if let Some(nodes_reader) = data.nodes_reader {
        dump_nodes(format!("{}/nodes.nodes", path), &nodes_reader.file);
        dump_nodes_metadata(format!("{}/nodes_metadata.csv", path), &nodes_reader);
    }

    if let Ok(r) = graph.textual_report(false) {
        std::fs::write(format!("{}/report.txt", path), r).expect("Cannot write the edge file");
    }

    std::fs::write(format!("{}/debug.txt", &path), format!("{:#4?}", graph))
        .expect("Cannot write the edge file");
}

/// This function takes the data used for the current fuzz case and dump it.
/// this is needed for the automatic generation of unit tests from fuzzing.
pub(crate) fn handle_panics_from_vec(info: Option<&std::panic::PanicInfo>, data: FromVecHarnessParams, sig_num: Option<i32>) -> String {
    println!("Panic: {:?}", info);
    let path = get_folder();
    dump_backtrace(&path);
    // Dump the informations
    std::fs::write(format!("{}/data.txt", &path), format!("{:#4?}", &data))
        .expect("Cannot write the edge file");

    if let Some(sn) = sig_num {
        std::fs::write(format!("{}/signal.txt", &path), format!("Received signal {}\n", sn))
        .expect("Cannot write the signal file");
    }

    if let Some(info) = info {
        dump_panic_info(format!("{}/panic.csv", path), info);
    }
    dump_graph_metadata_from_vec(format!("{}/graph_metadata.csv", path), &data);
    dump_edges_from_vec(format!("{}/edges.edges", path), &data);
    dump_edges_metadata_from_vec(format!("{}/edges_metadata.csv", path), &data);

    // If there is a node files
    if let Some(nodes) = &data.nodes {
        dump_nodes_from_vec(format!("{}/nodes.nodes", path), &nodes);
        dump_nodes_metadata_from_vec(format!("{}/nodes_metadata.csv", path), &data);
    }

    path 
}

/// This function takes the data used for the current fuzz case and dump it.
/// this is needed for the automatic generation of unit tests from fuzzing.
pub(crate) fn handle_panics_from_vec_once_loaded(
    info: Option<&std::panic::PanicInfo>,
    data: FromVecHarnessParams,
    graph: Graph,
) -> String {
    println!("Panic: {:?}", info);
    let path = get_folder();
    dump_backtrace(&path);
    // Dump the informations
    std::fs::write(format!("{}/data.txt", &path), format!("{:#4?}", &data))
        .expect("Cannot write the edge file");
    if let Some(info) = info {
        dump_panic_info(format!("{}/panic.csv", path), info);
    }
    dump_graph_metadata_from_vec(format!("{}/graph_metadata.csv", path), &data);
    dump_edges_from_vec(format!("{}/edges.edges", path), &data);
    dump_edges_metadata_from_vec(format!("{}/edges_metadata.csv", path), &data);

    // If there is a node files
    if let Some(nodes) = &data.nodes {
        dump_nodes_from_vec(format!("{}/nodes.nodes", path), &nodes);
        dump_nodes_metadata_from_vec(format!("{}/nodes_metadata.csv", path), &data);
    }

    if let Ok(r) = graph.textual_report(false) {
        std::fs::write(format!("{}/report.txt", path), r).expect("Cannot write the edge file");
    }

    std::fs::write(format!("{}/debug.txt", &path), format!("{:#4?}", graph))
        .expect("Cannot write the edge file");

    path
}

/// This function takes the data used for the current fuzz case and dump it.
/// this is needed for the automatic generation of unit tests from fuzzing.
pub(crate) fn handle_panics_mega_test(info: Option<&std::panic::PanicInfo>, data: TheUltimateFuzzer, sig_num: Option<i32>) {
    let path = handle_panics_from_vec(info, data.from_vec.clone(), None);
    dump_backtrace(&path);
    
    std::fs::write(format!("{}/mega_test.txt", &path), format!("{:#4?}", &data))
        .expect("Cannot write the megatest file");
}

/// This function takes the data used for the current fuzz case and dump it.
/// this is needed for the automatic generation of unit tests from fuzzing.
pub(crate) fn handle_panics_mega_test_once_loaded(
    info: Option<&std::panic::PanicInfo>,
    data: TheUltimateFuzzer,
    graph: Graph,
) {    
    let path = handle_panics_from_vec_once_loaded(info, data.from_vec.clone(), graph);
    dump_backtrace(&path);

    std::fs::write(format!("{}/mega_test.txt", &path), format!("{:#4?}", &data))
        .expect("Cannot write the megatest file");
}

/// Return a path stopping at the first occurence of wanted_folder.
fn get_path(wanted_folder: &str) -> std::path::PathBuf {
    let curr_dir = std::env::current_dir().unwrap().canonicalize().unwrap();

    let mut new_path = std::path::PathBuf::new();

    for part in curr_dir.iter() {
        new_path.push(part);
        if part == wanted_folder {
            break;
        }
    }

    new_path
}

fn dump_backtrace(&path: &str) {
    let current_backtrace = Backtrace::new();
    std::fs::write(format!("{}/backtrace.txt", &path), format!("{:#4?}", &current_backtrace))
        .expect("Cannot write the backtrace file");
}

fn dump_graph_metadata_from_vec(path: String, data: &FromVecHarnessParams) {
    let mut file = File::create(path).unwrap();
    dump!(file, "directed", data.directed);
    dump!(file, "directed_edge_list", data.directed_edge_list);
}

fn dump_nodes_from_vec(path: String, nodes: &Vec<Result<(String, Option<Vec<String>>), String>>) {
    let mut file = File::create(path).unwrap();

    for node_and_type in nodes {
        if let Ok((node, node_type)) = &node_and_type {
            write!(
                file,
                "{},{}\n",
                node,
                node_type.clone().map_or("".to_string(), |x| x.join("|")),
            )
            .expect("Cannot write to file.");
        }
    }
}

fn dump_nodes_metadata_from_vec(path: String, data: &FromVecHarnessParams) {
    let mut file = File::create(path).unwrap();
    dump!(file, "verbose", Some(false));
    dump!(file, "separator", Some(","));
    dump!(file, "header", Some(false));
    dump!(file, "rows_to_skip", Some(0));
    dump!(
        file,
        "ignore_duplicates",
        Some(data.ignore_duplicated_nodes)
    );
    dump!(file, "max_rows_number", None::<u64>);
    // nodes specific
    dump!(file, "default_node_type", None::<u64>);
    dump!(file, "node_types_separator", Some("|"));

    dump!(file, "numeric_node_ids", Some(data.numeric_node_ids));
    dump!(
        file,
        "numeric_node_type_ids",
        Some(data.numeric_node_types_ids)
    );
    dump!(file, "skip_node_types_if_unavailable", Some(false));
    dump!(file, "nodes_column", None::<u64>);
    dump!(file, "nodes_column_number", Some(0));

    dump!(file, "node_types_column", None::<u64>);
    dump!(
        file,
        "node_types_column_number",
        if data.has_node_types { Some(1) } else { None }
    );
}

fn dump_edges_from_vec(path: String, data: &FromVecHarnessParams) {
    let mut file = File::create(&path).unwrap();
    for edge in &data.edges {
        if let Ok((src, dst, edge_type, weight)) = &edge {
            write!(
                file,
                "{},{},{},{}\n",
                src,
                dst,
                edge_type.clone().unwrap_or("".to_string()),
                weight.map_or("".to_string(), |x| x.to_string())
            )
            .expect("Cannot write to file.");
        }
    }
}

fn dump_edges_metadata_from_vec(path: String, data: &FromVecHarnessParams) {
    let mut file = File::create(path).unwrap();
    dump!(file, "verbose", Some(false));
    dump!(file, "separator", Some(","));
    dump!(file, "header", Some(false));
    dump!(file, "rows_to_skip", Some(0));
    dump!(
        file,
        "ignore_duplicates",
        Some(data.ignore_duplicated_edges)
    );
    dump!(file, "max_rows_number", None::<u64>);
    // edge specific
    dump!(file, "sources_column_number", Some(0));
    dump!(file, "sources_column", None::<u64>);
    dump!(file, "destinations_column_number", Some(1));
    dump!(file, "destinations_column", None::<u64>);

    dump!(
        file,
        "edge_types_column_number",
        if data.has_edge_types { Some(2) } else { None }
    );

    dump!(file, "edge_types_column", None::<u64>);

    dump!(
        file,
        "weights_column_number",
        if data.has_weights { Some(3) } else { None }
    );

    dump!(file, "weights_column", None::<u64>);
    dump!(file, "default_weight", None::<u64>);
    dump!(file, "default_edge_type", None::<u64>);
    dump!(file, "skip_self_loops", Some(false));
    dump!(
        file,
        "numeric_edge_type_ids",
        Some(data.numeric_edge_types_ids)
    );
    dump!(file, "numeric_node_ids", Some(data.numeric_node_ids));
    dump!(file, "skip_weights_if_unavailable", Some(false));
    dump!(file, "skip_edge_types_if_unavailable", Some(false));
}

/// Dump the informations about the panic
fn dump_panic_info(path: String, info: &std::panic::PanicInfo) {
    let mut file = File::create(path).unwrap();
    if let Some(s) = info.location() {
        dump!(file, "file", s.file());
        dump!(file, "line", s.line());
        dump!(file, "col", s.column());
    }

    if let Some(s) = info.message() {
        dump!(file, "message", s);
    }
}

/// Dump the metadata specific for the graphs
fn dump_graph_metadata(path: String, data: &FromCsvHarnessParams) {
    let mut file = File::create(path).unwrap();
    dump!(file, "directed", data.directed);
    dump!(file, "directed_edge_list", data.directed_edge_list);
}

/// Dump the edges file
fn dump_edges(path: String, edges: &str) {
    std::fs::write(path, edges).expect("Cannot write the edge file");
}

/// Dump the parameters used to load the edges file
fn dump_edges_metadata(path: String, data: &EdgeFileReaderParams) {
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
    dump!(
        file,
        "destinations_column_number",
        data.destinations_column_number
    );
    dump!(file, "destinations_column", data.destinations_column);
    dump!(
        file,
        "edge_types_column_number",
        data.edge_types_column_number
    );
    dump!(file, "edge_types_column", data.edge_types_column);
    dump!(file, "weights_column_number", data.weights_column_number);
    dump!(file, "weights_column", data.weights_column);
    dump!(file, "default_weight", data.default_weight);
    dump!(file, "default_edge_type", data.sources_column_number);
    dump!(file, "skip_self_loops", data.skip_self_loops);
    dump!(file, "numeric_edge_type_ids", data.numeric_edge_type_ids);
    dump!(file, "numeric_node_ids", data.numeric_node_ids);
    dump!(
        file,
        "skip_weights_if_unavailable",
        data.skip_weights_if_unavailable
    );
    dump!(
        file,
        "skip_edge_types_if_unavailable",
        data.skip_edge_types_if_unavailable
    );
}

/// Dump the nodes file
fn dump_nodes(path: String, nodes: &str) {
    std::fs::write(path, nodes).expect("Cannot write the edge file");
}

/// Dump the parameters used to load the node files
fn dump_nodes_metadata(path: String, data: &NodeFileReaderParams) {
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
    dump!(
        file,
        "node_types_column_number",
        data.node_types_column_number
    );
    dump!(file, "numeric_node_ids", data.numeric_node_ids);
    dump!(file, "numeric_node_type_ids", data.numeric_node_type_ids);
    dump!(
        file,
        "skip_node_types_if_unavailable",
        data.skip_node_types_if_unavailable
    );
    dump!(file, "nodes_column", data.nodes_column);
    dump!(file, "node_types_column", data.node_types_column);
}
