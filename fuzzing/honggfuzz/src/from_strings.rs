#[macro_use]
extern crate honggfuzz;
extern crate graph_harness;

use graph_harness::*;
fn main() {
    loop {
        fuzz!(|data: FromStringsParameters| {
            // We ignore this error because we execute only the fuzzing to find
            // the panic situations that are NOT just errors, but unhandled errors.
            let _ = build_graph_from_strings_harness(data);
        });
    }
}