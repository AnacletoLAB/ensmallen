#[macro_use] extern crate honggfuzz;
extern crate graph_harness;

use graph_harness::*;
fn main() {
    loop {
        fuzz!(|data: ToFuzz| {
            harness(data);
        });
    }
}