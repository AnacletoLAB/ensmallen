use rust_parser::*;
use libcodeanalysis::*;

fn main() {
    let mut checker = Checker::new(get_library_sources());
    checker.check();
    checker.display(); 
    checker.exit();
}