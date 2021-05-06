use super::*;

#[derive(Debug, Clone, PartialEq)]
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
}

impl Default for Type {
    fn default() -> Self {
        Type::None
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
            x if x.starts_with(b"Fn") => {
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
                loop {
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
                assert_eq!(traits, Vec::new());
            },
            _ => panic!("The value is expected to be parsed as a simple type.s")
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
                assert_eq!(traits, Vec::new());
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
                assert_eq!(traits, Vec::new());
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
                            traits: Vec::new(),
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
                            traits: Vec::new(),
                        }),
                    ])
                );
            },
            _ => panic!("The value is expected to be parsed as a simple type.s")
        }
    }


}