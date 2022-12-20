use super::*;

impl CSR {
    pub unsafe fn iter_unchecked_edge_ids_from_source_node_id(
        &self,
        src: NodeT,
    ) -> std::ops::Range<usize> {
        let (min_edge_id, max_edge_id) =
            self.get_unchecked_minmax_edge_ids_from_source_node_id(src);
        min_edge_id as usize..max_edge_id as usize
    }

    pub unsafe fn iter_unchecked_neighbour_node_ids_from_source_node_id(
        &self,
        src: NodeT,
    ) -> impl Iterator<Item = NodeT> + Send + '_ {
        self.get_unchecked_neighbours_node_ids_from_src_node_id(src)
            .iter()
            .cloned()
}

    pub fn iter_unique_edge_node_ids(
        &self,
        directed: bool,
    ) -> impl Iterator<Item = (NodeT, NodeT)> + '_ {
        // this fails if you have a graph with only this edge, but fuck you
        let mut previous_edge = (NodeT::MAX, NodeT::MAX);
        
        self.iter_edge_node_ids(directed)
            .filter_map(move |(_edge_id, src, dst)| {
                if (src, dst) != previous_edge {
                    previous_edge = (src, dst);
                    Some(previous_edge)
                } else {
                    None
                }
            })
    }

    pub fn iter_directed_edge_node_ids(
        &self,
    ) -> impl Iterator<Item = (EdgeT, NodeT, NodeT)> + Send + '_ {
        self.outbounds_degrees
            .windows(2)
            .enumerate()
            .flat_map(|(src, outbounds_tuple)| {
                let start: usize = outbounds_tuple[0] as usize;
                let end: usize = outbounds_tuple[1] as usize;
                (start..end).map(move |_| src as NodeT)
            })
            .zip(self.destinations.iter().copied())
            .enumerate()
            .map(|(edge_id, (src, dst))| (edge_id as EdgeT, src, dst))
    }

    pub fn iter_edge_node_ids(
        &self,
        directed: bool,
    ) -> impl Iterator<Item = (EdgeT, NodeT, NodeT)> + '_ {
        self.iter_directed_edge_node_ids()
            .filter(move |(_edge_id, src, dst)| {
                directed || src <= dst
            })
    }
}