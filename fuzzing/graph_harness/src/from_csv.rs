use super::*;
use arbitrary::Arbitrary;

use std::fs::File;
use std::io::prelude::*;
use std::fs::remove_file;

#[derive(Arbitrary, Debug)]
pub struct FromCsvHarnessParams {
    edge_reader: EdgeFileReaderParams,
    nodes_reader: Option<NodeFileReaderParams>,
    directed: bool,
}

#[derive(Arbitrary, Debug)]
pub struct CSVFileReaderParams {
    pub verbose: Option<bool>,
    pub separator: Option<String>,
    pub header: Option<bool>,
    pub rows_to_skip: Option<usize>,
    pub ignore_duplicates: Option<bool>,
}

#[derive(Arbitrary, Debug)]
pub struct NodeFileReaderParams {
    pub file: String,
    pub reader: CSVFileReaderParams,
    pub default_node_type: Option<String>,
    pub nodes_column_number: Option<usize>,
    pub nodes_column: Option<String>,
    pub node_types_column_number: Option<usize>,
    pub node_types_column: Option<String>,
}


#[derive(Arbitrary, Debug)]
pub struct EdgeFileReaderParams {
    pub file: String,
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
}

pub fn from_csv_harness(data: FromCsvHarnessParams) -> Result<(), String> {
    // generate random paths
    let edges_path = graph::test_utilities::random_path();
    let nodes_path = graph::test_utilities::random_path();
    // run the harness
    let result = internal_harness(&edges_path, &nodes_path, data);
    // cleanup
    let _ = remove_file(&edges_path);
    let _ = remove_file(&nodes_path);
    result
}

fn internal_harness(edges_path: &str, nodes_path: &str, data: FromCsvHarnessParams) -> Result<(), String> {
    // create the edge file
    let mut edges_file = File::create(edges_path).unwrap();
    edges_file.write_all(&data.edge_reader.file.as_bytes()).unwrap();
    
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
            let mut nodes_file = File::create(&nodes_path).unwrap();
            nodes_file.write_all(&nr.file.as_bytes()).unwrap();

            // return the reader
            Some(
                NodeFileReader::new(nodes_path.to_string())?
                    .set_verbose(Some(false))
                    .set_node_types_column_number(nr.node_types_column_number)?
                    .set_nodes_column_number(nr.node_types_column_number)?
                    .set_node_types_column(nr.node_types_column)?
                    .set_default_node_type(nr.default_node_type)
                    .set_nodes_column(nr.nodes_column)?
                    .set_ignore_duplicates(nr.reader.ignore_duplicates)
                    .set_separator(nr.reader.separator)?
                    .set_header(nr.reader.header)
                    .set_rows_to_skip(nr.reader.rows_to_skip)
            )
        }
    };

    Graph::from_unsorted_csv(edges_reader, nodes_reader, data.directed)?;
    Ok(())
}
