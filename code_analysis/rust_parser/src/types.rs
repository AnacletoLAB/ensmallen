use super::*;

pub const PRIMITIVE_TYPES: &'static [&'static str] = &[
    "bool",
    "usize",
    "u8",
    "u16",
    "u32",
    "u64",
    "u128",
    "isize",
    "i8",
    "i16",
    "i32",
    "i64",
    "i128",
    "f32",
    "f64",
    "NodeT",
    "EdgeT",
    "WeightT",
    "NodeTypeT",
    "EdgeTypeT",
];

#[derive(Debug, Clone)]
/// Enumeration of the possible types
/// Here we need to use boxes because otherwise
/// the type would be recursive and thus of infinite size
pub enum Type {
    SelfType,
    SimpleType{
        name: String,
        modifiers: TypeModifiers,
        generics: Generics,
        traits: Vec<Type>,
    },
    FnType{
        args: Vec<Arg>,
        return_type: Option<Box<Type>>,
    },
    TupleType(Vec<Type>),
    ImplType(Box<Type>),
    DynType(Box<Type>),
    SliceType(Box<Type>),
    None,
    DontCare,
    Primitive,
}

impl Default for Type {
    fn default() -> Self {
        Type::None
    }
}
impl Type {
    /// Iff this is a simple type, return the name of it
    /// otherwise panic
    pub fn get_name(&self) -> Option<String> {
        match self {
            Type::SimpleType {
                name,
                ..
            } => Some(name.to_string()),
            Type::None => Some("()".to_string()),
            _ => None,
        }
    }
}

impl std::ops::Index<usize> for Type {
    type Output = Type;
    
    fn index(&self, index: usize) -> &Self::Output {
        use Type::*;
        match self {
            SimpleType{
                generics,
                ..
            } => {
                match &generics[index] {
                    GenericValue::Type{sub_type, ..} => sub_type,
                    _ => panic!("Cannot index a non type generic value"),
                }
            },
            _ => panic!("It's not possible to index the current kind of type"),
        }
    }
}

impl PartialEq for Type {
    fn eq(&self, other:&Self) -> bool {
        use Type::*;
        match (self, other) {
            (DontCare, _) => true,
            (_, DontCare) => true,
            (SelfType, SelfType) => true,
            (None, None) => true,
            (SliceType(t1), SliceType(t2)) => t1 == t2,
            (DynType(t1), DynType(t2))     => t1 == t2,
            (ImplType(t1), ImplType(t2))   => t1 == t2,
            (TupleType(t1), TupleType(t2)) => t1 == t2,
            (FnType{
                args:a1,
                return_type:r1
            },
            FnType{
                args:a2,
                return_type:r2
            }) => a1 == a2 && r1 == r2,
            (
                SimpleType{
                    name: n1,
                    modifiers: m1,
                    generics: g1,
                    traits: t1,
                },
                SimpleType{
                    name: n2,
                    modifiers: m2,
                    generics: g2,
                    traits: t2,
                },
            ) => n1 == n2 && m1 == m2 && g1 == g2 && t1 == t2,
            (Primitive, SimpleType{name, ..}) | (SimpleType{name, ..}, Primitive) => {
                PRIMITIVE_TYPES.contains(&name.as_str())
            }
            _ => false,
        }
    }
}

impl CmpWithoutModifiers for Type {
    fn cmp_without_modifiers(&self, other:&Type) -> bool {
        use Type::*;
        match (self, other) {
            (DontCare, _) => true,
            (_, DontCare) => true,
            (SelfType, SelfType) => true,
            (None, None) => true,
            (SliceType(t1), SliceType(t2)) => t1.cmp_without_modifiers(t2),
            (DynType(t1), DynType(t2))     => t1.cmp_without_modifiers(t2),
            (ImplType(t1), ImplType(t2))   => t1.cmp_without_modifiers(t2),
            (TupleType(t1), TupleType(t2)) => {
                if t1.len() != t2.len() {
                    return false;
                }
                for i in 0..t1.len() {
                    if !t1[i].cmp_without_modifiers(&t2[i]) {
                        return false;
                    }
                }
                true
            },
            (FnType{
                args:a1,
                return_type:r1
            },
            FnType{
                args:a2,
                return_type:r2
            }) => a1 == a2 && r1 == r2,
            (
                SimpleType{
                    name: n1,
                    generics: g1,
                    ..
                },
                SimpleType{
                    name: n2,
                    generics: g2,
                    ..
                },
            ) => n1 == n2 && g1.cmp_without_modifiers(g2),
            (Primitive, SimpleType{name, ..}) | (SimpleType{name, ..}, Primitive) => {
                PRIMITIVE_TYPES.contains(&name.as_str())
            }
            _ => false,
        }
    }
}

