use super::*;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

#[derive(Debug, Clone)]
pub struct NodeTypeVocabulary {
    /// This is the vector with the node types of each node
    /// Moreover, for the node x it's node type is ids[x]
    /// it's an option since the node might not have the node type
    /// and it contains a vector since we support multiple node types
    /// on the same node
    pub ids: Vec<Option<Vec<NodeTypeT>>>,
    pub vocabulary: Vocabulary<NodeTypeT>,
    pub counts: Vec<NodeT>,
    pub min_count: NodeT,
    pub max_count: NodeT,
    /// Maximum number of node type given to any node.
    /// TODO: update this value in a way that is always correct and minimal.
    pub max_multilabel_count: NodeTypeT,
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

impl Default for NodeTypeVocabulary {
    fn default() -> NodeTypeVocabulary {
        NodeTypeVocabulary {
            ids: Vec::new(),
            vocabulary: Vocabulary::default(),
            counts: Vec::new(),
            min_count: 0,
            max_count: 0,
            max_multilabel_count: 0,
            unknown_count: NodeT::from_usize(0),
            multilabel: false,
        }
    }
}

impl NodeTypeVocabulary {
    pub fn from_structs(
        ids: Vec<Option<Vec<NodeTypeT>>>,
        vocabulary: Vocabulary<NodeTypeT>,
    ) -> NodeTypeVocabulary {
        let multilabel = ids
            .iter()
            .any(|node_types| node_types.as_ref().map_or(false, |nts| nts.len() > 1));
        let mut vocabvec = NodeTypeVocabulary {
            ids,
            vocabulary,
            counts: Vec::new(),
            min_count: 0,
            max_count: 0,
            max_multilabel_count: 0,
            unknown_count: NodeT::from_usize(0),
            multilabel,
        };
        vocabvec.build_counts();
        vocabvec
    }

