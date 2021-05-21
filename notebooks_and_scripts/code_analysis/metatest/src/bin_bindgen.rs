use rust_parser::*;
use libmetatest::*;
use std::fs;
use std::fs::read_dir;
use std::collections::HashSet;

/// List of the files we will skip in the analysis
/// becasue they have features we don't have implmented yet
/// nor we care about.
const BLACKLIST: &'static [&'static str] = &[
    "utils.rs", // macro rules
    "types.rs", // macro rules
    "walks.rs", // mods
    "lib.rs",   // mods
    "core.c",   // it is C
    "macros.rs"
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

fn get_binding_names() -> HashSet<String> {
    let bindings_files: Vec<String> = read_dir("../../../bindings/python/src")
    .unwrap()
    .map(|path| 
        path.unwrap().path().into_os_string()
            .into_string().unwrap().to_string()
    )
    .filter(|path| !skip_file(&path))
    .collect();
    let mut bindings_modules = Vec::new();
    let mut method_names = HashSet::new();
    for path in bindings_files{
        println!("{:?}", path);
        // read the file
        let contents = fs::read_to_string(path).expect("File not found");
        // parse the file
        let (_reminder, module) = Module::parse(contents.as_bytes());
        method_names.extend(module.get_function_names());
        bindings_modules.push(module);
    }
    method_names
}

fn gen_binding(method: &Function) -> String {

    

    format!(r#"
    pub fn {name}(&self) {{
        self.{name}()
    }}
        "#, 
        name=method.name
    )
}



fn main() {
    let method_names = get_binding_names();
    println!("{:?}", method_names);


    let src_files: Vec<String> = read_dir("../../../graph/src")
        .unwrap()
        .map(|path| 
            path.unwrap().path().into_os_string()
                .into_string().unwrap().to_string()
        )
        .filter(|path| !skip_file(&path))
        .collect();

    for path in src_files{
        // read the file
        let contents = fs::read_to_string(path).expect("File not found");
        // parse the file
        let (_reminder, module) = Module::parse(contents.as_bytes());

        for imp in module.impls {
            if String::from(imp.struct_name) != "Graph".to_string() {
                continue
            }
            for method in imp.methods {
                if !method_names.contains(&method.name)
                    && !method.name.starts_with("iter") 
                    && !method.name.starts_with("par_iter") 
                    && method.visibility == Visibility::Public
                {
                    println!("MISSING {:?}", gen_binding(&method));
                }
            }
        }
    }
}