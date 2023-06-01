use super::types::*;
use itertools::Itertools;
use rayon::iter::ParallelIterator;
use rayon::iter::{IndexedParallelIterator, IntoParallelIterator};
use rayon::prelude::ParallelSliceMut;
use std::collections::hash_map::DefaultHasher;
use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::fmt::Debug;
use std::hash::{Hash, Hasher};
use std::ops::Range;

/// which integer type the string hashes will
type HashType = u64;

#[inline]
pub(crate) fn compute_hash<T: Hash>(t: &T) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}

#[derive(Debug, Clone, PartialEq)] // Arbitrary
pub enum Vocabulary<IndexT: ToFromUsize + Sync + Debug> {
    // If the values are arbitrary and we cannot make any assumptions
    // about them
    String {
        // TODO: is there a way to have a fast compressed mapping between
        // integers?, specifically the output is dense from 0 to n
        map: HashMap<HashType, IndexT>,
        name: String,
        reverse_map: Option<Vec<String>>,
    },

    // If the values are in a dense integer range
    Numeric {
        range: Range<usize>,
        count: usize,
        name: String,
    },
}

#[derive(Debug, Clone)]
pub enum VocabularyMemoryStats {
    String {
        map: usize,
        reverse_map: usize,
        metadata: usize,
    },
    Numeric {
        metadata: usize,
    },
}

impl VocabularyMemoryStats {
    pub fn total(&self) -> usize {
        use VocabularyMemoryStats::*;
        match self {
            String {
                map,
                reverse_map,
                metadata,
            } => map + reverse_map + metadata,
            Numeric { metadata } => *metadata,
        }
    }
}

impl<IndexT: ToFromUsize + Sync + Debug> Vocabulary<IndexT> {
    pub fn memory_stats(&self) -> VocabularyMemoryStats {
        use std::mem::size_of;

        match self {
            Vocabulary::String {
                map, reverse_map, ..
            } => {
                VocabularyMemoryStats::String {
                    // https://github.com/servo/servo/issues/6908
                    map: (map.capacity() as f64 * 1.1) as usize
                        * (size_of::<HashType>() + size_of::<IndexT>() + size_of::<usize>()),
                    reverse_map: reverse_map.as_ref().map_or(
                        size_of::<Option<Vec<String>>>(),
                        |reverse_map| {
                            reverse_map
                                .iter()
                                .map(|s| size_of::<String>() + s.capacity() * size_of::<char>())
                                .sum::<usize>()
                        },
                    ),
                    metadata: size_of::<Vocabulary<IndexT>>(),
                }
            }
            Vocabulary::Numeric { .. } => VocabularyMemoryStats::Numeric {
                metadata: size_of::<Vocabulary<IndexT>>(),
            },
        }
    }
}

/// # Constructors
impl<IndexT: ToFromUsize + Sync + Debug> Vocabulary<IndexT> {
    pub fn new(use_reverse_map: bool, name: String) -> Vocabulary<IndexT> {
        Vocabulary::String {
            map: HashMap::new(),
            name,
            reverse_map: if use_reverse_map {
                Some(Vec::new())
            } else {
                None
            },
        }
    }

    /// Return whether this vocabulary is numeric or string based.
    pub fn is_numeric(&self) -> bool {
        match self {
            Vocabulary::Numeric { .. } => true,
            _ => false,
        }
    }

    /// Returns the minimum id of the vocabulary.
    pub fn get_minimum_id(&self) -> Option<IndexT> {
        match self {
            Vocabulary::Numeric { range, .. } => Some(IndexT::from_usize(range.start)),
            Vocabulary::String { map, .. } => map.values().min().cloned(),
        }
    }

