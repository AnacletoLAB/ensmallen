use crate::constructors::build_graph_from_strings;

use super::*;

impl Graph {
    /// Return graph renderized from given files.
    ///
    /// # Arguments
    /// * `edge_file_reader`: Option<EdgeFileReader> - Reader of the edge file.
    /// * `node_file_reader`: Option<NodeFileReader> - Reader of the node file.
    /// * `node_type_file_reader`: Option<TypeFileReader> - Reader of the node type file.
    /// * `edge_type_file_reader`: Option<TypeFileReader> - Reader of the edge type file.
    /// * `directed`: bool - Whether the graph is to be read as directed or undirected.
    /// * `name`: S - The name for the graph.
    pub fn from_csv<S: Clone + Into<String>>(
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
}
