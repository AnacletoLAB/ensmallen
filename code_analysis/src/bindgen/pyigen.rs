use super::*;

/// List of the files we will skip in the analysis
/// becasue they have features we don't have implmented yet
/// nor we care about.
const DENY_LIST: &[&str] = &[
    "lib.rs",    // mod
    "macros.rs", // macro calls
];


fn gen_module_pyi(module: &BindingsModule) -> String {
    let mut result = String::new();
    let user_defined_types = module.structs.keys().map(String::as_str).collect::<Vec<_>>();
    println!("user_defined_types: {:?}", user_defined_types);
    for (struct_name, ztruct) in &module.structs {
        // Skip non-binding structs
        if !ztruct.ztruct.attributes.iter().any(|x| x == "pyclass")  {
            continue;
        }

        // create a class for each struct
        result.push_str(&format!("\nclass {}:", struct_name));
        for imp in &ztruct.impls {

            // skip non-bindings impls
            if !imp.attributes.iter().any(|x| x == "pymethods") {
                continue
            }
            // parse each method
            for method in &imp.methods {
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
                        translate_type(&arg.arg_type,  &user_defined_types)
                    ));
                }

                // Generate the method doc
                result = format!(
r#"{result}
    {static_method}def {method_name}({args}){return_type}: ..."#,     
                result=result,
                args=args.join(", "),
                method_name=&method.name,
                static_method=if method.is_static() {
                    "@staticmethod\n    "
                } else {
                    ""
                },
                return_type=if let Some(rt) = &method.return_type {
                    let translated_type = translate_type(rt, &user_defined_types);
                    if translated_type == "None" {
                        "".to_string()
                    } else {
                        format!(" -> {}", translated_type)
                    }
                } else {
                    "".to_string()
                },
                );
            }
        }

        result.push_str("\n    pass\n");
    }

    result
}

pub fn gen_pyi(bindings_path: &str, target_file: &str) {
    let modules = group_data(parse_crate(bindings_path, DENY_LIST));

    let file_content = format!(
r#"
from typing import *
import numpy as np

{content}
"#, content=gen_module_pyi(&modules),
);
    fs::write(
        target_file,
        file_content,
    )
    .expect(&format!("Cannot write pyifile at {}", target_file));

}