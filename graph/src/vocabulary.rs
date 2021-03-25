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

    fn normalize_value(&self, value: &str) -> Result<(String, usize), String> {
        Ok(if self.numeric_ids {
            let parsed_value = match value.parse::<usize>() {
                Ok(val) => Ok(val),
                Err(_) => Err(format!(
                    "The given ID `{}` is not a numeric positive integer.",
                    value
                )),
            }?;

            // Check that there are no extra zeros or separators in the number
            // E.g. 000 is not supported since it will be traduced to 0
            if value != parsed_value.to_string() {
                return Err(format!(
                    concat!(
                        "The given ID is numeric but is not symmetric.\n",
                        "Specifically, {} != {} where the first value is the user's one ",
                        "and the second one is the result of parsing the value as an ",
                        " integer and casting back to string."
                    ),
                    value,
                    parsed_value.to_string()
                ));
            }

            (parsed_value.to_string(), parsed_value)
        } else {
            (value.to_string(), self.map.len())
        })
    }

    /// Returns id of given value inserted.
    ///
    /// # Arguments
    ///
    /// * `value`: String - The value to be inserted.
    pub fn insert<S: AsRef<str>>(&mut self, value: S) -> Result<IndexT, String> {
        let value = value.as_ref();

        if value.is_empty() {
            return Err("The value given to the vocabulary was empty".to_string());
        }

        let (normalized_value, index) = self.normalize_value(value)?;

        if !self.map.contains_key(&normalized_value) {
            self.map
                .insert(normalized_value.clone(), IndexT::from_usize(index));
        }

        Ok(*self.get(&normalized_value).unwrap())
    }

    /// Compute the reverse mapping vector for fast decoding
    pub fn build_reverse_mapping(&mut self) -> Result<(), String> {
        if !self.reverse_map.is_empty() {
            panic!("Build reverse mapping called multiple times!");
        }
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
            let i = IndexT::to_usize(*v);
            if !self.reverse_map[i].is_empty() {
                return Err(format!(
                    concat!(
                        "During the building of the reverse mapping, ",
                        "one of the elements of the reverse mapping was attempted ",
                        "to be assigned multiple times. This means that in the map ",
                        "there are multiple nodes with the same id.\n",
                        "In the past this was caused by improper handling of numeric ",
                        "node id.\n",
                        "In this case, the value is {} and its index is {}."
                    ),
                    k, i
                ));
            }
            self.reverse_map[i] = k.clone();
        }
        Ok(())
    }

    /// Returns whether the value is empty or not.
    pub fn is_empty(&self) -> bool {
        self.map.is_empty()
    }

    /// Returns string name of given id.
    ///
    /// # Arguments
    ///
    /// * `id`: IndexT - Id to be translated.
    pub fn unchecked_translate(&self, id: IndexT) -> String {
        self.reverse_map[IndexT::to_usize(id)].clone()
    }

    /// Returns option with string name of given id.
    ///
    /// # Arguments
    ///
    /// * `id`: IndexT - Id to be translated.
    pub fn translate(&self, id: IndexT) -> Result<&String, String> {
        match self.reverse_map.get(IndexT::to_usize(id)) {
            Some(name) => Ok(name),
            None => Err("The requested ID is not available in current dictionary.".to_string()),
        }
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
