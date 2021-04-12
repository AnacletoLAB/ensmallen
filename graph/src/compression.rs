use super::*;

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
    pub fn encode_edge(&self, src: NodeT, dst: NodeT) -> u64 {
        encode_edge(src, dst, self.node_bits)
    }

    #[inline(always)]
    pub fn decode_edge(&self, edge: u64) -> (NodeT, NodeT) {
        decode_edge(edge, self.node_bits, self.node_bit_mask)
    }

    #[inline(always)]
    pub(crate) fn get_node_ids_from_edge_id(&self, edge_id: EdgeT) -> (NodeT, NodeT) {
        if let (Some(sources), Some(destinations)) = (&self.sources, &self.destinations) {
            return (sources[edge_id as usize], destinations[edge_id as usize]);
        }
        self.decode_edge(self.edges.unchecked_select(edge_id))
    }

    #[inline(always)]
    pub fn get_edge_id_by_node_ids(&self, src: NodeT, dst: NodeT) -> Result<EdgeT, String> {
        match self
            .edges
            .rank(self.encode_edge(src, dst))
            .map(|value| value as EdgeT) {
                Some(edge_id) => Ok(edge_id),
                None => Err(format!("The edge composed by the source node {} and destination node {} does not exist in this graph.", src, dst))
            }
    }

    #[inline(always)]
    pub(crate) fn get_unchecked_edge_id_from_tuple(&self, src: NodeT, dst: NodeT) -> EdgeT {
        self.edges.unchecked_rank(self.encode_edge(src, dst)) as EdgeT
    }

    #[inline(always)]
    pub(crate) fn get_unique_source(&self, source_id: NodeT) -> NodeT {
        self.unique_sources
            .as_ref()
            .map_or(source_id, |x| x.unchecked_select(source_id as u64) as NodeT)
    }
}
