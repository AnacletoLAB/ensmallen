use libcodeanalysis::*;

fn main() {
    let mut checker = Checker::new(get_sources("../crates/graph/src"));
    checker.check();
    checker.display(); 
    checker.exit();
}