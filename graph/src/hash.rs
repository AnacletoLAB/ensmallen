use super::*;
use std::hash::{Hash, Hasher};

#[inline(always)]
/// Hashing floats is usually a bad idea
/// But we want to know if any weight changed significantly
/// THUS we will hash only the order of magnitude and the 
/// first few bits of the mantissa.
/// 
/// This should be an hash which is kinda robust to float erros.
fn hash_float<H: Hasher>(x: f32, state: &mut H){

    // basically we are converting the float to a u32 and 
    // clear out the lower bits of the mantissa.
    let mut hack = u32::from_le_bytes(x.to_le_bytes());

    // Clear the lower bits of the mantissa
    //        seeeeeeeemmmmmmmmmmmmmmmmmmmmmmm
    hack &= 0b11111111111111111111000000000000;

    state.write_u32(hack);
}

impl Hash for Graph {
    fn hash<H: Hasher>(&self, state: &mut H) {
        // These fields are fundamentals
        self.directed.hash(state);
        self.nodes.hash(state);
        self.edges.hash(state);

        if let Some(ws) = &self.weights {
            for w in ws {
                hash_float(*w, state);
            }
        }

        if let Some(nt) = &self.node_types {
            nt.hash(state);
        }

        if let Some(et) = &self.edge_types {
            et.hash(state);
        }

        // These fields are derivative from the other ones and thus not needed.
        // self.unique_sources.hash(state);
        // self.node_bits.hash(state);
        // self.node_bit_mask.hash(state);
        // self.unique_self_loop_number.hash(state);
        // self.self_loop_number.hash(state);
        // self.not_singleton_nodes_number.hash(state);
        // self.singleton_nodes_with_self_loops_number.hash(state);
        // self.unique_edges_number.hash(state);

        // These fields are not meaningfull to hash imho
        // self.name.hash(state);
        // self.singleton_nodes_with_self_loops_number.hash(state);
        // self.sources.hash(state);
        // self.outbounds.hash(state);
        // self.cached_destinations.hash(state);
        // self.embedding.hash(state);
    }
}

impl<IndexT: ToFromUsize> Hash for Vocabulary<IndexT> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        // The hashmap is not hashable, so we convert it to a
        // sorted array of tuples.

        let mut vector: Vec<(&String, &IndexT)> = self.map.iter().collect();
        vector.sort();
        vector.hash(state);

        self.reverse_map.hash(state);
        self.numeric_ids.hash(state);
    }
}

impl<IndexT: ToFromUsize, CountT: ToFromUsize> Hash for VocabularyVec<IndexT, CountT> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.ids.hash(state);
        self.vocabulary.hash(state);
        self.counts.hash(state);
    }
}