impl PartialEq<&str> for Type {
    fn eq(&self, other:&&str) -> bool {
        self == &Type::parse_lossy(other.as_bytes())
    }
}

impl PartialEq<str> for Type {
    fn eq(&self, other:&str) -> bool {
        self == &Type::parse_lossy(other.as_bytes())
    }
}

impl Parse for Type {
    fn parse(mut data: &[u8]) -> (&[u8], Self) {
        let mut modifiers = parse!(data, TypeModifiers);

        let result = match data {
            // Slice Type
            x if x.starts_with(b"[") => {
                let (tmp, matching) = get_next_matching(data, b'[',b']');
                data = tmp;
                let (_, inner) = Type::parse(matching);
                Type::SliceType(Box::new(inner))
            },
            x if x.starts_with(b"()") => {
                data = &data[2..];
                Type::None
            }
            // Tuple type
            x if x.starts_with(b"(") => {
                let (tmp, mut matching) = get_next_matching(data, b'(',b')');
                data = tmp;

                let mut result = Vec::new();
                loop {
                    matching = skip_whitespace(matching);

                    if matching.is_empty(){
                        break;
                    }

                    if matching.starts_with(b",") {
                        matching = skip_whitespace(&matching[1..]);
                    }
                    result.push(parse!(matching, Type));
                }

                Type::TupleType(result)
            },
            // Fn type
            x if x.starts_with(b"Fn") || x.starts_with(b"fn") => {
                // TODO! Save which variant it is
                if x.starts_with(b"FnMut") {
                    data = skip_whitespace(&data[5..]);
                } else if x.starts_with(b"FnOnce") {
                    data = skip_whitespace(&data[6..]);
                } else {
                    data = skip_whitespace(&data[2..]);
                }

                let (inner, mut args_content) = get_next_matching(data, b'(', b')');
                data = inner;
                let mut args = Vec::new();
                args_content = skip_whitespace(args_content);
                while !args_content.is_empty() {
                    args_content = skip_whitespace(args_content);
                    if args_content.starts_with(b",") {
                        args_content = skip_whitespace(&args_content[1..]);
                    }
        
                    if args_content.is_empty() {
                        break;
                    }
        
                    let modifier = parse!(args_content, TypeModifiers);
                    let arg_type = parse!(args_content, Type);
                    
                    args.push(Arg{
                        name: format!("{}", args.len()),
                        arg_modifier: modifier,
                        arg_type: arg_type,
                    });
                    args_content = skip_whitespace(args_content);
                }

                // skip the )
                data = skip_whitespace(&data[1..]);

                let mut return_type = None;
                if data.starts_with(b"->") {
                    data = &data[2..];
                    return_type = Some(Box::new(parse!(data, Type)));
                }
                Type::FnType{
                    args:args,
                    return_type: return_type,
                }
            },
            // Impl type
            x if x.starts_with(b"impl") => {
                data = skip_whitespace(&data[4..]);
                let impl_type = parse!(data, Type);
                Type::ImplType(Box::new(impl_type))
            },
            // Dyn type
            x if x.starts_with(b"dyn") => {
                data = skip_whitespace(&data[3..]);
                let impl_type = parse!(data, Type);
                Type::ImplType(Box::new(impl_type))
            },
            // Simple type
            _ => {
                // parse the name of the type
                let mut name: String = parse!(data, Identifier).into();

                if name == "_" {
                    return (data, Type::DontCare);
                }

                if name == "Primitive" {
                    return (data, Type::Primitive);
                }

                if name == "self" {
                    return (data, Type::SelfType);
                }

                // parse eventual mod import
                while data.starts_with(b"::") {
                    name.push_str("::");
                    data = skip_whitespace(&data[2..]);
                    let sub_name: String = parse!(data, Identifier).into();
                    name.push_str(sub_name.as_str());
                }

                // If it has generics parse them:
                let generics = if Generics::can_parse(data) {
                    parse!(data, Generics)
                } else {
                    Generics::default()
                };

                // Parse eventual lifetime or traits
                let mut traits = Vec::new();
                while data.starts_with(b"+") {
                    data = skip_whitespace(&data[1..]);

                    if data.starts_with(b"'") {
                        let lifetime = parse!(data, Lifetime);
                        modifiers.lifetime = Some(lifetime);
                        continue;
                    }

                    let ttrait = parse!(data, Type);
                    traits.push(ttrait);
                }

                Type::SimpleType{
                    name,
                    modifiers,
                    generics,
                    traits,
                }
            },
        };
        (data, result)
    }
}

