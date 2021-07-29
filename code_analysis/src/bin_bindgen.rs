use libcodeanalysis::*;
use rust_parser::*;
use std::collections::HashSet;
use std::fs;
use std::fs::read_dir;

mod bindgen;
use bindgen::*;


fn main() {
    // Generate the bindings
    let mut modules = Vec::new();
    modules.extend(gen_bindings("graph", false));
    modules.extend(gen_bindings("edge_list_utils", true));
    modules.extend(gen_bindings("url_utilities", true));


    fix_init(
        modules,
        "../bindings/python/ensmallen_graph/__init__.py",
        "../bindings/python/src/auto_import.rs",
    );

    // Generate the tfidf weights
    tfidf_gen("../bindings/python/src/method_names_list.rs");

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