    // TODO! properly extend Iterator
    pub fn iter(&self) -> impl Iterator<Item = (IndexT, String)> + '_ {
        self.iter_keys()
            .enumerate()
            .map(|(i, name)| (IndexT::from_usize(i), name))
    }

    /// Returns new STRING Vocabulary with given capacity.
    ///
    /// # Arguments
    /// * `capacity`: usize - The capacity of the vocabulary.
    /// * `use_reverse_map`: bool - Whether we expect to use the reverse map or not.
    /// * `name`: String - The name of the vocabulary.
    pub fn with_capacity(
        capacity: usize,
        use_reverse_map: bool,
        name: String,
    ) -> Vocabulary<IndexT> {
        Vocabulary::String {
            map: HashMap::with_capacity(capacity),
            name,
            reverse_map: if use_reverse_map {
                Some(Vec::with_capacity(capacity))
            } else {
                None
            },
        }
    }

    pub fn from_range(range: Range<IndexT>, name: String) -> Vocabulary<IndexT> {
        Vocabulary::Numeric {
            range: Range {
                start: IndexT::to_usize(range.start),
                end: IndexT::to_usize(range.end),
            },
            name,
            count: 0,
        }
    }

    /// Returns a vocabulary from a reverse map.
    ///
    /// # Arguments
    /// * `reverse_map`: Vec<String> - The reverse map to be used to build the vocabulary.
    /// * `name`: String - The name of the vocabulary.
    ///
    /// # Raises
    /// * If the reverse map contains duplicated values.
    /// * If the reverse map contains empty values.
    pub fn from_reverse_map(
        mut reverse_map: Vec<String>,
        name: String,
    ) -> Result<Vocabulary<IndexT>> {
        let map = reverse_map
            .iter()
            .cloned()
            .enumerate()
            .map(|(i, x)| {
                if x.is_empty() {
                    Err(format!(
                        concat!(
                            "An error was encountered while attempting to build a vocabulary called '{}'. ",
                            "The reverse map provided contains an empty string at index {} out of {}.\n",
                            "This is not allowed since the reverse map is used to build the vocabulary. ",
                            "Some other values that are present in the reverse map are {:?}. ",
                            "{}"
                        ),
                        name,
                        i,
                        reverse_map.len(),
                        reverse_map.iter().filter(|x| !x.is_empty()).take(10).collect::<Vec<_>>(),
                        // We check whether all values from i to the end are empty, as it may mean
                        // that the reverse map was built from a provided number of nodes which resulted
                        // wrong, as the actual number of nodes present in the graph is lower. For such
                        // cases, the reverse map ends up being padded with empty strings.
                        if reverse_map.iter().skip(i).all(|x| x.is_empty()) {
                            format!(
                                concat!(
                                    "We have checked that all values from index {} to the end are empty. ",
                                    "This may mean that the reverse map was built from a provided number of nodes '{}' ",
                                    "which resulted wrong, as the actual number of nodes present in the graph is lower. ",
                                ),
                                i,
                                reverse_map.len()
                            )
                        } else {
                            format!(
                                "We have checked that all values from index {} to the end are not empty.",
                                i
                            )
                        }
                    ))
                } else {
                    Ok((compute_hash(&x), IndexT::from_usize(i)))
                }
            })
            .collect::<Result<HashMap<HashType, IndexT>>>()?;

        if map.len() != reverse_map.len() {
            let reverse_map_length = reverse_map.len();
            let expected_duplicates_number = reverse_map.len() - map.len();
            reverse_map.par_sort_unstable();
            let up_to_ten_duplicates = reverse_map
                .windows(2)
                .filter_map(|a| {
                    if a[0] == a[1] {
                        Some(a[0].clone())
                    } else {
                        None
                    }
                })
                .unique()
                .take(10)
                .collect::<Vec<String>>();
            // We need to provide a meaningful and extensive error message in the case
            // of detected duplicates, providing the number of duplicates and up to
            // 10 examples of the values we have identified as duplicates.
            return Err(format!(
                concat!(
                    "An error was encountered while attempting to build a vocabulary called '{}'. ",
                    "The reverse map provided contains {} duplicated values out of {}.\n",
                    "This is not allowed since the reverse map is used to build the vocabulary. ",
                    "Some of the duplicated values are {:?}.",
                ),
                name,
                expected_duplicates_number, reverse_map_length, up_to_ten_duplicates,
            ));
        }

        Ok(Vocabulary::String {
            map,
            name,
            reverse_map: Some(reverse_map),
        })
    }
}

