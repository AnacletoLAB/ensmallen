use super::*;

impl Graph {
    pub fn read_csv(
        edge_file_reader: EdgeFileReader,
        node_file_reader: Option<NodeFileReader>,
        directed: bool,
        ignore_duplicated_nodes: bool,
        ignore_duplicated_edges: bool,
        skip_self_loops: bool,
    ) -> Result<Graph, String> {
        Graph::new(
            edge_file_reader.read_lines()?,
            match &node_file_reader {
                Some(nfr) => Some(nfr.read_lines()?),
                None => None,
            },
            directed,
            ignore_duplicated_nodes,
            ignore_duplicated_edges,
            skip_self_loops,
        )
    }    
}
