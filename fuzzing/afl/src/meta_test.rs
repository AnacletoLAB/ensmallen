#[macro_use]
extern crate afl;
use arbitrary::{Arbitrary, Unstructured};
extern crate graph_harness;

use graph_harness::*;
fn main() {
    fuzz!(|data: &[u8]| {
        // We ignore this error because we execute only the fuzzing to find
        // the panic situations that are NOT just errors, but unhandled errors.
        let mut raw_data = Unstructured::new(data);
        let maybe_params = MetaParams::arbitrary(&mut raw_data);
        if let Ok(params) = maybe_params {
            let _ = meta_test_harness_with_panic_handling(params);
        }
    });
}
