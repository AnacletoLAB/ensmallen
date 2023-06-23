use super::*;
use std::path::Path;

/// List of the files we will skip in the analysis
/// becasue they have features we don't have implmented yet
/// nor we care about.
const DENY_LIST: &[&str] = &[
    "lib.rs",    // mod
    "heterogeneous_graphlets.rs", 
    "macros.rs", // macro calls
];

fn tabbify(code: &str, strip_space: bool) -> String {
    code.split("\n")
        .map(|mut x| {
            if strip_space && x.starts_with(" ") {
                x = &x[1..];
            }
            format!("    {}", x)
        })
        .collect::<Vec<_>>()
        .join("\n")
}

fn translate_function(method: &Function, user_defined_types: &[&str], is_method: bool) -> String {
    // Translate the args of the function
    let mut args = Vec::new();
    for arg in &method.args.0 {
        if arg.name == "self" {
            args.push("self".into());
            continue;
        }
        if arg.name == "py_kwargs" {
            args.push("**kwargs".into());
            continue;
        }
        args.push(format!(
            "{}: {}",
            &arg.name,
            arg.arg_type.to_python_type(user_defined_types)
        ));
    }

    // Generate the method doc
    format!(
        r#"
{static_method}def {method_name}({args}){return_type}:
    """{doc}"""
    pass
"#,
        doc = tabbify(&method.doc, true).trim(),
        args = args.join(", "),
        method_name = &method.name,
        static_method = if is_method && method.is_static() {
            "@staticmethod\n"
        } else {
            ""
        },
        return_type = if let Some(rt) = &method.return_type {
            let translated_type = rt.to_python_type(user_defined_types);
            if translated_type == "None" {
                "".to_string()
            } else {
                format!(" -> {}", translated_type)
            }
        } else {
            "".to_string()
        },
    )
}

fn gen_module_pyi(module: &BindingsModule, target_folder: &str, filename: &str) {
    let mut result = String::new();
    let user_defined_types = module
        .structs
        .keys()
        .map(String::as_str)
        .collect::<Vec<_>>();
    for (struct_name, ztruct) in &module.structs {
        // Skip non-binding structs
        if !ztruct.ztruct.attributes.iter().any(|x| x == "pyclass") {
            continue;
        }

        // create a class for each struct
        result.push_str(&format!(
            r#"
class {}:
    """{}"""
"#,
            struct_name,
            ztruct.ztruct.doc.trim()
        ));
        for imp in &ztruct.impls {
            // skip non-bindings impls
            if !imp.attributes.iter().any(|x| x == "pymethods") {
                continue;
            }
            // parse each method
            for method in &imp.methods {
                result.push_str(&tabbify(
                    &translate_function(method, &user_defined_types, true),
                    false,
                ));
            }
        }
    }

    for func in &module.funcs {
        if !func
            .attributes
            .iter()
            .any(|x| x.0.starts_with("pyfunction"))
        {
            continue;
        }
        result.push_str(&translate_function(func, &user_defined_types, false));
    }

    let file_content = format!(
        r#"
from __future__ import annotations
from typing import *
import numpy as np
{imports}

{content}
"#,
        content = result,
        imports = module
            .modules
            .keys()
            .map(|x| format!("from . import {}", x))
            .collect::<Vec<_>>()
            .join("\n"),
    );
    let target_file = Path::new(target_folder).join(filename);
    fs::write(&target_file, file_content)
        .expect(&format!("Cannot write pyifile at {:?}", &target_file));

    for (submod_name, submod) in &module.modules {
        gen_module_pyi(submod, target_folder, &format!("{}.py", submod_name))
    }
}

pub fn gen_skeleton(bindings_path: &str, target_folder: &str) {
    fs::create_dir_all(target_folder).expect("Could not create the skeleton folder");
    let modules = group_data(parse_crate(bindings_path, DENY_LIST));
    gen_module_pyi(&modules, target_folder, "__init__.py");
}
