use super::*;

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

        let this_struct = self.class.as_ref().map(|x| x.get_name()).unwrap_or("".to_string());
        
        for arg in self.iter_args() {
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
                x if *x == this_struct.as_str() => {
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
            };

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
        let mut body = format!(
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
        let return_type = match &self.return_type {
            None => {
                body = format!("{};", body);
                String::new()
            }
            Some(r_type) => {
                match r_type {
                    x if x == this_struct.as_str() 
                            || x == format!("& {}", this_struct).as_str() 
                            || x == format!("& mut {}", this_struct).as_str() => {
                        match (is_self_ref, is_self_mut) {
                            (true, true) => {
                                body = format!("{};", body);
                                "".to_string()
                            }
                            (true, false) => {
                                body = format!("{}.into()", body);

                                if r_type == format!("& {}", this_struct).as_str() {
                                    body = format!("{}.to_owned().into()", body);
                                }

                                format!(" -> {} ", this_struct)
                            }
                            _ => {
                                panic!("Not implemented yet!");
                            }
                        }
                    }
                    x if x == format!("Result<{}>", this_struct).as_str()
                        || x == format!("Result<& {}>", this_struct).as_str()
                        || x == format!("Result<& mut {}>", this_struct).as_str() =>
                    {
                        match (is_self_ref, is_self_mut) {
                            (true, true) => {
                                body = format!("pe!({})?;\nOk(())", body);
                                " -> PyResult<()> ".to_string()
                            }
                            (true, false) => {
                                body = format!("Ok(pe!({})?.into())", body);

                                if r_type == format!("Result<& {}>", this_struct).as_str() {
                                    body = format!("Ok(pe!({})?.to_owned().into())", body);
                                }

                                format!(" -> PyResult<{}> ", this_struct)
                            }
                            (false, false) => {
                                body = format!("Ok(pe!({})?.into())", body);

                                format!(" -> PyResult<{}> ", this_struct)
                            }
                            _ => {
                                panic!("Not implemented yet!");
                            }
                        }
                    }
                    x if x == format!("({name}, {name})", name=this_struct).as_str() => {
                        body = format!("let (g1, g2) = {}; (g1.into(), g2.into())", body);
                        format!(" -> ({name}, {name}) ", name=this_struct)
                    }
                    x if x == format!("Result<({name}, {name})>", name=this_struct).as_str() => {
                        body = format!("let (g1, g2) = pe!({})?; Ok((g1.into(), g2.into()))", body);
                        format!(" -> PyResult<({name}, {name})> ", name=this_struct)
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
                    x if! self.attributes.iter().any(|x| x == "no_numpy_binding")
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
                    x if !self.attributes.iter().any(|x| x == "no_numpy_binding")
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
                        body = format!("Ok(pe!({})?.into())", body);

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
                    _ => {
                        body = format!("{}.into()", body);
                        format!(" -> {} ", r_type)
                    },
                }
            }
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
                        doc = translate_doc(&self.doc),
                        text_signature = text_signature,
                        name = &self.name,
                        return_type = return_type,
                        args = &args[..args.len().saturating_sub(2)],
                        body=body,
                        is_unsafe = if self.is_unsafe { "unsafe " } else { "" },
                    )
    }
}