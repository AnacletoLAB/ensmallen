use std::fs;
use std::sync::Arc;
use std::sync::atomic::{AtomicU64, AtomicUsize, Ordering};
use std::time::{Instant, Duration};
use arbitrary::{Arbitrary, Unstructured};

use graph_harness::{from_vec_harness, FromVecHarnessParams};

const THREADS: u64 = 12;
const SEED: u64 = 0xbad5eeddeadbeef;
const BATCH_SIZE: usize = 0xff;
const CORUPUS_DIR: &str = "../corpus/from_vec/";


struct Statistics{
    pub fuzz_cases: AtomicUsize,
}

impl Statistics {
    pub fn new() -> Arc<Statistics> {
        Arc::new(
            Statistics{
                fuzz_cases: AtomicUsize::new(0),
            }
        )
    }
}

fn load_corpus() -> Arc<Vec<Vec<u8>>> {
    let mut corpus = Vec::new();
    for filename in std::fs::read_dir(CORUPUS_DIR).expect("CANNOT READ CORPUS") {
        let filename = filename.expect("Cannot get file in dir").path();
        corpus.push(std::fs::read(filename).expect("Cannot read a file."));
    }

    Arc::new(corpus)
}

fn fuzz(input: &[u8]) {
    let mut raw_data = Unstructured::new(input);
    if let Ok(params) = FromVecHarnessParams::arbitrary(&mut raw_data) {
        let _ = from_vec_harness(params);
    }
}

fn worker(mut seed: u64, stats: Arc<Statistics>, corpus:  Arc<Vec<Vec<u8>>>){
    let mut rng = Rng(seed);
    let mut fuzz_input = Vec::new();

    loop {
        for _ in 0..BATCH_SIZE {
            fuzz_input.clear();
            fuzz_input.extend_from_slice(
                &corpus[rng.next() % corpus.len()]
            );

            for _ in 0..(rng.next() % 16) {
                let idx = rng.next() % fuzz_input.len();
                fuzz_input[idx] = (rng.next() & 0xff) as u8;
            }

            fuzz(&mut fuzz_input);
        }

        stats.fuzz_cases.fetch_add(BATCH_SIZE, Ordering::Relaxed);
    }
}


struct Rng(u64);
impl Rng {
    pub fn new(seed: u64) -> Rng {
        Rng(seed)
    }

    #[inline]
    pub fn next(&mut self) -> usize {
        let mut x = self.0;
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
        self.0 = x;
        x as usize
    }
}

fn splitmix64(x: u64) -> u64 {
    let mut z: u64 = x.wrapping_add(0x9e3779b97f4a7c15);
    z = (z ^ (z >> 30)).wrapping_mul(0xbf58476d1ce4e5b9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94d049bb133111eb);
    return z ^ (z >> 31);
}

fn pretty_print_int(i: usize) -> String {
    let mut s = String::new();
    let i_str = i.to_string();
    let a = i_str.chars().rev().enumerate();
    for (idx, val) in a {
        if idx != 0 && idx % 3 == 0 {
            s.insert(0, ',');
        }
        s.insert(0, val);
    }
    s
}

fn main() {
    let corpus = load_corpus();
    let statistics = Statistics::new();

    // Start all the workers
    let mut seed = SEED;
    let mut threads = Vec::new();
    for thr_id in 0..THREADS {
        // Create a new seed
        seed = splitmix64(seed);

        // Get references to the statistic and corpus arcs
        let stats = statistics.clone();
        let corpus = corpus.clone();
        
        // Start the thread
        threads.push(
            std::thread::spawn(move || worker(seed, stats, corpus))
        );
    }

    let start = Instant::now();

    // Periodically print the statistics
    loop {
        let elapsed = start.elapsed().as_secs_f64();
        let cases = statistics.fuzz_cases.load(Ordering::Relaxed);
        println!("[{:>10}] cases {:>13} | fcps {:>10.2}",
            elapsed as u64, pretty_print_int(cases), cases as f64 / elapsed
        );

        std::thread::sleep(Duration::from_millis(1000));
    }
}