impl<IndexT: ToFromUsize + Sync + Debug> Vocabulary<IndexT> {
    // TODO! properly extend Iterator
    pub fn iter_keys(&self) -> Box<dyn Iterator<Item = String> + '_> {
        match self {
            Vocabulary::String { reverse_map, .. } => {
                let iterator: Box<dyn Iterator<Item = String>> =
                    if let Some(reverse_map) = reverse_map {
                        Box::new(reverse_map.iter().cloned())
                    } else {
                        Box::new((0..self.len()).map(|value| format!("{}", value)))
                    };
                iterator
            }
            Vocabulary::Numeric { range, .. } => {
                Box::new(range.clone().map(|value| format!("{}", value)))
            }
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
            Vocabulary::String {
                map, reverse_map, ..
            } => {
                let current_length = map.len();
                match map.entry(compute_hash(&value)) {
                    Entry::Occupied(index) => *index.get(),
                    Entry::Vacant(vacant_entry) => {
                        if let Some(reverse_map) = reverse_map {
                            reverse_map.push(value);
                        }
                        *vacant_entry.insert(IndexT::from_usize(current_length))
                    }
                }
            }

            Vocabulary::Numeric { range, count, .. } => {
                let value = value.parse::<usize>().unwrap();
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
            return Err(concat!(
                "The given value is empty, ",
                "we cannot insert an empty value into the vocabulary.\n",
            )
            .into());
        }

        let (normalized_value, index) = self.normalize_value(value)?;

        match self {
            Vocabulary::String {
                map, reverse_map, ..
            } => Ok(match map.entry(compute_hash(&normalized_value)) {
                Entry::Occupied(extracted_index) => (*extracted_index.get(), true),
                Entry::Vacant(vacant_entry) => {
                    if let Some(reverse_map) = reverse_map {
                        reverse_map.push(value.to_string());
                    }
                    (*vacant_entry.insert(index), false)
                }
            }),
            Vocabulary::Numeric { range, count, .. } => {
                let value = { value.parse::<usize>().unwrap() };
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
            Vocabulary::Numeric { range, count, .. } => {
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
            Vocabulary::String { .. } => {
                // TODO! Check if the following comment can be deleted!
                /*
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
                }*/
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
            Vocabulary::String { reverse_map, .. } => reverse_map
                .as_ref()
                .map_or(format!("{}", id), |reverse_map| {
                    reverse_map[IndexT::to_usize(id)].clone()
                }),
            Vocabulary::Numeric { range, .. } => format!("{}", range.start + IndexT::to_usize(id)),
        }
    }

    /// Returns option with string name of given id.
    ///
    /// # Arguments
    ///
    /// * `id`: IndexT - Id to be translated.
    pub fn translate(&self, id: IndexT) -> Result<String> {
        if id >= IndexT::from_usize(self.len()) {
            return Err(format!(
                concat!(
                    "The provided ID `{}` is higher or equal to the length ",
                    "of the vocabulary `{}`."
                ),
                id,
                self.len()
            ));
        }
        Ok(self.unchecked_translate(id))
    }

    /// Return the id of given key.
    ///
    /// # Arguments
    ///
    /// * `key`: &str - the key whose Id is to be retrieved.
    pub fn get(&self, key: &str) -> Option<IndexT> {
        match self {
            Vocabulary::String { map, .. } => map.get(&compute_hash(&key)).map(|x| *x),
            Vocabulary::Numeric { range, .. } => {
                let id = key.parse::<usize>();
                if id.is_err() {
                    return None;
                }
                let id = id.unwrap();
                if range.contains(&id) {
                    Some(IndexT::from_usize(id - range.start))
                } else {
                    None
                }
            }
        }
    }

    /// Return vector of keys of the map.
    pub fn keys(&self) -> Vec<String> {
        self.iter_keys().collect()
    }

    /// Return vector of keys of the map.
    pub fn map(&self) -> HashMap<String, IndexT> {
        self.iter().map(|(index, name)| (name, index)).collect()
    }

    /// Return boolean representing if given key is present.
    ///
    /// # Arguments
    ///
    /// * `key`: &str - the key to check existance of.
    pub fn contains_key(&self, key: &str) -> bool {
        match self {
            Vocabulary::String { map, .. } => map.contains_key(&compute_hash(&key)),
            Vocabulary::Numeric { range, .. } => range.contains(&{ key.parse::<usize>().unwrap() }),
        }
    }

    /// Return length of the vocabulary.
    pub fn len(&self) -> usize {
        match self {
            Vocabulary::String { map, .. } => map.len(),
            Vocabulary::Numeric { range, .. } => range.end - range.start,
        }
    }

    // Return whether the keys are sorted by lexicographical order.
    pub fn is_sorted_by_lexicographic_order(&self) -> bool {
        match self {
            Vocabulary::String { reverse_map, .. } => reverse_map
                .as_ref()
                .map_or_else(|| self.len() < 10, |reverse_map| reverse_map.is_sorted()),
            Vocabulary::Numeric { .. } => self.len() < 10,
        }
    }

    pub fn replace_inplace(&mut self, original: String, replace: String) -> Result<()> {
        if !self.contains_key(&original) {
            return Err(format!(
                concat!(
                    "The key provided as value to replace {} does not exist ",
                    "in the current graph vocabulary."
                ),
                original
            ));
        }
        if self.contains_key(&replace) {
            return Err(format!(
                concat!(
                    "The key provided as value to replace the key {} with, ",
                    "{}, is already present in the current graph vocabulary."
                ),
                original, replace
            ));
        }
        let id = self.get(&original).unwrap();
        match self {
            Vocabulary::String {
                map, reverse_map, ..
            } => {
                map.remove(&compute_hash(&original));
                map.insert(compute_hash(&replace), id);
                if let Some(reverse_map) = reverse_map {
                    reverse_map[IndexT::to_usize(id)] = replace;
                }
            }
            Vocabulary::Numeric { .. } => {
                self.to_string_vocabulary();
                self.replace_inplace(original, replace)?;
            }
        };

        Ok(())
    }

    /// Convert the current vocabulary to a string one.
    pub fn to_string_vocabulary(&mut self) {
        match self {
            Vocabulary::String { .. } => {}
            Vocabulary::Numeric { range, name, .. } => {
                *self = Vocabulary::String {
                    map: range
                        .map(|i| (compute_hash(&format!("{}", i)), IndexT::from_usize(i)))
                        .collect(),
                    reverse_map: Some(range.map(|i| format!("{}", i)).collect()),
                    name: name.clone(),
                }
            }
        }
    }

    /// Removes given values from the vocabulary
    ///
    /// # Arguments
    /// * `type_ids_to_remove`: Vec<IndexT> - The values to be removed.
    ///
    /// # Safety
    /// This method will panic if you try to remove values that do not exist
    /// in the current vocabulary.
    pub unsafe fn unchecked_remove_values(
        &mut self,
        mut type_ids_to_remove: Vec<IndexT>,
    ) -> Vec<Option<usize>> {
        let result = match self {
            Vocabulary::Numeric { range, name, .. } => {
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
                        name: name.clone(),
                    };
                    result
                } else {
                    self.to_string_vocabulary();
                    self.unchecked_remove_values(type_ids_to_remove)
                }
            }
            Vocabulary::String {
                map, reverse_map, ..
            } => {
                // compute the new dense mapping of the indices
                let new_type_ids_map = (0..map.len())
                    .scan(0, |offset, type_id| {
                        Some(
                            if type_ids_to_remove.contains(&IndexT::from_usize(type_id)) {
                                *offset += 1;
                                None
                            } else {
                                Some(type_id - *offset)
                            },
                        )
                    })
                    .collect::<Vec<_>>();

                // update the mapping
                *map = map
                    .iter()
                    .filter_map(|(key, previous_identifier)| {
                        new_type_ids_map[IndexT::to_usize(*previous_identifier)]
                            .map(|new_identifier| (key.clone(), IndexT::from_usize(new_identifier)))
                    })
                    .collect();

                // re-build the reverse mapping
                // since we start from a valid state this should never fail
                // unless there are bugs in the code
                if let Some(reverse_map) = reverse_map {
                    reverse_map.retain(|previous_identifier_name| {
                        map.contains_key(&compute_hash(previous_identifier_name))
                    });
                }

                new_type_ids_map
            }
        };

        result
    }
}