impl std::fmt::Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

impl Type {
    fn to_string(&self) -> String {
        String::from(self.clone())
    }
}

impl From<Type> for String{
    fn from(x: Type) -> String {
        match x {
            Type::None => {
                "()".to_string()
            }
            Type::DontCare => {
                "_".to_string()
            }
            Type::Primitive => {
                "Primitive".to_string()
            }
            Type::SelfType => {
                "self".to_string()
            }
            Type::SimpleType{
                name,
                modifiers,
                generics,
                traits,
            } => {
                let mut result: String = modifiers.into();
                result.push_str(&String::from(name));
                result.push_str(&String::from(generics));
                for traitt in traits {
                    result.push_str(" + ");
                    let traitt: String = traitt.into();
                    result.push_str(&traitt);
                }
                result
            }
            Type::DynType(val) => {
                format!("dyn {}", String::from(*val))
            }
            Type::ImplType(val) => {
                format!("impl {}", String::from(*val))
            }
            Type::TupleType(vals) => {
                let mut result = "(".to_string();

                for val in vals {
                    result.push_str(&String::from(val));
                    result.push_str(", ");
                }

                result = (&result[.. result.len().saturating_sub(2)]).to_string();
                result.push(')');
                result

            }
            Type::SliceType(val) => {
                format!("[{}]", &String::from(*val))
            }
            Type::FnType{
                args,
                return_type
            } => {
                // TODO! change the str based on the type (Fn FnMut FnOnce)
                let mut result = "Fn(".to_string();
                for arg in args{
                    result.push_str(&String::from(arg.arg_type));
                    result.push_str(" ,");
                }
                result = result.trim_end_matches(&", ").to_string();
                result.push(')');

                if let Some(rt) = return_type {
                    result.push_str(" -> ");
                    result.push_str(&String::from(*rt));
                }
                result
            }
        }
    }
}


