use super::*;

#[inline(always)]
pub(crate) fn encode_edge(src: NodeT, dst: NodeT, node_bits: u8) -> EdgeT {
    (src << node_bits) | dst
}

#[inline(always)]
pub(crate) fn decode_edge(edge: EdgeT, node_bits: u8, node_bit_mask: EdgeT) -> (NodeT, NodeT) {
    (edge >> node_bits, edge & node_bit_mask)
}

impl Graph {
    pub(crate) fn encode_edge(&self, src: NodeT, dst: NodeT) -> EdgeT {
        encode_edge(src, dst, self.node_bits)
    }

    pub(crate) fn decode_edge(&self, edge: EdgeT) -> (NodeT, NodeT) {
        decode_edge(edge, self.node_bits, self.node_bit_mask)
    }
}
