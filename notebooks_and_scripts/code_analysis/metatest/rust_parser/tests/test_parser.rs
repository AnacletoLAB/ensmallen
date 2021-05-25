use std::fs;
use rust_parser::*;

/// List of the files we will skip in the analysis
/// becasue they have features we don't have implmented yet
/// nor we care about.
const BLACKLIST: &'static [&'static str] = &[
    "utils.rs", // macro rules
    "types.rs", // macro rules
    "walks.rs", // mods
    "lib.rs",   // mods
    "core.c",   // it is C
];

#[test]
fn test_parsing() {
    'outer: for path in fs::read_dir("../../../../graph/src").unwrap() {
        let path = path.unwrap().path().into_os_string().into_string().unwrap();
        println!("Parsing file: {}", path);

        for deny in BLACKLIST.iter(){
            if path.contains(deny) {
                println!("SKIPPING");
                continue 'outer;
            }
        }

        let contents = fs::read_to_string(path)
            .expect("File not found");
        let (_reminder, parsing) = Module::parse(contents.as_bytes());
        for im in parsing.impls {
            println!("doc:'{}'\nimpl: {}", im.doc, String::from(im.struct_name));
            for method in im.methods {
                println!("\t{:?} {:?} {}", method.visibility, method.attributes, method.name);
            }
        }
    }
}