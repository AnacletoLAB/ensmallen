use super::*;
use derive_getters::Getters;

#[derive(Debug, Clone, Getters, PartialEq, Default)]
pub struct VocabularyVec<IndexT: ToFromUsize> {
    pub ids: Vec<IndexT>,
    pub vocabulary: Vocabulary<IndexT>,
}

impl<IndexT: ToFromUsize + Clone + Copy> VocabularyVec<IndexT> {
    /// Returns id of given value inserted.
    ///
    /// # Arguments
    ///
    /// * `value`: String - The value to be inserted.
    pub fn insert(&mut self, value: String) -> IndexT {
        self.vocabulary.insert(value.clone());
        let id = *self.get(&value).unwrap();
        self.ids.push(id);
        id
    }

    /// Returns wethever the value is empty or not.
    pub fn is_empty(&self) -> bool {
        self.vocabulary.is_empty()
    }

    /// Returns string name of given id.
    ///
    /// # Arguments
    ///
    /// * `id`: IndexT - Id to be translated.
    pub fn translate(&self, id: IndexT) -> &str {
        self.vocabulary.translate(id)
    }

    /// Return the id of given key.
    ///
    /// # Arguments
    ///
    /// * `key`: &str - the key whose Id is to be retrieved.
    pub fn get(&self, key: &str) -> Option<&IndexT> {
        self.vocabulary.get(key)
    }

    /// Return boolean representing if given key is present.
    ///
    /// # Arguments
    ///
    /// * `key`: &str - the key to check existance of.
    pub fn contains_key(&self, key: &str) -> bool {
        self.vocabulary.contains_key(key)
    }

    /// Return length of the vocabulary.    
    pub fn len(&self) -> usize {
        self.vocabulary.len()
    }
}
