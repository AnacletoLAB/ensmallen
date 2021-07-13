use super::types::*;
use rayon::iter::ParallelIterator;
use rayon::iter::{IndexedParallelIterator, IntoParallelIterator};
use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::ops::Range;

#[derive(Debug, Clone, PartialEq)] // Arbitrary
pub enum Vocabulary<IndexT: ToFromUsize + Sync> {
    // If the values are arbitrary and we cannot make any assumptions
    // about them
    String {
        // TODO! avoid duplication and use references
        map: HashMap<String, IndexT>,
        reverse_map: Vec<String>,
    },

    // If the values are in a dense integer range
    Numeric {
        range: Range<usize>,
        count: usize,
    },
}

impl<IndexT: ToFromUsize + Sync> Default for Vocabulary<IndexT> {
    fn default() -> Self {
        Self::new()
    }
}

/// # Constructors
impl<IndexT: ToFromUsize + Sync> Vocabulary<IndexT> {
    pub fn new() -> Vocabulary<IndexT> {
        Vocabulary::String {
            map: HashMap::new(),
            reverse_map: Vec::new(),
        }
    }

    // TODO! properly extend Iterator
    pub fn iter(&self) -> Box<dyn Iterator<Item = (String, IndexT)> + '_> {
        match self {
            Vocabulary::String { reverse_map, .. } => Box::new(
                reverse_map
                    .iter()
                    .enumerate()
                    .map(|(value, key)| (key.clone(), IndexT::from_usize(value))),
            ),
            Vocabulary::Numeric { range, .. } => Box::new(
                range
                    .enumerate()
                    .map(|(value, key)| (format!("{}", key), IndexT::from_usize(value))),
            ),
        }
    }

    pub fn with_capacity(capacity: usize) -> Vocabulary<IndexT> {
        Vocabulary::String {
            map: HashMap::with_capacity(capacity),
            reverse_map: Vec::with_capacity(capacity),
        }
    }

    pub fn from_range(range: Range<IndexT>) -> Vocabulary<IndexT> {
        Vocabulary::Numeric {
            range: Range {
                start: IndexT::to_usize(range.start),
                end: IndexT::to_usize(range.end),
            },
            count: 0,
        }
    }

    pub fn from_reverse_map(mut reverse_map: Vec<String>) -> Result<Vocabulary<IndexT>> {
        let map = reverse_map
            .iter()
            .cloned()
            .enumerate()
            .map(|(i, x)| (x, IndexT::from_usize(i)))
            .collect::<HashMap<String, IndexT>>();

        if map.len() != reverse_map.len() {
            reverse_map.sort_unstable();
            let duplicates = reverse_map
                .into_iter()
                .scan(None, |last_object, object| {
                    let result = if (*last_object)
                        .as_ref()
                        .map_or(false, |&last_object| last_object == object)
                    {
                        None
                    } else {
                        Some(object)
                    };
                    (*last_object).insert(object);
                    result
                })
                .collect::<Vec<String>>();
            return Err(format!(
                concat!(
                    "Duplicated values found while building the vocabulary!\n",
                    "Specifically the duplicated values are:\n{:#4?}."
                ),
                duplicates
            ));
        }

        Ok(Vocabulary::String { map, reverse_map })
    }
}

impl<IndexT: ToFromUsize + Sync> Vocabulary<IndexT> {


