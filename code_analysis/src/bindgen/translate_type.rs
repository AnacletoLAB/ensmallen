use super::*;

pub fn translate_type_str(value: String) -> String {
    translate_type(Type::parse_lossy_string(value))
}

pub fn translate_type(value: Type) -> String {
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
            generics,
            ..
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
                panic!("Cannot translate '{:?}' as a python unknown type", value.to_string());
            }
        },
        _ => {
            panic!("Cannot translate '{:?}' as a python type", value.to_string());
        }
    }
}
