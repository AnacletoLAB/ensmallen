use super::*;
use std::collections::HashMap;
use regex::Regex;

mod binding;
pub use binding::*;

mod get_binding_names;
pub use get_binding_names::*;

mod translate_doc;
pub use translate_doc::*;

mod translate_type;
pub use translate_type::*;

mod tfidf_gen;
pub use tfidf_gen::*;

mod fix_init;
pub use fix_init::fix_init;

pub fn extract_module_name_from_path(path: &str) -> Option<String> {
    let re = Regex::new(r"\.\./crates/graph/src/(.+)/.+\.rs").unwrap();
    re.captures(path).map(|x| x.get(1).unwrap().as_str().to_string())
}

pub enum ValueToWrap {
    Function(String),
    Module{
        module_name: String,
        values: Vec<ValueToWrap>,
    },
}

impl ValueToWrap {
    pub fn get(&self) -> String {
        match self {
            ValueToWrap::Function(func) => {
                func.to_string()
            },
            ValueToWrap::Module{module_name, ..} => {
                module_name.to_string()
            }
        }
    }

    pub fn wrap(&self) -> String {
        match self {
            ValueToWrap::Function(func) => {
                format!("wrap_pyfunction!({})", func)
            },
            ValueToWrap::Module{module_name, ..} => {
                format!("wrap_pymodule!({})", module_name)
            }
        }
    }

    pub fn gen_module(&self) -> String {
        match self {
            ValueToWrap::Function(func) => {
                String::new()
            },
            ValueToWrap::Module{module_name, values} => {
                format!(
r#"
#[pymodule]
fn {module_name}(_py: Python, m: &PyModule) -> PyResult<()> {{
{wraps}
    Ok(())
}}
"#,
    module_name=module_name,
    wraps = values.iter().map(|x| format!("\t{};\n", x.wrap())).collect::<Vec<_>>().join(""),
                )
            }
        }
    }
}


pub fn gen_bindings(lib_name: &str, is_submodule: bool) -> Vec<ValueToWrap> {

    let src_path = format!("../crates/{}/src", lib_name);
    let path = format!("../bindings/python/src/auto_{}.rs", lib_name);

    print_sep();
    println!("Parsing the library source files");
    print_sep();
    let modules = get_sources(src_path.as_str());

    print_sep();
    println!("Generating the bindings");
    print_sep();

    let functions = modules.iter()
        .flat_map(|module| module.iter_on_functions_and_methods().cloned());

    let mut method_bindings = vec![];
    let mut function_bindings = vec![];
    let mut functions_modules: HashMap<String, Vec<Binding>> = HashMap::new();

    let submodule = if is_submodule {
        Some(lib_name.to_string())
    } else {
        None
    };

    for mut func in functions {
        if !func.name.starts_with("iter")
            && !func.name.starts_with("par_iter")
            && func.class.as_ref().map_or(true, |class| class == "Graph")
            && func.visibility == Visibility::Public
            && !func.attributes.iter().any(|x| x == "no_binding")
            && !func.attributes.iter().any(|x| x == "manual_binding")
        {
            func.module_name = lib_name.to_string();
            println!("Generating binding for {}:{}", &func.file_path, &func.name);
            let binding: Binding = func.clone().into();
            
            if binding.is_method {
                method_bindings.push(binding);
            } else {
                match extract_module_name_from_path(binding.file_path.as_str()).or(submodule.clone()) {
                    Some(module_name) => {
                        let module_functions = functions_modules.entry(module_name).or_insert_with(Vec::new);
                        module_functions.push(binding);
                    }
                    None => {
                        function_bindings.push(binding);
                    }
                }
            }
        }
    }



    print_sep();
    println!(
        "Generated bindings for {} functions, {} methods, and {} modules at {}",  
        function_bindings.len() + functions_modules.iter().map(|(_, funcs)| funcs.len()).sum::<usize>(), 
        method_bindings.len(),
        functions_modules.len(),
        path,
    );
    print_sep();

    let file_content = format!(
        r#"
#[allow(unused_imports)]
use tags::*;
#[allow(unused_imports)]
use shared::*;
#[allow(unused_imports)]
use pyo3::prelude::*;
#[allow(unused_imports)]
use pyo3::types::PyDict;
#[allow(unused_imports)]
use numpy::{{PyArray, PyArray1, PyArray2}};
#[allow(unused_imports)]
use crate::types::EnsmallenGraph;
#[allow(unused_imports)]
use graph::{{DumpGraph, Graph}};
#[allow(unused_imports)]
use std::collections::{{HashMap, HashSet}};

{function_bindings}

{function_modules_bindings}

{methods}
"#,

    function_bindings=function_bindings.iter()
        .map(|x| x.clone().into())
        .collect::<Vec<String>>().join(""),

    function_modules_bindings=functions_modules.iter()
        .flat_map(|(_module_name, functions)| {
            functions.iter()
                .map(|function| function.clone().into())
                .collect::<Vec<String>>()
        
        })
        .collect::<Vec<String>>().join(""),

    methods = if method_bindings.is_empty() {
        String::new()
    } else {
        format!(r#"
#[pymethods]
impl EnsmallenGraph {{
{method_bindings}
}}
"#,
        method_bindings=method_bindings.into_iter()
            .map(|x| x.into())
            .collect::<Vec<String>>().join(""),
        )}
    );
    

    fs::write(
        path,
        file_content,
    )
    .expect("Cannot write the automatically generated bindings file");



    
    functions_modules.iter()
        .map(|(module_name, bindings)| ValueToWrap::Module{
            module_name: module_name.clone(),
            values: bindings.iter().map(|x| ValueToWrap::Function(x.name.clone())).collect::<Vec<_>>()
        })
        .chain(
            function_bindings.iter()
                .map(|x| ValueToWrap::Function(x.name.clone()))
        )
        .collect::<Vec<_>>()
}