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

        let this_struct = self
            .class
            .as_ref()
            .map(|x| x.get_name().unwrap())
            .unwrap_or("".to_string());

        let mut handle_walk_parameters = false;
        let mut is_kwarg_only = false;

        for arg in self.iter_args() {
            // bad hardocded stuff but fuck it it's 2am
            if &arg.arg_type.to_string() == "&WalksParameters" {
                handle_walk_parameters = true;
                args_names.push_str(&format!(
                    r#"&{{
let py = pyo3::Python::acquire_gil();
let kwargs = normalize_kwargs!(py_kwargs, py.python());
pe!(validate_kwargs(
    kwargs,
    build_walk_parameters_list(&[]).as_slice()
))?;
build_walk_parameters(kwargs)?
}}"#,
                ));
                args_names.push_str(", ");
                continue;
            }

            let (mut arg_name, mut arg_call) = arg.to_python_bindings_arg(&this_struct);

            if let Some((_, tipe)) = arg_name.split_once(':') {
                if tipe.trim().starts_with("Option") {
                    is_kwarg_only = true;
                } else {
                    if is_kwarg_only {
                        panic!("Argument '{}' of function '{}'  of class '{:?}' from file '{}' is not kwargs compatible",
                            arg_name,
                            self.name,
                            self.class.as_ref().map(|x| x.get_name()),
                            self.file_path,
                        );
                    }
                }
            }

            // bad hack
            if arg_name.contains("Option<&Vec<NodeT>>") {
                arg_name = arg_name.replace("Option<&Vec<NodeT>>", "Option<Vec<NodeT>>");
                arg_call = Some(format!("{}.as_ref()", arg.name));
            }

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

        if handle_walk_parameters {
            args_signatures.extend(vec![
                "*".into(),
                "return_weight".into(),
                "explore_weight".into(),
                "change_edge_type_weight".into(),
                "change_node_type_weight".into(),
                "max_neighbours".into(),
                "random_state".into(),
                "iterations".into(),
                "dense_node_mapping".into(),
                "normalize_by_degree".into(),
                "walk_length".into(),
            ]);

            args.push_str("py_kwargs: Option<&PyDict>, ");
        }

        let text_signature = format!(
            "#[pyo3(text_signature = \"({})\")]",
            args_signatures.join(", ")
        );

        // build the call
        let body = format!(
            "{prefix}{name}({args_names})",
            prefix = match (self.is_method(), self.is_static()) {
                (true, true) => format!("graph::{}::", this_struct),
                (true, false) => "self.inner.".to_string(),
                (false, true) => "graph::".to_string(),
                (false, false) =>
                    unreachable!("A selftion cannot accept self! It would be a method!"),
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
            Some(r_type) => r_type.to_python_bindings_return_type(
                &self.attributes,
                body,
                &this_struct,
                self.is_static(),
                is_self_ref,
                is_self_mut,
            ),
        };

        format!(
            r#"
            {other_annotations}
            {type_annotation}
            #[automatically_generated_binding]
            {text_signature}
            {doc}
            pub {is_unsafe}fn {name}({args}){return_type}{{
            {body}
            }}
            "#,
            other_annotations = if handle_walk_parameters {
                "#[args(py_kwargs = \"**\")]"
            } else {
                ""
            },
            type_annotation = match (self.is_method(), self.is_static()) {
                (true, true) => "#[staticmethod]",
                (true, false) => "", //"#[classmethod]", for some reason if we add this crash!!
                (false, true) => "#[pyfunction]",
                (false, false) =>
                    unreachable!("it cant be both a function and take self as argument!"),
            },
            // TODO!: FIX THIS SHIT to allows proper translation of user types
            doc = translate_doc(&self.doc, &vec![]),
            text_signature = text_signature,
            name = &self.name,
            return_type = return_type
                .map(|x| format!("-> {}", x))
                .unwrap_or("".into()),
            args = &args[..args.len().saturating_sub(2)],
            body = body,
            is_unsafe = if self.is_unsafe { "unsafe " } else { "" },
        )
    }
}
