use rust_parser::*;
use libmetatest::*;

fn main() {
    let mut checker = Checker::new(get_library_sources());
    checker.check();
    checker.display(); 
}