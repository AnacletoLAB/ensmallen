use super::*;

/// Take the return type and return the body of the function and the translated return type
fn translate_return_type(
    attributes: &[Attribute],
    return_type: &Type, 
    mut body: String, 
    this_struct: &str, 
    is_static: bool,
    is_self_ref: bool, 
    is_self_mut: bool, 
    depth: usize,

) -> (String, Option<String>) {
    let (body, r_type) = match return_type {
        // handle typles recursively
        Type::TupleType(sub_types) => {
            let mut bodies = Vec::new();
            let mut return_types = Vec::new();
            
            for (i, sub_type) in sub_types.iter().enumerate() {
                let (inner_body, inner_return_type) = translate_return_type(
                    attributes, 
                    sub_type,
                    format!("subresult_{}", i), 
                    this_struct, 
                    is_static,
                    is_self_ref, 
                    is_self_mut,
                    depth + 1,
                );
                bodies.push(inner_body);
                return_types.push(inner_return_type);
            }

            let subresult_splitter = format!(
                "({})", 
                (0..sub_types.len())
                    .map(|i| format!("subresult_{}", i))
                    .collect::<Vec<_>>().join(", ")
                );
            
            let mut final_body = format!(
                "let {} = {};\n({})", 
                subresult_splitter,
                body,
                bodies.join(", ")
            );

            if depth != 0 {
                final_body = format!("{{{}}}", final_body);
            }

            let final_type = format!(
                "({})", 
                return_types.into_iter()
                    .map(|x| x.unwrap_or("()".to_string()))
                    .collect::<Vec<_>>().join(", ")
            );

            (
                final_body,
                Some(final_type),
            )
        }

        // handle null type
        Type::None => (body, Some(return_type.to_string())),

        // handle a method that return the current struct
        x if !is_static && x.get_name() == Some(this_struct.to_string()) => {
            match (is_self_ref, is_self_mut) {
                (true, true) => {
                    if body.ends_with(".into()") {
                        body = body.strip_suffix(".into()").unwrap().to_string();
                    }
                    if depth == 0 {
                        (format!("{};()", body), None)
                    } else {
                        (format!("{{{};()}}", body), None)
                    }
                },
                (true, false) => {
                    body = format!("{}.into()", body);

                    if return_type == format!("& {}", this_struct).as_str() {
                        body = format!("{}.to_owned().into()", body);
                    }

                    (body, Some(this_struct.to_string()))
                }
                _ => {
                    panic!("Not implemented yet!");
                }
            }
        }

        // handle the Result type
        x if x == "Result<_>" => {
            let needs_into = match &return_type[0] {
                x if x == "()" => false,
                _ => true,
            };

            let mut sub_body = format!(
                "pe!({})?{}", 
                body,
                if needs_into {
                    ".into()"
                } else {
                    ""
                }
            );

            if depth != 0 {
                sub_body = format!("{{{}}}", sub_body);
            }

            let (inner_body, inner_type) = translate_return_type(
                attributes, 
                &return_type[0], 
                sub_body, 
                this_struct, 
                is_static,
                is_self_ref, 
                is_self_mut,
                depth + 1,
            );
            
            (
                if depth == 0 {
                    format!("Ok({})", inner_body)
                } else {
                    format!("Ok({{{}}})", inner_body)
                }, 
                Some(format!("PyResult<{}>", inner_type.unwrap_or("()".into())))
            )
        }

        // handle 1d numpy arrays
        x if x == "Vec<Primitive>" 
            && !attributes.iter().any(|x| x == "no_numpy_binding") 
            => {
            let inner_type = return_type[0].to_string();

            if body.ends_with(".into()") {
                body = body.strip_suffix(".into()").unwrap().to_string();
            }

            let mut body = format!(
                concat!(
                    "let gil = pyo3::Python::acquire_gil();\n",
                    "to_ndarray_1d!(gil, {body}, {inner_type})"
                ),
                body = body,
                inner_type = inner_type,
            );
            
            if depth != 0 {
                body = format!("{{{}}}", body);
            }

            (
                body,
                Some(
                    format!("Py<PyArray1<{}>>", inner_type)
                )
            )
        }

        // handle other vec with maybe complex types
        x if x == "Vec<_>" => {
            // TODO! make this recursive??
            let mut res_body = format!(
                "{}.into_iter().map(|x| x.into()).collect::<Vec<_>>()", 
                body.strip_suffix(".into()").unwrap_or(&body)
            );

            (
                res_body,
                Some(
                    format!("Vec<{}>", &return_type[0])
                )
            )
        }

        // handle 2d numpy arrays
        x if x == "Vec<Vec<Primitive>>" 
            && !attributes.iter().any(|x| x == "no_numpy_binding") 
            => {
            let inner_type = return_type[0][0].to_string();

            if body.ends_with(".into()") {
                body = body.strip_suffix(".into()").unwrap().to_string();
            }

            let mut body = format!(
                concat!(
                    "let gil = pyo3::Python::acquire_gil();\n",
                    "to_ndarray_2d!(gil, {body}, {inner_type})"
                ),
                body = body,
                inner_type = inner_type,
            );

            if depth != 0 {
                body = format!("{{{}}}", body);
            }

            (
                body,
                Some(
                    format!("Py<PyArray2<{}>>", inner_type)
                )
            )
        }

        // we don't have special rules so we can just use the default case
        _ => (format!("{}.into()", body), Some(return_type.to_string())),
    };

    (body.replace(".into().into()", ".into()"), r_type)
}