    // TODO! properly extend Iterator
    pub fn iter_keys(&self) -> Box<dyn Iterator<Item = String> + '_> {
        match self {
            Vocabulary::String { reverse_map, .. } => {
                Box::new(reverse_map.iter().map(|key| key.clone()))
            }
            Vocabulary::Numeric { range, .. } => Box::new(range.map(|key| format!("{}", key))),
        }
    }

    // TODO! properly extend Iterator
    pub fn par_iter_keys(&self) -> impl IndexedParallelIterator<Item = String> + '_ {
        (0..self.len())
            .into_par_iter()
            .map(move |index| self.unchecked_translate(IndexT::from_usize(index)))
    }

    fn normalize_value(&self, value: &str) -> Result<(String, IndexT)> {
        Ok(match self {
            Vocabulary::Numeric { .. } => {
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
            }

            Vocabulary::String { map, .. } => (value.to_string(), IndexT::from_usize(map.len())),
        })
    }

    /// Returns id of given value inserted.
    ///
    /// # Arguments
    ///
    /// * `value`: String - The value to be inserted.
    pub unsafe fn unchecked_insert(&mut self, value: String) -> IndexT {
        match self {
            Vocabulary::String { map, reverse_map } => {
                let current_length = map.len();
                *map.entry(value)
                    .or_insert_with_key(|value| IndexT::from_usize(current_length))
            }

            Vocabulary::Numeric { range, count } => {
                let value = value.parse::<usize>().unwrap_unchecked();
                range.end = std::cmp::max(range.end, value);
                *count += 1;
                IndexT::from_usize(value - range.start)
            }
        }
    }

    /// Returns id of given value inserted.
    ///
    /// # Arguments
    ///
    /// * `value`: String - The value to be inserted.
    pub fn insert<S: AsRef<str>>(&mut self, value: S) -> Result<(IndexT, bool)> {
        let value = value.as_ref();

        if value.is_empty() {
            return Err(
                "The given value is empty, we cannot insert an empty value into the vocabulary"
                    .to_string(),
            );
        }

        let (normalized_value, index) = self.normalize_value(value)?;

        match self {
            Vocabulary::String { map, reverse_map } => Ok(match map.entry(normalized_value) {
                Entry::Occupied(extracted_index) => (*extracted_index.get(), true),
                Entry::Vacant(vacant_entry) => (*vacant_entry.insert(index), false),
            }),
            Vocabulary::Numeric { range, count } => {
                let value = unsafe { value.parse::<usize>().unwrap_unchecked() };
                if value < range.start {
                    return Err(
                        "The given numeric id is smaller than the minimum given on construction."
                            .to_string(),
                    );
                }
                range.end = range.end.max(value);
                *count += 1;
                // we always retrun false because thsi boolean is meant to be used to
                // check for duplicated, which is already done on build.
                Ok((IndexT::from_usize(value - range.start), false))
            }
        }
    }

    /// Compute the reverse mapping vector for fast decoding
    pub fn build(&mut self) -> Result<()> {
        match self {
            Vocabulary::Numeric { range, count } => {
                let len = range.end - range.start;
                if len == *count {
                    Ok(())
                } else {
                    Err(format!(concat!(
                        "The given numeric values were not dense or they contained duplicates.",
                        "Specifically this vocabulary was initialized with the range {}..{} which has ",
                        "{} values, but insert was called {} times. To be dense these values ",
                        "must match.",
                    ), range.start, range.end, len, count,
                    ))
                }
            }
            Vocabulary::String { map, reverse_map } => {
                if !reverse_map.is_empty() {
                    panic!("Build reverse mapping called multiple times!");
                }

                *reverse_map = vec!["".to_string(); map.len()];
                for (k, v) in map.iter() {
                    if *v >= IndexT::from_usize(map.len()) {
                        return Err(format!(
                            concat!(
                                "The given set of values is not dense. Found the tuple k:{} v:{} ",
                                "which has index bigger than the number of elements in the map {}."
                            ),
                            k,
                            v,
                            map.len()
                        ));
                    }
                    if !reverse_map[IndexT::to_usize(*v)].is_empty() {
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
                            k, v,
                        );
                    }
                    reverse_map[IndexT::to_usize(*v)] = k.clone();
                }
                Ok(())
            }
        }
    }

    /// Returns whether the value is empty or not.
    pub fn is_empty(&self) -> bool {
        match self {
            Vocabulary::String { map, .. } => map.is_empty(),
            Vocabulary::Numeric { range, .. } => range.is_empty(),
        }
    }

    /// Returns string name of given id.
    ///
    /// # Arguments
    ///
    /// * `id`: IndexT - Id to be translated.
    pub fn unchecked_translate(&self, id: IndexT) -> String {
        match self {
            Vocabulary::String { reverse_map, .. } => reverse_map[IndexT::to_usize(id)].clone(),
            Vocabulary::Numeric { .. } => format!("{}", id),
        }
    }

    /// Returns option with string name of given id.
    ///
    /// # Arguments
    ///
    /// * `id`: IndexT - Id to be translated.
    pub fn translate(&self, id: IndexT) -> Result<String> {
        match self {
            Vocabulary::String { reverse_map, .. } => match reverse_map.get(IndexT::to_usize(id)) {
                Some(name) => Ok(name.clone()),
                None => Err("The requested ID is not available in current dictionary.".to_string()),
            },
            Vocabulary::Numeric { range, .. } => {
                if range.contains(&IndexT::to_usize(id)) {
                    Ok(format!("{}", id))
                } else {
                    Err(
                        "The requested id is over the range of the current numeric vocabulary."
                            .to_string(),
                    )
                }
            }
        }
    }

    /// Return the id of given key.
    ///
    /// # Arguments
    ///
    /// * `key`: &str - the key whose Id is to be retrieved.
    pub fn get(&self, key: &str) -> Option<IndexT> {
        match self {
            Vocabulary::String { map, .. } => map.get(key).map(|x| *x),
            Vocabulary::Numeric { range, .. } => {
                let id = unsafe { key.parse::<usize>().unwrap_unchecked() };
                if range.contains(&id) {
                    Some(IndexT::from_usize(id))
                } else {
                    None
                }
            }
        }
    }

    /// Return vector of keys of the map.
    pub fn keys(&self) -> Vec<String> {
        match self {
            Vocabulary::String { map, reverse_map } => reverse_map.clone(),
            Vocabulary::Numeric { range, .. } => range.map(|i| format!("{}", i)).collect::<_>(),
        }
    }

    /// Return vector of keys of the map.
    pub fn map(&self) -> HashMap<String, IndexT> {
        match self {
            Vocabulary::String { map, reverse_map } => map.clone(),
            Vocabulary::Numeric { range, .. } => range
                .map(|i| (format!("{}", i), IndexT::from_usize(i)))
                .collect::<HashMap<String, IndexT>>(),
        }
    }

    /// Return boolean representing if given key is present.
    ///
    /// # Arguments
    ///
    /// * `key`: &str - the key to check existance of.
    pub fn contains_key(&self, key: &str) -> bool {
        match self {
            Vocabulary::String { map, .. } => map.contains_key(key),
            Vocabulary::Numeric { range, .. } => {
                range.contains(&unsafe { key.parse::<usize>().unwrap_unchecked() })
            }
        }
    }

    /// Return length of the vocabulary.
    pub fn len(&self) -> usize {
        match self {
            Vocabulary::String { map, .. } => map.len(),
            Vocabulary::Numeric { range, .. } => range.end - range.start,
        }
    }

    /// Convert the current vocabulary to a string one.
    pub fn to_string_vocabulary(&mut self) {
        match self {
            Vocabulary::String { .. } => {}
            Vocabulary::Numeric { range, .. } => {
                *self = Vocabulary::String {
                    map: range
                        .map(|i| (format!("{}", i), IndexT::from_usize(i)))
                        .collect(),
                    reverse_map: range.map(|i| format!("{}", i)).collect(),
                }
            }
        }
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
        let result = match self {
            Vocabulary::Numeric { range, count } => {
                type_ids_to_remove.sort();

                // scan from the left to remove all the extremants ids
                let mut min = range.start;
                let mut i = 0;
                while type_ids_to_remove[i] == IndexT::from_usize(min) {
                    min += 1;
                    i += 1;
                }

                // scan from the right to remove all the extremants ids
                let mut max = range.end;
                let mut j = type_ids_to_remove.len() - 1;
                while type_ids_to_remove[j] == IndexT::from_usize(max) {
                    max += 1;
                    j -= 1;
                }

                // if the indices matches, then we are just removing values from the
                // extremants of the range, and thus it remains dense.
                // otherwise we need to convert this to a String vocabulary
                // because we have no-longer a dense set of values
                if i == j {
                    let new_range = min..max;
                    let result = range
                        .map(|i| {
                            if new_range.contains(&i) {
                                Some(i - min)
                            } else {
                                None
                            }
                        })
                        .collect::<Vec<Option<usize>>>();
                    *self = Vocabulary::Numeric {
                        range: new_range,
                        count: max - min,
                    };
                    result
                } else {
                    self.to_string_vocabulary();
                    self.unchecked_remove_values(type_ids_to_remove)
                }
            }
            Vocabulary::String { map, reverse_map } => {
                // compute the new dense mapping of the indices
                let new_type_ids_map = (0..reverse_map.len())
                    .scan(0, |offset, type_id| {
                        if type_ids_to_remove.contains(&IndexT::from_usize(type_id)) {
                            *offset += 1;
                            return Some(None);
                        }
                        Some(Some(type_id - *offset))
                    })
                    .collect::<Vec<_>>();

                // update the mapping
                *map = map
                    .iter()
                    .filter_map(|(key, val)| {
                        new_type_ids_map[IndexT::to_usize(*val)]
                            .map(|x| (key.clone(), IndexT::from_usize(x)))
                    })
                    .collect();

                // re-build the reverse mapping
                // since we start from a valid state this should never fail
                // unless there are bugs in the code
                reverse_map.clear();

                self.build().unwrap();

                new_type_ids_map
            }
        };

        result
    }
}
