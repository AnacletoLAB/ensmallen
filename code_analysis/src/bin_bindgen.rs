use libcodeanalysis::*;
use rust_parser::*;
use std::collections::HashSet;
use std::fs;
use std::fs::read_dir;

mod bindgen;
use bindgen::*;


fn main() {
    // Generate the bindings
    gen_bindings(
        "graph",
        r#"
#[allow(unused_imports)]
use utils::*;
#[allow(unused_imports)]
use crate::types::*;
#[allow(unused_imports)]
use graph::{{DumpGraph, Graph}};
"#,
"../bindings/python/crates/py_graph/src/auto.rs"
    );
    gen_bindings(
        "url_utilities",
        r#"
#[allow(unused_imports)]
use utils::*;
"#,
    "../bindings/python/crates/py_url_utilities/src/lib.rs"
    );
    gen_bindings(
        "edge_list_utils",
        r#"
#[allow(unused_imports)]
use utils::*;
"#,
        "../bindings/python/crates/py_edge_list_utils/src/lib.rs"
    );

    // Generate the tfidf weights
    tfidf_gen("../bindings/python/crates/py_graph/src/method_names_list.rs");

    // Format the files
    format_crate("../bindings/python/crates/py_graph/");
    format_crate("../bindings/python/crates/py_url_utilities/");
    format_crate("../bindings/python/crates/py_edge_list_utils/");
}