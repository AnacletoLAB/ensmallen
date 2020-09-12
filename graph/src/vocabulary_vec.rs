use super::*;
use derive_getters::Getters;

#[derive(Debug, Clone, Getters, PartialEq)]
pub(crate) struct VocabularyVec<IndexT: ToFromUsize> {
    pub(crate) ids: Vec<IndexT>,
    pub(crate) vocabulary: Vocabulary<IndexT>,
}

impl<IndexT: ToFromUsize + Clone + Copy> VocabularyVec<IndexT> {
    pub fn new() -> VocabularyVec<IndexT> {
        VocabularyVec {
            ids: Vec::new(),
            vocabulary: Vocabulary::new(),
        }
    }

    pub fn insert(&mut self, value: String) -> IndexT {
        self.vocabulary.insert(value.clone());
        let id = *self.get(&value).unwrap();
        self.ids.push(id);
        id
    }

    pub fn is_empty(&self) -> bool {
        self.vocabulary.is_empty()
    }

    pub fn translate(&self, id: IndexT) -> &str {
        self.vocabulary.translate(id)
    }

    pub fn get(&self, value: &str) -> Option<&IndexT> {
        self.vocabulary.get(value)
    }

    pub fn contains_key(&self, value: &str) -> bool {
        self.vocabulary.contains_key(value)
    }

    pub fn len(&self) -> usize {
        self.vocabulary.len()
    }
}
