use super::*;

use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

impl EdgeTypeVocabulary {
    fn compute_hash(&self) -> u64 {
        let mut hasher = DefaultHasher::new();
        self.hash(&mut hasher);
        hasher.finish()
    }
}

impl PartialEq for EdgeTypeVocabulary {
    fn eq(&self, other: &Self) -> bool {
        self.compute_hash() == other.compute_hash()
    }
}

#[derive(Debug, Clone)]
pub struct EdgeTypeVocabulary {
    pub ids: Vec<Option<EdgeTypeT>>,
    pub vocabulary: Vocabulary<EdgeTypeT>,
    pub counts: Vec<EdgeT>,
    pub unknown_count: EdgeT,
}

impl EdgeTypeVocabulary {
    pub fn default() -> EdgeTypeVocabulary {
        EdgeTypeVocabulary {
            ids: Vec::new(),
            vocabulary: Vocabulary::default(),
            counts: Vec::new(),
            unknown_count: EdgeT::from_usize(0),
        }
    }

    pub fn from_structs(
        ids: Vec<Option<EdgeTypeT>>,
        vocabulary: Vocabulary<EdgeTypeT>,
    ) -> EdgeTypeVocabulary {
        let mut vocabvec = EdgeTypeVocabulary {
            ids,
            vocabulary,
            counts: Vec::new(),
            unknown_count: EdgeT::from_usize(0),
        };

        vocabvec.build_counts();

        vocabvec
    }

    pub fn from_option_structs(
        ids: Option<Vec<Option<EdgeTypeT>>>,
        vocabulary: Option<Vocabulary<EdgeTypeT>>,
    ) -> Option<EdgeTypeVocabulary> {
        if let (Some(ids), Some(vocabulary)) = (ids, vocabulary) {
            Some(EdgeTypeVocabulary::from_structs(ids, vocabulary))
        } else {
            None
        }
    }

    pub fn build_counts(&mut self) {
        self.counts = vec![EdgeT::from_usize(0); self.vocabulary.len()];
        for index in self.ids.iter() {
            match index {
                Some(value) => {
                    self.counts[*value as usize] += 1;
                }
                None => self.unknown_count += EdgeT::from_usize(1),
            }
        }
    }

    /// Returns whether the value is empty or not.
    pub fn is_empty(&self) -> bool {
        self.vocabulary.is_empty()
    }

    /// Returns string name of given id.
    ///
    /// # Arguments
    ///
    /// * `id`: EdgeTypeT - Id to be translated.
    pub fn unchecked_translate(&self, id: EdgeTypeT) -> String {
        self.vocabulary.unchecked_translate(id)
    }

    /// Returns string name of given id.
    ///
    /// # Arguments
    ///
    /// * `id`: EdgeTypeT - Id to be translated.
    pub fn translate(&self, id: EdgeTypeT) -> Result<&String, String> {
        self.vocabulary.translate(id)
    }

    /// Return the id of given key.
    ///
    /// # Arguments
    ///
    /// * `key`: &str - the key whose Id is to be retrieved.
    pub fn get(&self, key: &str) -> Option<&EdgeTypeT> {
        self.vocabulary.get(key)
    }

    /// Return vector of keys of the map.
    pub fn keys(&self) -> Vec<String> {
        self.vocabulary.keys()
    }

    /// Return length of the vocabulary.    
    pub fn len(&self) -> usize {
        self.counts.len()
    }

    /// Set wether to load IDs as numeric.
    ///
    /// # Arguments
    /// * numeric_ids: bool - Wether to load the IDs as numeric
    ///
    pub fn set_numeric_ids(mut self, numeric_ids: bool) -> EdgeTypeVocabulary {
        self.vocabulary = self.vocabulary.set_numeric_ids(numeric_ids);
        self
    }

    /// Returns number of unknown edges.
    pub fn get_unknown_count(&self) -> EdgeT {
        self.unknown_count
    }

    /// Returns number of minimum edge-count.
    pub fn min_edge_type_count(&self) -> EdgeT {
        *self.counts.iter().min().unwrap_or(&0)
    }
}
