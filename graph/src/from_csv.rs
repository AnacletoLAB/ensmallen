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
            node_type_file_reader.as_ref().map(|ntfr| {
                ntfr.reader
                    .as_ref()
                    .map_or(true, |reader| reader.csv_is_correct)
            }),
            node_file_reader
                .as_ref()
                .map_or(Ok::<_, String>(None), |nfr| {
                    Ok(nfr.read_lines().transpose()?)
                })?,
            node_file_reader
                .as_ref()
                .and_then(|ntf| ntf.number_of_nodes.clone()),
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
            edge_type_file_reader.as_ref().map(|etfr| {
                etfr.reader
                    .as_ref()
                    .map_or(true, |reader| reader.csv_is_correct)
            }),
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
                .and_then(|etr| etr.number_of_edges.clone()),
            edge_file_reader
                .as_ref()
                .map(|etr| etr.numeric_node_ids.clone()),
            edge_file_reader
                .as_ref()
                .map(|etr| etr.numeric_edge_type_ids.clone()),
            node_file_reader
                .as_ref()
                .map(|etr| etr.skip_node_types_if_unavailable.clone()),
            edge_file_reader
                .as_ref()
                .map(|etr| etr.skip_edge_types_if_unavailable.clone()),
            may_have_singletons,
            may_have_singleton_with_selfloops,
            name.into(),
        )
    }

    /// Return graph renderized from given CSVs or TSVs-like files.
    ///
    /// # Arguments
    /// * `node_type_path`: Option<String> - The path to the file with the unique node type names.
    /// * `node_type_list_separator`: Option<char> - The separator to use for the node types file. Note that if this is not provided, one will be automatically detected among the following`: comma, semi-column, tab and space.
    /// * `node_types_column_number`: Option<usize> - The number of the column of the node types file from where to load the node types.
    /// * `node_types_column`: Option<String> - The name of the column of the node types file from where to load the node types.
    /// * `number_of_node_types`: Option<NodeTypeT> - The number of the unique node types. This will be used in order to allocate the correct size for the data structure.
    /// * `numeric_node_type_ids`: Option<bool> - Whether the node type names should be loaded as numeric values, i.e. casted from string to a numeric representation.
    /// * `minimum_node_type_id`: Option<NodeTypeT> - The minimum node type ID to be used when using numeric node type IDs.
    /// * `node_type_list_header`: Option<bool> - Whether the node type file has an header.
    /// * `node_type_list_support_balanced_quotes`: Option<bool> - Whether to support balanced quotes.
    /// * `node_type_list_rows_to_skip`: Option<usize> - The number of lines to skip in the node types file`: the header is already skipped if it has been specified that the file has an header.
    /// * `node_type_list_is_correct`: Option<bool> - Whether the node types file can be assumed to be correct, i.e. does not have something wrong in it. If this parameter is passed as true on a malformed file, the constructor will crash.
    /// * `node_type_list_max_rows_number`: Option<usize> - The maximum number of lines to be loaded from the node types file.
    /// * `node_type_list_comment_symbol`: Option<String> - The comment symbol to skip lines in the node types file. Lines starting with this symbol will be skipped.
    /// * `load_node_type_list_in_parallel`: Option<bool> - Whether to load the node type list in parallel. Note that when loading in parallel, the internal order of the node type IDs may result changed across different iterations. We are working to get this to be stable.
    /// * `node_path`: Option<String> - The path to the file with the unique node names.
    /// * `node_list_separator`: Option<char> - The separator to use for the nodes file. Note that if this is not provided, one will be automatically detected among the following`: comma, semi-column, tab and space.
    /// * `node_list_header`: Option<bool> - Whether the nodes file has an header.
    /// * `node_list_support_balanced_quotes`: Option<bool> - Whether to support balanced quotes.
    /// * `node_list_rows_to_skip`: Option<usize> - Number of rows to skip in the node list file.
    /// * `node_list_is_correct`: Option<bool> - Whether the nodes file can be assumed to be correct, i.e. does not have something wrong in it. If this parameter is passed as true on a malformed file, the constructor will crash.
    /// * `node_list_max_rows_number`: Option<usize> - The maximum number of lines to be loaded from the nodes file.
    /// * `node_list_comment_symbol`: Option<String> - The comment symbol to skip lines in the nodes file. Lines starting with this symbol will be skipped.
    /// * `default_node_type`: Option<String> - The node type to be used when the node type for a given node in the node file is None.
    /// * `nodes_column_number`: Option<usize> - The number of the column of the node file from where to load the node names.
    /// * `nodes_column`: Option<String> - The name of the column of the node file from where to load the node names.
    /// * `node_types_separator`: Option<char> - The node types separator.
    /// * `node_list_node_types_column_number`: Option<usize> - The number of the column of the node file from where to load the node types.
    /// * `node_list_node_types_column`: Option<String> - The name of the column of the node file from where to load the node types.
    /// * `node_ids_column`: Option<String> - The name of the column of the node file from where to load the node IDs.
    /// * `node_ids_column_number`: Option<usize> - The number of the column of the node file from where to load the node IDs
    /// * `number_of_nodes`: Option<NodeT> - The expected number of nodes. Note that this must be the EXACT number of nodes in the graph.
    /// * `minimum_node_id`: Option<NodeT> - The minimum node ID to be used, when loading the node IDs as numerical.
    /// * `numeric_node_ids`: Option<bool> - Whether to load the numeric node IDs as numeric.
    /// * `node_list_numeric_node_type_ids`: Option<bool> - Whether to load the node types IDs in the node file to be numeric.
    /// * `skip_node_types_if_unavailable`: Option<bool> - Whether to skip the node types without raising an error if these are unavailable.
    /// * `load_node_list_in_parallel`: Option<bool> - Whether to load the node list in parallel. When loading in parallel, without node IDs, the nodes may not be loaded in a deterministic order.
    /// * `edge_type_path`: Option<String> - The path to the file with the unique edge type names.
    /// * `edge_types_column_number`: Option<usize> - The number of the column of the edge types file from where to load the edge types.
    /// * `edge_types_column`: Option<String> - The name of the column of the edge types file from where to load the edge types.
    /// * `number_of_edge_types`: Option<EdgeTypeT> - The number of the unique edge types. This will be used in order to allocate the correct size for the data structure.
    /// * `numeric_edge_type_ids`: Option<bool> - Whether the edge type names should be loaded as numeric values, i.e. casted from string to a numeric representation.
    /// * `minimum_edge_type_id`: Option<EdgeTypeT> - The minimum edge type ID to be used when using numeric edge type IDs.
    /// * `edge_type_list_separator`: Option<char> - The separator to use for the edge type list. Note that, if None is provided, one will be attempted to be detected automatically between ';', ',', tab or space.
    /// * `edge_type_list_header`: Option<bool> - Whether the edge type file has an header.
    /// * `edge_type_list_support_balanced_quotes`: Option<bool> - Whether to support balanced quotes while reading the edge type list.
    /// * `edge_type_list_rows_to_skip`: Option<usize> - Number of rows to skip in the edge type list file.
    /// * `edge_type_list_is_correct`: Option<bool> - Whether the edge types file can be assumed to be correct, i.e. does not have something wrong in it. If this parameter is passed as true on a malformed file, the constructor will crash.
    /// * `edge_type_list_max_rows_number`: Option<usize> - The maximum number of lines to be loaded from the edge types file.
    /// * `edge_type_list_comment_symbol`: Option<String> - The comment symbol to skip lines in the edge types file. Lines starting with this symbol will be skipped.
    /// * `load_edge_type_list_in_parallel`: Option<bool> - Whether to load the edge type list in parallel. When loading in parallel, without edge type IDs, the edge types may not be loaded in a deterministic order.
    /// * `edge_path`: Option<String> - The path to the file with the edge list.
    /// * `edge_list_separator`: Option<char> - The separator to use for the edge list. Note that, if None is provided, one will be attempted to be detected automatically between ';', ',', tab or space.
    /// * `edge_list_header`: Option<bool> - Whether the edges file has an header.
    /// * `edge_list_support_balanced_quotes`: Option<bool> - Whether to support balanced quotes while reading the edge list.
    /// * `edge_list_rows_to_skip`: Option<usize> - Number of rows to skip in the edge list file.
    /// * `sources_column_number`: Option<usize> - The number of the column of the edges file from where to load the source nodes.
    /// * `sources_column`: Option<String> - The name of the column of the edges file from where to load the source nodes.
    /// * `destinations_column_number`: Option<usize> - The number of the column of the edges file from where to load the destinaton nodes.
    /// * `destinations_column`: Option<String> - The name of the column of the edges file from where to load the destinaton nodes.
    /// * `edge_list_edge_types_column_number`: Option<usize> - The number of the column of the edges file from where to load the edge types.
    /// * `edge_list_edge_types_column`: Option<String> - The name of the column of the edges file from where to load the edge types.
    /// * `default_edge_type`: Option<String> - The edge type to be used when the edge type for a given edge in the edge file is None.
    /// * `weights_column_number`: Option<usize> - The number of the column of the edges file from where to load the edge weights.
    /// * `weights_column`: Option<String> - The name of the column of the edges file from where to load the edge weights.
    /// * `default_weight`: Option<WeightT> - The edge weight to be used when the edge weight for a given edge in the edge file is None.
    /// * `edge_ids_column`: Option<String> - The name of the column of the edges file from where to load the edge IDs.
    /// * `edge_ids_column_number`: Option<usize> - The number of the column of the edges file from where to load the edge IDs.
    /// * `edge_list_numeric_edge_type_ids`: Option<bool> - Whether to load the edge type IDs as numeric from the edge list.
    /// * `edge_list_numeric_node_ids`: Option<bool> - Whether to load the edge node IDs as numeric from the edge list.
    /// * `skip_weights_if_unavailable`: Option<bool> - Whether to skip the weights without raising an error if these are unavailable.
    /// * `skip_edge_types_if_unavailable`: Option<bool> - Whether to skip the edge types without raising an error if these are unavailable.
    /// * `edge_list_is_complete`: Option<bool> - Whether to consider the edge list as complete, i.e. the edges are presented in both directions when loading an undirected graph.
    /// * `edge_list_may_contain_duplicates`: Option<bool> - Whether the edge list may contain duplicates. If the edge list surely DOES NOT contain duplicates, a validation step may be skipped. By default, it is assumed that the edge list may contain duplicates.
    /// * `edge_list_is_sorted`: Option<bool> - Whether the edge list is sorted. Note that a sorted edge list has the minimal memory peak, but requires the nodes number and the edges number.
    /// * `edge_list_is_correct`: Option<bool> - Whether the edges file can be assumed to be correct, i.e. does not have something wrong in it. If this parameter is passed as true on a malformed file, the constructor will crash.
    /// * `edge_list_max_rows_number`: Option<usize> - The maximum number of lines to be loaded from the edges file.
    /// * `edge_list_comment_symbol`: Option<String> - The comment symbol to skip lines in the edges file. Lines starting with this symbol will be skipped.
    /// * `number_of_edges`: Option<EdgeT> - The expected number of edges. Note that this must be the EXACT number of edges in the graph.
    /// * `load_edge_list_in_parallel`: Option<bool> - Whether to load the edge list in parallel. Note that, if the edge IDs indices are not given, it is NOT possible to load a sorted edge list. Similarly, when loading in parallel, without edge IDs, the edges may not be loaded in a deterministic order.
    /// * `remove_chevrons`: Option<bool> - Whether remove chevrons while reading elements.
    /// * `remove_spaces`: Option<bool> - Whether remove spaces while reading elements.
    /// * `verbose`: Option<bool> - Whether to show a loading bar while reading the files. Note that, if parallel loading is enabled, loading bars will not be showed because they are a synchronization bottleneck.
    /// * `may_have_singletons`: Option<bool> - Whether the graph may be expected to have singleton nodes. If it is said that it surely DOES NOT have any, it will allow for some speedups and lower mempry peaks.
    /// * `may_have_singleton_with_selfloops`: Option<bool> - Whether the graph may be expected to have singleton nodes with selfloops. If it is said that it surely DOES NOT have any, it will allow for some speedups and lower mempry peaks.
    /// * `directed`: bool - Whether to load the graph as directed or undirected.
    /// * `name`: Option<String> - The name of the graph to be loaded.
    ///
    pub fn from_csv(
        directed: bool,

        node_type_path: Option<String>,
        node_type_list_separator: Option<char>,
        node_types_column_number: Option<usize>,
        node_types_column: Option<String>,
        node_types_ids_column_number: Option<usize>,
        node_types_ids_column: Option<String>,
        number_of_node_types: Option<NodeTypeT>,
        numeric_node_type_ids: Option<bool>,
        minimum_node_type_id: Option<NodeTypeT>,
        node_type_list_header: Option<bool>,
        node_type_list_support_balanced_quotes: Option<bool>,
        node_type_list_rows_to_skip: Option<usize>,
        node_type_list_is_correct: Option<bool>,
        node_type_list_max_rows_number: Option<usize>,
        node_type_list_comment_symbol: Option<String>,
        load_node_type_list_in_parallel: Option<bool>,

        node_path: Option<String>,
        node_list_separator: Option<char>,
        node_list_header: Option<bool>,
        node_list_support_balanced_quotes: Option<bool>,
        node_list_rows_to_skip: Option<usize>,
        node_list_is_correct: Option<bool>,
        node_list_max_rows_number: Option<usize>,
        node_list_comment_symbol: Option<String>,
        default_node_type: Option<String>,
        nodes_column_number: Option<usize>,
        nodes_column: Option<String>,
        node_types_separator: Option<char>,
        node_list_node_types_column_number: Option<usize>,
        node_list_node_types_column: Option<String>,
        node_ids_column: Option<String>,
        node_ids_column_number: Option<usize>,
        number_of_nodes: Option<NodeT>,
        minimum_node_id: Option<NodeT>,
        numeric_node_ids: Option<bool>,
        node_list_numeric_node_type_ids: Option<bool>,
        skip_node_types_if_unavailable: Option<bool>,
        load_node_list_in_parallel: Option<bool>,

        edge_type_path: Option<String>,
        edge_types_column_number: Option<usize>,
        edge_types_column: Option<String>,
        edge_types_ids_column_number: Option<usize>,
        edge_types_ids_column: Option<String>,
        number_of_edge_types: Option<EdgeTypeT>,
        numeric_edge_type_ids: Option<bool>,
        minimum_edge_type_id: Option<EdgeTypeT>,
        edge_type_list_separator: Option<char>,
        edge_type_list_header: Option<bool>,
        edge_type_list_support_balanced_quotes: Option<bool>,
        edge_type_list_rows_to_skip: Option<usize>,
        edge_type_list_is_correct: Option<bool>,
        edge_type_list_max_rows_number: Option<usize>,
        edge_type_list_comment_symbol: Option<String>,
        load_edge_type_list_in_parallel: Option<bool>,

        edge_path: Option<String>,
        edge_list_separator: Option<char>,
        edge_list_header: Option<bool>,
        edge_list_support_balanced_quotes: Option<bool>,
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
        number_of_edges: Option<EdgeT>,
        load_edge_list_in_parallel: Option<bool>,

        remove_chevrons: Option<bool>,
        remove_spaces: Option<bool>,

        verbose: Option<bool>,
        may_have_singletons: Option<bool>,
        may_have_singleton_with_selfloops: Option<bool>,
        name: Option<String>,
    ) -> Result<Graph> {
        // We check whether some parameters regarding
        // node type files were provided, yet no
        // node type file path was provided.
        if node_type_path.is_none()
            && [
                node_type_list_comment_symbol.is_some(),
                node_type_list_header.is_some(),
                node_type_list_support_balanced_quotes.is_some(),
                node_type_list_max_rows_number.is_some(),
                node_type_list_separator.is_some(),
                node_types_column_number.is_some(),
                node_types_column.is_some(),
                node_types_ids_column.is_some(),
                node_types_ids_column_number.is_some(),
                node_type_list_is_correct.is_some(),
                load_node_type_list_in_parallel.is_some(),
            ]
            .iter()
            .any(|&x| x)
        {
            return Err(concat!(
                "The path to the node type file (not the node list!) was not provided ",
                "but one or more arguments that make sense only when the node type path ",
                "is provided where provided!"
            )
            .to_string());
        }

        // Conversely, we check whether a node type
        // file was provided, and no node type column
        // was given for the node file.
        if node_type_path.is_some()
            && [
                node_list_node_types_column.is_none() &&  node_list_node_types_column_number.is_none(),
            ]
            .iter()
            .any(|&x| x)
        {
            return Err(format!(
                concat!(
                "The path to the node type file (not the node list!) was provided and is ",
                "`{:?}`, ",
                "but you did not provide either `node_list_node_types_column` or ",
                "`node_list_node_types_column_number` so to specify which column in ",
                "the node list should be loaded. Do note that the file provided ",
                "to the node type path should contain the UNIQUE node types, and not ",
                "the node type for each node. The node type file is primarily used to ",
                "ensure all node types in the node list are known before starting to ",
                "process the node list itself, which allows for additional assumptions ",
                "and therefore significantly faster processing."
            ),
            node_type_path));
        }

        let name = name.unwrap_or("Graph".to_string());
        let node_type_file_reader: Option<TypeFileReader<NodeTypeT>> =
            if node_type_path.is_some() || number_of_node_types.is_some() {
                Some(
                    TypeFileReader::new(node_type_path)?
                        .set_comment_symbol(node_type_list_comment_symbol)?
                        .set_header(node_type_list_header)?
                        .set_support_balanced_quotes(node_type_list_support_balanced_quotes)?
                        .set_max_rows_number(node_type_list_max_rows_number)?
                        .set_rows_to_skip(node_type_list_rows_to_skip)?
                        .set_separator(node_type_list_separator)?
                        .set_type_column_number(node_types_column_number)?
                        .set_type_column(node_types_column)?
                        .set_type_ids_column(node_types_ids_column)?
                        .set_type_ids_column_number(node_types_ids_column_number)?
                        .set_minimum_type_id(minimum_node_type_id)
                        .set_numeric_type_ids(numeric_node_type_ids)
                        .set_csv_is_correct(node_type_list_is_correct)?
                        .set_types_number(number_of_node_types)
                        .set_parallel(load_node_type_list_in_parallel)?
                        .set_remove_chevrons(remove_chevrons)
                        .set_remove_spaces(remove_spaces)
                        .set_verbose(verbose),
                )
            } else {
                None
            };
        
        // We check whether some parameters regarding
        // edge type files were provided, yet no
        // edge type file path was provided.
        if edge_type_path.is_none()
            && [
                edge_type_list_comment_symbol.is_some(),
                edge_type_list_header.is_some(),
                edge_type_list_support_balanced_quotes.is_some(),
                edge_type_list_max_rows_number.is_some(),
                edge_type_list_separator.is_some(),
                edge_types_column_number.is_some(),
                edge_types_column.is_some(),
                edge_types_ids_column.is_some(),
                edge_types_ids_column_number.is_some(),
                edge_type_list_is_correct.is_some(),
                load_edge_type_list_in_parallel.is_some(),
            ]
            .iter()
            .any(|&x| x)
        {
            return Err(concat!(
                "The path to the edge type file (not the edge list!) was not provided ",
                "but one or more arguments that make sense only when the edge type path ",
                "is provided where provided!"
            )
            .to_string());
        }

        // Conversely, we check whether a edge type
        // file was provided, and no edge type column
        // was given for the edge file.
        if edge_type_path.is_some()
            && [
                edge_list_edge_types_column.is_none() &&  edge_list_edge_types_column_number.is_none(),
            ]
            .iter()
            .any(|&x| x)
        {
            return Err(format!(
                concat!(
                "The path to the edge type file (not the edge list!) was provided and is ",
                "`{:?}`, ",
                "but you did not provide either `edge_list_edge_types_column` or ",
                "`edge_list_edge_types_column_number` so to specify which column in ",
                "the edge list should be loaded. Do note that the file provided ",
                "to the edge type path should contain the UNIQUE edge types, and not ",
                "the edge type for each edge. The edge type file is primarily used to ",
                "ensure all edge types in the edge list are known before starting to ",
                "process the edge list itself, which allows for additional assumptions ",
                "and therefore significantly faster processing."
            ),
            edge_type_path));
        }
        
        let edge_type_file_reader: Option<TypeFileReader<EdgeTypeT>> =
            if edge_type_path.is_some() || number_of_edge_types.is_some() {
                Some(
                    TypeFileReader::new(edge_type_path)?
                        .set_comment_symbol(edge_type_list_comment_symbol)?
                        .set_header(edge_type_list_header)?
                        .set_support_balanced_quotes(edge_type_list_support_balanced_quotes)?
                        .set_max_rows_number(edge_type_list_max_rows_number)?
                        .set_rows_to_skip(edge_type_list_rows_to_skip)?
                        .set_separator(edge_type_list_separator)?
                        .set_type_column_number(edge_types_column_number)?
                        .set_type_column(edge_types_column)?
                        .set_type_ids_column(edge_types_ids_column)?
                        .set_type_ids_column_number(edge_types_ids_column_number)?
                        .set_minimum_type_id(minimum_edge_type_id)
                        .set_numeric_type_ids(numeric_edge_type_ids)
                        .set_csv_is_correct(edge_type_list_is_correct)?
                        .set_types_number(number_of_edge_types)
                        .set_parallel(load_edge_type_list_in_parallel)?
                        .set_remove_chevrons(remove_chevrons)
                        .set_remove_spaces(remove_spaces)
                        .set_verbose(verbose),
                )
            } else {
                None
            };

        let node_file_reader = if node_path.is_some() || number_of_nodes.is_some() {
            Some(
                NodeFileReader::new(node_path)?
                    .set_comment_symbol(node_list_comment_symbol)?
                    .set_header(node_list_header)?
                    .set_support_balanced_quotes(node_list_support_balanced_quotes)?
                    .set_max_rows_number(node_list_max_rows_number)?
                    .set_rows_to_skip(node_list_rows_to_skip)?
                    .set_separator(node_list_separator)?
                    .set_skip_node_types_if_unavailable(skip_node_types_if_unavailable)?
                    .set_nodes_column(nodes_column)?
                    .set_nodes_column_number(nodes_column_number)?
                    .set_node_ids_column(node_ids_column)?
                    .set_node_ids_column_number(node_ids_column_number)?
                    .set_minimum_node_id(minimum_node_id)
                    .set_node_types_column_number(node_list_node_types_column_number)?
                    .set_node_types_column(node_list_node_types_column)?
                    .set_node_types_separator(node_types_separator)?
                    .set_default_node_type(default_node_type)
                    .set_numeric_node_ids(numeric_node_ids)
                    .set_numeric_node_type_ids(node_list_numeric_node_type_ids)?
                    .set_csv_is_correct(node_list_is_correct)?
                    .set_number_of_nodes(number_of_nodes)
                    .set_parallel(load_node_list_in_parallel)?
                    .set_remove_chevrons(remove_chevrons)
                    .set_remove_spaces(remove_spaces)
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
                    .set_support_balanced_quotes(edge_list_support_balanced_quotes)
                    .set_max_rows_number(edge_list_max_rows_number)?
                    .set_rows_to_skip(edge_list_rows_to_skip)?
                    .set_separator(edge_list_separator)?
                    .set_skip_edge_types_if_unavailable(skip_edge_types_if_unavailable)
                    .set_skip_weights_if_unavailable(skip_weights_if_unavailable)
                    .set_sources_column_number(sources_column_number)?
                    .set_sources_column(sources_column)?
                    .set_destinations_column_number(destinations_column_number)?
                    .set_destinations_column(destinations_column)?
                    .set_edge_types_column_number(edge_list_edge_types_column_number)?
                    .set_edge_types_column(edge_list_edge_types_column)?
                    .set_default_edge_type(default_edge_type)
                    .set_weights_column_number(weights_column_number)?
                    .set_weights_column(weights_column)?
                    .set_default_weight(default_weight)?
                    .set_edge_ids_column(edge_ids_column)?
                    .set_edge_ids_column_number(edge_ids_column_number)?
                    .set_numeric_node_ids(edge_list_numeric_node_ids)
                    .set_numeric_edge_type_ids(edge_list_numeric_edge_type_ids)
                    .set_complete(edge_list_is_complete)
                    .set_sorted(edge_list_is_sorted)
                    .set_may_have_duplicates(edge_list_may_contain_duplicates)
                    .set_csv_is_correct(edge_list_is_correct)
                    .set_number_of_edges(number_of_edges)
                    .set_parallel(load_edge_list_in_parallel)
                    .set_remove_chevrons(remove_chevrons)
                    .set_remove_spaces(remove_spaces)
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
