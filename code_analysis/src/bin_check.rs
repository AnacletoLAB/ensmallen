use libcodeanalysis::*;

fn main() {
    let mut checker = Checker::new(parse_crate("../src/graph", DENY_LIST));
    checker.check();
    checker.display();
    checker.exit();
}
