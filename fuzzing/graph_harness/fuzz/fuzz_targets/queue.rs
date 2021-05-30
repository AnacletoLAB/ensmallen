#![no_main]
use libfuzzer_sys::fuzz_target;
extern crate graph_harness;
use graph_harness::*;

fuzz_target!(|data: QueueParams| {
    // We ignore this error because we execute only the fuzzing to find
    // the panic situations that are NOT just errors, but unhandled errors.
    let _ = queue_harness(data);
});
