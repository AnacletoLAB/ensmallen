use super::*;
use arbitrary::Arbitrary;
use std::fs::remove_file;

#[derive(Arbitrary, Debug, Clone)]
pub struct FromCsvHarnessParams {
    pub directed: bool,
    pub directed_edge_list: bool,
    pub edge_reader: EdgeFileReaderParams,
    pub nodes_reader: Option<NodeFileReaderParams>,
}

#[derive(Arbitrary, Debug, Clone)]
pub struct CSVFileReaderParams {
    pub verbose: Option<bool>,
    pub separator: Option<String>,
    pub header: Option<bool>,
    pub rows_to_skip: Option<usize>,
    pub ignore_duplicates: Option<bool>,
    pub max_rows_number: Option<u16>,
}

#[derive(Arbitrary, Debug, Clone)]
pub struct NodeFileReaderParams {
    pub reader: CSVFileReaderParams,
    pub default_node_type: Option<String>,
    pub nodes_column_number: Option<usize>,
    pub node_types_separator: Option<String>,
    pub node_types_column_number: Option<usize>,
    pub numeric_node_ids: Option<bool>,
    pub numeric_node_type_ids: Option<bool>,
    pub skip_node_types_if_unavailable: Option<bool>,
    pub nodes_column: Option<String>,
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
    pub skip_selfloops: Option<bool>,
    pub numeric_edge_type_ids: Option<bool>,
    pub numeric_node_ids: Option<bool>,
    pub skip_weights_if_unavailable: Option<bool>,
    pub skip_edge_types_if_unavailable: Option<bool>,
    pub file: String,
}

pub fn from_csv_harness(data: FromCsvHarnessParams) {
    let edges_path = graph::test_utilities::random_path(None);
    let nodes_path = graph::test_utilities::random_path(None);

    let data_copy = data.clone();
    let data_copy2 = data.clone();
    std::panic::set_hook(Box::new(move |info| {
        handle_panics_from_csv(Some(info), data_copy.clone());
    }));

    let graph = load_graph(&edges_path, &nodes_path, data);

    if let Ok(mut g) = graph {
        let g_copy = g.clone();
        std::panic::set_hook(Box::new(move |info| {
            handle_panics_from_csv_once_loaded(Some(info), data_copy2.clone(), g_copy.clone());
        }));

        let _ = graph::test_utilities::default_test_suite(&mut g, Some(false));
    }

    let _ = remove_file(edges_path);
    let _ = remove_file(nodes_path);
}

fn load_graph(
    edges_path: &str,
    nodes_path: &str,
    data: FromCsvHarnessParams,
) -> Result<Graph, String> {
    // create the edge file
    std::fs::write(edges_path, data.edge_reader.file).expect("Cannot write the edges file.");

    // create the reader
    let edges_reader = EdgeFileReader::new(edges_path.to_string())?
        // Csv reader
        .set_verbose(Some(false))
        .set_separator(data.edge_reader.reader.separator)?
        .set_header(data.edge_reader.reader.header)
        .set_rows_to_skip(data.edge_reader.reader.rows_to_skip)
        .set_ignore_duplicates(data.edge_reader.reader.ignore_duplicates)
        .set_max_rows_number(data.edge_reader.reader.max_rows_number.map(|x| x as u64))
        // edge reader specific
        .set_sources_column_number(data.edge_reader.sources_column_number)?
        .set_sources_column(data.edge_reader.sources_column)?
        .set_destinations_column_number(data.edge_reader.destinations_column_number)?
        .set_destinations_column(data.edge_reader.destinations_column)?
        .set_edge_types_column_number(data.edge_reader.edge_types_column_number)?
        .set_edge_types_column(data.edge_reader.edge_types_column)?
        .set_weights_column_number(data.edge_reader.weights_column_number)?
        .set_weights_column(data.edge_reader.weights_column)?
        .set_default_weight(data.edge_reader.default_weight)
        .set_default_edge_type(data.edge_reader.default_edge_type)
        .set_skip_selfloops(data.edge_reader.skip_selfloops)
        .set_numeric_edge_type_ids(data.edge_reader.numeric_edge_type_ids)
        .set_numeric_node_ids(data.edge_reader.numeric_node_ids)
        .set_skip_weights_if_unavailable(data.edge_reader.skip_weights_if_unavailable)
        .set_skip_edge_types_if_unavailable(data.edge_reader.skip_edge_types_if_unavailable);

    let nodes_reader = match data.nodes_reader {
        None => None,
        Some(nr) => {
            // create the node file
            std::fs::write(nodes_path, nr.file).expect("Cannot write the nodes file.");

            // return the reader
            Some(
                NodeFileReader::new(nodes_path.to_string())?
                    // Csv reader
                    .set_verbose(Some(false))
                    .set_separator(nr.reader.separator)?
                    .set_header(nr.reader.header)
                    .set_rows_to_skip(nr.reader.rows_to_skip)
                    .set_ignore_duplicates(nr.reader.ignore_duplicates)
                    .set_max_rows_number(nr.reader.max_rows_number.map(|x| x as u64))
                    // node reader specific
                    .set_default_node_type(nr.default_node_type)
                    .set_nodes_column_number(nr.nodes_column_number)
                    .set_node_types_separator(nr.node_types_separator)?
                    .set_node_types_column_number(nr.node_types_column_number)
                    .set_numeric_node_ids(nr.numeric_node_ids)
                    .set_numeric_node_type_ids(nr.numeric_node_type_ids)
                    .set_skip_node_types_if_unavailable(nr.skip_node_types_if_unavailable)?
                    .set_nodes_column(nr.nodes_column)?
                    .set_node_types_column(nr.node_types_column)?,
            )
        }
    };

    let g = Graph::from_unsorted_csv(
        edges_reader,
        nodes_reader,
        data.directed,
        data.directed_edge_list,
        "Fuzz Graph",
    )?;

    Ok(g)
}
