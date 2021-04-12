use std::{cell::UnsafeCell, cmp::max};
use std::collections::HashMap;
use std::hash::Hash;
use std::thread::ThreadId;
use rayon::prelude::*;
use rayon::iter::*;
use std::sync::atomic::AtomicU64;
use std::sync::atomic::Ordering;
use std::sync::Mutex;

/// Since there aren't AtomicF64 in rust's std library,
/// we use a dirty trick which uses AtomicU64 and
/// compare and swap.
pub(crate) struct AtomicF64{
    value: AtomicU64,
}

unsafe impl Sync for AtomicF64 {}
unsafe impl Send for AtomicF64 {}

impl AtomicF64 {
    #[inline]
    fn encode(x: f64) -> u64 {
        unsafe{std::mem::transmute::<f64, u64>(x)}
    }
    
    #[inline]
    fn decode(x: u64) -> f64 {
        unsafe{std::mem::transmute::<u64, f64>(x)}
    }

    #[inline]
    pub fn new(x: f64) -> AtomicF64 {
        AtomicF64{
            value: AtomicU64::new(AtomicF64::encode(x))
        }
    }

    #[inline]
    pub fn into_inner(self) -> f64 {
        AtomicF64::decode(self.value.into_inner())
    }

    #[inline]
    pub fn fetch_max(&self, other: f64) -> f64 {
        let mut old = self.value.load(Ordering::Relaxed);
        loop {
            let decoded_old = AtomicF64::decode(old);
            let new = AtomicF64::encode(
                if decoded_old >= other {
                    decoded_old
                } else {
                    other
                }
            );
            match self.value.compare_exchange_weak(old, new, Ordering::SeqCst, Ordering::Relaxed) {
                // Retry
                Err(x) => old = x,
                // Ok
                Ok(_) => break AtomicF64::decode(new),
            }
        }
    }

    #[inline]
    pub fn fetch_add(&self, increment: f64) -> f64 {
        let mut old = self.value.load(Ordering::Relaxed);
        loop {
            let new = AtomicF64::encode(AtomicF64::decode(old) + increment);
            match self.value.compare_exchange_weak(old, new, Ordering::SeqCst, Ordering::Relaxed) {
                // Retry
                Err(x) => old = x,
                // Ok
                Ok(_) => break AtomicF64::decode(new),
            }
        }
    }
}


pub(crate) struct AtomicF64HashMap {
    srcs: UnsafeCell<Vec<u32>>,
    dsts: UnsafeCell<Vec<u32>>,
    frequencies: UnsafeCell<Vec<AtomicF64>>,
    table: Vec<AtomicU64>,
    max_frequency: AtomicF64,
    write_mutex: Mutex<()>,
}
unsafe impl Sync for AtomicF64HashMap {}

impl AtomicF64HashMap {
    pub fn with_capacity(capacity: usize) -> AtomicF64HashMap {

        let mut table =  Vec::with_capacity(capacity * 2);

        for _ in 0..capacity * 2 {
            table.push(AtomicU64::new(u64::MAX));
        }

        AtomicF64HashMap{
            srcs:        UnsafeCell::new(Vec::with_capacity(capacity)),
            dsts:        UnsafeCell::new(Vec::with_capacity(capacity)),
            frequencies: UnsafeCell::new(Vec::with_capacity(capacity)),
            max_frequency: AtomicF64::new(f64::NEG_INFINITY),
            write_mutex: Mutex::new(()),
            table: table,
        }
    }

    #[inline]
    fn hash(src: u32, dst: u32) -> usize {
        let mut encode = (src  as usize) << 32 | (dst as usize);
        encode ^= encode << 17;
        encode ^= encode >> 7;
        encode ^= encode << 13;
        encode
    }

    #[inline]
    fn insert(&self, src: u32, dst: u32, value: f64) -> usize {
        let lock = self.write_mutex.lock();

        let idx = unsafe{(*self.srcs.get()).len()};

        unsafe{(*self.srcs.get()).push(src)};
        unsafe{(*self.dsts.get()).push(dst)};
        unsafe{(*self.frequencies.get()).push(AtomicF64::new(value))};

        return idx;
    }

    pub fn fetch_add(&self, src: u32, dst: u32, increment: f64) {
        let mut idx = AtomicF64HashMap::hash(src, dst) % self.table.len();

        loop {
            // if it the cell is empty insert the value
            let table_value = self.table[idx].load(Ordering::Relaxed);
            if table_value == u64::MAX {
                self.max_frequency.fetch_max(increment);
                // Insert in the results the values
                let result_idx = self.insert(src, dst, increment);
                // try to insert in the table the index.
                loop {
                    match self.table[idx].compare_exchange_weak(u64::MAX, result_idx as u64, Ordering::SeqCst, Ordering::Relaxed) {
                        // Probe, this is unlucky but another thread might have filled
                        // the current cell while we where computing the values
                        Err(_) => {
                            idx = (idx + 1) % self.table.len();
                        },
                        // Ok inserted!
                        Ok(_) => break,
                    }
                }
                break;
            }

            // otherwise check if the src and dst match
            if unsafe{(*self.srcs.get())[table_value as usize] == src} && unsafe{(*self.dsts.get())[table_value as usize] == dst} {
                // do a fetch add
                let new = unsafe{
                    (*self.frequencies.get())[
                        self.table[idx].load(Ordering::Relaxed) as usize
                    ].fetch_add(increment)
                };
                self.max_frequency.fetch_max(new);
                break;
            }

            // linear probe
            idx = (idx + 1) % self.table.len();
        }
    }

    pub fn into_inner(self) -> (Vec<u32>, Vec<u32>, Vec<f64>) {
        (
            self.srcs.into_inner(),
            self.dsts.into_inner(),
            self.frequencies.into_inner().into_iter().map(|x| x.into_inner()).collect::<Vec<f64>>(),
        )
    }
}

/// Parallel hashmap of f64 values.
/// This is ment to be used to concurrently compute
/// the co-occurence matrix.
///
/// This hashsmap allocates an hashmap for each logical core
/// and once the all the values are in, the hashmaps are merged.
/// sadly this last part takes a significant amount of time.
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
pub(crate) struct ParallelF64HashMap<K: Eq + Hash + Copy + 'static> {
    value: UnsafeCell<HashMap<ThreadId, HashMap<K, f64>>>,
    merged: HashMap<K, f64>,
    max_value: f64,
}

unsafe impl<K: Eq + Hash + Copy + 'static> Sync for ParallelF64HashMap<K> {}

impl<K: Eq + Hash + Copy + 'static> ParallelF64HashMap<K> {
    /// Create a new instance of the map
    pub fn new() -> ParallelF64HashMap<K> {
        ParallelF64HashMap {
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