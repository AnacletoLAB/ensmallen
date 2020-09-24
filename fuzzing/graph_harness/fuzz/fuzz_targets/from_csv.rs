#![no_main]
use libfuzzer_sys::fuzz_target;
extern crate graph_harness;
use graph_harness::*;

fuzz_target!(|data: FromCsvHarnessParams| {
    from_csv_harness(data);
});
