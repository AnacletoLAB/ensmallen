#[macro_use] extern crate honggfuzz;
extern crate graph_harness;
use graph_harness::*;

fn main() {
    loop {
        fuzz!(|data: FromCsvHarnessParams| {
            from_csv_harness(data);
        });
    }
}