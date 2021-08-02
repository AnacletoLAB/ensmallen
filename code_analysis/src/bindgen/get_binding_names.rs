use super::*;
use std::collections::HashSet;

pub fn get_binding_names() -> HashSet<String> {
    let bindings_files: Vec<String> = read_dir("../bindings/python/crates/py_graph/src")
        .unwrap()
        .map(|path| {
            path.unwrap()
                .path()
                .into_os_string()
                .into_string()
                .unwrap()
        })
        .filter(|path| !skip_file(path))
        .collect();
    let mut bindings_modules = Vec::new();
    let mut method_names = HashSet::new();
    for path in bindings_files {
        eprintln!("Getting the name of the bindings in {}", path);
        // read the file
        let contents = fs::read_to_string(path).expect("File not found");
        // parse the file
        let (_reminder, module) = Module::parse(contents.as_bytes());
        method_names.extend(module.get_function_names());
        bindings_modules.push(module);
    }
    method_names
}
