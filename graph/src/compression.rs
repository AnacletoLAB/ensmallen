use super::*;

#[inline(always)]
pub(crate) fn encode_edge(src: NodeT, dst: NodeT, node_bits: u8) -> u64 {
    ((src << node_bits) | dst) as u64
}

#[inline(always)]
pub(crate) fn decode_edge(edge: u64, node_bits: u8, node_bit_mask: u64) -> (NodeT, NodeT) {
    ((edge >> node_bits) as NodeT, (edge & node_bit_mask) as NodeT)
}

#[inline(always)]
pub(crate) fn get_node_bits(top_node: NodeT) -> u8 {
    (1.0 + top_node as f64).log2().ceil() as u8
}

impl Graph {
    #[inline(always)]
    pub(crate) fn encode_edge(&self, src: NodeT, dst: NodeT) -> u64 {
        encode_edge(src, dst, self.node_bits)
    }

    #[inline(always)]
    pub(crate) fn decode_edge(&self, edge: u64) -> (NodeT, NodeT) {
        decode_edge(edge, self.node_bits, self.node_bit_mask)
    }

    #[inline(always)]
    pub(crate) fn get_edge_from_edge_id(&self, edge_id: EdgeT) -> (NodeT, NodeT) {
        let edge = self.edges.unchecked_select(edge_id as u64);
        self.decode_edge(edge)
    }

    #[inline(always)]
    pub(crate) fn get_edge_from_tuple(&self, src: NodeT, dst: NodeT) -> Option<EdgeT> {
        self.edges.rank(self.encode_edge(src, dst)).map(|value| value as EdgeT)
    }

    #[inline(always)]
    pub(crate) fn get_unchecked_edge_from_tuple(&self, src: NodeT, dst: NodeT) -> EdgeT {
        self.edges.unchecked_rank(self.encode_edge(src, dst)) as EdgeT
    }

    #[inline(always)]
    pub(crate) fn get_unique_source(&self, source_id:NodeT) -> NodeT {
        self.unique_sources.unchecked_select((source_id % self.unique_sources.len() as NodeT) as u64) as NodeT
    }
}
