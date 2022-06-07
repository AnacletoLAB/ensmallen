use libcodeanalysis::*;

fn main() {
    // Generate the bindings
    gen_skeleton("../bindings/python/src/", "../bindings/python/ensmallen/ensmallen/");
}