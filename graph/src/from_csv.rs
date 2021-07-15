use crate::constructors::build_graph_from_strings;

use super::*;

impl Graph {
    #[no_binding]
    /// Return graph renderized from given file readers.
    ///
    /// # Arguments
    /// * `edge_file_reader`: Option<EdgeFileReader> - Reader of the edge file.
    /// * `node_file_reader`: Option<NodeFileReader> - Reader of the node file.
    /// * `node_type_file_reader`: Option<TypeFileReader> - Reader of the node type file.
    /// * `edge_type_file_reader`: Option<TypeFileReader> - Reader of the edge type file.
    /// * `directed`: bool - Whether the graph is to be read as directed or undirected.
    /// * `name`: S - The name for the graph.
    pub fn from_file_readers<S: Clone + Into<String>>(
        mut edge_file_reader: Option<EdgeFileReader>,
        mut node_file_reader: Option<NodeFileReader>,
        mut node_type_file_reader: Option<TypeFileReader<NodeTypeT>>,
        mut edge_type_file_reader: Option<TypeFileReader<EdgeTypeT>>,
        directed: bool,
        name: S,
    ) -> Result<Graph> {
        edge_file_reader = edge_file_reader.map(|efr| efr.set_graph_name(name.clone().into()));
        node_file_reader = node_file_reader.map(|nfr| nfr.set_graph_name(name.clone().into()));
        node_type_file_reader =
            node_type_file_reader.map(|ntfr| ntfr.set_graph_name(name.clone().into()));
        edge_type_file_reader =
            edge_type_file_reader.map(|etfr| etfr.set_graph_name(name.clone().into()));
        build_graph_from_strings(
            node_type_file_reader
                .as_ref()
                .map_or(Ok::<_, String>(None), |nt| Ok(Some(nt.read_lines()?)))?,
            node_type_file_reader
                .as_ref()
                .and_then(|ntf| ntf.types_number.clone()),
            node_type_file_reader
                .as_ref()
                .map(|ntf| ntf.numeric_type_ids.clone()),
            node_type_file_reader
                .as_ref()
                .and_then(|ntf| ntf.minimum_type_id.clone()),
            node_file_reader
                .as_ref()
                .map_or(false, |nfr| nfr.has_node_types()),
            node_file_reader
                .as_ref()
                .map_or(Ok::<_, String>(None), |nfr| {
                    Ok(nfr.read_lines().transpose()?)
                })?,
            node_file_reader
                .as_ref()
                .and_then(|ntf| ntf.nodes_number.clone()),
            node_file_reader.as_ref().map_or(false, |nfr| {
                nfr.reader
                    .as_ref()
                    .map_or(true, |reader| reader.csv_is_correct)
            }),
            node_file_reader
                .as_ref()
                .map_or(false, |nfr| nfr.numeric_node_ids),
            node_file_reader
                .as_ref()
                .map_or(false, |nfr| nfr.numeric_node_type_ids),
            node_file_reader
                .as_ref()
                .and_then(|nfr| nfr.minimum_node_id),
            edge_type_file_reader
                .as_ref()
                .map_or(Ok::<_, String>(None), |etr| Ok(Some(etr.read_lines()?)))?,
            edge_type_file_reader
                .as_ref()
                .and_then(|etr| etr.types_number.clone()),
            edge_type_file_reader
                .as_ref()
                .map(|etr| etr.numeric_type_ids.clone()),
            edge_type_file_reader
                .as_ref()
                .and_then(|etr| etr.minimum_type_id.clone()),
            edge_file_reader
                .as_ref()
                .map_or(false, |nfr| nfr.has_edge_types()),
            edge_file_reader
                .as_ref()
                .map_or(Ok::<_, String>(None), |efr| Ok(Some(efr.read_lines()?)))?,
            edge_file_reader
                .as_ref()
                .map_or(false, |efr| efr.has_edge_weights()),
            directed,
            edge_file_reader
                .as_ref()
                .map(|efr| efr.reader.csv_is_correct),
            edge_file_reader.as_ref().and_then(|efr| efr.complete),
            edge_file_reader
                .as_ref()
                .and_then(|efr| efr.reader.may_have_duplicates),
            edge_file_reader.as_ref().and_then(|efr| efr.sorted.clone()),
            edge_file_reader
                .as_ref()
                .and_then(|etr| etr.edges_number.clone()),
            edge_file_reader
                .as_ref()
                .map(|etr| etr.numeric_node_ids.clone()),
            edge_file_reader
                .as_ref()
                .map(|etr| etr.numeric_edge_type_ids.clone()),
            name.into(),
        )
    }

