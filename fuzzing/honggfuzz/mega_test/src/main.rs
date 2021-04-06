#[macro_use]
extern crate honggfuzz;
extern crate graph_harness;

use graph_harness::*;
fn main() {
    loop {
        fuzz!(|data: TheUltimateFuzzer| {
            // We ignore this error because we execute only the fuzzing to find
            // the panic situations that are NOT just errors, but unhandled errors.
            let _ = mega_test(data);
        });
    }
}
