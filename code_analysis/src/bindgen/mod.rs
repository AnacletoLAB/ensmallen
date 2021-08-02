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


pub fn gen_bindings(lib_name: &str, imports: &str, target_file: &str) {
    let src_path = format!("../crates/{}/src", lib_name);

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
                function_bindings.push(binding);
            }
        }
    }



    print_sep();
    println!(
        "Generated bindings for {} functions, {} methods, at {}",  
        function_bindings.len(), 
        method_bindings.len(),
        target_file,
    );
    print_sep();

    let file_content = format!(
        r#"
{imports}

#[pymodule]
fn py_{lib_name}(_py: Python, m: &PyModule) -> PyResult<()> {{
    {maybe_ensmallen}
    {func_wrapping}
    Ok(())
}}


{function_bindings}

{methods}
"#,
    imports=imports,
    lib_name =lib_name,
    maybe_ensmallen = if lib_name == "graph" {
        "m.add_class::<EnsmallenGraph>()?;\nenv_logger::init();\n".to_string()
    } else {
        String::new()
    },
    func_wrapping = function_bindings.iter()
        .map(|func| {
            format!("m.add_wrapped(wrap_pyfunction!({}))?;", func.name)
        })
        .collect::<Vec<_>>().join("\n"),

    function_bindings=function_bindings.iter()
        .map(|x| x.clone().into())
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
        target_file,
        file_content,
    )
    .expect("Cannot write the automatically generated bindings file");
}