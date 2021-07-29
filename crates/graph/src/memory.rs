use tags::*;
use shared::*;
use crate::graph::Graph;
use crate::vocabularies::*;
use bitvec::prelude::*;
use elias_fano_rust::EliasFanoMemoryStats;


#[derive(Clone, Debug)]
pub struct GraphMemoryStats {
    pub edges: EliasFanoMemoryStats,
    pub weights: usize,

    pub node_types: Option<NodeTypeVocabularyMemoryStats>,
    pub edge_types: Option<EdgeTypeVocabularyMemoryStats>,
    pub nodes: VocabularyMemoryStats,
    pub cache: usize,

    /// Graph name
    pub name: usize,
    pub connected_nodes: usize,
    pub unique_sources: usize,

    pub destinations: usize,
    pub sources: usize,
    pub cumulative_node_degrees: usize,

    pub metadata: usize,
}

impl GraphMemoryStats {
    /// Return the total memory used
    pub fn total(&self) -> usize {
        self.edges.total()
            + self.weights
            + self.node_types.as_ref().map_or(0, |x| x.total())
            + self.edge_types.as_ref().map_or(0, |x| x.total())
            + self.nodes.total()
            + self.name
            + self.connected_nodes
            + self.unique_sources
            + self.destinations
            + self.sources
            + self.cumulative_node_degrees
            + self.metadata
            + self.cache
    }
}

impl Graph {
    #[no_binding]
    /// Returns the memory usage of all the fields of graph
    /// This methods is intended for internal and testing uses only.
    pub fn memory_stats(&self) -> GraphMemoryStats {
        use std::mem::size_of;
        GraphMemoryStats {
            // Exact main structures
            edges: self.edges.memory_stats(),
            weights: size_of::<Option<Vec<WeightT>>>()
                + self
                    .weights
                    .as_ref()
                    .map_or(0, |v| v.capacity() * size_of::<WeightT>()),

            node_types: self.node_types.as_ref().map(|nt| nt.memory_stats()),
            edge_types: self.edge_types.as_ref().map(|et| et.memory_stats()),
            nodes: self.nodes.memory_stats(),

            // Exact metadata
            metadata: size_of::<u8>() + size_of::<u64>() + size_of::<bool>(),
            name: size_of::<String>() + self.name.capacity() * size_of::<char>(),

            cache: unsafe { (*self.cache.get()).total() },

            // Exact caching data
            destinations: size_of::<Option<Vec<NodeT>>>()
                + self
                    .destinations
                    .as_ref()
                    .map_or(0, |v| v.capacity() * size_of::<NodeT>()),
            sources: size_of::<Option<Vec<NodeT>>>()
                + self
                    .sources
                    .as_ref()
                    .map_or(0, |v| v.capacity() * size_of::<NodeT>()),
            cumulative_node_degrees: size_of::<Option<Vec<EdgeT>>>()
                + self
                    .cumulative_node_degrees
                    .as_ref()
                    .map_or(0, |v| v.capacity() * size_of::<EdgeT>()),

            unique_sources: self.unique_sources.as_ref().map_or(0, |e| e.size()),
            connected_nodes: size_of::<Option<BitVec<Lsb0, u8>>>()
                + self
                    .connected_nodes
                    .as_ref()
                    .map_or(0, |bv| bv.capacity() * size_of::<u8>()),
        }
    }

    /// Returns a string describing the memory usage of all the fields of all the
    /// structures used to store the current graph.
    pub fn get_memory_stats(&self) -> String {
        format!("{:#4?}", self.memory_stats())
    }

    /// Returns how many bytes are currently used to store the given graph.
    pub fn get_total_memory_used(&self) -> usize {
        self.memory_stats().total()
    }
}


impl Graph {
    /// Return vector of edges to be inserted in the holdout.
    pub(crate) fn compute_edge_ids_vector(
        &self,
        edge_id: EdgeT,
        src: NodeT,
        dst: NodeT,
        include_all_edge_types: bool,
    ) -> Vec<EdgeT> {
        if include_all_edge_types {
            let (min_edge_id, max_edge_id) =
                unsafe { self.get_unchecked_minmax_edge_ids_from_node_ids(src, dst) };
            (min_edge_id..max_edge_id).collect::<Vec<EdgeT>>()
        } else {
            vec![edge_id]
        }
    }
}
