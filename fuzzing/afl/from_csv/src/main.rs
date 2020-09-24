
#[macro_use]
extern crate afl;
extern crate graph_harness;
use graph_harness::*;


fn main() {
    fuzz!(|data: FromCsvHarnessParams| {
	    from_csv_harness(data);
    });

}