#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_simple_type() {
        let data = "u64".to_string();
        let ptr = data.as_bytes();
        let (reminder, res) = Type::parse(ptr);
        assert_eq!(reminder, "".as_bytes());
        match res {
            Type::SimpleType { 
                name, 
                modifiers,
                generics, 
                traits,
            } => {
                assert_eq!(name, "u64");
                assert_eq!(modifiers, TypeModifiers::default());
                assert_eq!(generics, Generics::default());
                assert!(traits.is_empty());
            },
            _ => panic!("The value is expected to be parsed as a simple type.s")
        }
    }

    #[test]
    fn test_primitive_type() {
        for ptype in PRIMITIVE_TYPES {
            assert_eq!(Type::parse_lossy_str(ptype), "Primitive");
        }
    }

    #[test]
    fn test_impl() {
        let data = "impl Iterator<Item = (EdgeT, NodeT, NodeT)> + '_ + Clone".to_string();
        let ptr = data.as_bytes();
        let (reminder, res) = Type::parse(ptr);
        assert_eq!(reminder, "".as_bytes());
        match res {
            Type::ImplType(
                impl_type
            ) => {
                match *impl_type {
                    Type::SimpleType{
                        name,
                        modifiers,
                        generics,
                        traits,
                    } => {

                        assert_eq!(name, "Iterator");
                        let mut truth_mod = TypeModifiers::default();
                        truth_mod.lifetime = Some(Lifetime("_".to_string()));
                        assert_eq!(modifiers, truth_mod);
                        assert_eq!(generics, Generics(
                            vec![
                                GenericValue::TypeAssignement(
                                    Type::SimpleType{
                                        name: "Item".to_string(),
                                        modifiers: TypeModifiers::default(),
                                        generics: Generics::default(),
                                        traits: Vec::new(),
                                    },
                                    Type::TupleType(vec![
                                        Type::SimpleType{
                                            name: "EdgeT".to_string(),
                                            modifiers: TypeModifiers::default(),
                                            generics: Generics::default(),
                                            traits: Vec::new(),
                                        },
                                        Type::SimpleType{
                                            name: "NodeT".to_string(),
                                            modifiers: TypeModifiers::default(),
                                            generics: Generics::default(),
                                            traits: Vec::new(),
                                        },
                                        Type::SimpleType{
                                            name: "NodeT".to_string(),
                                            modifiers: TypeModifiers::default(),
                                            generics: Generics::default(),
                                            traits: Vec::new(),
                                        }
                                    ]),
                                )
                            ]
                        ));
                        assert_eq!(traits, vec![
                            Type::SimpleType{
                                name: "Clone".to_string(),
                                modifiers: TypeModifiers::default(),
                                generics: Generics::default(),
                                traits: Vec::new(),
                            }
                        ]);
                    }
                    _ => panic!("the value is expected to be a simple type")
                }
            },
            _ => panic!("The value is expected to be parsed as an impl type")
        }
    }

    #[test]
    fn test_simple_type_with_generics() {
        let data = "Option<u64, String>".to_string();
        let ptr = data.as_bytes();
        let (reminder, res) = Type::parse(ptr);
        assert_eq!(reminder, "".as_bytes());
        match res {
            Type::SimpleType { 
                name, 
                modifiers,
                generics, 
                traits,
            } => {
                assert_eq!(name, "Option");
                assert!(traits.is_empty());
                assert_eq!(modifiers, TypeModifiers::default());
                assert_eq!(
                    generics, 
                    Generics(vec![
                        GenericValue::Type(Type::SimpleType{
                            name:"u64".to_string(),
                            modifiers: TypeModifiers::default(),
                            generics: Generics::default(),
                            traits: Vec::new(),
                        }),
                        GenericValue::Type(Type::SimpleType{
                            name:"String".to_string(),
                            modifiers: TypeModifiers::default(),
                            generics: Generics::default(),
                            traits: Vec::new(),
                        }),
                    ])
                );
            },
            _ => panic!("The value is expected to be parsed as a simple type.s")
        }

    }
    #[test]
    fn test_simple_type_with_generics_and_modifiers() {
        let data = "&mut 'a Result<&u64, *mut String>".to_string();
        let ptr = data.as_bytes();
        let (reminder, res) = Type::parse(ptr);
        assert_eq!(reminder, "".as_bytes());
        match res {
            Type::SimpleType { 
                name, 
                modifiers,
                generics, 
                traits,
            } => {
                assert_eq!(name, "Result");
                assert_eq!(modifiers, TypeModifiers{
                    pointer: false,
                    reference: true,
                    mutable: true,
                    constant: false,
                    lifetime: Some(Lifetime("a".to_string())),
                });
                assert!(traits.is_empty());
                assert_eq!(
                    generics, 
                    Generics(vec![
                        GenericValue::Type(Type::SimpleType{
                            name:"u64".to_string(),
                            modifiers: TypeModifiers{
                                pointer: false,
                                reference: true,
                                mutable: false,
                                constant: false,
                                lifetime: None,
                            },
                            generics: Generics::default(),
                            traits: vec![],
                        }),
                        GenericValue::Type(Type::SimpleType{
                            name:"String".to_string(),
                            modifiers: TypeModifiers{
                                pointer: true,
                                reference: false,
                                mutable: true,
                                constant: false,
                                lifetime: None,
                            },
                            generics: Generics::default(),
                            traits: vec![],
                        }),
                    ])
                );
            },
            _ => panic!("The value is expected to be parsed as a simple type.s")
        }
    }


    #[test]
    fn test_impls_equivalence() {
        assert_eq!(
            Type::parse_lossy("Result<impl ParallelIterator<Item=_>, _>".as_bytes()),
            Type::parse_lossy("Result<impl ParallelIterator<Item=(u64, Vec<f64>, )>, String>".as_bytes())
        );

        assert_ne!(
            Type::parse_lossy("Result<impl ParallelIterator<Item=_>, _>".as_bytes()),
            Type::parse_lossy("Result<impl ParallelIterator<Item=(u64, Vec<f64>, )> + '_, String>".as_bytes())
        );
        
        assert!(
            Type::parse_lossy("Result<impl ParallelIterator<Item=_>, _>".as_bytes())
                .cmp_str_without_modifiers(&"Result<impl ParallelIterator<Item=(u64, Vec<f64>, )> + '_, String>")
        );
    }

    #[test]
    fn test_equivalence() {
        let data = "Result<Vec<f64>, String>".as_bytes();
        let (reminder, res) = Type::parse(data);
        assert_eq!(reminder, "".as_bytes());
        assert_eq!(
            res,
            Type::SimpleType{
                name: "Result".to_string(),
                modifiers: TypeModifiers::default(),
                generics: Generics(vec![
                    GenericValue::Type(Type::SimpleType{
                        name: "Vec".to_string(),
                        modifiers: TypeModifiers::default(),
                        generics: Generics(vec![
                            GenericValue::Type(Type::SimpleType{
                                name: "f64".to_string(),
                                modifiers: TypeModifiers::default(),
                                generics: Generics::default(),
                                traits: vec![],
                            })
                        ]),
                        traits: vec![],
                    }),
                    GenericValue::Type(Type::SimpleType{
                        name: "String".to_string(),
                        modifiers: TypeModifiers::default(),
                        generics: Generics::default(),
                        traits: vec![],
                    })
                ]),
                traits: vec![],
            }
        );
        assert_eq!(
            res,
            Type::SimpleType{
                name: "Result".to_string(),
                modifiers: TypeModifiers::default(),
                generics: Generics(vec![
                    GenericValue::Type(Type::SimpleType{
                        name: "Vec".to_string(),
                        modifiers: TypeModifiers::default(),
                        generics: Generics(vec![
                            GenericValue::Type(Type::DontCare)
                        ]),
                        traits: vec![],
                    }),
                    GenericValue::Type(Type::SimpleType{
                        name: "String".to_string(),
                        modifiers: TypeModifiers::default(),
                        generics: Generics::default(),
                        traits: vec![],
                    })
                ]),
                traits: vec![],
            }
        );
        assert_eq!(
            res,
            Type::SimpleType{
                name: "Result".to_string(),
                modifiers: TypeModifiers::default(),
                generics: Generics(vec![
                    GenericValue::Type(Type::DontCare),
                    GenericValue::Type(Type::SimpleType{
                        name: "String".to_string(),
                        modifiers: TypeModifiers::default(),
                        generics: Generics::default(),
                        traits: vec![],
                    })
                ]),
                traits: vec![],
            }
        );
        assert_eq!(
            res,
            Type::SimpleType{
                name: "Result".to_string(),
                modifiers: TypeModifiers::default(),
                generics: Generics(vec![
                    GenericValue::Type(Type::DontCare),
                    GenericValue::Type(Type::DontCare),
                ]),
                traits: vec![],
            }
        );
        assert_eq!(
            res,
            Type::DontCare,
        );
        assert_eq!(
            res,
            "Result<Vec<_>, _>",
        );
    }

}