use super::*;

pub fn translate_type_str(value: String) -> String {
    translate_type(&Type::parse_lossy_string(value))
}

pub fn translate_type(value: &Type) -> String {
    match value.clone() {
        Type::TupleType(vals) => {
            format!(
                "Tuple[{}]",
                vals.iter()
                    .map(|t| translate_type(t))
                    .collect::<Vec<String>>()
                    .join(", ")
            )
        }
        Type::SliceType(inner_type) => {
            format!("List[{}]", translate_type(&inner_type))
        }
        Type::SimpleType {
            name,
            generics,
            ..
        } => match name.as_str() {
            "Graph" => "EnsmallenGraph".to_string(),
            "NodeT" | "usize" | "EdgeT"
                | "u64" | "NodeTypeT"
                | "EdgeTypeT" => "int".to_string(),
            "WeightT" | "f64" | "f32" => "float".to_string(),
            "bool" => "bool".to_string(),
            "str" | "S" | "String" => "str".to_string(),
            "RoaringBitmap" => "List[int]".to_string(),
            "HashSet" => {
                let mut result = "Set[".to_string();
                for value in generics.0 {
                    match value {
                        GenericValue::Type(t) => result.push_str(&translate_type(&t)),
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
                            vals.push(translate_type(&t));
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
                        GenericValue::Type(t) => result.push_str(&translate_type(&t)),
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
                        GenericValue::Type(t) => result.push_str(&translate_type(&t)),
                        _ => panic!("Cannot traduce to python the generic value {:?}", value),
                    }
                }

                result.push(']');
                result
            }
            _ => {
                panic!("Cannot translate '{:?}' as a python unknown type", value.to_string());
            }
        },
        _ => {
            panic!("Cannot translate '{:?}' as a python type", value.to_string());
        }
    }
}
