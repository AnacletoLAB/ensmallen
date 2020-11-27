use std::sync::atomic::{AtomicU64, Ordering};
use std::collections::HashMap;
use std::sync::Mutex;
use std::hash::Hash;

#[inline(always)]
/// utility method to read the bytes of a u64 as if it were an f64
fn u64_to_f64(x:u64) -> f64 {
    f64::from_le_bytes(x.to_le_bytes())
}

#[inline(always)]
/// utility method to read the bytes of a f64 as if it were an u64
fn f64_to_u64(x:f64) -> u64 {
    u64::from_le_bytes(x.to_le_bytes())
}

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
    value: HashMap<K, AtomicU64>,
    insert_mutex: Mutex<()>,
    max_value: AtomicU64,
}

impl<K: Eq + Hash + Copy+ 'static> AtomicF64HashMap<K> {
    /// Create a new instance of the map
    pub fn new() -> AtomicF64HashMap<K>{
        AtomicF64HashMap{
            value: HashMap::new(),
            insert_mutex: Mutex::new(()),
            max_value: AtomicU64::new(f64_to_u64(f64::NEG_INFINITY)),
        }
    }

    /// Return the biggest value in the map
    pub fn max_value(&self) -> f64 {
        u64_to_f64(self.max_value.load(Ordering::Relaxed))
    }

    /// Return an iterator over the keys and values of the map
    pub fn into_iter(self) -> impl Iterator<Item=(K, f64)> {
        self.value.into_iter().map(|(k, v)| {
            let v = v.load(Ordering::Relaxed);
            (
                k.clone(),
                u64_to_f64(v)
            )
        })
    }


    /// Return an iterator over the keys and values of the map
    pub fn into_iter_normalized(self) -> impl Iterator<Item=(K, f64)> {
        let max = self.max_value();
        self.into_iter().map(move |(k, v)| {(k, v / max)})
    }


    /// Return the number of elements in the map
    pub fn len(&self) -> usize {
        self.value.len()
    }

    /// Add a value to the the element of the choosen key.
    /// The default value is 0.0 .
    pub fn add(&self, key: &K, increment: f64){
        let map = unsafe{&mut *(&self.value as *const HashMap<K, AtomicU64> as *mut HashMap<K, AtomicU64>)};
        let value = match map.get(key) {
            Some(val) => val,
            None => {
                let _lock = self.insert_mutex.lock();
                map.entry(*key).or_insert_with(|| AtomicU64::new(f64_to_u64(0.0)))
            }
        };
        // update the value
        let mut new_value;
        let mut stored = value.load(Ordering::Relaxed);
        loop {
            new_value = f64_to_u64(u64_to_f64(stored) + increment);
            let inner = value.compare_and_swap(stored, new_value, Ordering::Relaxed);
            // if sucessfully updated exit
            if inner == stored {
                break;
            }
            stored = inner;
        }
        // update the max
        let mut stored = self.max_value.load(Ordering::Relaxed);
        while u64_to_f64(stored) < u64_to_f64(new_value) {
            
            let inner = self.max_value.compare_and_swap(stored, new_value, Ordering::Relaxed);
            // if sucessfully updated exit
            if inner == stored {
                break;
            }
            stored = inner;
        }
    }
}