use libcodeanalysis::*;
use rust_parser::*;
use std::collections::HashSet;
use std::fs;
use std::fs::read_dir;

mod bindgen;
use bindgen::*;


fn main() {
    // Generate the bindings
    gen_bindings("../bindings/python/src/auto_generated_bindings.rs", "../bindings/python/ensmallen/__init__.py");

    // Format the files
    assert!(
        std::process::Command::new("cargo")
            .args(&["fmt"])
            .current_dir("../bindings/python")
            .status()
            .expect("Could not run format on the python bindings")
            .success(),
        "The cargo format failed and returned non-zero exit status"
    );
}