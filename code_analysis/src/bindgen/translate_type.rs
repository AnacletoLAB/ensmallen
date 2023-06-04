use super::*;
use rust_parser::CmpWithoutModifiers;

pub fn translate_type_str(value: String, user_defined_types: &[&str]) -> String {
    Type::parse_lossy_string(value).to_python_type(user_defined_types)
}

pub trait TranslateArg {
    fn to_python_bindings_arg(&self, this_struct: &str) -> (String, Option<String>);
}

impl TranslateArg for Arg {
    fn to_python_bindings_arg(&self, this_struct: &str) -> (String, Option<String>) {
        match &self.arg_type {
            Type::SelfType => (format!("{}self", self.arg_modifier), None),
            x if x == "S" => (
                format!("{}: String", self.name),
                Some(self.name.to_string()),
            ),
            x if x == "str" => (
                format!("{}: String", self.name),
                Some(format!("&{}", self.name)),
            ),
            Type::SliceType(inner_type) => (
                format!("{}: Vec<{}>", self.name, inner_type),
                Some(format!("&{}", self.name)),
            ),
            x if *x == this_struct => (
                format!("{}: {}", self.name, this_struct),
                Some(format!("{}.inner", self.name)),
            ),
            x if *x == format!("& {}", this_struct).as_str() => (
                format!("{}: &{}", self.name, this_struct),
                Some(format!("&{}.inner", self.name)),
            ),
            x if x == format!("Option<{}>", this_struct).as_str() => (
                format!("{}: Option<{}>", self.name, this_struct),
                Some(format!("{}.map(|sg| sg.inner)", self.name)),
            ),
            x if x == format!("Option<& {}>", this_struct).as_str() => (
                format!("{}: Option<& {}>", self.name, this_struct),
                Some(format!("{}.map(|sg| &sg.inner)", self.name)),
            ),
            t => {
                let (t, call) = t.to_python_bindings_arg(&self.name);
                (format!("{}: {}", self.name, t), Some(call))
            }
        }
    }
}

pub trait TranslateType {
    /// Rust Type -> Python doc Type
    fn to_python_type(&self, user_defined_types: &[&str]) -> String;

    /// Ryst type -> Pyo3 bindings usage and arg def
    fn to_python_bindings_arg(&self, call: &str) -> (String, String);

    /// function to call
    /// Ryst type -> Pyo3 bindings call and return type (if present)
    fn to_python_bindings_return_type(
        &self,
        attributes: &[Attribute],
        body: String,
        this_struct: &str,
        is_static: bool,
        is_self_ref: bool,
        is_self_mut: bool,
    ) -> (String, Option<String>) {
        self.to_python_bindings_return_type_inner(
            attributes,
            body,
            this_struct,
            is_static,
            is_self_ref,
            is_self_mut,
            0,
        )
    }

    /// inner recursive call, do not use directly
    fn to_python_bindings_return_type_inner(
        &self,
        attributes: &[Attribute],
        body: String,
        this_struct: &str,
        is_static: bool,
        is_self_ref: bool,
        is_self_mut: bool,
        depth: usize,
    ) -> (String, Option<String>);
}

