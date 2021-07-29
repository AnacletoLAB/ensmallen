use crate::graph::Graph;
use shared::*;

#[inline(always)]
pub(crate) fn encode_edge(src: NodeT, dst: NodeT, node_bits: u8) -> EdgeT {
    ((src as EdgeT) << node_bits) | dst as EdgeT
}

#[inline(always)]
pub(crate) fn encode_max_edge(node: NodeT, node_bits: u8) -> EdgeT {
    ((node as EdgeT) << node_bits) | node as EdgeT
}

#[inline(always)]
pub(crate) fn decode_edge(edge: u64, node_bits: u8, node_bit_mask: u64) -> (NodeT, NodeT) {
    (
        (edge >> node_bits) as NodeT,
        (edge & node_bit_mask) as NodeT,
    )
}

#[inline(always)]
pub(crate) fn get_node_bits(top_node: NodeT) -> u8 {
    (1.0 + top_node as f64).log2().ceil() as u8
}

impl Graph {
    #[inline(always)]
    /// Return edge value corresponding to given node IDs.
    ///
    /// # Arguments
    /// * `src`: NodeT - The source node ID.
    /// * `dst`: NodeT - The destination node ID.
    pub fn encode_edge(&self, src: NodeT, dst: NodeT) -> u64 {
        encode_edge(src, dst, self.node_bits)
    }

    #[inline(always)]
    /// Returns source and destination nodes corresponding to given edge ID.
    ///
    /// # Arguments
    /// * `edge`: u64 - The edge value to decode.
    pub fn decode_edge(&self, edge: u64) -> (NodeT, NodeT) {
        decode_edge(edge, self.node_bits, self.node_bit_mask)
    }

    /// Return maximum encodable edge number.
    pub fn get_max_encodable_edge_number(&self) -> EdgeT {
        encode_max_edge(
            self.get_nodes_number(),
            get_node_bits(self.get_nodes_number()),
        )
    }
}
