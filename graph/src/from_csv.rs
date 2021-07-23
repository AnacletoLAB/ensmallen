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
    /// * `may_have_singletons`: bool - Whether the graph may contain singletons.
    /// * `may_have_singleton_with_selfloops`: bool - Whether the graph may contain singleton with selfloops.
    /// * `name`: S - The name for the graph.
    pub fn from_file_readers<S: Clone + Into<String>>(
        mut edge_file_reader: Option<EdgeFileReader>,
        mut node_file_reader: Option<NodeFileReader>,
        mut node_type_file_reader: Option<TypeFileReader<NodeTypeT>>,
        mut edge_type_file_reader: Option<TypeFileReader<EdgeTypeT>>,
        may_have_singletons: bool,
        may_have_singleton_with_selfloops: bool,
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
                .map_or(Ok::<_, String>(None), |nfr| {
                    Ok(nfr.read_lines().transpose()?)
                })?,
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
                .map_or(Ok::<_, String>(None), |etfr| {
                    Ok(etfr.read_lines().transpose()?)
                })?,
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
            may_have_singletons,
            may_have_singleton_with_selfloops,
            name.into(),
        )
    }

    /// Return graph renderized from given CSVs or TSVs-like files.
    ///
    /// # Arguments
    /// * `node_type_path: Option<String> - The path to the file with the unique node type names.
    /// * `node_type_list_separator: Option<String> - The separator to use for the node types file. Note that if this is not provided, one will be automatically detected among the following: comma, semi-column, tab and space.
    /// * `node_types_column_number: Option<usize> - The number of the column of the node types file from where to load the node types.
    /// * `node_types_column: Option<String> - The name of the column of the node types file from where to load the node types.
    /// * `node_types_number: Option<NodeTypeT> - The number of the unique node types. This will be used in order to allocate the correct size for the data structure.
    /// * `numeric_node_type_ids: Option<bool> - Whether the node type names should be loaded as numeric values, i.e. casted from string to a numeric representation.
    /// * `minimum_node_type_id: Option<NodeTypeT> - The minimum node type ID to be used when using numeric node type IDs.
    /// * `node_type_list_header: Option<bool> - Whether the node type file has an header.
    /// * `node_type_list_rows_to_skip: Option<usize> - The number of lines to skip in the node types file: the header is already skipped if it has been specified that the file has an header.
    /// * `node_type_list_is_correct: Option<bool> - Whether the node types file can be assumed to be correct, i.e. does not have something wrong in it. If this parameter is passed as true on a malformed file, the constructor will crash.
    /// * `node_type_list_max_rows_number: Option<usize> - The maximum number of lines to be loaded from the node types file.
    /// * `node_type_list_comment_symbol: Option<String> - The comment symbol to skip lines in the node types file. Lines starting with this symbol will be skipped.
    /// * `load_node_type_list_in_parallel: Option<bool> - Whether to load the node type list in parallel. Note that when loading in parallel, the internal order of the node type IDs may result changed across different iterations. We are working to get this to be stable.
    /// * `node_path: Option<String> - The path to the file with the unique node names.
    /// * `node_list_separator: Option<String> - The separator to use for the nodes file. Note that if this is not provided, one will be automatically detected among the following: comma, semi-column, tab and space.
    /// * `node_list_header: Option<bool> - Whether the nodes file has an header.
    /// * `node_list_rows_to_skip: Option<usize>,
    /// * `node_list_is_correct: Option<bool>,
    /// * `node_list_max_rows_number: Option<usize>,
    /// * `node_list_comment_symbol: Option<String>,
    /// * `default_node_type: Option<String>,
    /// * `nodes_column_number: Option<usize>,
    /// * `nodes_column: Option<String>,
    /// * `node_types_separator: Option<String>,
    /// * `node_list_node_types_column_number: Option<usize>,
    /// * `node_list_node_types_column: Option<String>,
    /// * `nodes_number: Option<NodeT>,
    /// * `minimum_node_id: Option<NodeT>,
    /// * `numeric_node_ids: Option<bool>,
    /// * `node_list_numeric_node_type_ids: Option<bool>,
    /// * `skip_node_types_if_unavailable: Option<bool>,
    /// * `load_node_list_in_parallel: Option<bool>,
    /// * `edge_type_path: Option<String>,
    /// * `edge_types_column_number: Option<usize>,
    /// * `edge_types_column: Option<String>,
    /// * `edge_types_number: Option<NodeTypeT>,
    /// * `numeric_edge_type_ids: Option<bool>,
    /// * `minimum_edge_type_id: Option<NodeTypeT>,
    /// * `edge_type_list_separator: Option<String>,
    /// * `edge_type_list_header: Option<bool>,
    /// * `edge_type_list_rows_to_skip: Option<usize>,
    /// * `edge_type_list_is_correct: Option<bool>,
    /// * `edge_type_list_max_rows_number: Option<usize>,
    /// * `edge_type_list_comment_symbol: Option<String>,
    /// * `load_edge_type_list_in_parallel: Option<bool>,
    /// * `edge_path: Option<String>,
    /// * `edge_list_separator: Option<String>,
    /// * `edge_list_header: Option<bool>,
    /// * `edge_list_rows_to_skip: Option<usize>,
    /// * `sources_column_number: Option<usize>,
    /// * `sources_column: Option<String>,
    /// * `destinations_column_number: Option<usize>,
    /// * `destinations_column: Option<String>,
    /// * `edge_list_edge_types_column_number: Option<usize>,
    /// * `edge_list_edge_types_column: Option<String>,
    /// * `default_edge_type: Option<String>,
    /// * `weights_column_number: Option<usize>,
    /// * `weights_column: Option<String>,
    /// * `default_weight: Option<WeightT>,
    /// * `edge_ids_column: Option<String>,
    /// * `edge_ids_column_number: Option<usize>,
    /// * `edge_list_numeric_edge_type_ids: Option<bool>,
    /// * `edge_list_numeric_node_ids: Option<bool>,
    /// * `skip_weights_if_unavailable: Option<bool>,
    /// * `skip_edge_types_if_unavailable: Option<bool>,
    /// * `edge_list_is_complete: Option<bool>,
    /// * `edge_list_may_contain_duplicates: Option<bool>,
    /// * `edge_list_is_sorted: Option<bool>,
    /// * `edge_list_is_correct: Option<bool>,
    /// * `edge_list_max_rows_number: Option<usize>,
    /// * `edge_list_comment_symbol: Option<String>,
    /// * `edges_number: Option<EdgeT>,
    /// * `load_edge_list_in_parallel: Option<bool>,
    /// * `verbose: Option<bool>,
    /// * `may_have_singletons: Option<bool>,
    /// * `may_have_singleton_with_selfloops: Option<bool>,
    /// * `directed: bool,
    /// * `name: S,
    ///
    pub fn from_csv<S: Clone + Into<String>>(
        node_type_path: Option<String>,
        node_type_list_separator: Option<String>,
        node_types_column_number: Option<usize>,
        node_types_column: Option<String>,
        node_types_number: Option<NodeTypeT>,
        numeric_node_type_ids: Option<bool>,
        minimum_node_type_id: Option<NodeTypeT>,
        node_type_list_header: Option<bool>,
        node_type_list_rows_to_skip: Option<usize>,
        node_type_list_is_correct: Option<bool>,
        node_type_list_max_rows_number: Option<usize>,
        node_type_list_comment_symbol: Option<String>,
        load_node_type_list_in_parallel: Option<bool>,
        node_path: Option<String>,
        node_list_separator: Option<String>,
        node_list_header: Option<bool>,
        node_list_rows_to_skip: Option<usize>,
        node_list_is_correct: Option<bool>,
        node_list_max_rows_number: Option<usize>,
        node_list_comment_symbol: Option<String>,
        default_node_type: Option<String>,
        nodes_column_number: Option<usize>,
        nodes_column: Option<String>,
        node_types_separator: Option<String>,
        node_list_node_types_column_number: Option<usize>,
        node_list_node_types_column: Option<String>,
        nodes_number: Option<NodeT>,
        minimum_node_id: Option<NodeT>,
        numeric_node_ids: Option<bool>,
        node_list_numeric_node_type_ids: Option<bool>,
        skip_node_types_if_unavailable: Option<bool>,
        load_node_list_in_parallel: Option<bool>,
        edge_type_path: Option<String>,
        edge_types_column_number: Option<usize>,
        edge_types_column: Option<String>,
        edge_types_number: Option<NodeTypeT>,
        numeric_edge_type_ids: Option<bool>,
        minimum_edge_type_id: Option<NodeTypeT>,
        edge_type_list_separator: Option<String>,
        edge_type_list_header: Option<bool>,
        edge_type_list_rows_to_skip: Option<usize>,
        edge_type_list_is_correct: Option<bool>,
        edge_type_list_max_rows_number: Option<usize>,
        edge_type_list_comment_symbol: Option<String>,
        load_edge_type_list_in_parallel: Option<bool>,
        edge_path: Option<String>,
        edge_list_separator: Option<String>,
        edge_list_header: Option<bool>,
        edge_list_rows_to_skip: Option<usize>,
        sources_column_number: Option<usize>,
        sources_column: Option<String>,
        destinations_column_number: Option<usize>,
        destinations_column: Option<String>,
        edge_list_edge_types_column_number: Option<usize>,
        edge_list_edge_types_column: Option<String>,
        default_edge_type: Option<String>,
        weights_column_number: Option<usize>,
        weights_column: Option<String>,
        default_weight: Option<WeightT>,
        edge_ids_column: Option<String>,
        edge_ids_column_number: Option<usize>,
        edge_list_numeric_edge_type_ids: Option<bool>,
        edge_list_numeric_node_ids: Option<bool>,
        skip_weights_if_unavailable: Option<bool>,
        skip_edge_types_if_unavailable: Option<bool>,
        edge_list_is_complete: Option<bool>,
        edge_list_may_contain_duplicates: Option<bool>,
        edge_list_is_sorted: Option<bool>,
        edge_list_is_correct: Option<bool>,
        edge_list_max_rows_number: Option<usize>,
        edge_list_comment_symbol: Option<String>,
        edges_number: Option<EdgeT>,
        load_edge_list_in_parallel: Option<bool>,
        verbose: Option<bool>,
        may_have_singletons: Option<bool>,
        may_have_singleton_with_selfloops: Option<bool>,
        directed: bool,
        name: S,
    ) -> Result<Graph> {
        let node_type_file_reader: Option<TypeFileReader<NodeTypeT>> =
            if node_type_path.is_some() || node_types_number.is_some() {
                Some(
                    TypeFileReader::new(node_type_path)?
                        .set_comment_symbol(node_type_list_comment_symbol)?
                        .set_header(node_type_list_header)?
                        .set_max_rows_number(node_type_list_max_rows_number)?
                        .set_rows_to_skip(node_type_list_rows_to_skip)?
                        .set_separator(node_type_list_separator)?
                        .set_type_column_number(node_types_column_number)?
                        .set_type_column(node_types_column)?
                        .set_minimum_type_id(minimum_node_type_id)
                        .set_numeric_type_ids(numeric_node_type_ids)
                        .set_csv_is_correct(node_type_list_is_correct)?
                        .set_types_number(node_types_number)
                        .set_parallel(load_node_type_list_in_parallel)?
                        .set_verbose(verbose),
                )
            } else {
                None
            };

        let edge_type_file_reader: Option<TypeFileReader<EdgeTypeT>> =
            if edge_type_path.is_some() || edge_types_number.is_some() {
                Some(
                    TypeFileReader::new(edge_type_path)?
                        .set_comment_symbol(edge_type_list_comment_symbol)?
                        .set_header(edge_type_list_header)?
                        .set_max_rows_number(edge_type_list_max_rows_number)?
                        .set_rows_to_skip(edge_type_list_rows_to_skip)?
                        .set_separator(edge_type_list_separator)?
                        .set_type_column_number(edge_types_column_number)?
                        .set_type_column(edge_types_column)?
                        .set_minimum_type_id(minimum_edge_type_id)
                        .set_numeric_type_ids(numeric_edge_type_ids)
                        .set_csv_is_correct(edge_type_list_is_correct)?
                        .set_types_number(edge_types_number)
                        .set_parallel(load_edge_type_list_in_parallel)?
                        .set_verbose(verbose),
                )
            } else {
                None
            };

        let node_file_reader = if node_path.is_some() || nodes_number.is_some() {
            Some(
                NodeFileReader::new(node_path)?
                    .set_comment_symbol(node_list_comment_symbol)?
                    .set_header(node_list_header)?
                    .set_max_rows_number(node_list_max_rows_number)?
                    .set_rows_to_skip(node_list_rows_to_skip)?
                    .set_separator(node_list_separator)?
                    .set_nodes_column_number(nodes_column_number)?
                    .set_nodes_column(nodes_column)?
                    .set_minimum_node_id(minimum_node_id)
                    .set_node_types_column_number(node_list_node_types_column_number)?
                    .set_node_types_column(node_list_node_types_column)?
                    .set_node_types_separator(node_types_separator)?
                    .set_skip_node_types_if_unavailable(skip_node_types_if_unavailable)?
                    .set_default_node_type(default_node_type)
                    .set_numeric_node_ids(numeric_node_ids)
                    .set_numeric_node_type_ids(node_list_numeric_node_type_ids)?
                    .set_csv_is_correct(node_list_is_correct)?
                    .set_nodes_number(nodes_number)
                    .set_parallel(load_node_list_in_parallel)?
                    .set_verbose(verbose),
            )
        } else {
            None
        };

        let edge_file_reader = edge_path.map_or(Ok::<_, String>(None), |edge_path| {
            Ok(Some(
                EdgeFileReader::new(edge_path)?
                    .set_comment_symbol(edge_list_comment_symbol)?
                    .set_header(edge_list_header)?
                    .set_max_rows_number(edge_list_max_rows_number)?
                    .set_rows_to_skip(edge_list_rows_to_skip)?
                    .set_separator(edge_list_separator)?
                    .set_sources_column_number(sources_column_number)?
                    .set_sources_column(sources_column)?
                    .set_destinations_column_number(destinations_column_number)?
                    .set_destinations_column(destinations_column)?
                    .set_edge_types_column_number(edge_list_edge_types_column_number)?
                    .set_edge_types_column(edge_list_edge_types_column)?
                    .set_skip_edge_types_if_unavailable(skip_edge_types_if_unavailable)
                    .set_default_edge_type(default_edge_type)
                    .set_weights_column_number(weights_column_number)?
                    .set_weights_column(weights_column)?
                    .set_skip_weights_if_unavailable(skip_weights_if_unavailable)
                    .set_default_weight(default_weight)?
                    .set_edge_ids_column(edge_ids_column)?
                    .set_edge_ids_column_number(edge_ids_column_number)?
                    .set_numeric_node_ids(edge_list_numeric_node_ids)
                    .set_numeric_edge_type_ids(edge_list_numeric_edge_type_ids)
                    .set_complete(edge_list_is_complete)
                    .set_sorted(edge_list_is_sorted)
                    .set_may_have_duplicates(edge_list_may_contain_duplicates)
                    .set_csv_is_correct(edge_list_is_correct)
                    .set_edges_number(edges_number)
                    .set_parallel(load_edge_list_in_parallel)
                    .set_verbose(verbose),
            ))
        })?;

        Graph::from_file_readers(
            edge_file_reader,
            node_file_reader,
            node_type_file_reader,
            edge_type_file_reader,
            may_have_singletons.unwrap_or(true),
            may_have_singleton_with_selfloops.unwrap_or(true),
            directed,
            name,
        )
    }
}