impl TranslateType for Type {
    fn to_python_bindings_arg(&self, call: &str) -> (String, String) {
        match self {
            Type::None => ("()".to_string(), "()".to_string()),
            Type::SliceType(sub_type) => {
                let (t, sub_call) = sub_type.to_python_bindings_arg("x");
                (
                    format!("Vec<{}>", t),
                    format!(
                        "{}.as_slice()",
                        call, //sub_call
                    ),
                )
            }
            Type::TupleType(sub_types) => {
                let mut result_type = String::new();
                let mut result_call = format!("let temp = {};(", call);

                for (i, sub_type) in sub_types.iter().enumerate() {
                    let (sub_type, sub_call) =
                        sub_type.to_python_bindings_arg(&format!("temp.{}", i));

                    result_type.push_str(&sub_type);
                    result_call.push_str(&sub_call);

                    result_type.push_str(", ");
                    result_call.push_str(", ");
                }

                (
                    format!("({})", result_type),
                    format!("{{{})}}", result_call),
                )
            }
            x if x.cmp_str_without_modifiers("Option<&[_]>") => (
                format!("Option<Vec<{}>>", x[0][0].to_string()),
                format!("{}.as_ref().map(|x| x.as_slice())", call),
            ),
            x @ Type::SimpleType { name, generics, .. } => {
                if name == "Vec" {
                    if self[0] == "& Primitive" {
                        return (self.to_string(), format!("{}", call,));
                    } else {
                        let (sub_type_str, sub_call) = self[0].to_python_bindings_arg("x");
                        return (format!("Vec<{}>", sub_type_str), format!("{}", call,));
                    }
                }
                if name == "HashSet" {
                    let (sub_type_str, sub_call) = self[0].to_python_bindings_arg("x");
                    return (
                        format!("HashSet<{}>", sub_type_str),
                        format!(
                            "{}.into_iter().map(|x| {{{}}}).collect::<HashSet<_>>()",
                            call, sub_call
                        ),
                    );
                }
                if name == "Option" {
                    let (sub_type_str, sub_call) = self[0].to_python_bindings_arg("x");
                    return (format!("Option<{}>", sub_type_str), format!("{}", call));
                }
                if name == "str" {
                    return ("&str".to_string(), call.to_string());
                }
                if self == "Primitive" {
                    return (self.to_string(), format!("{}.clone()", call));
                }

                let mut result = name.clone();
                if generics.0.len() == 1 {
                    result.push('<');
                    let sub_call = match &generics.0[0] {
                        GenericValue::Type {
                            sub_type,
                            modifiers,
                        } => {
                            let (sub_type_str, sub_call) = sub_type.to_python_bindings_arg(call);
                            result.push_str(&sub_type_str);
                            sub_call
                        }
                        gen => unimplemented!("{:?}", gen),
                    };
                    result.push('>');
                    (result, format!("{}.into()", sub_call))
                } else {
                    (self.to_string(), format!("{}.into()", call))
                }
            }
            t => unimplemented!("{:?}", t),
        }
    }

