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
        vocabulary: Option<Vocabulary<EdgeTypeT>>,
    ) -> Option<EdgeTypeVocabulary> {
        match vocabulary {
            Some(vocab) => {
                let mut vocabvec = EdgeTypeVocabulary {
                    ids,
                    vocabulary: vocab,
                    counts: Vec::new(),
                    unknown_count: EdgeT::from_usize(0),
                };
                vocabvec.build_counts();
                Some(vocabvec)
            }
            None => None,
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

    pub fn build_reverse_mapping(&mut self) -> Result<(), String> {
        self.vocabulary.build_reverse_mapping()
    }

    /// Returns ids of given values inserted.
    ///
    /// # Arguments
    ///
    /// * `maybe_value`: Option<S> - The values to be inserted.
    pub fn insert_value<S: AsRef<str>>(
        &mut self,
        maybe_value: Option<S>,
    ) -> Result<Option<EdgeTypeT>, String> {
        let id: Result<Option<EdgeTypeT>, String> = maybe_value.map_or(Ok(None), |value| {
            Ok(Some(self.vocabulary.insert(value.as_ref())?))
        });
        let id = id?;
        self.ids.push(id);
        Ok(id)
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
    pub fn translate(&self, id: EdgeTypeT) -> &str {
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

    /// Return boolean representing if values are numeric.
    pub fn has_numeric_ids(&self) -> bool {
        self.vocabulary.has_numeric_ids()
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
}
