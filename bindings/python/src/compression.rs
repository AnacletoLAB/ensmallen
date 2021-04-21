use super::*;
use graph::{EdgeT, NodeT};

#[pymethods]
impl EnsmallenGraph {
    #[text_signature = "($self, edge)"]
    /// Returns source and destination nodes corresponding to given edge ID.
    ///
    /// Parameters
    /// --------------
    /// edge: int,
    /// 	The edge value to decode.
    ///
    /// [Automatically generated binding]
    /// [Automatically generated documentation]
    fn decode_edge(&self, edge: u64) -> (NodeT, NodeT) {
        self.graph.decode_edge(edge)
    }

    #[text_signature = "($self, src, dst)"]
    /// Return edge value corresponding to given node IDs.
    ///
    /// Parameters
    /// --------------
    /// src: int,
    /// 	The source node ID.
    /// dst: int,
    /// 	The destination node ID.
    ///
    /// [Automatically generated binding]
    /// [Automatically generated documentation]
    fn encode_edge(&self, src: NodeT, dst: NodeT) -> u64 {
        self.graph.encode_edge(src, dst)
    }

    #[text_signature = "($self)"]
    /// Return maximum encodable edge number.
    ///
    /// [Automatically generated binding]
    /// [Automatically generated documentation]
    fn get_max_encodable_edge_number(&self) -> EdgeT {
        self.graph.get_max_encodable_edge_number()
    }
}
