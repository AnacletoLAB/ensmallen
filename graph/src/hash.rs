use super::*;
use std::collections::hash_map::DefaultHasher;
use std::fmt::Debug;
use std::hash::{Hash, Hasher};

#[inline(always)]
/// Hashing floats is usually a bad idea
/// But we want to know if any weight changed significantly
/// THUS we will hash only the order of magnitude and the
/// first few bits of the mantissa.
///
/// This should be an hash which is kinda robust to float erros.
pub(crate) fn hash_f32<H: Hasher>(x: f32, state: &mut H) {
    // basically we are converting the float to a u32 and
    // clear out the lower bits of the mantissa.
    let mut hack = u32::from_le_bytes(x.to_le_bytes());

    // Clear the lower bits of the mantissa
    //        seeeeeeeemmmmmmmmmmmmmmmmmmmmmmm
    hack &= 0b11111111111111111111000000000000;

    state.write_u32(hack);
}

//#[inline(always)]
/// Hashing floats is usually a bad idea
/// But we want to know if any weight changed significantly
/// THUS we will hash only the order of magnitude and the
/// first few bits of the mantissa.
///
/// This should be an hash which is kinda robust to float erros.
// pub(crate) fn hash_f64<H: Hasher>(x: f64, state: &mut H) {
//     // basically we are converting the float to a u32 and
//     // clear out the lower bits of the mantissa.
//     let mut hack = u64::from_le_bytes(x.to_le_bytes());

//     // Clear the lower bits of the mantissa
//     //        seeeeeeeeeeemmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmm
//     hack &= 0b1111111111111111111100000000000000000000000000000000000000000000;

//     state.write_u64(hack);
// }

impl Graph {
    #[no_binding]
    pub fn compute_hash(&self) -> u64 {
        let mut hasher = DefaultHasher::new();
        self.hash(&mut hasher);
        hasher.finish()
    }
}

impl PartialEq for Graph {
    fn eq(&self, other: &Self) -> bool {
        self.compute_hash() == other.compute_hash()
    }
}

impl Hash for Graph {
    fn hash<H: Hasher>(&self, state: &mut H) {
        // These fields are fundamentals
        self.directed.hash(state);
        self.nodes.hash(state);
        self.edges.hash(state);

        if let Some(ws) = &*self.weights {
            for w in ws {
                hash_f32(*w, state);
            }
        }

        if let Some(nt) = &*self.node_types {
            nt.hash(state);
        }

        if let Some(et) = &*self.edge_types {
            et.hash(state);
        }

        // These fields are derivative from the other ones and thus not needed.
        // self.unique_sources.hash(state);
        // self.unique_selfloop_number.hash(state);
        // self.selfloop_number.hash(state);
        // self.connected_number_of_nodes.hash(state);
        // self.singleton_nodes_with_selfloops_number.hash(state);
        // self.unique_number_of_edges.hash(state);

        // These fields are not meaningfull to hash imho
        // self.name.hash(state);
        // self.singleton_nodes_with_selfloops_number.hash(state);
        // self.sources.hash(state);
        // self.cumulative_node_degrees.hash(state);
        // self.cached_destinations.hash(state);
        // self.embedding.hash(state);
    }
}

impl<IndexT: ToFromUsize + Sync + Debug> Hash for Vocabulary<IndexT> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        // The hashmap is not hashable, so we convert it to a
        // sorted array of tuples.

        let mut vector: Vec<(String, IndexT)> = self.map().into_iter().collect::<Vec<_>>();
        vector.sort();
        vector.hash(state);

        self.keys().hash(state);
    }
}

impl Hash for NodeTypeVocabulary {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.ids.hash(state);
        self.vocabulary.hash(state);
        self.counts.hash(state);
    }
}

impl Hash for EdgeTypeVocabulary {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.ids.hash(state);
        self.vocabulary.hash(state);
        self.counts.hash(state);
    }
}

impl Hash for WalkWeights {
    fn hash<H: Hasher>(&self, state: &mut H) {
        // These fields are fundamentals
        hash_f32(self.return_weight, state);
        hash_f32(self.explore_weight, state);
        hash_f32(self.change_node_type_weight, state);
        hash_f32(self.change_edge_type_weight, state);
    }
}

impl Hash for GraphBuilder {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.edges.hash(state);
        self.nodes.hash(state);
        self.has_node_types.hash(state);
        self.has_edge_types.hash(state);
        self.has_edge_weights.hash(state);
        self.directed.hash(state);
        self.name.hash(state);
        hash_f32(self.default_weight, state);
    }
}

impl Hash for GraphCSVBuilder {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.edges_path.hash(state);
        self.nodes_path.hash(state);
    }
}
