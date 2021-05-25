use rust_parser::*;
use libmetatest::*;
use std::fs::read_dir;

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

fn skip_file(path: &str) -> bool {
    for deny in BLACKLIST.iter(){
        if path.contains(deny) {
            println!("SKIPPING");
            return true;
        }
    }
    false
}

fn main() {
    let files: Vec<String> = read_dir("../../../graph/src")
        .unwrap()
        .map(|path| 
            path.unwrap().path().into_os_string()
                .into_string().unwrap().to_string()
        )
        .filter(|path| !skip_file(&path))
        .collect();

    let mut checker = Checker::parse_files(files);
    checker.check();
    checker.display(); 
}