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
#[no_binding]
pub struct EdgeTypeVocabulary {
    pub ids: Vec<Option<EdgeTypeT>>,
    pub vocabulary: Vocabulary<EdgeTypeT>,
    pub counts: Vec<EdgeT>,
    pub unknown_count: EdgeT,
}

#[derive(Debug, Clone)]
#[no_binding]
pub struct EdgeTypeVocabularyMemoryStats {
    pub ids: usize,
    pub vocabulary: VocabularyMemoryStats,
    pub counts: usize,
    pub metadata: usize,
}

impl EdgeTypeVocabularyMemoryStats {
    pub fn total(&self) -> usize {
        self.ids + self.vocabulary.total() + self.counts + self.metadata
    }
}

impl EdgeTypeVocabulary {
    pub fn memory_stats(&self) -> EdgeTypeVocabularyMemoryStats {
        use std::mem::size_of;
        EdgeTypeVocabularyMemoryStats {
            ids: size_of::<Vec<Option<EdgeTypeT>>>()
                + self.ids.capacity() * size_of::<Option<EdgeTypeT>>(),
            vocabulary: self.vocabulary.memory_stats(),
            counts: size_of::<Vec<EdgeT>>() + self.counts.capacity() * size_of::<EdgeT>(),
            metadata: size_of::<EdgeT>(),
        }
    }
}

impl EdgeTypeVocabulary {
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
    pub fn translate(&self, id: EdgeTypeT) -> Result<String> {
        self.vocabulary.translate(id)
    }

    /// Return the id of given key.
    ///
    /// # Arguments
    ///
    /// * `key`: &str - the key whose Id is to be retrieved.
    pub fn get(&self, key: &str) -> Option<EdgeTypeT> {
        self.vocabulary.get(key)
    }

    /// Return a reference to the underlaying ids vector.
    pub fn get_ids(&self) -> &[Option<EdgeTypeT>] {
        self.ids.as_slice()
    }

    /// Return vector of keys of the map.
    pub fn keys(&self) -> Vec<String> {
        self.vocabulary.keys()
    }

    /// Return length of the vocabulary.    
    pub fn len(&self) -> usize {
        self.counts.len()
    }

    /// Returns number of unknown edges.
    pub fn get_unknown_count(&self) -> EdgeT {
        self.unknown_count
    }

    /// Returns number of minimum edge-count.
    pub fn min_edge_type_count(&self) -> EdgeT {
        *self.counts.iter().min().unwrap_or(&0)
    }

    /// Returns number of maximum edge-count.
    /// 
    pub fn max_edge_type_count(&self) -> EdgeT {
        *self.counts.iter().max().unwrap_or(&0)
    }

    /// Remove a edge type from the vocabulary
    ///
    /// # Safety
    /// If any of the given values to be removed to not exist in the vocabulary
    /// this method will panic.
    pub unsafe fn unchecked_remove_values(
        &mut self,
        edge_type_ids_to_remove: Vec<EdgeTypeT>,
    ) -> Vec<Option<usize>> {
        // this assumes that the new ids are obtained by "removing" the values
        // so the new ids will keep the relative ordering between each others
        self.counts = self
            .counts
            .iter()
            .enumerate()
            .filter_map(|(i, v)| {
                if !edge_type_ids_to_remove.contains(&(i as EdgeTypeT)) {
                    Some(*v)
                } else {
                    None
                }
            })
            .collect();

        self.vocabulary
            .unchecked_remove_values(edge_type_ids_to_remove)
    }

    pub fn add_edge_type_name_inplace(&mut self, edge_type_name: String) -> Result<EdgeTypeT> {
        if self.get(&edge_type_name).is_some() {
            return Err(format!(
                concat!("The given edge type name {} already exists in the graph."),
                edge_type_name
            ));
        }
        let edge_type_id = unsafe { self.vocabulary.unchecked_insert(edge_type_name) };
        self.counts.push(0);

        Ok(edge_type_id)
    }
}
