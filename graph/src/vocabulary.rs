use super::types::*;
use derive_getters::Getters;
use std::collections::HashMap;

#[derive(Debug, Clone, Getters, PartialEq, Default)]
pub struct Vocabulary<IndexT: ToFromUsize> {
    pub map: HashMap<String, IndexT>,
    pub reverse_map: Vec<String>,
}

impl<IndexT: ToFromUsize + Clone + Copy> Vocabulary<IndexT> {
    pub fn insert(&mut self, value: String) -> IndexT {
        if !self.map.contains_key(&value) {
            self.map.insert(value.clone(), IndexT::from_usize(self.map.len()));
            self.reverse_map.push(value.clone());
        }
        *self.get(&value).unwrap()
    }

    pub fn is_empty(&self) -> bool {
        self.map.is_empty()
    }

    pub fn translate(&self, id: IndexT) -> &str {
        &self.reverse_map[IndexT::to_usize(id)]
    }

    pub fn get(&self, value: &str) -> Option<&IndexT> {
        self.map.get(value)
    }

    pub fn contains_key(&self, value: &str) -> bool {
        self.map.contains_key(value)
    }

    pub fn len(&self) -> usize {
        self.map.len()
    }
}