    /// Return graph renderized from given CSVs or TSVs-like files.
    ///
    /// TODO! Add docstrings
    /// TODO! Add parameters for node type list and edge type list
    pub fn from_csv<S: Clone + Into<String>>(
        node_path: Option<String>,
        node_list_separator: Option<String>,
        node_list_header: Option<bool>,
        node_list_rows_to_skip: Option<usize>,
        node_list_is_correct: Option<bool>,
        node_list_max_rows_number: Option<EdgeT>,
        node_list_comment_symbol: Option<String>,
        default_node_type: Option<String>,
        nodes_column_number: Option<usize>,
        nodes_column: Option<String>,
        node_types_separator: Option<String>,
        node_types_column_number: Option<usize>,
        node_types_column: Option<String>,
        nodes_number: Option<NodeT>,
        minimum_node_id: Option<NodeT>,
        numeric_node_ids: Option<bool>,
        numeric_node_type_ids: Option<bool>,
        skip_node_types_if_unavailable: Option<bool>,
        edge_path: Option<String>,
        edge_list_separator: Option<String>,
        edge_list_header: Option<bool>,
        edge_list_rows_to_skip: Option<usize>,
        sources_column_number: Option<usize>,
        sources_column: Option<String>,
        destinations_column_number: Option<usize>,
        destinations_column: Option<String>,
        edge_types_column_number: Option<usize>,
        edge_types_column: Option<String>,
        default_edge_type: Option<String>,
        weights_column_number: Option<usize>,
        weights_column: Option<String>,
        default_weight: Option<WeightT>,
        skip_selfloops: Option<bool>,
        numeric_edge_type_ids: Option<bool>,
        edge_list_numeric_node_ids: Option<bool>,
        skip_weights_if_unavailable: Option<bool>,
        skip_edge_types_if_unavailable: Option<bool>,
        edge_list_is_complete: Option<bool>,
        edge_list_may_contain_duplicates: Option<bool>,
        edge_list_is_sorted: Option<bool>,
        edge_list_is_correct: Option<bool>,
        edge_list_max_rows_number: Option<EdgeT>,
        edge_list_comment_symbol: Option<String>,
        edges_number: Option<EdgeT>,
        verbose: Option<bool>,
        directed: bool,
        name: S,
    ) -> Result<Graph> {
        let node_file_reader = if node_path.is_some() || nodes_column.is_some() {
            Some(
                NodeFileReader::new(node_path)?
                    .set_separator(node_list_separator)?
                    .set_comment_symbol(node_list_comment_symbol)?
                    .set_header(node_list_header)?
                    .set_max_rows_number(node_list_max_rows_number)?
                    .set_rows_to_skip(node_list_rows_to_skip)?
                    .set_nodes_column_number(nodes_column_number)?
                    .set_nodes_column(nodes_column)?
                    .set_minimum_node_id(minimum_node_id)
                    .set_node_types_column_number(node_types_column_number)?
                    .set_node_types_column(node_types_column)?
                    .set_node_types_separator(node_types_separator)?
                    .set_skip_node_types_if_unavailable(skip_node_types_if_unavailable)?
                    .set_default_node_type(default_node_type)
                    .set_numeric_node_ids(numeric_node_ids)
                    .set_numeric_node_type_ids(numeric_node_type_ids)?
                    .set_csv_is_correct(node_list_is_correct)?
                    .set_nodes_number(nodes_number)
                    .set_verbose(verbose)?,
            )
        } else {
            None
        };

        let edge_file_reader = edge_path.map_or(Ok::<_, String>(None), |edge_path| {
            Ok(Some(
                EdgeFileReader::new(edge_path)?
                    .set_separator(edge_list_separator)?
                    .set_comment_symbol(edge_list_comment_symbol)?
                    .set_header(edge_list_header)
                    .set_max_rows_number(edge_list_max_rows_number)
                    .set_rows_to_skip(edge_list_rows_to_skip)
                    .set_sources_column_number(sources_column_number)?
                    .set_sources_column(sources_column)?
                    .set_destinations_column_number(destinations_column_number)?
                    .set_destinations_column(destinations_column)?
                    .set_edge_types_column_number(edge_types_column_number)?
                    .set_edge_types_column(edge_types_column)?
                    .set_skip_edge_types_if_unavailable(skip_edge_types_if_unavailable)
                    .set_default_edge_type(default_edge_type)
                    .set_weights_column_number(weights_column_number)?
                    .set_weights_column(weights_column)?
                    .set_skip_weights_if_unavailable(skip_weights_if_unavailable)
                    .set_default_weight(default_weight)
                    .set_skip_selfloops(skip_selfloops)
                    .set_numeric_node_ids(edge_list_numeric_node_ids)
                    .set_numeric_edge_type_ids(numeric_edge_type_ids)
                    .set_complete(edge_list_is_complete)
                    .set_sorted(edge_list_is_sorted)
                    .set_may_have_duplicates(edge_list_may_contain_duplicates)
                    .set_csv_is_correct(edge_list_is_correct)
                    .set_edges_number(edges_number)
                    .set_verbose(verbose),
            ))
        })?;

        Graph::from_file_readers(
            edge_file_reader,
            node_file_reader,
            None,
            None,
            directed,
            name,
        )
    }
}
