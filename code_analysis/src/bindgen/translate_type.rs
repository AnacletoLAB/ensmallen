use super::*;

pub fn translate_type_str(value: String, user_defined_types: &[&str]) -> String {
    translate_type(&Type::parse_lossy_string(value), user_defined_types)
}

pub fn translate_type(value: &Type, user_defined_types: &[&str]) -> String {

    // If the type was defined by the user it's fine
    let value_str = value.to_string();
    if user_defined_types.contains(&value_str.as_str()) {
        return value.to_string();
    }

    match value.clone() {
        Type::None => "None".into(),
        Type::TupleType(vals) => {
            format!(
                "Tuple[{}]",
                vals.iter()
                    .map(|t| translate_type(t, user_defined_types))
                    .collect::<Vec<String>>()
                    .join(", ")
            )
        }
        Type::SliceType(inner_type) => {
            format!("List[{}]", translate_type(&inner_type, user_defined_types))
        }
        Type::SimpleType {
            name,
            generics,
            ..
        } => match name.as_str() {
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
                        GenericValue::Type(t) => result.push_str(&translate_type(&t, user_defined_types)),
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
                            vals.push(translate_type(&t, user_defined_types));
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
                        GenericValue::Type(t) => result.push_str(&translate_type(&t, user_defined_types)),
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
                        GenericValue::Type(t) => result.push_str(&translate_type(&t, user_defined_types)),
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
                        GenericValue::Type(t) => result.push_str(&translate_type(&t, user_defined_types)),
                        _ => panic!("Cannot traduce to python the generic value {:?}", value),
                    }
                }
                result
            }
            "PyResult" => {
                let mut result = String::new();
                for value in generics.0 {
                    match value {
                        GenericValue::Type(t) => result.push_str(&translate_type(&t, user_defined_types)),
                        _ => panic!("Cannot traduce to python the generic value {:?}", value),
                    }
                }
                result
            }
            "PyDict" => {
                let mut result = "Dict[".to_string();
                for value in generics.0 {
                    match value {
                        GenericValue::Type(t) => result.push_str(&translate_type(&t, user_defined_types)),
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
                        GenericValue::Type(t) => result.push_str(&translate_type(&t, user_defined_types)),
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
                println!("Cannot handle translation of '{}' to a known python type.", name);
                name.trim_start_matches("&").to_string()
            }
        },
        _ => {
            panic!("Cannot translate '{}' as a python type", value.to_string());
        }
    }
}
