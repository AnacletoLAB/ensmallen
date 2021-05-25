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

fn translate_type_str(value: String) -> String {
    let (_, t) = Type::parse(value.as_bytes());
    translate_type(t)
}

fn translate_type(value: Type) -> String {
    match value.clone() {
        Type::SimpleType{
            name,
            modifiers,
            generics,
            traits
        } => {
            match name.as_str() {
                "Graph" => "EnsmallenGraph".to_string(),
                "NodeT" => "int".to_string(),
                "usize" => "int".to_string(),
                "EdgeT" => "int".to_string(),
                "bool" => "bool".to_string(),
                "str" => "str".to_string(),
                "String" => "str".to_string(),
                "NodeTypeT" => "int".to_string(),
                "EdgeTypeT" => "int".to_string(),
                "S" => "str".to_string(),
                "RoaringBitmap" => "List[int]".to_string(),
                "Option" => {
                    let mut result = "Optional[".to_string();
                    for value in generics.0 {
                        match value {
                            GenericValue::Type(t) => {
                                result.extend(translate_type(t).chars())
                            }
                            _ => panic!("Cannot traduce to python the generic value {:?}", value)
                        }
                    }
                    result.push(']');
                    result
                }
                "Vec" => {
                    let mut result = "List[".to_string();
                    for value in generics.0 {
                        match value {
                            GenericValue::Type(t) => {
                                result.extend(translate_type(t).chars())
                            }
                            _ => panic!("Cannot traduce to python the generic value {:?}", value)
                        }
                    }
                    
                    result.push(']');
                    result
                }
                _ => {
                    panic!("Cannot translate '{:?}' as a python unknown type", value);
                }
            }
        }
        _ => {
            panic!("Cannot translate '{:?}' as a python type", value);
        }
    }
}

fn translate_doc(doc: String) -> String {
    let mut result = String::new();

    // parse the documentation into sections
    let (_, doc) = Doc::parse(doc.as_bytes());
    let sections = doc.sections;

    for section in sections {
        match section {
            DocSection::Introduction(intro) => {
                result.extend(bytes_to_string(trim(intro.as_bytes())).chars());
                result.push('\n');
            }
            DocSection::Arguments{
                prologue,
                arguments,
                epilogue,
            } => {

                result.extend("\nParameters\n----------\n".chars());

                //args_sec.extend(prologue.chars());

                for argument in arguments {
                    match argument {
                        Argument::Parsable(DocArg{
                            name,
                            arg_type,
                            description,
                        }) => {
                            result.extend(
                                format!(
                                    "{name}: {arg_type},\n    {description}\n",
                                    name=name,
                                    arg_type=translate_type_str(arg_type),
                                    description=description,
                                ).chars()
                            )
                        },
                        Argument::NotParsable(_) => {}
                    }
                }


                //args_sec.extend(epilogue.chars());
            }
            DocSection::Raises{
                prologue,
                exceptions,
                epilogue,
            } => {
                result.extend("\nRaises\n-------\n".chars());

                for excp in exceptions {
                    result.extend(format!(
                        "ValueError\n    {}\n",
                        excp
                    ).chars());
                }
            }
            DocSection::Unsafe{
                text,
            } => {
                result.extend("\nSafety\n------\n".chars());
                result.extend(text.chars());
            }
            _ => {}
        }
    }

    result.push('\n');

    result.split("\n")
    .map(|x| format!("    /// {}", x))
    .collect::<Vec<_>>()
    .join("\n")
}

