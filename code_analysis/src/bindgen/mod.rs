use super::*;

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


pub fn gen_bindings(path: &str) {
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
                function_bindings.push(binding);
            }
        }
    }

    print_sep();
    println!("Generated bindings for {} functions and {} methods.",  function_bindings.len(), method_bindings.len());
    print_sep();

    let file_content = format!(
        r#"use super::*;
use pyo3::{{wrap_pyfunction, wrap_pymodule}};

#[pymodule]
fn ensmallen_graph(_py: Python, m: &PyModule) -> PyResult<()> {{
    m.add_class::<EnsmallenGraph>()?;
    m.add_wrapped(wrap_pymodule!(preprocessing))?;
    {function_bindings_wrapping}
    env_logger::init();
    Ok(())
}}

{function_bindings}

#[pymethods]
impl EnsmallenGraph {{
{method_bindings}
}}"#,
    function_bindings_wrapping=function_bindings.iter()
        .map(|binding| format!("m.add_wrapped(wrap_pyfunction!({name}))?;", name=binding.name))
        .collect::<Vec<_>>().join("\n"),

    function_bindings=function_bindings.into_iter()
        .map(|x| x.into())
        .collect::<Vec<String>>().join(""),

    method_bindings=method_bindings.into_iter()
        .map(|x| x.into())
        .collect::<Vec<String>>().join(""),
    );

    fs::write(
        path,
        file_content,
    )
    .expect("Cannot weite the automatically generated bindings file");

}