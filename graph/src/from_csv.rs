use super::*;

impl Graph {
    /// Return graph renderized from given files.
    ///
    /// # Arguments
    ///
    /// * `edge_file_reader`: EdgeFileReader - Reader of the edge file.
    /// * `node_file_reader`: Option<NodeFileReader> - Reader of the node file.
    /// * `directed`: bool - whether the graph is to be read as directed or undirected.
    /// * `directed_edge_list`: bool - Wether to read the edge list as directed.
    /// * `ignore_duplicated_nodes`: bool - whether to ignore duplicated nodes while reading.
    /// * `ignore_duplicated_edges`: bool - whether to ignore duplicated edges while reading.
    /// * `skip_self_loops`: bool - whether to skip self-loops while reading the edge file.
    pub fn from_sorted_csv<S: Into<String>>(
        edge_file_reader: EdgeFileReader,
        node_file_reader: Option<NodeFileReader>,
        directed: bool,
        directed_edge_list: bool,
        edges_number: usize,
        nodes_number: NodeT,
        name: S,
    ) -> Result<Graph, String> {
        Graph::from_string_sorted(
            edge_file_reader.read_lines()?,
            match &node_file_reader {
                Some(nfr) => Some(nfr.read_lines()?),
                None => None,
            },
            directed,
            directed_edge_list,
            node_file_reader
                .as_ref()
                .map_or(false, |nfr| nfr.reader.ignore_duplicates),
            edge_file_reader.reader.ignore_duplicates,
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
            edge_file_reader.has_weights(),
            name,
        )
    }

    /// Return graph renderized from given files.
    ///
    /// # Arguments
    ///
    /// * `edge_file_reader`: EdgeFileReader - Reader of the edge file.
    /// * `node_file_reader`: Option<NodeFileReader> - Reader of the node file.
    /// * `directed`: bool - whether the graph is to be read as directed or undirected.
    /// * `directed_edge_list`: bool - Wether to read the edge list as directed.
    /// * `ignore_duplicated_nodes`: bool - whether to ignore duplicated nodes while reading.
    /// * `ignore_duplicated_edges`: bool - whether to ignore duplicated edges while reading.
    /// * `skip_self_loops`: bool - whether to skip self-loops while reading the edge file.
    pub fn from_unsorted_csv<S: Into<String>>(
        edge_file_reader: EdgeFileReader,
        node_file_reader: Option<NodeFileReader>,
        directed: bool,
        directed_edge_list: bool,
        name: S,
    ) -> Result<Graph, String> {
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
            edge_file_reader.reader.ignore_duplicates,
            edge_file_reader.reader.verbose,
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
            edge_file_reader.has_weights(),
        )
    }
}
