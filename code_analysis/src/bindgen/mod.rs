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

pub fn extract_module_name_from_path(path: &str) -> Option<String> {
    let re = Regex::new(r"\.\./graph/src/(.+)/.+\.rs").unwrap();
    re.captures(path).map(|x| x.get(1).unwrap().as_str().to_string())
}

pub fn gen_bindings(path: &str, init_path: &str) {
    print_sep();
    println!("Parsing the library source files");
    print_sep();
    let modules = get_library_sources();

    print_sep();
    println!("Generating the bindings");
    print_sep();

    let functions = modules.iter()
        .flat_map(|module| module.iter_on_functions_and_methods().cloned());

    let mut method_bindings = vec![];
    let mut function_bindings = vec![];
    let mut functions_modules: HashMap<String, Vec<Binding>> = HashMap::new();

    for func in functions {
        if !func.name.starts_with("iter")
            && !func.name.starts_with("par_iter")
            && func.class.as_ref().map_or(true, |class| class == "Graph")
            && func.visibility == Visibility::Public
            && !func.attributes.iter().any(|x| x == "no_binding")
            && !func.attributes.iter().any(|x| x == "manual_binding")
        {
            println!("Generating binding for {}:{}", &func.file_path, &func.name);
            let binding: Binding = func.clone().into();
            
            if binding.is_method {
                method_bindings.push(binding);
            } else {
                match extract_module_name_from_path(binding.file_path.as_str()) {
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
        r#"use super::*;
use pyo3::{{wrap_pyfunction, wrap_pymodule}};

#[pymodule]
fn ensmallen_graph(_py: Python, m: &PyModule) -> PyResult<()> {{
    m.add_class::<EnsmallenGraph>()?;
    m.add_wrapped(wrap_pymodule!(preprocessing))?;
    {function_modules_bindings_registration}
    {function_bindings_wrapping}
    env_logger::init();
    Ok(())
}}

{function_modules_bindings_definition}

{function_bindings}

{function_modules_bindings}

#[pymethods]
impl EnsmallenGraph {{
{method_bindings}
}}"#,
    function_bindings_wrapping=function_bindings.iter()
        .map(|binding| format!("m.add_wrapped(wrap_pyfunction!({name}))?;", name=binding.name))
        .collect::<Vec<_>>().join("\n"),

    function_modules_bindings_registration=functions_modules.iter()
        .map(|(module_name, _)| {
            format!("m.add_wrapped(wrap_pymodule!({}))?;", module_name)
        }).collect::<Vec<String>>().join(""),

    function_modules_bindings_definition=functions_modules.iter()
        .map(|(module_name, functions)| {
            format!(
r#"
#[pymodule]
fn {module_name}(_py: Python, m: &PyModule) -> PyResult<()> {{
    {functions_registration}
    Ok(())
}}
"#,
    module_name=module_name,
    functions_registration=functions.iter()
        .map(|func| {
            format!("m.add_wrapped(wrap_pyfunction!({name}))?;", name=func.name)
        }).collect::<Vec<String>>().join(""),
            )
        }).collect::<Vec<String>>().join(""),

    function_bindings=function_bindings.into_iter()
        .map(|x| x.into())
        .collect::<Vec<String>>().join(""),

    function_modules_bindings=functions_modules.iter()
        .flat_map(|(_module_name, functions)| {
            functions.iter()
                .map(|function| function.clone().into())
                .collect::<Vec<String>>()
        
        })
        .collect::<Vec<String>>().join(""),

    method_bindings=method_bindings.into_iter()
        .map(|x| x.into())
        .collect::<Vec<String>>().join(""),
    );

    fs::write(
        path,
        file_content,
    )
    .expect("Cannot write the automatically generated bindings file");


    let mut lines = vec!["\"\"\"Module offering fast graph processing and graph datasets.\"\"\"".to_string()];

    let mut elements = functions_modules.keys().cloned().collect::<Vec<_>>();
    elements.push("EnsmallenGraph".to_string());

    for module in elements.iter() {
        lines.push(format!("from .ensmallen_graph import {} # pylint: disable=import-error", module));
    }

    lines.push(format!(
        "__all__ = {:?}", elements
    ));

    fs::write(
        init_path,
        lines.join("\n"),
    )
    .expect("Cannot write the init file");
}