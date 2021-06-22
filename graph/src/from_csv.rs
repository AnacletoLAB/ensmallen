use super::*;

impl Graph {
    /// Return graph renderized from given files.
    ///
    /// # Arguments
    ///
    /// * `edge_file_reader`: EdgeFileReader - Reader of the edge file.
    /// * `node_file_reader`: Option<NodeFileReader> - Reader of the node file.
    /// * `directed`: bool - Whether the graph is to be read as directed or undirected.
    /// * `directed_edge_list`: bool - Whether to read the edge list as directed.
    /// * `edges_number`: usize - Number of edges of the graph.
    /// * `nodes_number`: NodeT - Number of the nodes of the graph.
    /// * `name`: S - Name of the graph.
    ///
    pub fn from_sorted_csv<S: Clone + Into<String>>(
        mut edge_file_reader: EdgeFileReader,
        mut node_file_reader: Option<NodeFileReader>,
        directed: bool,
        directed_edge_list: bool,
        edges_number: usize,
        nodes_number: NodeT,
        name: S,
    ) -> Result<Graph> {
        edge_file_reader = edge_file_reader.set_graph_name(name.clone().into());
        node_file_reader = node_file_reader.map(|nfr| nfr.set_graph_name(name.clone().into()));
        Graph::from_string_sorted(
            edge_file_reader.read_lines()?,
            node_file_reader
                .as_ref()
                .map_or(Ok::<_, String>(None), |nfr| Ok(Some(nfr.read_lines()?)))?,
            directed,
            directed_edge_list,
            name,
            node_file_reader
                .as_ref()
                .map_or(false, |nfr| nfr.reader.ignore_duplicates),
            node_file_reader
                .as_ref()
                .map_or(false, |nfr| nfr.reader.csv_is_correct),
            edge_file_reader.reader.ignore_duplicates,
            edge_file_reader.reader.csv_is_correct,
            edges_number,
            nodes_number,
            edge_file_reader.numeric_edge_type_ids,
            node_file_reader
                .as_ref()
                .map_or(false, |nfr| nfr.numeric_node_ids),
            edge_file_reader.numeric_node_ids,
            node_file_reader
                .as_ref()
                .map_or(false, |nfr| nfr.numeric_node_type_ids),
            node_file_reader
                .as_ref()
                .map_or(false, |nfr| nfr.has_node_types()),
            edge_file_reader.has_edge_types(),
            edge_file_reader.has_edge_weights(),
            // TODO: expose this parameter to use in the future.
            true,
            node_file_reader
                .as_ref()
                .map_or(false, |nfr| nfr.might_contain_singletons),
            edge_file_reader.might_contain_singletons_with_selfloops,
            edge_file_reader.might_contain_trap_nodes,
        )
    }

    /// Return graph renderized from given files.
    ///
    /// # Arguments
    ///
    /// * `edge_file_reader`: EdgeFileReader - Reader of the edge file.
    /// * `node_file_reader`: Option<NodeFileReader> - Reader of the node file.
    /// * `directed`: bool - Whether the graph is to be read as directed or undirected.
    /// * `directed_edge_list`: bool - Whether to read the edge list as directed.
    /// * `name`: S - The name for the graph.
    pub fn from_unsorted_csv<S: Clone + Into<String>>(
        mut edge_file_reader: EdgeFileReader,
        mut node_file_reader: Option<NodeFileReader>,
        directed: bool,
        directed_edge_list: bool,
        name: S,
    ) -> Result<Graph> {
        edge_file_reader = edge_file_reader.set_graph_name(name.clone().into());
        node_file_reader = node_file_reader.map(|nfr| nfr.set_graph_name(name.clone().into()));
        Graph::from_string_unsorted(
            edge_file_reader.read_lines()?,
            node_file_reader
                .as_ref()
                .map_or(Ok::<_, String>(None), |nfr| Ok(Some(nfr.read_lines()?)))?,
            directed,
            directed_edge_list,
            name,
            node_file_reader
                .as_ref()
                .map_or(false, |nfr| nfr.reader.ignore_duplicates),
            node_file_reader
                .as_ref()
                .map_or(false, |nfr| nfr.reader.csv_is_correct),
            edge_file_reader.reader.ignore_duplicates,
            edge_file_reader.reader.csv_is_correct,
            edge_file_reader.numeric_edge_type_ids,
            node_file_reader
                .as_ref()
                .map_or(false, |nfr| nfr.numeric_node_ids),
            edge_file_reader.numeric_node_ids,
            node_file_reader
                .as_ref()
                .map_or(false, |nfr| nfr.numeric_node_type_ids),
            node_file_reader
                .as_ref()
                .map_or(false, |nfr| nfr.has_node_types()),
            edge_file_reader.has_edge_types(),
            edge_file_reader.has_edge_weights(),
            // TODO: expose this parameter to use in the future.
            true,
            node_file_reader
                .as_ref()
                .map_or(false, |nfr| nfr.might_contain_singletons),
            edge_file_reader.might_contain_singletons_with_selfloops,
            edge_file_reader.might_contain_trap_nodes,
            edge_file_reader.reader.verbose,
        )
    }
}
