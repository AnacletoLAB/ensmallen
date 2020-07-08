#![no_main]
use libfuzzer_sys::fuzz_target;
extern crate graph;

use graph::fuzzing_harness::*;

fuzz_target!(|data: ToFuzz| {
    harness(data);
});
