extern crate graph;

use graph::FromCsvBuilder;
use graph::{WalksParameters, SingleWalkParameters, WalkWeights};
use core::arch::x86_64::{_rdtsc, __rdtscp};
use vec_rand;

fn rdtsc() -> u64{
    let mut x: u32 = 0;
    // __rdtscp it's the serialized version of _rdtsc
    // this should give us more consistent results
    unsafe{
        __rdtscp(& mut x)
    }
}

const SIZE: usize = 10;
const WALK_LENGTH: usize = 3;

fn main() {
    let edge_path = "benches/data/test.tsv";
    let graph = FromCsvBuilder::new(edge_path, "subject", "object", false, None)
    .unwrap()
    .build()
    .unwrap();
    println!("node,cycles");
    for node in 0..graph.get_nodes_number(){
        let start: u64 = rdtsc();
        for _ in 0..SIZE {
            graph.single_walk_no_traps(node, 0xbed533d, &SingleWalkParameters::new(WALK_LENGTH, WalkWeights::default()).unwrap());
        }
        let v = (rdtsc() - start) as f64 / SIZE as f64;
        println!("{}, {}", node, v);
    }
}