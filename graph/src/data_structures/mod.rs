use crate::types::*;
use std::hash::{Hash, Hasher};
use std::intrinsics::unlikely;

mod builder;
pub(crate) use builder::*;

mod iter;
pub use iter::*;

mod edges_iter;
pub use edges_iter::*;

mod par_iter;
pub use par_iter::*;

#[derive(Debug)]
pub(crate) struct CSR {
    pub(crate) outbounds_degrees: Vec<EdgeT>,
    pub(crate) destinations: Vec<NodeT>,
    /// Vector of sources to execute fast link prediction sequences if required.
    sources: Option<Vec<NodeT>>,
}

impl Hash for CSR {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.outbounds_degrees.hash(state);
        self.destinations.hash(state);
    }
}

impl CSR {
    pub fn new() -> Self {
        CSR {
            outbounds_degrees: Vec::new(),
            destinations: Vec::new(),
            sources: None,
        }
    }

    /// Return the used memory in bytes
    pub fn memory_stats(&self) -> usize {
        core::mem::size_of::<Vec<EdgeT>>()
            + core::mem::size_of::<EdgeT>() * self.outbounds_degrees.len()
            + core::mem::size_of::<Vec<NodeT>>()
            + core::mem::size_of::<NodeT>() * self.destinations.len()
            + core::mem::size_of::<Option<Vec<NodeT>>>()
            + core::mem::size_of::<NodeT>() * self.sources.as_ref().map(|s| s.len()).unwrap_or(0)
    }

    #[inline(always)]
    pub fn get_number_of_directed_edges(&self) -> EdgeT {
        self.destinations.len() as EdgeT
    }

    #[inline(always)]
    pub fn get_number_of_nodes(&self) -> NodeT {
        self.outbounds_degrees.len() as NodeT - 1
    }

    #[inline(always)]
    pub fn get_cumulative_node_degrees(&self) -> &[EdgeT] {
        &self.outbounds_degrees[1..]
    }

    #[inline(always)]
    pub fn has_sources_tradeoff_enabled(&self) -> bool {
        self.sources.is_some()
    }

    pub fn enable_sources(&mut self) {
        self.sources = Some(
            self.outbounds_degrees
                .windows(2)
                .enumerate()
                .flat_map(|(src, outbounds_tuple)| {
                    let start: usize = outbounds_tuple[0] as usize;
                    let end: usize = outbounds_tuple[1] as usize;
                    (start..end).map(move |_| src as NodeT)
                })
                .collect(),
        );
    }

    pub fn disable_sources(&mut self) {
        self.sources = None;
    }

    pub unsafe fn get_unchecked_node_ids_from_edge_id(&self, edge_id: EdgeT) -> (NodeT, NodeT) {
        (
            self.get_unchecked_source_node_id_from_edge_id(edge_id),
            self.get_unchecked_destination_node_id_from_edge_id(edge_id),
        )
    }

    pub unsafe fn get_unchecked_source_node_id_from_edge_id(&self, edge_id: EdgeT) -> NodeT {
        self.outbounds_degrees.partition_point(|&x| x <= edge_id) as NodeT - 1
    }

    pub unsafe fn get_unchecked_destination_node_id_from_edge_id(&self, edge_id: EdgeT) -> NodeT {
        self.destinations[edge_id as usize]
    }

    pub unsafe fn get_unchecked_edge_id_from_node_ids(&self, src: NodeT, dst: NodeT) -> EdgeT {
        let (min_edge_id, max_edge_id) =
            self.get_unchecked_minmax_edge_ids_from_source_node_id(src);

        let neighbour_idx = match self.destinations[min_edge_id as usize..max_edge_id as usize]
            .binary_search(&dst)
        {
            Ok(idx) => idx,  // the edge exists
            Err(idx) => idx, // the edge doesn't exists so this is the smallest edge_id bigger than where it would be
        };

        min_edge_id + neighbour_idx as EdgeT
    }

    pub fn get_edge_id_from_node_ids(&self, src: NodeT, dst: NodeT) -> Result<EdgeT> {
        if unlikely(src >= self.get_number_of_nodes()) {
            return Err("".into());
        }
        if unlikely(dst >= self.get_number_of_nodes()) {
            return Err("".into());
        }
        let (min_edge_id, max_edge_id) =
            unsafe { self.get_unchecked_minmax_edge_ids_from_source_node_id(src) };

        Ok(min_edge_id
            + (match self.destinations[min_edge_id as usize..max_edge_id as usize]
                .binary_search(&dst)
            {
                Ok(neighbour_idx) => Ok::<_, String>(neighbour_idx),
                Err(_) => Err("".into()),
            })? as EdgeT)
    }

    #[inline(always)]
    pub unsafe fn get_unchecked_minmax_edge_ids_from_source_node_id(
        &self,
        src: NodeT,
    ) -> (EdgeT, EdgeT) {
        (
            self.outbounds_degrees[src as usize],
            self.outbounds_degrees[src as usize + 1],
        )
    }

    pub unsafe fn get_unchecked_neighbours_node_ids_from_src_node_id(
        &self,
        src: NodeT,
    ) -> &[NodeT] {
        let (min_edge_id, max_edge_id) =
            self.get_unchecked_minmax_edge_ids_from_source_node_id(src);
        &self.destinations[min_edge_id as usize..max_edge_id as usize]
    }
}
