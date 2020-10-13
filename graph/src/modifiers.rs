use super::*;

impl Graph {
    /// Drop all edge types (if presents) and set all the edge to edge_type.
    /// 
    /// # Arguments
    /// - `edge_type`: String - The edge type to assing to all the edges.
    pub fn set_all_edge_types(&mut self, edge_type: String){
        let mut vocabulary = Vocabulary::new(false);
        vocabulary.insert(edge_type).unwrap();
        vocabulary.build_reverse_mapping().unwrap();
        self.edge_types = VocabularyVec::from_structs( vec![0; self.get_edges_number() as usize], Some(vocabulary));
    }

    /// Enable fast walk, using more memory.
    /// 
    /// # Arguments
    /// - vector_destinations: bool, wether to cache destinations into a vector for faster walks.
    /// - vector_outbounds: bool, wether to cache outbounds into a vector for faster walks.
    pub fn enable_fast_walk(
        &mut self,
        vector_destinations: bool,
        vector_outbounds: bool
    ) {
        if vector_destinations{
            self.destinations = Some(self.get_destinations());
        }
        if vector_outbounds{
            self.outbounds = Some(self.get_outbounds());
        }
    }

    /// Disable fast walk, using less memory.
    pub fn disable_fast_walk(&mut self) {
        self.destinations = None;
    }
}