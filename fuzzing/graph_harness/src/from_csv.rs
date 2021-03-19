use super::*;
use arbitrary::Arbitrary;

use std::fs::File;
use std::io::prelude::*;
use std::fs::remove_file;
use std::ffi::OsStr;

#[derive(Arbitrary, Debug, Clone)]
pub struct FromCsvHarnessParams {
    edge_reader: EdgeFileReaderParams,
    nodes_reader: Option<NodeFileReaderParams>,
    directed: bool,
}

#[derive(Arbitrary, Debug, Clone)]
pub struct CSVFileReaderParams {
    pub verbose: Option<bool>,
    pub separator: Option<String>,
    pub header: Option<bool>,
    pub rows_to_skip: Option<usize>,
    pub ignore_duplicates: Option<bool>,
}

#[derive(Arbitrary, Debug, Clone)]
pub struct NodeFileReaderParams {
    pub reader: CSVFileReaderParams,
    pub default_node_type: Option<String>,
    pub nodes_column_number: Option<usize>,
    pub nodes_column: Option<String>,
    pub node_types_column_number: Option<usize>,
    pub node_types_column: Option<String>,
    pub file: String,
}


#[derive(Arbitrary, Debug, Clone)]
pub struct EdgeFileReaderParams {
    pub reader: CSVFileReaderParams,
    pub sources_column_number: Option<usize>,
    pub sources_column: Option<String>,
    pub destinations_column_number: Option<usize>,
    pub destinations_column: Option<String>,
    pub edge_types_column_number: Option<usize>,
    pub edge_types_column: Option<String>,
    pub weights_column_number: Option<usize>,
    pub weights_column: Option<String>,
    pub default_weight: Option<WeightT>,
    pub default_edge_type: Option<String>,
    pub skip_self_loops: Option<bool>,
    pub file: String,
}


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

fn handle_panics(data: FromCsvHarnessParams) {
    let mut currdir = get_path("ensmallen_graph");
    currdir.push("fuzzing");
    currdir.push("unit_tests");

    let path = graph::test_utilities::random_path(currdir.to_str());
    std::fs::create_dir_all(&path).unwrap();

    let edge_path = format!("{}/edges.csv", path);
    std::fs::write(edge_path, data.edge_reader.file);

    let edge_metadata_path = format!("{}/edges_metadata.csv", path);
    let mut file = File::create(edge_metadata_path).unwrap();
    write!(file, "{},{:?}\n", "directed", data.directed);
    write!(file, "{},{:?}\n", "sources_column_number", data.edge_reader.sources_column_number);
    write!(file, "{},{:?}\n", "sources_column", data.edge_reader.sources_column);
    write!(file, "{},{:?}\n", "destinations_column_number", data.edge_reader.destinations_column_number);
    write!(file, "{},{:?}\n", "destinations_column", data.edge_reader.destinations_column);
    write!(file, "{},{:?}\n", "edge_types_column_number", data.edge_reader.edge_types_column_number);
    write!(file, "{},{:?}\n", "edge_types_column", data.edge_reader.edge_types_column);
    write!(file, "{},{:?}\n", "weights_column_number", data.edge_reader.weights_column_number);
    write!(file, "{},{:?}\n", "weights_column", data.edge_reader.weights_column);
    write!(file, "{},{:?}\n", "default_weight", data.edge_reader.default_weight);
    write!(file, "{},{:?}\n", "default_edge_type", data.edge_reader.sources_column_number);
    write!(file, "{},{:?}\n", "skip_self_loops", data.edge_reader.skip_self_loops);
    write!(file, "{},{:?}\n", "verbose", data.edge_reader.reader.verbose);
    write!(file, "{},{:?}\n", "separator", data.edge_reader.reader.separator);
    write!(file, "{},{:?}\n", "header", data.edge_reader.reader.header);
    write!(file, "{},{:?}\n", "rows_to_skip", data.edge_reader.reader.rows_to_skip);
    write!(file, "{},{:?}\n", "ignore_duplicates", data.edge_reader.reader.ignore_duplicates);
    
    if let Some(nodes_reader) = data.nodes_reader{
        let node_path = format!("{}/nodes.csv", path);
        std::fs::write(node_path, nodes_reader.file);

        let node_metadata_path = format!("{}/nodes_metadata.csv", path);
        let mut file = File::create(node_metadata_path).unwrap();
        write!(file, "{},{:?}\n", "default_node_type", nodes_reader.default_node_type);
        write!(file, "{},{:?}\n", "nodes_column_number", nodes_reader.nodes_column_number);
        write!(file, "{},{:?}\n", "nodes_column", nodes_reader.nodes_column);
        write!(file, "{},{:?}\n", "node_types_column_number", nodes_reader.node_types_column_number);
        write!(file, "{},{:?}\n", "node_types_column", nodes_reader.node_types_column);
        write!(file, "{},{:?}\n", "verbose", nodes_reader.reader.verbose);
        write!(file, "{},{:?}\n", "separator", nodes_reader.reader.separator);
        write!(file, "{},{:?}\n", "header", nodes_reader.reader.header);
        write!(file, "{},{:?}\n", "rows_to_skip", nodes_reader.reader.rows_to_skip);
        write!(file, "{},{:?}\n", "ignore_duplicates", nodes_reader.reader.ignore_duplicates);
    }
}