fn gen_binding(method: &Function) -> String {
    // build the doc
    let doc = translate_doc(method.doc.clone());

    // parse the arguments
    let mut is_self_ref = false;
    let mut is_self_mut = false;
    let mut args = String::new();
    let mut args_names = String::new();
    let mut args_signatures = vec!["$self".to_string()];
    for arg in &method.args.0 {
        match arg.arg_type {
            Type::SelfType => {
                args.extend(format!(
                    "{}self, ",
                    String::from(arg.arg_modifier.clone())
                ).chars());

                is_self_ref = arg.arg_modifier.reference;
                is_self_mut = arg.arg_modifier.mutable;
            }
            _ => {
                args.extend(format!(
                    "{}: {}, ", 
                    arg.name, 
                    String::from(arg.arg_type.clone())
                ).chars());
                args_names.extend(format!(
                    "{}, ",
                    arg.name
                ).chars());
                args_signatures.push(arg.name.clone());
            }
        }
    }

    let text_signature = format!(
        "#[text_signature = \"({})\"]",
        args_signatures.join(", ")
    );

    // build the call
    let mut body = format!(
        "self.{name}({args_names})",
        name=method.name,
        args_names=&args_names[..args_names.len().saturating_sub(2)],
    );



    // parse the return type
    let return_type = match &method.return_type {
        None => String::new(),
        Some(r_type) => {
            match &r_type {
                Type::SimpleType{
                    name,
                    modifiers,
                    generics,
                    traits,
                } => {
                    match name.as_str() {
                        "Graph" => {
                            match (is_self_ref, is_self_mut, modifiers.reference) {
                                (true, true, _) => {
                                    "".to_string()
                                },
                                (true, false, false) => {
                                    body = format!("EnsmallenGraph{{graph: {}}}", body);
                                    "-> EnsmallenGraph".to_string()
                                },
                                (true, false, true) => {
                                    body = format!("EnsmallenGraph{{graph: {}}}.to_owned()", body);
                                    "-> EnsmallenGraph".to_string()
                                }
                                _ => {
                                    panic!("Not implemented yet!");
                                }
                            }
                        }
                        "Result" => {
                            body = format!("pe!({})", body);


                            let mut ok_type = match generics.0[0].clone() {
                                GenericValue::Type(Type::SimpleType {
                                    name: mut ok_name, 
                                    modifiers: ok_modifiers,
                                    generics: ok_generics,
                                    traits: ok_traits,
                                }) => {
                                    if ok_name == "Graph" {
                                        ok_name = "EnsmallenGraph".to_string();
        
                                        match (is_self_ref, is_self_mut, ok_modifiers.reference) {
                                            (true, true, _) => {
                                                body = format!("{body}?;\n\tOk(())", body=body);
                                            },
                                            (true, false, false) => {
                                                body = format!("Ok(EnsmallenGraph{{graph: {}?}})", body);
                                            },
                                            (true, false, true) => {
                                                body = format!("Ok(EnsmallenGraph{{graph: {}?}}.to_owned())", body);
                                            }
                                            _ => {
                                                panic!("Not implemented yet!");
                                            }
                                        }
                                    }
                                    
                                    GenericValue::Type(Type::SimpleType{
                                        name:ok_name,
                                        modifiers: ok_modifiers,
                                        generics: ok_generics,
                                        traits: ok_traits,
                                    })
                                }
                                x @ _ => x 
                            };

                            format!(
                                " -> {} ",
                                String::from(Type::SimpleType{
                                    name:"PyResult".to_string(),
                                    modifiers: modifiers.clone(),
                                    generics: Generics(vec![ok_type]),
                                    traits: traits.clone(),
                                })
                            )
                        }
                        _ => {
                            format!(
                                " -> {} ",
                                String::from(r_type.clone())
                            )
                        }
                    }
                }
                _ => {
                    format!(
                        " -> {} ",
                        String::from(r_type.clone())
                    )
                }
            }
        }
    };

    // build the binding
    format!(r#"
    {text_signature}
{doc}
    pub fn {name}({args}){return_type}{{
        {body}
    }}
        "#, 
        doc=doc,
        text_signature=text_signature,
        name=method.name,
        return_type=return_type,
        args=&args[..args.len().saturating_sub(2)],
        body=body,
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
                    && !method.name.starts_with("from") 
                    && method.visibility == Visibility::Public
                    && !method.attributes.contains(&"no_binding".to_string())
                    && !method.attributes.contains(&"manual_binding".to_string())
                {
                    println!("MISSING {}", gen_binding(&method));
                }
            }
        }
    }
}