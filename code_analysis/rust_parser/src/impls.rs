use super::*;

#[derive(Debug, Clone, PartialEq)]
pub struct Impl {
    pub attributes: Vec<Attribute>,
    pub doc: String,
    pub struct_name: Type,
    pub impl_trait: Option<Type>,
    pub macro_calls: Vec<MacroCall>,
    pub methods: Vec<Function>,
    pub type_defs: Vec<TypeDefinition>,
    pub generics: Generics,
}

impl CanParse for Impl {
    fn can_parse(mut data: &[u8]) -> bool {
        if data.starts_with(b"unsafe") {
            data = skip_whitespace(&data[6..]);
        }
        data.starts_with(b"impl")
    }
}

impl Parse for Impl {
    fn parse(mut data: &[u8]) -> (&[u8], Self) {
        // skip impl
        if data.starts_with(b"unsafe") {
            data = skip_whitespace(&data[6..]);
            // in the future we might want to save this info
        }
        data = &data[4..];

        // If it has generics parse them:
        let generics = if Generics::can_parse(data) {
            parse!(data, Generics)
        } else {
            Generics::default()
        };

        let mut struct_type = parse!(data, Type);
        let mut impl_trait = None;

        // Parse the trait if present
        if data.starts_with(b"for") {
            data = &data[3..];
            impl_trait = Some(struct_type);
            struct_type = parse!(data, Type);
        }

        // body
        let (data, mut impl_content) = get_next_matching(data, b'{', b'}');

        let mut macro_calls = Vec::new();
        let mut methods = Vec::new();
        let mut type_defs = Vec::new();
        let mut doc = String::new();
        let mut attrs = Vec::new();
        loop {
            impl_content = skip_whitespace(impl_content);

            if impl_content.is_empty() {
                break;
            }

            maybe_parse!(impl_content, doc, attrs, MacroCall,  macro_calls);
            if Function::can_parse(impl_content) {
                let mut method = parse!(impl_content, Function);
                method.set_class(struct_type.clone());
                // TODO!: add attributes and doc
                method.doc = doc;
                doc = String::new();
                method.attributes = attrs;
                attrs = Vec::new();
                methods.push(method);
                continue;
            }
            if TypeDefinition::can_parse(impl_content) {
                let mut type_definition = parse!(impl_content, TypeDefinition);
                // TODO!: add attributes and doc
                type_definition.doc = doc;
                doc = String::new();
                type_definition.attributes = attrs;
                attrs = Vec::new();
                type_defs.push(type_definition);
                continue;
            }
            if DocLine::can_parse(impl_content){
                let doc_line: String = parse!(impl_content, DocLine).into();
                doc.push_str(&doc_line);
                doc.push('\n');
                continue;
            }
            if Attribute::can_parse(impl_content){
                attrs.push(parse!(impl_content, Attribute));
                continue;
            }
            if impl_content.starts_with(b"//") {
                let (inner, _comment) = split_at(impl_content, b'\n');
                impl_content = inner;
                continue;
            }

            panic!("impl body cannot parse {}", String::from_utf8(impl_content.to_vec()).unwrap());
        }

        (
            data, 
            Impl{
                attributes: Vec::new(),
                doc: String::new(),
                struct_name: struct_type,
                impl_trait: impl_trait,
                macro_calls: macro_calls,
                methods: methods,
                type_defs: type_defs,
                generics: generics,
            }
        )
        
    }
}