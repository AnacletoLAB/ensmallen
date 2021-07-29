use super::*;
use std::fs;

pub fn fix_init(modules: Vec<ValueToWrap>, init_path: &str, auto_import_path: &str) {
    let mut lines = vec!["\"\"\"Module offering fast graph processing and graph datasets.\"\"\"".to_string()];

    let mut elements = modules.iter().map(ValueToWrap::get).collect::<Vec<_>>();
    elements.push("EnsmallenGraph".to_string());
    elements.push("preprocessing".to_string());

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


    let file_content = format!(
        r#"        
#[allow(unused_imports)]
use super::*;
#[allow(unused_imports)]
use pyo3::{{wrap_pyfunction, wrap_pymodule}};
#[allow(unused_imports)]
use pyo3::prelude::*;

#[pymodule]
fn ensmallen_graph(_py: Python, m: &PyModule) -> PyResult<()> {{
    m.add_class::<EnsmallenGraph>()?;
    m.add_wrapped(wrap_pymodule!(preprocessing))?;
    {function_modules_bindings_registration}
    env_logger::init();
    Ok(())
}}

#[pymodule]
fn preprocessing(_py: Python, m: &PyModule) -> PyResult<()> {{
    m.add_wrapped(wrap_pyfunction!(word2vec))?;
    m.add_wrapped(wrap_pyfunction!(cooccurence_matrix))?;
    m.add_wrapped(wrap_pyfunction!(okapi_bm25_tfidf))?;
    Ok(())
}}

{modules}
"#,
function_modules_bindings_registration=modules.iter()
        .map(|value| {
            format!("m.add_wrapped({})?;", value.wrap())
        }).collect::<Vec<String>>().join("\n"),

    modules=modules.iter()
        .map(|x| x.gen_module())
        .collect::<Vec<_>>()
        .join("\n")
    );

    fs::write(
        auto_import_path,
        file_content,
    )
    .expect("Cannot write the auto_import file");
}