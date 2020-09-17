use super::*;

impl Graph {
    /// Return graph renderized from given files.
    ///
    /// # Arguments
    ///
    /// * `edge_file_reader`: EdgeFileReader - Reader of the edge file.
    /// * `node_file_reader`: Option<NodeFileReader> - Reader of the node file.
    /// * `directed`: bool - Wethever the graph is to be read as directed or undirected.
    /// * `ignore_duplicated_nodes`: bool - Wethever to ignore duplicated nodes while reading.
    /// * `ignore_duplicated_edges`: bool - Wethever to ignore duplicated edges while reading.
    /// * `skip_self_loops`: bool - Wethever to skip self-loops while reading the edge file.
    pub fn from_csv(
        edge_file_reader: EdgeFileReader,
        node_file_reader: Option<NodeFileReader>,
        directed: bool,
    ) -> Result<Graph, String> {
        Graph::new(
            edge_file_reader.read_lines()?,
            match &node_file_reader {
                Some(nfr) => Some(nfr.read_lines()?),
                None => None,
            },
            directed,
            match &node_file_reader {
                Some(nfr) => nfr.reader.ignore_duplicates,
                None => false,
            },
            edge_file_reader.reader.ignore_duplicates,
            edge_file_reader.skip_self_loops,
        )
    }
}
