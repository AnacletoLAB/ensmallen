use super::types::*;
use std::collections::HashMap;


pub(crate) struct Vocabolary<IndexT: ToFromUsize> {
    pub(crate) map: HashMap<String, IndexT>,
    pub(crate) reverse_map: Vec<String>,
}

impl<IndexT: ToFromUsize> Vocabolary<IndexT> {
    pub fn new() -> Vocabolary<IndexT> {
        Vocabolary {
            map: HashMap::new(),
            reverse_map: Vec::new(),
        }
    }

    pub fn add(&mut self, value: String) -> IndexT {
        if !self.map.contains_key(&value) {
            self.map.insert(value, IndexT::from_usize(self.map.len()));
            self.reverse_map.push(value);
        }
        *self.get(&value).unwrap()
    }

    pub fn translate(&self, id: IndexT) -> String {
        self.reverse_map[IndexT::to_usize(id)]
    }

    pub fn get(&self, value: &str) -> Option<&IndexT> {
        self.map.get(value)
    }
}
