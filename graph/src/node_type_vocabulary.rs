use super::*;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

#[derive(Debug, Clone)]
pub struct NodeTypeVocabulary {
    pub ids: Vec<Option<Vec<NodeTypeT>>>,
    pub vocabulary: Vocabulary<NodeTypeT>,
    pub counts: Vec<NodeT>,
    pub unknown_count: NodeT,
    pub multilabel: bool,
}

impl NodeTypeVocabulary {
    fn compute_hash(&self) -> u64 {
        let mut hasher = DefaultHasher::new();
        self.hash(&mut hasher);
        hasher.finish()
    }
}

impl PartialEq for NodeTypeVocabulary {
    fn eq(&self, other: &Self) -> bool {
        self.compute_hash() == other.compute_hash()
    }
}

impl NodeTypeVocabulary {
    pub fn default() -> NodeTypeVocabulary {
        NodeTypeVocabulary {
            ids: Vec::new(),
            vocabulary: Vocabulary::default(),
            counts: Vec::new(),
            unknown_count: NodeT::from_usize(0),
            multilabel: false,
        }
    }

    pub fn from_structs(
        ids: Vec<Option<Vec<NodeTypeT>>>,
        vocabulary: Option<Vocabulary<NodeTypeT>>,
    ) -> Option<NodeTypeVocabulary> {
        match vocabulary {
            Some(vocab) => {
                let multilabel = ids
                .iter()
                .any(|node_types| node_types.as_ref().map_or(false, |nts| nts.len() > 1));
                let mut vocabvec = NodeTypeVocabulary {
                    ids,
                    vocabulary: vocab,
                    counts: Vec::new(),
                    unknown_count: NodeT::from_usize(0),
                    multilabel
                };
                vocabvec.build_counts();
                Some(vocabvec)
            }
            None => None,
        }
    }

    pub fn build_counts(&mut self) {
        let mut counts = vec![NodeT::from_usize(0); self.vocabulary.len()];
        for index in self.ids.iter() {
            match index {
                Some(values) => {
                    values.iter().for_each(|value| {
                        counts[NodeTypeT::to_usize(*value)] += NodeT::from_usize(1)
                    });
                }
                None => self.unknown_count += NodeT::from_usize(1),
            }
        }
        self.counts = counts;
    }

    pub fn build_reverse_mapping(&mut self) -> Result<(), String> {
        self.vocabulary.build_reverse_mapping()
    }

    /// Returns ids of given values inserted.
    ///
    /// # Arguments
    ///
    /// * `maybe_values`: Option<Vec<S>> - The values to be inserted.
    pub fn insert_values<S: AsRef<str>>(
        &mut self,
        maybe_values: Option<Vec<S>>,
    ) -> Result<Option<Vec<NodeTypeT>>, String> {
        Ok(match maybe_values {
            Some(values) => {
                // Retrieve the IDs
                let mut ids = values
                    .iter()
                    .map(|value| {
                        self.vocabulary.insert(value.as_ref())?;
                        Ok(*self.get(value.as_ref()).unwrap())
                    })
                    .collect::<Result<Vec<NodeTypeT>, String>>()?;
                // Sort the slice
                ids.sort();
                // Push the sorted IDs
                self.ids.push(Some(ids.clone()));
                Some(ids)
            }
            None => None,
        })
    }

    /// Returns whether the vocabulary is empty or not.
    pub fn is_empty(&self) -> bool {
        self.vocabulary.is_empty()
    }

    /// Returns whether the node types are multi-label or not.
    pub fn is_multilabel(&self) -> bool {
        self.multilabel
    }

    /// Returns number of minimum node-count.
    pub fn min_node_type_count(&self) -> NodeT {
        *self.counts.iter().min().unwrap()
    }

    /// Returns number of unknown nodes.
    pub fn get_unknown_count(&self) -> NodeT {
        self.unknown_count
    }

    /// Returns string name of given id.
    ///
    /// # Arguments
    ///
    /// * `id`: NodeTypeT - Node Type ID to be translated.
    pub fn translate(&self, id: NodeTypeT) -> &str {
        self.vocabulary.translate(id)
    }

    /// Returns string name of given id.
    ///
    /// # Arguments
    ///
    /// * `ids`: Vec<NodeTypeT> - Node Type IDs to be translated.
    pub fn translate_vector(&self, ids: Vec<NodeTypeT>) -> Vec<&str> {
        ids.into_iter().map(|id| self.translate(id)).collect()
    }

    /// Return the id of given key.
    ///
    /// # Arguments
    ///
    /// * `key`: &str - the key whose Id is to be retrieved.
    pub fn get(&self, key: &str) -> Option<&NodeTypeT> {
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
    pub fn set_numeric_ids(mut self, numeric_ids: bool) -> NodeTypeVocabulary {
        self.vocabulary = self.vocabulary.set_numeric_ids(numeric_ids);
        self
    }
}
