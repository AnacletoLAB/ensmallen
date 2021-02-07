use super::types::*;
use arbitrary::Arbitrary;
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Arbitrary)]
pub struct Vocabulary<IndexT: ToFromUsize> {
    pub map: HashMap<String, IndexT>,
    pub reverse_map: Vec<String>,
    pub numeric_ids: bool,
}

impl<IndexT: ToFromUsize> Vocabulary<IndexT> {
    pub fn default() -> Vocabulary<IndexT> {
        Vocabulary {
            map: HashMap::new(),
            reverse_map: Vec::new(),
            numeric_ids: false,
        }
    }

    /// Returns id of given value inserted.
    ///
    /// # Arguments
    ///
    /// * `value`: String - The value to be inserted.
    pub fn insert<S: AsRef<str>>(&mut self, value: S) -> Result<IndexT, String> {
        let value = value.as_ref();
        if !self.map.contains_key(value) {
            self.map.insert(
                value.to_string(),
                IndexT::from_usize(if self.numeric_ids {
                    match value.parse::<usize>() {
                        Ok(val) => Ok(val),
                        Err(_) => Err(format!("The given ID `{}` is not numeric.", value)),
                    }?
                } else {
                    self.map.len()
                }),
            );
        }
        Ok(*self.get(value).unwrap())
    }

    /// Compute the reverse mapping vector for fast decoding
    pub fn build_reverse_mapping(&mut self) -> Result<(), String> {
        self.reverse_map = vec!["".to_string(); self.map.len()];
        for (k, v) in self.map.iter() {
            if *v >= IndexT::from_usize(self.map.len()) {
                return Err(format!(
                    concat!(
                        "The given set of values is not dense. Found the tuple k:{} v:{} ",
                        "which has index bigger than the number of elements in the map {}."
                    ),
                    k,
                    v,
                    self.map.len()
                ));
            }
            self.reverse_map[IndexT::to_usize(*v)] = k.clone();
        }
        Ok(())
    }

    /// Returns wethever the value is empty or not.
    pub fn is_empty(&self) -> bool {
        self.map.is_empty()
    }

    /// Returns string name of given id.
    ///
    /// # Arguments
    ///
    /// * `id`: IndexT - Id to be translated.
    pub fn translate(&self, id: IndexT) -> &str {
        &self.reverse_map[IndexT::to_usize(id)]
    }

    /// Return the id of given key.
    ///
    /// # Arguments
    ///
    /// * `key`: &str - the key whose Id is to be retrieved.
    pub fn get(&self, key: &str) -> Option<&IndexT> {
        self.map.get(key)
    }

    /// Return vector of keys of the map.
    pub fn keys(&self) -> Vec<String> {
        self.map.keys().cloned().collect()
    }

    /// Return boolean representing if given key is present.
    ///
    /// # Arguments
    ///
    /// * `key`: &str - the key to check existance of.
    pub fn contains_key(&self, key: &str) -> bool {
        self.map.contains_key(key)
    }

    /// Return length of the vocabulary.
    pub fn len(&self) -> usize {
        self.map.len()
    }

    /// Return boolean representing if values are numeric.
    pub fn has_numeric_ids(&self) -> bool {
        self.numeric_ids
    }

    /// Set wether to load IDs as numeric.
    ///
    /// # Arguments
    /// * numeric_ids: bool - Wether to load the IDs as numeric
    ///
    pub fn set_numeric_ids(mut self, numeric_ids: bool) -> Vocabulary<IndexT> {
        self.numeric_ids = numeric_ids;
        self
    }
}
