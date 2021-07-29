use shared::*;
use super::Graph;
use readers_and_writers::{EdgeFileWriter, NodeFileWriter};

pub trait DumpGraph {
    fn dump_graph(self: &Self, graph: &Graph) -> Result<()>;
}

impl DumpGraph for EdgeFileWriter {
    /// Write edge file from graph.
    ///  
    /// # Arguments
    /// * `graph`: &Graph - the graph to write out.
    fn dump_graph(&self, graph: &Graph) -> Result<()> {
        let directed: bool = self.directed.unwrap_or_else(|| graph.is_directed());
        self.dump_iterator(
            Some(graph.get_directed_edges_number() as usize),
            graph.iter_edge_node_names_and_edge_type_name_and_edge_weight(directed),
        )
    }
}

impl DumpGraph for NodeFileWriter {
    /// Write nodes to file.
    ///
    /// # Arguments
    ///
    /// * `graph`: &Graph, reference to graph to use.
    fn dump_graph(&self, graph: &Graph) -> Result<()> {
        // If the graph has multiple node labels we need a separator to join them.
        if self.node_types_separator.is_none()
            && graph.has_node_types()
            && graph.has_multilabel_node_types().unwrap()
        {
            return Err(concat!(
                "The current graph instance has multilabel node types ",
                "but no node type separator was provided!"
            )
            .to_string());
        }
        self.dump_iterator(
            Some(graph.get_nodes_number() as usize),
            graph.iter_node_names_and_node_type_names(),
        )
    }
}