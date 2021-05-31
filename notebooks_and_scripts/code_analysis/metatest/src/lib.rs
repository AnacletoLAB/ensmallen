use rust_parser::*;

mod error;
use error::*;

mod check_doc;
pub use check_doc::*;

mod checker;
pub use checker::*;

mod utils;
pub use utils::*;

#[macro_export]
macro_rules! log {
    ($($arg:tt)*) => {
        println!($($arg)*);
        ERRORS_WHERE_FOUND.set();
    };
}