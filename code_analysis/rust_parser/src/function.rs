use super::*;

#[derive(Debug, Clone, PartialEq)]
pub struct Function{
    pub doc: String,
    pub attributes: Vec<Attribute>,
    pub name: String,
    pub visibility: Visibility,
    pub generics: Generics,
    pub args: Args,
    pub return_type: Option<Type>,
    pub body: String,
    pub is_unsafe: bool,
    pub class: Option<Type>,
    pub file_path: String,
}

impl Default for Function{
    fn default() -> Self {
        Function {
            doc: String::new(),
            attributes: Vec::new(),
            name: String::new(),
            visibility: Visibility::Private,
            generics: Generics::default(),
            args: Args::default(),
            return_type: None,
            body: String::new(),
            is_unsafe: false,
            class: None,
            file_path: String::new(),
        }
    }
}

impl Function {
    /// Returns if the current function has any arguments
    /// EXCLUDING THE SELF
    pub fn has_arguments(&self) -> bool {
        self.args.0.iter().any(|x| {
            match x.arg_type {
                Type::SelfType => false,
                _ => true,
            }
        })
    }

    pub fn get_self_modifiers(&self) -> Option<TypeModifiers> {
        match self.args.0.get(0) {
            Some(arg) => Some(arg.arg_modifier.clone()),
            None => None,
        }
    }

    pub fn iter_args(&self) -> impl Iterator<Item=&Arg> {
        self.args.0.iter()
    }
    
    /// Return if the current function is unsafe
    pub fn is_unsafe(&self) -> bool {
        self.is_unsafe
    }

    /// Return if this function is a method of a class
    pub fn is_method(&self) -> bool {
        self.class.is_some()
    }

    /// Return if it's a static function / method meaning that it does not
    /// requires self
    pub fn is_static(&self) -> bool {
        match self.args.0.get(0) {
            Some(arg) => {
                match arg.arg_type {
                    Type::SelfType => false,
                    _ => true,
                }
            },
            None => true,
        }
    }

    /// Set this function as a method of the given class
    pub fn set_class(&mut self, class: Type){
        self.class = Some(class);
    }

    /// Returns if the current function returns a result
    pub fn returns_result(&self) -> bool {
        match &self.return_type {
            Some(Type::SimpleType{
                name,
                ..
            }) => {
                name == "Result"
            }
            _ => false,
        }
    }
}

impl CanParse for Function {
    fn can_parse(mut data: &[u8]) -> bool {
        data = skip_whitespace(data);
        let _visibility = parse!(data, Visibility);
        data = skip_whitespace(data);
        if data.starts_with(b"unsafe") {
            data = skip_whitespace(&data[6..]);
        }
        data.starts_with(b"fn")
    }
}

impl Parse for Function {
    fn parse(mut data: &[u8]) -> (&[u8], Self) {
        let visibility = parse!(data, Visibility);

        let mut is_unsafe = false;
        if data.starts_with(b"unsafe") {
            is_unsafe = true;
            data = skip_whitespace(&data[6..]);
        }

        assert!(data.starts_with(b"fn"));
        data = skip_whitespace(&data[2..]);

        // this is tecnically not right but we want an identifier
        // that might have generics.
        let (function_name, generics) = match parse!(data, Type) {
            Type::SimpleType{
                name,
                modifiers: _,
                generics,
                traits: _,
            } => {
                (name, generics)
            },
            _ => unreachable!("unexpected problem parsing the function name and generics"),
        };
        assert!(data.starts_with(b"("));

        let args = parse!(data, Args);

        let mut return_type = None;
        if data.starts_with(b"->") {
            data = &data[2..];
            return_type = Some(parse!(data, Type));
        }
        
        // parse the body
        let (data, mut body_content) = get_next_matching(data, b'{', b'}');
        body_content = skip_whitespace(body_content);
        
        (
            data, 
            Function {
                doc: String::new(),
                attributes: Vec::new(),
                name: function_name,
                visibility: visibility,
                generics: generics,
                args: args,
                return_type: return_type,
                body: String::from_utf8(body_content.to_vec()).unwrap(),
                is_unsafe: is_unsafe,
                class: None,
                file_path: String::new(),
            }
        )
    }
}
