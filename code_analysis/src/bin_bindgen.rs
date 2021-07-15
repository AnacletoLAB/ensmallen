use libcodeanalysis::*;
use rust_parser::*;
use std::collections::HashSet;
use std::fs;
use std::fs::read_dir;

use graph::okapi_bm25_tfidf;

fn get_binding_names() -> HashSet<String> {
    let bindings_files: Vec<String> = read_dir("../bindings/python/src")
        .unwrap()
        .map(|path| {
            path.unwrap()
                .path()
                .into_os_string()
                .into_string()
                .unwrap()
                .to_string()
        })
        .filter(|path| !skip_file(&path))
        .collect();
    let mut bindings_modules = Vec::new();
    let mut method_names = HashSet::new();
    for path in bindings_files {
        eprintln!("{:?}", path);
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
    translate_type(Type::parse_lossy_string(value))
}

fn translate_type(value: Type) -> String {
    match value.clone() {
        Type::TupleType(vals) => {
            format!(
                "Tuple[{}]",
                vals.iter()
                    .map(|t| translate_type(t.clone()))
                    .collect::<Vec<String>>()
                    .join(", ")
            )
        }
        Type::SliceType(inner_type) => {
            format!("List[{}]", translate_type(*inner_type))
        }
        Type::SimpleType {
            name,
            modifiers,
            generics,
            traits,
        } => match name.as_str() {
            "Graph" => "EnsmallenGraph".to_string(),
            "NodeT" => "int".to_string(),
            "usize" => "int".to_string(),
            "EdgeT" => "int".to_string(),
            "WeightT" => "float".to_string(),
            "u64" => "int".to_string(),
            "f64" => "float".to_string(),
            "f32" => "float".to_string(),
            "bool" => "bool".to_string(),
            "str" => "str".to_string(),
            "String" => "str".to_string(),
            "NodeTypeT" => "int".to_string(),
            "EdgeTypeT" => "int".to_string(),
            "S" => "str".to_string(),
            "RoaringBitmap" => "List[int]".to_string(),
            "HashSet" => {
                let mut result = "Set[".to_string();
                for value in generics.0 {
                    match value {
                        GenericValue::Type(t) => result.extend(translate_type(t).chars()),
                        _ => panic!("Cannot traduce to python the generic value {:?}", value),
                    }
                }
                result.push(']');
                result
            }
            "HashMap" => {
                let mut result = "Dict[".to_string();
                let mut vals = Vec::new();
                for value in generics.0 {
                    match value {
                        GenericValue::Type(t) => {
                            vals.push(translate_type(t));
                        }
                        _ => panic!("Cannot traduce to python the generic value {:?}", value),
                    }
                }
                result.extend(vals.join(", ").chars());
                result.push(']');
                result
            }
            "Option" => {
                let mut result = "Optional[".to_string();
                for value in generics.0 {
                    match value {
                        GenericValue::Type(t) => result.extend(translate_type(t).chars()),
                        _ => panic!("Cannot traduce to python the generic value {:?}", value),
                    }
                }
                result.push(']');
                result
            }
            "Vec" => {
                let mut result = "List[".to_string();
                for value in generics.0 {
                    match value {
                        GenericValue::Type(t) => result.extend(translate_type(t).chars()),
                        _ => panic!("Cannot traduce to python the generic value {:?}", value),
                    }
                }

                result.push(']');
                result
            }
            _ => {
                panic!("Cannot translate '{:?}' as a python unknown type", value);
            }
        },
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
            }
            DocSection::Arguments {
                prologue,
                arguments,
                epilogue,
            } => {
                result.extend("\n\nParameters\n----------\n".chars());

                //args_sec.extend(prologue.chars());

                for argument in arguments {
                    match argument {
                        Argument::Parsable(DocArg {
                            name,
                            arg_type,
                            description,
                        }) => result.extend(
                            format!(
                                "{name}: {arg_type},\n    {description}\n",
                                name = name,
                                arg_type = translate_type_str(arg_type),
                                description = description,
                            )
                            .chars(),
                        ),
                        Argument::NotParsable(_) => {}
                    }
                }

                //args_sec.extend(epilogue.chars());
            }
            DocSection::Raises {
                prologue,
                exceptions,
                epilogue,
            } => {
                result.extend("\n\nRaises\n-------\n".chars());

                for excp in exceptions {
                    result.extend(format!("ValueError\n    {}\n", excp).chars());
                }
            }
            DocSection::Unsafe { text } => {
                result.extend("\n\nSafety\n------\n".chars());
                result.extend(text.chars());
            }
            _ => {}
        }
    }

    result
        .split("\n")
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
        match &arg.arg_type {
            Type::SelfType => {
                args.extend(format!("{}self, ", arg.arg_modifier).chars());

                is_self_ref = arg.arg_modifier.reference;
                is_self_mut = arg.arg_modifier.mutable;
            }
            x if x == "S" => {
                args.extend(format!("{}: String, ", arg.name,).chars());
                args_names.extend(format!("{}, ", arg.name).chars());
                args_signatures.push(arg.name.clone());
            }
            x if x == "Graph" => {
                args.extend(format!("{}: EnsmallenGraph, ", arg.name,).chars());
                args_names.extend(format!("{}.graph, ", arg.name).chars());
                args_signatures.push(arg.name.clone());
            }
            x if x == "&Graph" => {
                args.extend(format!("{}: &EnsmallenGraph, ", arg.name,).chars());
                args_names.extend(format!("&{}.graph, ", arg.name).chars());
                args_signatures.push(arg.name.clone());
            }
            x if x == "Option<Graph>" => {
                args.extend(format!("{}: Option<EnsmallenGraph>, ", arg.name,).chars());
                args_names.extend(format!("{}.map(|sg| sg.graph), ", arg.name).chars());
                args_signatures.push(arg.name.clone());
            }
            x if x == "Option<&Graph>" => {
                args.extend(format!("{}: Option<&EnsmallenGraph>, ", arg.name,).chars());
                args_names.extend(format!("{}.map(|sg| &sg.graph), ", arg.name).chars());
                args_signatures.push(arg.name.clone());
            }
            _ => {
                args.extend(format!("{}: {}, ", arg.name, arg.arg_type).chars());
                args_names.extend(format!("{}, ", arg.name).chars());
                args_signatures.push(arg.name.clone());
            }
        }
    }

    let text_signature = format!("#[text_signature = \"({})\"]", args_signatures.join(", "));

    // build the call
    let mut body = format!(
        "self.graph.{name}({args_names})",
        name = method.name,
        args_names = &args_names[..args_names.len().saturating_sub(2)],
    );

    // parse the return type
    let return_type = match &method.return_type {
        None => {
            body = format!("{};", body);
            String::new()
        }
        Some(r_type) => {
            match r_type {
                x if x == "Graph" || x == "&Graph" || x == "&mut Graph" => {
                    match (is_self_ref, is_self_mut) {
                        (true, true) => {
                            body = format!("{};", body);
                            "".to_string()
                        }
                        (true, false) => {
                            body = format!("EnsmallenGraph{{graph: {}}}", body);

                            if r_type == "&Graph" {
                                body = format!("{}.to_owned()", body);
                            }

                            " -> EnsmallenGraph ".to_string()
                        }
                        _ => {
                            panic!("Not implemented yet!");
                        }
                    }
                }
                x if x == "Result<Graph, _>"
                    || x == "Result<&Graph, _>"
                    || x == "Result<& mut Graph, _>" =>
                {
                    match (is_self_ref, is_self_mut) {
                        (true, true) => {
                            body = format!("pe!({})?;\nOk(())", body);
                            " -> PyResult<()> ".to_string()
                        }
                        (true, false) => {
                            body = format!("Ok(EnsmallenGraph{{graph: pe!({})?}})", body);

                            if r_type == "Result<&Graph, _>" {
                                body = format!("Ok(pe!({})?.to_owned())", body);
                            }

                            " -> PyResult<EnsmallenGraph> ".to_string()
                        }
                        _ => {
                            panic!("Not implemented yet!");
                        }
                    }
                }
                x if x == "(Graph, Graph)" => {
                    body = format!("let (g1, g2) = {}; (EnsmallenGraph{{graph: g1}}, EnsmallenGraph{{graph: g2}})", body);
                    " -> (EnsmallenGraph, EnsmallenGraph) ".to_string()
                }
                x if x == "Result<(Graph, Graph), _>" => {
                    body = format!("let (g1, g2) = pe!({})?; Ok((EnsmallenGraph{{graph: g1}}, EnsmallenGraph{{graph: g2}}))", body);
                    " -> PyResult<(EnsmallenGraph, EnsmallenGraph)> ".to_string()
                }
                // TODO!: add also recursive numpy conversion for tuples and such
                x if x == "Vec<Primitive>" => {
                    let inner_type = r_type[0].to_string();
                    body = format!(
                        concat!(
                            "let gil = pyo3::Python::acquire_gil();\n",
                            "to_ndarray_1d!(gil, {body}, {inner_type})"
                        ),
                        body = body,
                        inner_type = inner_type,
                    );
                    format!(
                        " -> {} ",
                        Type::parse_lossy_string(format!("Py<PyArray1<{}>>", inner_type))
                    )
                }
                x if! method.attributes.iter().any(|x| x == "no_numpy_binding")
                    && x == "Vec<Vec<Primitive>>" =>
                {
                    let inner_type = r_type[0][0].to_string();
                    body = format!(
                        concat!(
                            "let gil = pyo3::Python::acquire_gil();\n",
                            "to_ndarray_2d!(gil, {body}, {inner_type})"
                        ),
                        body = body,
                        inner_type = inner_type,
                    );
                    format!(
                        " -> {} ",
                        Type::parse_lossy_string(format!("Py<PyArray2<{}>>", inner_type))
                    )
                }
                x if x == "Result<Vec<Primitive>, _>" => {
                    let inner_type = r_type[0][0].to_string();
                    body = format!(
                        concat!(
                            "let gil = pyo3::Python::acquire_gil();\n",
                            "Ok(to_ndarray_1d!(gil, pe!({body})?, {inner_type}))"
                        ),
                        body = body,
                        inner_type = inner_type,
                    );
                    format!(
                        " -> {} ",
                        Type::parse_lossy_string(format!("PyResult<Py<PyArray1<{}>>>", inner_type))
                    )
                }
                x if !method.attributes.iter().any(|x| x == "no_numpy_binding")
                    && x == "Result<Vec<Vec<Primitive>>, _>" =>
                {
                    let inner_type = r_type[0][0][0].to_string();
                    body = format!(
                        concat!(
                            "let gil = pyo3::Python::acquire_gil();\n",
                            "Ok(to_ndarray_2d!(gil, pe!({body})?, {inner_type}))"
                        ),
                        body = body,
                        inner_type = inner_type,
                    );
                    format!(
                        " -> {} ",
                        Type::parse_lossy_string(format!("PyResult<Py<PyArray2<{}>>>", inner_type))
                    )
                }
                x if x == "Result<_, _>" => {
                    body = format!("pe!({})", body);

                    let r_type = match r_type {
                        Type::SimpleType {
                            name,
                            modifiers,
                            generics,
                            traits,
                        } => Type::SimpleType {
                            name: "PyResult".to_string(),
                            modifiers: modifiers.clone(),
                            generics: Generics(vec![generics.0[0].clone()]),
                            traits: traits.clone(),
                        },
                        _ => unreachable!(),
                    };

                    format!(" -> {} ", r_type)
                }
                _ => format!(" -> {} ", r_type),
            }
        }
    };

    // build the binding
    format!(
        r#"
    #[automatically_generated_binding]
    {text_signature}
{doc}
    pub {is_unsafe}fn {name}({args}){return_type}{{
        {body}
    }}
        "#,
        doc = doc,
        text_signature = text_signature,
        name = method.name,
        return_type = return_type,
        args = &args[..args.len().saturating_sub(2)],
        body = body,
        is_unsafe = if method.is_unsafe { "unsafe " } else { "" },
    )
}

