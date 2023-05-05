use super::*;

#[derive(Debug, Clone, PartialEq)]
pub struct Struct {
    pub visibility: Visibility,
    pub doc: String,
    pub attributes: Vec<Attribute>,
    pub fields: Vec<StructField>,
    pub struct_type: Type,
    pub file_path: String,
}

impl CanParse for Struct {
    fn can_parse(mut data: &[u8]) -> bool {
        let _visibility = parse!(data, Visibility);
        data.starts_with(b"struct")
    }
}

impl Parse for Struct {
    fn parse(mut data: &[u8]) -> (&[u8], Self) {
        let visibility = parse!(data, Visibility);

        assert!(data.starts_with(b"struct"));
        data = skip_whitespace(&data[6..]);

        let struct_type = parse!(data, Type);

        if data[0] == b';' {
            return (
                &data[1..],
                Struct{
                    visibility,
                    doc: "".into(),
                    attributes: vec![],
                    fields: vec![],
                    struct_type,
                    file_path: "".into(),
                }
            );
        }
        
        let (mut data, mut struct_body) = get_next_matching(data, b'{', b'}');

        let mut fields = Vec::new();
        let mut doc = String::new();
        loop {
            struct_body = skip_whitespace(struct_body);
            if struct_body.is_empty(){
                break;
            }

            if DocLine::can_parse(struct_body){
                let doc_line: String = parse!(struct_body, DocLine).into();
                doc.push_str(&doc_line);
                continue;
            }
            if struct_body.starts_with(b"//") {
                let (inner, _comment) = split_at(struct_body, b'\n');
                struct_body = inner;
                continue;
            }

            let mut field = parse!(struct_body, StructField);
            field.doc = doc;
            doc = String::new();

            fields.push(field);

            if struct_body.starts_with(b",") {
                struct_body = skip_whitespace(&struct_body[1..]);
            }
        }

        data = skip_whitespace(data);
        if data.starts_with(b";") {
            data = skip_whitespace(&data[1..]);
        }

        (
            data,
            Struct{
                fields,
                struct_type,
                visibility,
                doc: String::new(),
                attributes: Vec::new(),
                file_path: String::new(),
            }
        )

    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct StructField {
    visibility: Visibility,
    name: String,
    field_type: Type,
    doc: String,
    attributes: Vec<String>,
}

impl Parse for StructField {
    fn parse(mut data: &[u8]) -> (&[u8], Self) {
        let visibility = parse!(data, Visibility);
        let name: String = parse!(data, Identifier).into();
        assert!(data.starts_with(b":"));
        data = &data[1..];
        let field_type = parse!(data, Type);

        (
            data, 
            StructField{
                visibility,
                name: name,
                field_type,
                doc: String::new(),
                attributes: Vec::new(),
            }
        )
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_struct_field() {
        let data = "pub(crate) return_weight: ParamT,pub test: u64".to_string();
        let ptr = data.as_bytes();
        let (reminder, res) = StructField::parse(ptr);
        assert_eq!(reminder, ",pub test: u64".as_bytes());
        assert_eq!(res, StructField{
            visibility: Visibility::PublicCrate,
            name: "return_weight".to_string(),
            field_type: Type::SimpleType{
                name: "ParamT".to_string(),
                modifiers: TypeModifiers::default(),
                generics: Generics::default(),
                traits: Vec::new(),
            },
            doc: String::new(),
            attributes: Vec::new(),
        });
    }

    #[test]
    fn test_struct() {
        let data = "pub struct Test { pub(crate) return_weight: ParamT, pub test: u64 };AAAA".to_string();
        let ptr = data.as_bytes();
        let (reminder, res) = Struct::parse(ptr);
        assert_eq!(reminder, "AAAA".as_bytes());
        assert_eq!(res, Struct{
            file_path: String::new(),
            visibility: Visibility::Public,
            doc: String::new(),
            attributes: Vec::new(),
            struct_type: Type::SimpleType{
                name: "Test".to_string(),
                modifiers: TypeModifiers::default(),
                generics: Generics::default(),
                traits: Vec::new(),
            },
            fields:vec![
                StructField{
                    visibility: Visibility::PublicCrate,
                    name: "return_weight".to_string(),
                    field_type: Type::SimpleType{
                        name: "ParamT".to_string(),
                        modifiers: TypeModifiers::default(),
                        generics: Generics::default(),
                        traits: Vec::new(),
                    },
                    doc: String::new(),
                    attributes: Vec::new(),
                },
                StructField{
                    visibility: Visibility::Public,
                    name: "test".to_string(),
                    field_type: Type::SimpleType{
                        name: "u64".to_string(),
                        modifiers: TypeModifiers::default(),
                        generics: Generics::default(),
                        traits: Vec::new(),
                    },
                    doc: String::new(),
                    attributes: Vec::new(),
                }
        ],});
    }
}
