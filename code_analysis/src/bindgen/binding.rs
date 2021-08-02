use super::*;

#[derive(Debug, Clone)]
pub struct Binding {
    pub is_unsafe: bool,
    pub is_method: bool, 
    pub is_static: bool,
    pub doc: String,
    pub args: String,
    pub text_signature: String,
    pub name: String,
    pub body: String,
    pub return_type: String,
    pub file_path: String,
}

impl Default for Binding {
    fn default() -> Binding {
        Binding{
            is_unsafe: false,
            is_method: false,
            is_static: false,
            doc: String::new(),
            args: String::new(),
            text_signature: String::new(),
            name: String::new(),
            body: String::new(),
            return_type: String::new(),
            file_path: String::new(),
        }
    }
}

impl From<Function> for  Binding {
    fn from(func: Function) -> Binding {
        // parse the arguments
        let mut args = String::new();
        let mut args_names = String::new();
        let mut args_signatures = if func.is_static() {
            vec![]
        } else {
            vec!["$".to_string()]
        };

        for arg in func.iter_args() {
            let (arg_name, arg_call) = match &arg.arg_type {
                Type::SelfType => {
                    (
                        format!("{}self", arg.arg_modifier),
                        None,
                    )
                },
                x if x == "S" => {
                    (
                        format!("{}: String", arg.name),
                        Some(arg.name.to_string()),
                    )
                },
                x if x == "Graph" => {
                    (
                        format!("{}: EnsmallenGraph", arg.name),
                        Some(format!("{}.graph", arg.name)),
                    )
                },
                x if x == "&Graph" => {
                    (
                        format!("{}: &EnsmallenGraph", arg.name),
                        Some(format!("&{}.graph", arg.name)),
                    )
                },
                x if x == "Option<Graph>" => {
                    (
                        format!("{}: Option<EnsmallenGraph>", arg.name),
                        Some(format!("{}.map(|sg| sg.graph)", arg.name)),
                    )
                },
                x if x == "Option<&Graph>" => {
                    (
                        format!("{}: Option<&EnsmallenGraph>", arg.name),
                        Some(format!("{}.map(|sg| &sg.graph)", arg.name)),
                    )
                },
                _ => {
                    (
                        format!("{}: {}", arg.name, arg.arg_type),
                        Some(arg.name.to_string()),
                    )
                }
            };

            args.push_str(&arg_name);
            args.push_str(", ");

            if let Some(ac) = arg_call {
                args_names.push_str(&ac);
                args_names.push_str(", ");
            }

            args_signatures.push(arg.name.clone());
        }

        let text_signature = format!("#[pyo3(text_signature = \"({})\")]", args_signatures.join(", "));

        // build the call
        let mut body = format!(
            "{prefix}{name}({args_names})",
            prefix = match (func.is_method(), func.is_static()) {
                (true, true) => "Graph::".to_string(),
                (true, false) => "self.graph.".to_string(),
                (false, true) => format!("{}::", func.module_name),
                (false, false) => unreachable!("A function cannot accept self! It would be a method!"),
            },
            name = func.name,
            args_names = &args_names[..args_names.len().saturating_sub(2)],
        );

        let self_modifiers = func.get_self_modifiers();
        let is_self_ref = self_modifiers.as_ref().map_or(false, |val| val.reference);
        let is_self_mut = self_modifiers.as_ref().map_or(false, |val| val.mutable);

        // parse the return type
        let return_type = match &func.return_type {
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
                    x if x == "Result<Graph>"
                        || x == "Result<&Graph>"
                        || x == "Result<& mut Graph>" =>
                    {
                        match (is_self_ref, is_self_mut) {
                            (true, true) => {
                                body = format!("pe!({})?;\nOk(())", body);
                                " -> PyResult<()> ".to_string()
                            }
                            (true, false) => {
                                body = format!("Ok(EnsmallenGraph{{graph: pe!({})?}})", body);

                                if r_type == "Result<&Graph>" {
                                    body = format!("Ok(pe!({})?.to_owned())", body);
                                }

                                " -> PyResult<EnsmallenGraph> ".to_string()
                            }
                            (false, false) => {
                                body = format!("Ok(EnsmallenGraph{{graph: pe!({})?}})", body);

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
                    x if x == "Result<(Graph, Graph)>" => {
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
                    x if! func.attributes.iter().any(|x| x == "no_numpy_binding")
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
                    x if x == "Result<Vec<Primitive>>" => {
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
                    x if !func.attributes.iter().any(|x| x == "no_numpy_binding")
                        && x == "Result<Vec<Vec<Primitive>>>" =>
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
                    x if x == "Result<_>" => {
                        body = format!("pe!({})", body);

                        let r_type = match r_type {
                            Type::SimpleType {
                                modifiers,
                                generics,
                                traits,
                                ..
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

        Binding{
            is_unsafe: func.is_unsafe(),
            is_method: func.is_method(),
            is_static: func.is_static(),
            doc: translate_doc(&func.doc),
            name: func.name.clone(),
            args,
            text_signature,
            body,
            return_type,
            file_path: func.file_path,
        }
    }
}


impl From<Binding> for String {
    fn from(value: Binding) -> Self {
        format!(
r#"
{type_annotation}
#[automatically_generated_binding]
{text_signature}
{doc}
pub {is_unsafe}fn {name}({args}){return_type}{{
{body}
}}
"#,
            type_annotation= match (value.is_method, value.is_static) {
                (true, true) => "#[staticmethod]",
                (true, false) => "", //"#[classmethod]", for some reason if we add this crash!!
                (false, true) => "#[pyfunction]",
                (false, false) => unreachable!("it cant be both a function and take self as argument!"),
            },
            doc = value.doc,
            text_signature = value.text_signature,
            name = value.name,
            return_type = value.return_type,
            args = &value.args[..value.args.len().saturating_sub(2)],
            body =value.body,
            is_unsafe = if value.is_unsafe { "unsafe " } else { "" },
        )
    }
}