fn main() {
    let mut bindings = vec![];

    let modules = get_library_sources();
    for module in modules {
        for mut imp in module.impls {
            if imp.struct_name != "Graph" {
                continue;
            }

            // Parse methods generation macros and expand them to actual
            // methods
            imp.methods.extend(parse_macros(imp.macro_calls));

            for method in imp.methods {
                if
                // !method_names.contains(&method.name) &&
                !method.name.starts_with("iter")
                    && !method.name.starts_with("par_iter")
                    && !method.name.starts_with("from")
                    && method.visibility == Visibility::Public
                    && !method.attributes.iter().any(|x| x == "no_binding")
                    && !method.attributes.iter().any(|x| x == "manual_binding")
                {
                    let binding = gen_binding(&method);
                    println!("{}", binding);
                    bindings.push(binding);
                }
            }
        }
    }

    let file_content = format!(
        r#"use super::*;

#[pymethods]
impl EnsmallenGraph {{
{}
}}"#,
        bindings.join("")
    );

    fs::write(
        "../bindings/python/src/auto_generated_bindings.rs",
        file_content,
    )
    .expect("Cannot weite the automatically generated bindings file");

    let method_names = get_binding_names();

    let documents = method_names
        .iter()
        .map(|x| split_words(x))
        .collect::<Vec<Vec<String>>>();
    let vals = documents
        .iter()
        .map(|x| x.iter().map(|y| y.as_str()).collect::<Vec<&str>>())
        .collect::<Vec<Vec<&str>>>();

    let tfidf = okapi_bm25_tfidf(&vals[..], None, None, None, None).unwrap();

    let mut terms = HashSet::new();
    documents.iter().for_each(|document| {
        document.iter().for_each(|term| {
            terms.insert(term);
        });
    });

    let method_names_list = format!(
        r#"pub const METHODS_NAMES: &'static [&'static str] = &[
{}
];

pub const TERMS: &'static [&'static str] = &[
{}
];

