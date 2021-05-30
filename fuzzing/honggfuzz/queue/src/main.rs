#[macro_use]
extern crate honggfuzz;
extern crate graph_harness;
use graph_harness::*;
use arbitrary::Arbitrary;

fn main() {
    loop {
        fuzz!(|params: QueueParams| {
            queue_harness(params);
        });
    }
}
