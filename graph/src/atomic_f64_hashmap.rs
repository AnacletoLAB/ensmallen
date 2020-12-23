use std::cell::UnsafeCell;
use std::collections::HashMap;
use std::hash::Hash;
use std::thread::ThreadId;
use rayon::prelude::*;
use rayon::iter::*;

/// Atomic hashmap of f64 values.
/// This is ment to be used to concurrently compute
/// the co-occurence matrix.
///
/// Since there are now AtomicF64 in rust's std library,
/// we use a dirty trick which uses AtomicU64 and
/// compare and swap.
///
/// Here we care about 2 things, we want to be able to add floats
/// concurrently and fast and we care about the maximum value
/// since we will need to normalize the values between 0 and 1.
///
/// # Example
///
/// ```rust
/// // TODO!: debug, it has problems with Arc but this is how it's used
/// //        inside the cooccurence matrix
/// // use rayon::prelude::*;
/// // use std::sync::Arc;
/// // use atomic_f64_hashmap::AtomicF64HashMap;
/// // // Let's build the map with an arc so that we can share it between threads
/// // let cooccurence_matrix = Arc::new(AtomicF64HashMap::<u64>::new());
/// // // let's fill it
/// // (0..1_000).into_par_iter()
/// //     .for_each(|i| {
/// //         // get a personal reference to the matrix
/// //         // (this probably could be optimized)
/// //         let cooccurence_matrix = cooccurence_matrix.clone();
/// //         *cooccurence_matrix.add(1.0 / i as f64);
/// //     });
/// // // print the result
/// // for (k, v) in cooccurence_matrix.clone().into_iter() {
/// //     println!("The value of the key {} is {}", k, v);
/// // }
/// ```
#[derive(Debug)]
pub(crate) struct AtomicF64HashMap<K: Eq + Hash + Copy + 'static> {
    value: UnsafeCell<HashMap<ThreadId, HashMap<K, f64>>>,
    merged: HashMap<K, f64>,
    max_value: f64,
}

unsafe impl<K: Eq + Hash + Copy + 'static> Sync for AtomicF64HashMap<K> {}

impl<K: Eq + Hash + Copy + 'static> AtomicF64HashMap<K> {
    /// Create a new instance of the map
    pub fn new() -> AtomicF64HashMap<K> {
        AtomicF64HashMap {
            value: UnsafeCell::new(HashMap::new()),
            merged: HashMap::new(),
            max_value: f64::NEG_INFINITY,
        }
    }

    /// Return the biggest value in the map
    pub fn max_value(&self) -> f64 {
        self.max_value
    }

    /// Return an iterator over the keys and values of the map
    pub fn into_iter(self) -> impl Iterator<Item = (K, f64)> {
        self.merged.into_iter()
    }

    /// Return an iterator over the keys and values of the map
    pub fn into_iter_normalized(self) -> impl Iterator<Item = (K, f64)> {
        let max = self.max_value();
        assert_ne!(max, 0.0, "The max value in the co-occurence matrix is 0! This is not supposed to happen.");
        self.into_iter().map(move |(k, v)| (k, v / max))
    }

    /// Return the number of elements in the map
    pub fn len(&self) -> usize {
        unsafe { (*self.value.get()).len() }
    }

    /// Add a value to the the element of the choosen key.
    /// The default value is 0.0 .
    pub fn add(&self, key: &K, increment: f64) {
        let map = unsafe {
            (*self.value.get()).entry(
                std::thread::current().id()
            ).or_insert_with(|| HashMap::new())
        };

        *map.entry(*key).or_insert(0.0) += increment;
    }

    pub fn merge(&mut self){
        self.merged = unsafe{&*self.value.get()}
            .values().fold(
                HashMap::<K, f64>::new(),
                |mut a, b| {
                    for (key, value) in b {
                        let other_value = a.entry(*key).or_insert(0.0);
                        *other_value += value;
                    }
                    a
                }
            );
    }
}
