use super::*;

impl Graph {
    /// Return graph renderized from given files.
    ///
    /// # Arguments
    ///
    /// * `edge_file_reader`: EdgeFileReader - Reader of the edge file.
    /// * `node_file_reader`: Option<NodeFileReader> - Reader of the node file.
    /// * `directed`: bool - Wethever the graph is to be read as directed or undirected.
    /// * `directed_edge_list`: bool - Wether to read the edge list as directed.
    /// * `ignore_duplicated_nodes`: bool - Wethever to ignore duplicated nodes while reading.
    /// * `ignore_duplicated_edges`: bool - Wethever to ignore duplicated edges while reading.
    /// * `skip_self_loops`: bool - Wethever to skip self-loops while reading the edge file.
    pub fn from_sorted_csv<S: Into<String>>(
        edge_file_reader: EdgeFileReader,
        node_file_reader: Option<NodeFileReader>,
        directed: bool,
        directed_edge_list: bool,
        edges_number: EdgeT,
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
            match &node_file_reader {
                Some(nfr) => nfr.reader.ignore_duplicates,
                None => false,
            },
            edge_file_reader.reader.ignore_duplicates,
            edges_number,
            nodes_number,
            edge_file_reader.numeric_edge_type_ids,
            match &node_file_reader {
                Some(nfr) => nfr.numeric_node_ids,
                None => false,
            },
            edge_file_reader.numeric_node_ids,
            match &node_file_reader {
                Some(nfr) => nfr.numeric_node_type_ids,
                None => false,
            },
            match &node_file_reader {
                Some(nfr) => nfr.has_node_types(),
                None => false,
            },
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
    /// * `directed`: bool - Wethever the graph is to be read as directed or undirected.
    /// * `directed_edge_list`: bool - Wether to read the edge list as directed.
    /// * `ignore_duplicated_nodes`: bool - Wethever to ignore duplicated nodes while reading.
    /// * `ignore_duplicated_edges`: bool - Wethever to ignore duplicated edges while reading.
    /// * `skip_self_loops`: bool - Wethever to skip self-loops while reading the edge file.
    pub fn from_unsorted_csv<S: Into<String>>(
        edge_file_reader: EdgeFileReader,
        node_file_reader: Option<NodeFileReader>,
        directed: bool,
        directed_edge_list: bool,
        name: S,
    ) -> Result<Graph, String> {
        Graph::from_string_unsorted(
            edge_file_reader.read_lines()?,
            match &node_file_reader {
                Some(nfr) => Some(nfr.read_lines()?),
                None => None,
            },
            directed,
            directed_edge_list,
            name,
            match &node_file_reader {
                Some(nfr) => nfr.reader.ignore_duplicates,
                None => false,
            },
            edge_file_reader.reader.ignore_duplicates,
            edge_file_reader.reader.verbose,
            edge_file_reader.numeric_edge_type_ids,
            match &node_file_reader {
                Some(nfr) => nfr.numeric_node_ids,
                None => false,
            },
            edge_file_reader.numeric_node_ids,
            match &node_file_reader {
                Some(nfr) => nfr.numeric_node_type_ids,
                None => false,
            },
            match &node_file_reader {
                Some(nfr) => nfr.has_node_types(),
                None => false,
            },
            edge_file_reader.has_edge_types(),
            edge_file_reader.has_weights(),
        )
    }
}
