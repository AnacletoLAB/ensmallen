use super::types::*;
use arbitrary::Arbitrary;
use std::collections::hash_map::Entry;
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Arbitrary)]
pub struct Vocabulary<IndexT: ToFromUsize> {
    /// TODO: refactor the following to work with shared references
    /// in order to avoid doubling the amount of mempry required to store
    /// the string names into memory.
    pub map: HashMap<String, IndexT>,
    pub reverse_map: Vec<String>,
    pub numeric_ids: bool,
}

#[derive(Debug, Clone)]
pub struct VocabularyMemoryStats {
    pub map: usize,
    pub reverse_map: usize,
    pub metadata: usize,
}

impl VocabularyMemoryStats {
    pub fn total(&self) -> usize {
        self.map + self.reverse_map + self.metadata
    }
}

impl<IndexT: ToFromUsize> Vocabulary<IndexT> {
    pub fn memory_stats(&self) -> VocabularyMemoryStats {
        use std::mem::size_of;
        VocabularyMemoryStats{
            // https://github.com/servo/servo/issues/6908
            map: (self.map.capacity() as f64 * 1.1) as usize 
            * (size_of::<String>() + size_of::<IndexT>() + size_of::<usize>())
            + self.map.keys().map(|s| size_of::<String>() + s.capacity() * size_of::<char>()).sum::<usize>(),
            reverse_map: size_of::<Vec<String>>() + self.reverse_map.iter().map(|s| size_of::<String>() + s.capacity() * size_of::<char>()).sum::<usize>(),
            metadata: size_of::<bool>(),
        }
    }
}

impl<IndexT: ToFromUsize> Vocabulary<IndexT> {
    pub fn default() -> Vocabulary<IndexT> {
        Vocabulary {
            map: HashMap::new(),
            reverse_map: Vec::new(),
            numeric_ids: false,
        }
    }

    pub fn with_capacity(capacity: usize) -> Vocabulary<IndexT> {
        Vocabulary {
            map: HashMap::with_capacity(capacity),
            reverse_map: Vec::new(),
            numeric_ids: false,
        }
    }

    fn normalize_value(&self, value: &str) -> Result<(String, IndexT), String> {
        Ok(if self.numeric_ids {
            let parsed_value = value.parse::<usize>().map_err(|_| {
                format!(
                    "The given ID `{}` is not a numeric positive integer.",
                    value
                )
            })?;

            let string_parsed_value = parsed_value.to_string();

            // Check that there are no extra zeros or separators in the number
            // E.g. 000 is not supported since it will be traduced to 0
            if value != string_parsed_value {
                return Err(format!(
                    concat!(
                        "The given ID is numeric but is not symmetric.\n",
                        "Specifically, {} != {} where the first value is the user's one ",
                        "and the second one is the result of parsing the value as an ",
                        " integer and casting back to string."
                    ),
                    value, string_parsed_value
                ));
            }

            (string_parsed_value, IndexT::from_usize(parsed_value))
        } else {
            (value.to_string(), IndexT::from_usize(self.map.len()))
        })
    }

    /// Returns id of given value inserted.
    ///
    /// # Arguments
    ///
    /// * `value`: String - The value to be inserted.
    pub unsafe fn unchecked_insert(&mut self, value: String) -> IndexT {
        let current_length = self.map.len();
        let numeric_ids = self.numeric_ids;
        *self.map.entry(value).or_insert_with_key(|value| {
            IndexT::from_usize(if numeric_ids {
                unsafe { value.parse::<usize>().unwrap_unchecked() }
            } else {
                current_length
            })
        })
    }

    /// Returns id of given value inserted.
    ///
    /// # Arguments
    ///
    /// * `value`: String - The value to be inserted.
    pub fn insert<S: AsRef<str>>(&mut self, value: S) -> Result<(IndexT, bool), String> {
        let value = value.as_ref();

        if value.is_empty() {
            return Err("The value given to the vocabulary was empty".to_string());
        }

        let (normalized_value, index) = self.normalize_value(value)?;

        Ok(match self.map.entry(normalized_value) {
            Entry::Occupied(extracted_index) => (*extracted_index.get(), true),
            Entry::Vacant(vacant_entry) => (*vacant_entry.insert(index), false),
        })
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
                panic!(
                    concat!(
                        "During the building of the reverse mapping, ",
                        "one of the elements of the reverse mapping was attempted ",
                        "to be assigned multiple times. This means that in the map ",
                        "there are multiple nodes with the same id.\n",
                        "In the past this was caused by improper handling of numeric ",
                        "node id.\n",
                        "In this case, the value is {} and its index is {}."
                    ),
                    k, i,
                );
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
    pub fn translate(&self, id: IndexT) -> Result<String, String> {
        match self.reverse_map.get(IndexT::to_usize(id)) {
            Some(name) => Ok(name.clone()),
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

    /// Set whether to load IDs as numeric.
    ///
    /// # Arguments
    /// * numeric_ids: bool - Whether to load the IDs as numeric
    ///
    pub fn set_numeric_ids(mut self, numeric_ids: bool) -> Vocabulary<IndexT> {
        self.numeric_ids = numeric_ids;
        self
    }

    /// Removegiven values from the vocabulary
    ///
    /// # Arguments
    /// * `type_ids_to_remove`: Vec<IndexT> - The values to be removed.
    ///
    /// # Safety
    /// This method will panic if you try to remove values that do not exist
    /// in the current vocabulary.
    pub unsafe fn unchecked_remove_values(
        &mut self,
        type_ids_to_remove: Vec<IndexT>,
    ) -> Vec<Option<usize>> {
        // compute the new dense mapping of the indices
        let new_type_ids_map = (0..self.reverse_map.len())
            .scan(0, |offset, type_id| {
                if type_ids_to_remove.contains(&IndexT::from_usize(type_id)) {
                    *offset += 1;
                    return Some(None);
                }
                Some(Some(type_id - *offset))
            })
            .collect::<Vec<_>>();

        // update the mapping
        self.map = self
            .map
            .iter()
            .filter_map(|(key, val)| {
                new_type_ids_map[IndexT::to_usize(*val)]
                    .map(|x| (key.clone(), IndexT::from_usize(x)))
            })
            .collect();

        // re-build the reverse mapping
        // since we start from a valid state this should never fail
        // unless there are bugs in the code
        self.reverse_map.clear();
        self.build_reverse_mapping().unwrap();

        new_type_ids_map
    }
}