    fn to_python_type(&self, user_defined_types: &[&str]) -> String {
        // If the type was defined by the user it's fine
        let value_str = self.to_string();
        if user_defined_types.contains(&value_str.as_str()) {
            return self.to_string();
        }

        match self.clone() {
            Type::None => "None".into(),
            Type::TupleType(vals) => {
                format!(
                    "Tuple[{}]",
                    vals.iter()
                        .map(|t| t.to_python_type(user_defined_types))
                        .collect::<Vec<String>>()
                        .join(", ")
                )
            }
            Type::SliceType(inner_type) => {
                format!("List[{}]", inner_type.to_python_type(user_defined_types))
            }
            Type::SimpleType { name, generics, .. } => match name.as_str() {
                // BAD HACKS TODO! Figure out why it happens in pyigen
                "" => "".into(),
                "Graph" => "Graph".into(),

                "NodeT" | "NodeTypeT" | "EdgeT" | "EdgeTypeT" => "int".into(),
                "usize" | "u64" | "u32" | "u16" | "u8" => "int".into(),
                "isize" | "i64" | "i32" | "i16" | "i8" => "int".into(),
                "WeightT" | "f64" | "f32" => "float".into(),
                "bool" => "bool".into(),
                "char" | "str" | "S" | "String" => "str".into(),
                "RoaringBitmap" => "List[int]".into(),
                "HashSet" => {
                    let mut result = "Set[".to_string();
                    for value in generics.0 {
                        match value {
                            GenericValue::Type {
                                sub_type: t,
                                modifiers: m,
                            } => result.push_str(&format!(
                                "{}{}",
                                m,
                                t.to_python_type(user_defined_types)
                            )),
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
                            GenericValue::Type {
                                sub_type: t,
                                modifiers: m,
                            } => {
                                vals.push(format!("{}{}", m, t.to_python_type(user_defined_types)))
                            }
                            _ => panic!("Cannot traduce to python the generic value {:?}", value),
                        }
                    }
                    result.push_str(&vals.join(", "));
                    result.push(']');
                    result
                }
                "Option" => {
                    let mut result = "Optional[".to_string();
                    for value in generics.0 {
                        match value {
                            GenericValue::Type {
                                sub_type: t,
                                modifiers: m,
                            } => result.push_str(&format!(
                                "{}{}",
                                m,
                                t.to_python_type(user_defined_types)
                            )),
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
                            GenericValue::Type {
                                sub_type: t,
                                modifiers: m,
                            } => result.push_str(&format!(
                                "{}{}",
                                m,
                                t.to_python_type(user_defined_types)
                            )),
                            _ => panic!("Cannot traduce to python the generic value {:?}", value),
                        }
                    }

                    result.push(']');
                    result
                }
                "Result" => {
                    let mut result = String::new();
                    for value in generics.0 {
                        match value {
                            GenericValue::Type {
                                sub_type: t,
                                modifiers: m,
                            } => result.push_str(&format!(
                                "{}{}",
                                m,
                                t.to_python_type(user_defined_types)
                            )),
                            _ => panic!("Cannot traduce to python the generic value {:?}", value),
                        }
                    }
                    result
                }
                "PyResult" => {
                    let mut result = String::new();
                    for value in generics.0 {
                        match value {
                            GenericValue::Type {
                                sub_type: t,
                                modifiers: m,
                            } => result.push_str(&format!(
                                "{}{}",
                                m,
                                t.to_python_type(user_defined_types)
                            )),
                            _ => panic!("Cannot traduce to python the generic value {:?}", value),
                        }
                    }
                    result
                }
                "PyDict" => {
                    let mut result = "Dict[".to_string();
                    for value in generics.0 {
                        match value {
                            GenericValue::Type {
                                sub_type: t,
                                modifiers: m,
                            } => result.push_str(&format!(
                                "{}{}",
                                m,
                                t.to_python_type(user_defined_types)
                            )),
                            _ => panic!("Cannot traduce to python the generic value {:?}", value),
                        }
                    }
                    result.push(']');
                    result
                }
                "Py" => {
                    let mut result = String::new();
                    for value in generics.0 {
                        match value {
                            GenericValue::Type {
                                sub_type: t,
                                modifiers: m,
                            } => result.push_str(&format!(
                                "{}{}",
                                m,
                                t.to_python_type(user_defined_types)
                            )),
                            _ => panic!("Cannot traduce to python the generic value {:?}", value),
                        }
                    }
                    result
                }
                "PyArray1" => {
                    // Sadly we cannot specify the inner type
                    "np.ndarray".to_string()
                }
                "PyArray2" => {
                    // Sadly we cannot specify the inner type
                    "np.ndarray".to_string()
                }
                _ => {
                    println!(
                        "Cannot handle translation of '{}' to a known python type.",
                        name
                    );
                    name.trim_start_matches("&").to_string()
                }
            },
            _ => {
                panic!("Cannot translate '{}' as a python type", self.to_string());
            }
        }
    }

