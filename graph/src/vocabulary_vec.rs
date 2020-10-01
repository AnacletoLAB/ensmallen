use super::*;

#[derive(Debug, Clone, PartialEq, Default)]
pub struct VocabularyVec<IndexT: ToFromUsize> {
    pub ids: Vec<IndexT>,
    pub vocabulary: Vocabulary<IndexT>,
}

impl<IndexT: ToFromUsize + Clone + Copy + Default> VocabularyVec<IndexT> {
    /// Create a new vocabulary vec from a given vocabulary
    /// if the given vocabulary is none then initialize it with an empty one
    pub fn new(vocabulary: Option<Vocabulary<IndexT>>) -> VocabularyVec<IndexT> {
        if let Some(vocab) = vocabulary {
            VocabularyVec{
                vocabulary: vocab,
                ids: Vec::new(),
            }
        } else {
            VocabularyVec{
                vocabulary: Vocabulary::default(),
                ids: Vec::new(),
            }
        }
    }

    /// Add the id to the vocabulary vector
    ///
    /// # Arguments
    ///
    /// * `id`: IndexT - The Id to insert.
    pub fn add(&mut self, id: IndexT){
        self.ids.push(id)
    }

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

    /// Return vector of keys of the map.
    pub fn keys(&self) -> Vec<String> {
        self.vocabulary.keys()
    }

    /// Return length of the vocabulary.    
    pub fn len(&self) -> usize {
        self.vocabulary.len()
    }
}