pub const TFIDF_FREQUENCIES: &'static [&'static [(&'static str, f64)]] = &[
{}
];
"#,
        method_names
            .iter()
            .map(|x| format!("    \"{}\",", x))
            .collect::<Vec<String>>()
            .join("\n"),
        terms
            .iter()
            .map(|x| format!("    \"{}\",", x))
            .collect::<Vec<String>>()
            .join("\n"),
        tfidf
            .iter()
            .map(|vals| format!("&{:?},", vals.iter().collect::<Vec<(&String, &f64)>>()))
            .collect::<Vec<String>>()
            .join("\n"),
    );
    fs::write(
        "../bindings/python/src/method_names_list.rs",
        method_names_list,
    )
    .expect("Cannot write the method names list file");

    assert!(
        std::process::Command::new("cargo")
            .args(&["fmt"])
            .current_dir("../bindings/python")
            .status()
            .expect("Could not run format on the python bindings")
            .success(),
        "The cargo format failed and returned non-zero exit status"
    );
}

fn split_words(method_name: &str) -> Vec<String> {
    let mut result: Vec<String> = Vec::new();
    for word in method_name.split("_") {
        match word {
            "type" | "types" | "id" | "ids" | "name" | "names" => match result.last_mut() {
                Some(last) => {
                    last.push('_');
                    last.extend(word.chars());
                }
                None => {
                    result.push(word.to_string());
                }
            },
            _ => {
                result.push(word.to_string());
            }
        };
    }

    result.into_iter().filter(|x| !x.is_empty()).collect()
}