    fn to_python_bindings_return_type_inner(
        &self,
        attributes: &[Attribute],
        mut body: String,
        this_struct: &str,
        is_static: bool,
        is_self_ref: bool,
        is_self_mut: bool,
        depth: usize,
    ) -> (String, Option<String>) {
        let (body, r_type) = match self {
            // handle typles recursively
            Type::TupleType(sub_types) => {
                let mut bodies = Vec::new();
                let mut return_types = Vec::new();

                for (i, sub_type) in sub_types.iter().enumerate() {
                    let (inner_body, inner_return_type) = sub_type
                        .to_python_bindings_return_type_inner(
                            attributes,
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
                        .collect::<Vec<_>>()
                        .join(", ")
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
                    return_types
                        .into_iter()
                        .map(|x| x.unwrap_or("()".to_string()))
                        .collect::<Vec<_>>()
                        .join(", ")
                );

                (final_body, Some(final_type))
            }

            // handle null type
            Type::None => (body, Some(self.to_string())),

            x if x.cmp_str_without_modifiers("[Primitive]") => {
                let inner_type = self[0].to_string();

                if body.ends_with(".into()") {
                    body = body.strip_suffix(".into()").unwrap().to_string();
                }

                let mut body = format!(
                    concat!(
                        "let gil = pyo3::Python::acquire_gil();\n",
                        "to_ndarray_1d!(gil, {body}.to_vec(), {inner_type})"
                    ),
                    body = body,
                    inner_type = inner_type,
                );

                if depth != 0 {
                    body = format!("{{{}}}", body);
                }

                (body, Some(format!("Py<PyArray1<{}>>", inner_type)))
            }

            Type::SliceType(sub_type) => {
                let (inner_body, inner_return_type) = sub_type
                    .to_python_bindings_return_type_inner(
                        attributes,
                        "x".to_string(),
                        this_struct,
                        is_static,
                        is_self_ref,
                        is_self_mut,
                        depth + 1,
                    );
                (
                    format!(
                        "{}.into_iter().cloned().map(|x| {{{}}}).collect::<Vec<_>>()",
                        body, inner_body,
                    ),
                    inner_return_type.map(|x| format!("Vec<{}>", x)),
                )
            }

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
                    }
                    (true, false) => {
                        body = format!("{}.into()", body);

                        if self == format!("& {}", this_struct).as_str() {
                            body = format!("{}.to_owned().into()", body);
                        }

                        (body, Some(this_struct.to_string()))
                    }
                    (false, true) => (body, Some(this_struct.to_string())),
                    _ => {
                        panic!("Not implemented yet!");
                    }
                }
            }

            // handle the Result type
            x if x.cmp_without_modifiers(&Type::parse_lossy_str("Result<_>")) => {
                let needs_into = match &self[0] {
                    x if x == "()" => false,
                    _ => true,
                };

                let mut sub_body =
                    format!("pe!({})?{}", body, if needs_into { ".into()" } else { "" });

                if depth != 0 {
                    sub_body = format!("{{{}}}", sub_body);
                }

                let (inner_body, inner_type) = self[0].to_python_bindings_return_type_inner(
                    attributes,
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
                    Some(format!("PyResult<{}>", inner_type.unwrap_or("()".into()))),
                )
            }

            // handle the Option type
            x if x.cmp_without_modifiers(&Type::parse_lossy_str("Option<_>")) => {
                let (inner_body, inner_type) = self[0].to_python_bindings_return_type_inner(
                    attributes,
                    "x".into(),
                    this_struct,
                    is_static,
                    is_self_ref,
                    is_self_mut,
                    depth + 1,
                );
                let body = body.strip_suffix(".into()").unwrap_or(body.as_str());
                (
                    format!("{}.map(|x| {})", body, inner_body),
                    Some(format!("Option<{}>", inner_type.unwrap_or("()".into()))),
                )
            }

            // handle 1d numpy arrays
            x if x.cmp_without_modifiers(&Type::parse_lossy_str("Vec<Primitive>"))
                && !attributes.iter().any(|x| x == "no_numpy_binding") =>
            {
                let inner_type = self[0].to_string();

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

                (body, Some(format!("Py<PyArray1<{}>>", inner_type)))
            }

