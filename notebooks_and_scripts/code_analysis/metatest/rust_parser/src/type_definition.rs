use super::*;

#[derive(Debug, Clone, PartialEq)]
pub struct TypeDefinition{
    pub name: String,
    pub value: Type,
    pub visibility: Visibility,
    pub doc: String,
    pub attributes: Vec<String>,
}

impl CanParse for TypeDefinition {
    fn can_parse(mut data: &[u8]) -> bool {
        let _visibility = parse!(data, Visibility);
        data.starts_with(b"type")
    }
}

impl Parse for TypeDefinition {
    fn parse(mut data: &[u8]) -> (&[u8], Self) {
        let visibility = parse!(data, Visibility);
        assert!(data.starts_with(b"type"));
        data = &data[4..];
        let type_name = parse!(data, Identifier);
        assert!(data.starts_with(b"="));
        data = &data[1..];
        let type_value = parse!(data, Type);
        assert!(data.starts_with(b";"));
        data = &data[1..];

        (
            data,
            TypeDefinition{
                name: type_name.into(),
                value: type_value,
                visibility: visibility,
                doc: String::new(),
                attributes: Vec::new(),
            }
        )
    }
}