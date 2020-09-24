
#[macro_use]
extern crate afl;
extern crate graph_harness;
use graph_harness::*;


fn main() {
    fuzz!(|data: FromVecHarnessParams| {
	    from_vec_harness(data);
    });

}