            // handle 2d numpy arrays
            x if (x.cmp_without_modifiers(&Type::parse_lossy_str("Vec<Vec<Primitive>>"))
                || x.cmp_without_modifiers(&Type::parse_lossy_str("Vec<[Primitive; 1]>"))
                || x.cmp_without_modifiers(&Type::parse_lossy_str("Vec<[Primitive; 2]>"))
                || x.cmp_without_modifiers(&Type::parse_lossy_str("Vec<[Primitive; 3]>"))
                || x.cmp_without_modifiers(&Type::parse_lossy_str("Vec<[Primitive; 4]>"))
                || x.cmp_without_modifiers(&Type::parse_lossy_str("Vec<[Primitive; 5]>"))
                || x.cmp_without_modifiers(&Type::parse_lossy_str("Vec<[Primitive; 6]>")))
                && !attributes.iter().any(|x| x == "no_numpy_binding") =>
            {
                let inner_type = self[0][0].to_string();

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

                (body, Some(format!("Py<PyArray2<{}>>", inner_type)))
            }

            // handle other vec with maybe complex types
            x if x.cmp_without_modifiers(&Type::parse_lossy_str("Vec<_>")) => {
                let inner_type = &x[0];

                match inner_type {
                    Type::TupleType(subtypes) => {
                        // if its a non empty slice of homogeneous primitive types
                        // convert it to a numpy 2d array
                        if subtypes.len() == 2
                            && subtypes[0] == "Primitive"
                            && subtypes[0] == subtypes[1]
                        {
                            let inner_type = &subtypes[0];

                            if body.ends_with(".into()") {
                                body = body.strip_suffix(".into()").unwrap().to_string();
                            }

                            let mut body = format!(
                                r#"
    // Warning: this copies the array so it uses double the memory.
    // To avoid this you should directly generate data compatible with a numpy array
    // Which is a flat vector with row-first or column-first unrolling
    let gil = pyo3::Python::acquire_gil();
    let body = {body};
    let result_array = ThreadDataRaceAware {{t: unsafe{{PyArray2::<{inner_type}>::new(gil.python(), [body.len(), 2], false)}}}};
    body.into_par_iter().enumerate()
        .for_each(|(i, (a, b))| unsafe {{
            *(result_array.t.uget_mut([i, 0]))  = a;
            *(result_array.t.uget_mut([i, 1]))  = b;
        }});
    result_array.t.to_owned()"#,
                                body = body,
                                inner_type = inner_type,
                            );

                            if depth != 0 {
                                body = format!("{{{}}}", body);
                            }

                            return (body, Some(format!("Py<PyArray2<{}>>", inner_type)));
                        }
                    }
                    _ => {}
                }
                let body = body.strip_suffix(".into()").unwrap_or(&body);

                let (inner_body, inner_type) = self[0].to_python_bindings_return_type_inner(
                    attributes,
                    "x".to_string(),
                    this_struct,
                    is_static,
                    is_self_ref,
                    is_self_mut,
                    depth + 1,
                );
                let res_body = format!(
                    "{}.into_iter().map(|x| {}).collect::<Vec<_>>()",
                    body, inner_body
                );

                (res_body, Some(format!("Vec<{}>", inner_type.unwrap())))
            }

            // we don't have special rules so we can just use the default case
            x => match x {
                Type::SimpleType {
                    name,
                    modifiers,
                    generics,
                    traits,
                } => {
                    if name == "str" {
                        return (format!("{}.to_string()", body), Some("String".to_string()));
                    }

                    if modifiers.reference && name != "str" {
                        let mut new_modifiers = modifiers.clone();
                        new_modifiers.reference = false;

                        Type::SimpleType {
                            name: name.clone(),
                            modifiers: new_modifiers,
                            generics: generics.clone(),
                            traits: traits.clone(),
                        }
                        .to_python_bindings_return_type_inner(
                            attributes,
                            format!("{{{}}}.clone()", body),
                            this_struct,
                            is_static,
                            is_self_ref,
                            is_self_mut,
                            depth + 1,
                        )
                    } else {
                        (format!("{}.into()", body), Some(self.to_string()))
                    }
                }
                _ => (format!("{}.into()", body), Some(self.to_string())),
            },
        };

        (body.replace(".into().into()", ".into()"), r_type)
    }
}
