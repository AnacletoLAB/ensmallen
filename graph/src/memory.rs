use super::*;
use bitvec::prelude::*;
use std::mem::size_of;
use tags::no_binding;

/// Returns the given bytes number as a human readable number.
fn to_human_readable_memory_requirement(bytes_number: usize) -> String {
    format!("{}B", to_human_readable_high_integer(bytes_number))
}

#[derive(Clone, Debug)]
#[no_binding]
pub struct GraphMemoryStats {
    pub edges: usize,
    pub weights: usize,

    pub node_types: Option<NodeTypeVocabularyMemoryStats>,
    pub edge_types: Option<EdgeTypeVocabularyMemoryStats>,
    pub nodes: VocabularyMemoryStats,
    pub cache: usize,

    /// Graph name
    pub name: usize,
    pub connected_nodes: usize,
    pub unique_sources: usize,

    pub metadata: usize,
}

impl GraphMemoryStats {
    /// Return the total memory used
    pub fn total(&self) -> usize {
        self.edges
            + self.weights
            + self.node_types.as_ref().map_or(0, |x| x.total())
            + self.edge_types.as_ref().map_or(0, |x| x.total())
            + self.nodes.total()
            + self.name
            + self.connected_nodes
            + self.unique_sources
            + self.metadata
            + self.cache
    }
}

impl Graph {
    #[no_binding]
    /// Returns the memory usage of all the fields of graph
    /// This methods is intended for internal and testing uses only.
    pub fn memory_stats(&self) -> GraphMemoryStats {
        GraphMemoryStats {
            // Exact main structures
            edges: self.edges.memory_stats(),
            weights: self.get_edge_weights_total_memory_requirements(),
            node_types: self.get_node_types_memory_stats().ok(),
            edge_types: self.get_edge_types_memory_stats().ok(),
            nodes: self.nodes.memory_stats(),

            // Exact metadata
            metadata: size_of::<u8>() + size_of::<u64>() + size_of::<bool>(),
            name: size_of::<String>() + self.name.capacity() * size_of::<char>(),

            cache: unsafe { (*self.cache.get()).total() },

            // Exact caching data
            unique_sources: self
                .unique_sources
                .as_ref()
                .as_ref()
                .map_or(0, |e| e.size()),
            connected_nodes: size_of::<Option<BitVec<u8, Lsb0>>>()
                + self
                    .connected_nodes
                    .as_ref()
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

    /// Returns how many bytes are currently used to store the nodes.
    pub fn get_nodes_total_memory_requirement(&self) -> usize {
        self.nodes.memory_stats().total()
    }

    /// Returns human readable amount of how many bytes are currently used to store the nodes.
    pub fn get_nodes_total_memory_requirement_human_readable(&self) -> String {
        to_human_readable_memory_requirement(self.get_nodes_total_memory_requirement())
    }

    /// Returns how many bytes are currently used to store the edges.
    pub fn get_edges_total_memory_requirement(&self) -> usize {
        self.edges.memory_stats()
    }

    /// Returns human readable amount of how many bytes are currently used to store the edges.
    pub fn get_edges_total_memory_requirement_human_readable(&self) -> String {
        to_human_readable_memory_requirement(self.get_edges_total_memory_requirement())
    }

    /// Returns how many bytes are currently used to store the edge weights.
    pub fn get_edge_weights_total_memory_requirements(&self) -> usize {
        size_of::<Option<Vec<WeightT>>>()
            + self.weights.as_ref().as_ref().map_or(0, |edge_weights| {
                edge_weights.capacity() * size_of::<WeightT>()
            })
    }

    /// Returns human readable amount of how many bytes are currently used to store the edge weights.
    pub fn get_edge_weights_total_memory_requirements_human_readable(&self) -> String {
        to_human_readable_memory_requirement(self.get_edge_weights_total_memory_requirements())
    }

    #[no_binding]
    /// Returns node types memory stats.
    pub fn get_node_types_memory_stats(&self) -> Result<NodeTypeVocabularyMemoryStats> {
        self.must_have_node_types()
            .map(|node_types| node_types.memory_stats())
    }

    /// Returns how many bytes are currently used to store the node types.
    pub fn get_node_types_total_memory_requirements(&self) -> Result<usize> {
        self.get_node_types_memory_stats()
            .map(|node_types_memory_stats| node_types_memory_stats.total())
    }

    /// Returns human readable amount of how many bytes are currently used to store the node types.
    pub fn get_node_types_total_memory_requirements_human_readable(&self) -> Result<String> {
        self.get_node_types_total_memory_requirements()
            .map(|amount| to_human_readable_memory_requirement(amount))
    }

    #[no_binding]
    /// Returns edge types memory stats.
    pub fn get_edge_types_memory_stats(&self) -> Result<EdgeTypeVocabularyMemoryStats> {
        self.must_have_edge_types()
            .map(|edge_types| edge_types.memory_stats())
    }

    /// Returns how many bytes are currently used to store the edge types.
    pub fn get_edge_types_total_memory_requirements(&self) -> Result<usize> {
        self.get_edge_types_memory_stats()
            .map(|edge_types_memory_stats| edge_types_memory_stats.total())
    }

    /// Returns human readable amount of how many bytes are currently used to store the edge types.
    pub fn get_edge_types_total_memory_requirements_human_readable(&self) -> Result<String> {
        self.get_edge_types_total_memory_requirements()
            .map(|amount| to_human_readable_memory_requirement(amount))
    }
}
