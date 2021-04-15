#![feature(panic_info_message)]
#![feature(thread_id_value)]
extern crate graph;
pub(crate) use graph::*;

#[macro_use]
extern crate lazy_static;

mod from_csv;
pub use from_csv::*;

mod from_vec;
pub use from_vec::*;

mod handle_panics;
use handle_panics::*;

mod signal_handler;
use signal_handler::*;

mod mega_test;
pub use mega_test::*;