    pub fn from_option_structs(
        ids: Option<Vec<Option<NodeTypeT>>>,
        vocabulary: Option<Vocabulary<NodeTypeT>>,
    ) -> Option<NodeTypeVocabulary> {
        if let (Some(ids), Some(vocabulary)) = (ids, vocabulary) {
            Some(NodeTypeVocabulary::from_structs(ids, vocabulary))
        } else {
            None
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
        self.update_min_max_count();
    }

    fn update_min_max_count(&mut self) {
        self.min_count = self.counts.iter().cloned().min().unwrap_or(0);
        self.max_count = self.counts.iter().cloned().max().unwrap_or(0);
    }

    /// Computes the reverse terms mapping.
    ///
    /// # Raises
    /// * If the terms mapping is found to be not dense.
    pub fn build_reverse_mapping(&mut self) -> Result<()> {
        self.vocabulary.build_reverse_mapping()
    }

    /// Returns ids of given values inserted.
    ///
    /// This method will crash if improper parameters are used.
    ///
    /// # Arguments
    ///
    /// * `maybe_values`: Option<Vec<S>> - The values to be inserted.
    pub unsafe fn unchecked_insert_values<S: AsRef<str> + Into<String> + std::fmt::Debug>(
        &mut self,
        maybe_values: Option<Vec<S>>,
    ) -> Option<Vec<NodeTypeT>> {
        match maybe_values {
            Some(values) => {
                // Retrieve the ID
                let ids = values
                    .into_iter()
                    .map(|value| self.vocabulary.unchecked_insert(value.into()))
                    .collect::<Vec<NodeTypeT>>();

                self.multilabel = self.multilabel || ids.len() > 1;
                self.max_multilabel_count = self.max_multilabel_count.max(ids.len() as NodeTypeT);

                // Push the sorted IDs
                self.ids.push(Some(ids.clone()));
                Some(ids)
            }
            None => {
                self.ids.push(None);
                None
            }
        }
    }

    /// Returns ids of given values inserted.
    ///
    /// # Arguments
    ///
    /// * `maybe_values`: Option<Vec<S>> - The values to be inserted.
    pub fn insert_values<S: AsRef<str> + std::fmt::Debug>(
        &mut self,
        maybe_values: Option<Vec<S>>,
    ) -> Result<Option<Vec<NodeTypeT>>> {
        Ok(match maybe_values {
            Some(values) => {
                // Check if there is at least one node type
                if values.is_empty() {
                    return Err("The given node types vector is empty.".to_owned());
                }
                // Retrieve the ID
                let mut ids = values
                    .iter()
                    .map(|value| {
                        self.vocabulary
                            .insert(value.as_ref())
                            .map(|values| values.0)
                    })
                    .collect::<Result<Vec<NodeTypeT>>>()?;
                // Sort the slice
                ids.sort_unstable();

                // check for duplicates
                if ids[..ids.len() - 1]
                    .iter()
                    .zip(ids[1..].iter())
                    .any(|(a, b)| a == b)
                {
                    return Err(format!(
                        concat!(
                            "Node with duplicated node types was provided.\n",
                            "Specifically the node types vector of the node is {:?} ",
                        ),
                        values
                    ));
                }
                self.multilabel = self.multilabel || ids.len() > 1;
                self.max_multilabel_count = self.max_multilabel_count.max(ids.len() as NodeTypeT);
                // Push the sorted IDs
                self.ids.push(Some(ids.clone()));
                Some(ids)
            }
            None => {
                self.ids.push(None);
                None
            }
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
    pub fn get_minimum_node_type_count(&self) -> NodeT {
        self.min_count
    }

    /// Returns number of maximum node-count.
    pub fn get_maximum_node_type_count(&self) -> NodeT {
        self.max_count
    }

    /// Returns number of maximum multilabel count.
    ///
    /// This value is the maximum number of multilabel counts
    /// that appear in any given node in the graph.
    pub fn get_maximum_multilabel_count(&self) -> NodeTypeT {
        self.max_multilabel_count
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
    pub fn unchecked_translate(&self, id: NodeTypeT) -> String {
        self.vocabulary.unchecked_translate(id)
    }

    /// Returns string name of given id.
    ///
    /// # Arguments
    ///
    /// * `id`: NodeTypeT - Node Type ID to be translated.
    pub fn translate(&self, id: NodeTypeT) -> Result<String> {
        self.vocabulary.translate(id)
    }

    /// Returns string name of given id.
    ///
    /// # Arguments
    ///
    /// * `ids`: Vec<NodeTypeT> - Node Type IDs to be translated.
    pub fn unchecked_translate_vector(&self, ids: Vec<NodeTypeT>) -> Vec<String> {
        ids.into_iter()
            .map(|id| self.unchecked_translate(id))
            .collect()
    }

    /// Returns string name of given id.
    ///
    /// # Arguments
    ///
    /// * `ids`: Vec<NodeTypeT> - Node Type IDs to be translated.
    pub fn translate_vector(&self, ids: Vec<NodeTypeT>) -> Result<Vec<String>> {
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

    /// Set whether to load IDs as numeric.
    ///
    /// # Arguments
    /// * numeric_ids: bool - Whether to load the IDs as numeric
    ///
    pub fn set_numeric_ids(mut self, numeric_ids: bool) -> NodeTypeVocabulary {
        self.vocabulary = self.vocabulary.set_numeric_ids(numeric_ids);
        self
    }

    /// Remove a node type from the vocabulary
    ///
    /// # Safety
    /// If any of the given values to be removed to not exist in the vocabulary
    /// this method will panic.
    pub unsafe fn unchecked_remove_values(
        &mut self,
        node_type_ids_to_remove: Vec<NodeTypeT>,
    ) -> Vec<Option<usize>> {
        // this assumes that the new ids are obtained by "removing" the values
        // so the new ids will keep the relative ordering between each others
        self.counts = self
            .counts
            .iter()
            .enumerate()
            .filter_map(|(i, v)| {
                if !node_type_ids_to_remove.contains(&(i as NodeTypeT)) {
                    Some(*v)
                } else {
                    None
                }
            })
            .collect();
        self.update_min_max_count();
        self.vocabulary
            .unchecked_remove_values(node_type_ids_to_remove)
    }
}