pub fn from_csv_harness(data: FromCsvHarnessParams) -> Result<(), String> {
    let edges_path = graph::test_utilities::random_path(None);
    let nodes_path = graph::test_utilities::random_path(None);

    let data_copy = data.clone();
    std::panic::set_hook(Box::new(move |_| {
        handle_panics(data_copy.clone());
    }));

    internal_harness(&edges_path, &nodes_path, data)
}

fn internal_harness(edges_path: &str, nodes_path: &str, data: FromCsvHarnessParams) -> Result<(), String> {
    // create the edge file
    std::fs::write(edges_path, data.edge_reader.file);
    
    // create the reader
    let edges_reader = EdgeFileReader::new(edges_path.to_string())?
        .set_verbose(Some(false))
        .set_ignore_duplicates(data.edge_reader.reader.ignore_duplicates)
        .set_separator(data.edge_reader.reader.separator)?
        .set_header(data.edge_reader.reader.header)
        .set_rows_to_skip(data.edge_reader.reader.rows_to_skip)
        .set_sources_column_number(data.edge_reader.sources_column_number)?
        .set_sources_column(data.edge_reader.sources_column)?
        .set_destinations_column_number(data.edge_reader.destinations_column_number)?
        .set_destinations_column(data.edge_reader.destinations_column)?
        .set_weights_column_number(data.edge_reader.weights_column_number)?
        .set_weights_column(data.edge_reader.weights_column)?
        .set_edge_types_column_number(data.edge_reader.edge_types_column_number)?
        .set_edge_types_column(data.edge_reader.edge_types_column)?
        .set_default_edge_type(data.edge_reader.default_edge_type)
        .set_default_weight(data.edge_reader.default_weight)
        .set_skip_self_loops(data.edge_reader.skip_self_loops);

    let nodes_reader = match data.nodes_reader {
        None => None,
        Some(nr) => {

            // create the node file
            std::fs::write(nodes_path, nr.file);

            // return the reader
            Some(
                NodeFileReader::new(nodes_path.to_string())?
                    .set_verbose(Some(false))
                    .set_separator(nr.reader.separator)?
                    .set_node_types_column_number(nr.node_types_column_number)
                    .set_nodes_column_number(nr.node_types_column_number)
                    .set_node_types_column(nr.node_types_column)?
                    .set_default_node_type(nr.default_node_type)
                    .set_nodes_column(nr.nodes_column)?
                    .set_ignore_duplicates(nr.reader.ignore_duplicates)
                    .set_header(nr.reader.header)
                    .set_rows_to_skip(nr.reader.rows_to_skip)
            )
        }
    };

    let mut g = Graph::from_unsorted_csv(edges_reader, nodes_reader, data.directed, false, "Graph".to_owned())?;
    let _ = graph::test_utilities::default_test_suite(&mut g, false);

    Ok(())
}
