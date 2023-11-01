#![feature(core_intrinsics)]
use std::hash::{Hash, Hasher};
use std::intrinsics::unlikely;

type EdgeT = u64;
type NodeT = u32;
type Result<T> = std::result::Result<T, String>;

mod builder;
pub use builder::*;

mod iter;

mod edges_iter_upper_triangular;
pub use edges_iter_upper_triangular::*;

mod edges_iter_lower_triangular;

mod trait_triple_to_item;

mod edges_iter;
pub use edges_iter::*;

mod par_iter;

#[derive(Debug)]
pub struct CSR {
    pub outbounds_degrees: Vec<EdgeT>,
    pub destinations: Vec<NodeT>,
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

    /// Returns either the first edge id for the source and destination nodes or where they should be.
    ///
    /// # Arguments
    /// * `src`: NodeT - The source node for which to search for the edge id.
    /// * `dst`: NodeT - The destination node for which to search for the edge id.
    /// * `multigraph`: bool - Whether this is a multigraph, and therefore, the backwards scan is necessary.
    ///
    /// # Returns
    /// An edge id curresponding to the FIRST edge that has the `src` node as source
    /// and the `dst` node as destination. IF such condition is not found, the edge id
    /// curresponding to the expected position is used.
    ///
    /// # Safety
    /// This method returns a value both for existing and non existing edges,
    /// and is valuable when computing things such as the rank of a node.
    pub unsafe fn get_unchecked_edge_id_from_node_ids(
        &self,
        src: NodeT,
        dst: NodeT,
        multigraph: bool,
    ) -> EdgeT {
        // We retrieve the IDs curresponding to where the neighbours
        // of the source node can be found in the destinations vector
        let (min_edge_id, max_edge_id) =
            self.get_unchecked_minmax_edge_ids_from_source_node_id(src);

        // We retrieve the slice reference to that.
        let neighbours = &self.destinations[min_edge_id as usize..max_edge_id as usize];

        // We execute a binary search to find ANY EDGE THAT HAS
        // as source `src` and destination `dst`
        let mut neighbour_idx = match neighbours.binary_search(&dst) {
            Ok(idx) => idx,  // the edge exists
            Err(idx) => idx, // the edge doesn't exists so this is the smallest edge_id bigger than where it would be
        };

        // In a multigraph the edge we have identified is not necessarily the
        // first one, as in a binary search we may endup in a position where
        // we find not the first edge where that is valid.
        // In a multigraph therefore, we need to scan back up until
        // we find the first edge respecting the aforementioned condition.
        while multigraph
            && neighbour_idx > 0
            // We also need to check that the proposed node ID
            // is not OUTSIDE the neighbourhood, as the binary search
            // might suggest that this destination node should be inserted
            // at the end of the vector.
            && neighbour_idx < neighbours.len()
            && neighbours[neighbour_idx - 1] == dst
        {
            neighbour_idx -= 1;
        }

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

    #[inline(always)]
    pub unsafe fn get_unchecked_neighbours_node_ids_from_src_node_id(
        &self,
        src: NodeT,
    ) -> &[NodeT] {
        let (min_edge_id, max_edge_id) =
            self.get_unchecked_minmax_edge_ids_from_source_node_id(src);
        &self.destinations[min_edge_id as usize..max_edge_id as usize]
    }
}
