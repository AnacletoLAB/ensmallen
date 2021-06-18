use super::*;
use std::fs;
use std::fs::read_dir;

/// List of the files we will skip in the analysis
/// becasue they have features we don't have implmented yet
/// nor we care about.
const BLACKLIST: &'static [&'static str] = &[
    "types.rs", // macro calls
    "walks.rs", // mods
    "lib.rs",   // mods
    "core.c",   // it is C
    "test_utilities.rs",
];

pub fn skip_file(path: &str) -> bool {
    for deny in BLACKLIST.iter(){
        if path.contains(deny) {
            eprintln!("SKIPPING");
            return true;
        }
    }
    false
}

pub fn get_library_sources() -> Vec<Module> {
    let src_files: Vec<String> = read_dir("../graph/src")
        .unwrap()
        .map(|path| 
            path.unwrap().path().into_os_string()
                .into_string().unwrap().to_string()
        )
        .filter(|path| !skip_file(&path))
        .collect();

    let mut modules = Vec::new();
    for path in src_files {
        // read the file
        println!("Parsing: {:?}", path);
        let contents = fs::read_to_string(path).expect("File not found");
        // parse the file
        let (_reminder, module) = Module::parse(contents.as_bytes());
        modules.push(module);
    }

    modules
}