/// Given an argument, returns it's function definition
/// and how it should be passed to the calling function
fn translate_arg(arg: &Arg, this_struct: &str) -> (String, Option<String>) {
    match &arg.arg_type {
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
        x if x == "str" => {
            (
                format!("{}: String", arg.name),
                Some(format!("&{}", arg.name)),
            )
        },
        Type::SliceType(inner_type) => {
            (
                format!("{}: Vec<{}>", arg.name, inner_type),
                Some(format!("&{}", arg.name)),
            )
        },
        x if *x == this_struct => {
            (
                format!("{}: {}", arg.name, this_struct),
                Some(format!("{}.inner", arg.name)),
            )
        },
        x if *x == format!("& {}", this_struct).as_str() => {
            (
                format!("{}: &{}", arg.name, this_struct),
                Some(format!("&{}.inner", arg.name)),
            )
        },
        x if x == format!("Option<{}>", this_struct).as_str() => {
            (
                format!("{}: Option<{}>", arg.name, this_struct),
                Some(format!("{}.map(|sg| sg.inner)", arg.name)),
            )
        },
        x if x == format!("Option<& {}>", this_struct).as_str() => {
            (
                format!("{}: Option<& {}>", arg.name, this_struct),
                Some(format!("{}.map(|sg| &sg.inner)", arg.name)),
            )
        },
        _ => {
            (
                format!("{}: {}", arg.name, arg.arg_type),
                Some(format!("{}.into()", arg.name.to_string())),
            )
        }
    }
}

impl GenBinding for Function {

    fn gen_python_binding(self: &Self) -> String {
        // parse the arguments
        let mut args = String::new();
        let mut args_names = String::new();
        let mut args_signatures = if self.is_static() {
            vec![]
        } else {
            vec!["$self".to_string()]
        };

        let this_struct = self.class.as_ref().map(|x| x.get_name().unwrap()).unwrap_or("".to_string());
        
        for arg in self.iter_args() {
            let (arg_name, arg_call) = translate_arg(arg, &this_struct);

            args.push_str(&arg_name);
            args.push_str(", ");

            if let Some(ac) = arg_call {
                args_names.push_str(&ac);
                args_names.push_str(", ");
            }
            if !matches!(&arg.arg_type, Type::SelfType) {
                args_signatures.push(arg.name.clone());
            }
        }

        let text_signature = format!("#[text_signature = \"({})\"]", args_signatures.join(", "));

        // build the call
        let body = format!(
            "{prefix}{name}({args_names})",
            prefix = match (self.is_method(), self.is_static()) {
                (true, true) => format!("graph::{}::", this_struct),
                (true, false) => "self.inner.".to_string(),
                (false, true) => "graph::".to_string(),
                (false, false) => unreachable!("A selftion cannot accept self! It would be a method!"),
            },
            name = self.name,
            args_names = &args_names[..args_names.len().saturating_sub(2)],
        );

        let self_modifiers = self.get_self_modifiers();
        let is_self_ref = self_modifiers.as_ref().map_or(false, |val| val.reference);
        let is_self_mut = self_modifiers.as_ref().map_or(false, |val| val.mutable);

        // parse the return type
        let (body, return_type) = match &self.return_type {
                None => (format!("{};", body), None),
                Some(r_type) => translate_return_type(&self.attributes, r_type, body, &this_struct, self.is_static(), is_self_ref, is_self_mut,0),
            };

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
                        type_annotation= match (self.is_method(), self.is_static()) {
                            (true, true) => "#[staticmethod]",
                            (true, false) => "", //"#[classmethod]", for some reason if we add this crash!!
                            (false, true) => "#[pyfunction]",
                            (false, false) => unreachable!("it cant be both a function and take self as argument!"),
                        },
                        // TODO!: FIX THIS SHIT to allows proper translation of user types
                        doc = translate_doc(&self.doc, &vec![]),
                        text_signature = text_signature,
                        name = &self.name,
                        return_type = return_type.map(|x| format!("-> {}", x)).unwrap_or("".into()),
                        args = &args[..args.len().saturating_sub(2)],
                        body=body,
                        is_unsafe = if self.is_unsafe { "unsafe " } else { "" },
                    )
    }
}