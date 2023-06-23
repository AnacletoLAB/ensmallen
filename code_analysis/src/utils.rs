use super::*;
use std::fs;
use walkdir::WalkDir;

/// List of the files we will skip in the analysis
/// becasue they have features we don't have implmented yet
/// nor we care about.
pub const DENY_LIST: &[&str] = &[
    "types.rs", // macro calls
    "walks.rs", // mods
    "lib.rs",   // mods
    "core.c",   // it is C
    "test_utilities.rs",
    "mod.rs",
    "method_caller.rs",
    "parallel_lines_reader.rs",
    "iters_wrapper.rs",
    "parallel_lines_reader_with_index.rs",
    "heterogeneous_graphlets.rs",
    "url_utilities",
    "constructors",
    "argmax_argmin.rs",
    "minmax.rs",
    "splitter.rs",
    "mmap",
    "data_structures",
    "isomorphism_iter",
    "hashes",
    "edge_quadruple",
];

pub fn skip_file(path: &str, deny_list: &'static [&'static str]) -> bool {
    for deny in deny_list.iter() {
        if !path.ends_with(".rs") || path.contains(deny) {
            eprintln!("Skipping the file {}", path);
            return true;
        }
    }
    false
}

/// Returns a vector of modules that represent all the parsable files
/// of the library.
///
/// # Panics
/// If this function panics then probably the source folder path is wrong!
pub fn parse_crate(path: &str, deny_list: &'static [&'static str]) -> Vec<Module> {
    let src_files: Vec<String> = WalkDir::new(path)
        .into_iter()
        .filter_map(|entry| {
            let value = entry.unwrap();
            if value.file_type().is_dir() {
                None
            } else {
                Some(value.path().to_str().unwrap().to_string())
            }
        })
        .filter(|path| !skip_file(path, deny_list))
        .collect();

    let mut modules = Vec::new();
    for path in src_files {
        // read the file
        println!("Parsing: {:?}", path);
        let contents = fs::read_to_string(&path).expect("File not found");
        // parse the file
        let (_reminder, mut module) = Module::parse(contents.as_bytes());
        module.set_path(path);
        modules.push(parse_macros(module));
    }

    modules
}

pub fn print_sep() {
    println!("--------------------------------------------------------------------------------");
}
