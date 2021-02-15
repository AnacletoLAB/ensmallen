use super::*;
use rayon::iter::IntoParallelRefIterator;
use rayon::iter::ParallelIterator;
use std::collections::HashMap;

impl Graph {
    /// Replace all edge types (if present) and set all the edge to edge_type.
    ///
    /// # Arguments
    /// - `edge_type`: String - The edge type to assing to all the edges.
    pub fn set_all_edge_types(mut self, edge_type: String) -> Graph {
        let mut vocabulary = Vocabulary::default();
        vocabulary.insert(edge_type).unwrap();
        vocabulary.build_reverse_mapping().unwrap();
        let edge_types = VocabularyVec::from_structs(
            vec![0; self.get_edges_number() as usize],
            Some(vocabulary),
        );
        self.edge_types = edge_types;
        self
    }

    /// Replace all node types (if present) and set all the node to node_type.
    ///
    /// # Arguments
    /// - `node_type`: String - The node type to assing to all the nodes.
    pub fn set_all_node_types(mut self, node_type: String) -> Graph {
        let mut vocabulary = Vocabulary::default();
        vocabulary.insert(node_type).unwrap();
        vocabulary.build_reverse_mapping().unwrap();
        let node_types = VocabularyVec::from_structs(
            vec![0; self.get_nodes_number() as usize],
            Some(vocabulary),
        );
        self.node_types = node_types;
        self
    }

    /// Enable extra perks that buys you time as you accept to spend more memory.
    ///
    /// # Arguments
    /// * `vector_sources`: bool, wether to cache sources into a vector for faster walks.
    /// * `vector_destinations`: bool, wether to cache destinations into a vector for faster walks.
    /// * `vector_outbounds`: bool, wether to cache outbounds into a vector for faster walks.
    /// * `cache_size`: Option<f64>, percentage of nodes destinations to cache. This cannot be used with the vector destinations.
    pub fn enable(
        &mut self,
        vector_sources: bool,
        vector_destinations: bool,
        vector_outbounds: bool,
        cache_size: Option<f64>,
    ) -> Result<(), String> {
        if vector_destinations {
            if self.destinations.is_none() {
                self.destinations = Some(self.get_destinations(true));
            }
        } else {
            self.destinations = None;
        }
        if vector_sources {
            if self.sources.is_none() {
                self.sources = Some(self.get_sources(true));
            }
        } else {
            self.sources = None;
        }
        if vector_outbounds {
            if self.outbounds.is_none() {
                self.outbounds = Some(self.get_outbounds());
            }
        } else {
            self.outbounds = None;
        }
        if let Some(cs) = cache_size {
            if vector_destinations {
                return Err("You cannot use cache if you enable the destinations vector".to_owned());
            }
            if cs <= 0.0 || cs >= 1.0 {
                return Err("Cache size must be between strictly 0 and 1, otherwise just enable the destinations vector.".to_owned());
            }
            let cached_nodes_number: NodeT = (self.get_nodes_number() as f64 * cs) as NodeT;
            if cached_nodes_number == 0 || cached_nodes_number == self.get_nodes_number() {
                return Err("Required cached nodes number cannot be 0 or all the nodes.".to_owned());
            }
            self.cached_destinations = Some(
                self.get_top_k_central_nodes(cached_nodes_number)
                    .par_iter()
                    .map(|node_id| {
                        (
                            *node_id,
                            self.get_neighbours_range(*node_id)
                                .collect::<Vec<NodeT>>(),
                        )
                    })
                    .collect::<HashMap<NodeT, Vec<NodeT>>>(),
            );
        } else {
            self.cached_destinations = None;
        }
        Ok(())
    }

    /// Disable all extra perks, reducing memory impact but incresing time requirements.
    pub fn disable_all(&mut self) {
        self.destinations = None;
        self.sources = None;
        self.outbounds = None;
        self.cached_destinations = None;
    }
}