/// Expand the macro calls to the generated methods
///
/// This curently handle the following macros:
/// * `cached_property`
fn parse_macros(macro_calls: Vec<MacroCall>) -> Vec<Function> {
    let mut result = Vec::new();
    for macro_call in macro_calls {
        match macro_call.name.as_str() {
            "cached_property" => {
                let mut item = Function::default();

                let mut data = macro_call.content.as_bytes();
                item.name = parse!(data, Identifier).into();

                // skip the comma
                data = skip_whitespace(&data[1..]);

                item.return_type = Some(parse!(data, Type));
                // skip the comma
                data = skip_whitespace(&data[1..]);

                let _caching_method = parse!(data, Identifier);
                // skip the comma
                data = skip_whitespace(&data[1..]);

                let _caching_attribute = parse!(data, Identifier);
                // skip the comma
                data = skip_whitespace(&data[1..]);

                // parse the documentations
                let doc_lines = String::from_utf8(data.to_vec()).unwrap();
                let mut doc = String::new();
                for doc_line in doc_lines.split("\n") {
                    // remove extra white space
                    match doc_line.trim().strip_prefix("///") {
                        None => {
                            // maybe panic?
                        }
                        Some(doc_line) => {
                            doc.push_str(doc_line.trim());
                            doc.push('\n');
                        }
                    }
                }
                item.doc = doc;
                item.visibility = Visibility::Public;
                result.push(item);
            }
            // Macro not handled so it's ignored`
            _ => {}
        };
    }
    result
}