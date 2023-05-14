use super::*;

#[derive(Debug, Clone, PartialEq)]
pub struct TraitDefinition{
    pub name: Type,
    pub value: Option<Type>,
    pub visibility: Visibility,
    pub doc: String,
    pub attributes: Vec<Attribute>,
    pub body: String,
}

impl CanParse for TraitDefinition {
    fn can_parse(mut data: &[u8]) -> bool {
        let _visibility = parse!(data, Visibility);
        data.starts_with(b"trait")
    }
}

impl Parse for TraitDefinition {
    fn parse(mut data: &[u8]) -> (&[u8], Self) {
        let visibility = parse!(data, Visibility);
        assert!(data.starts_with(b"trait"));
        data = &data[5..];
        let type_name = parse!(data,  Type);
        let type_value = if data[0] == b':' {
            data = &data[1..];
            Some(parse!(data, Type))
        } else  {
            None
        };
        while data[0] != b'{' {
            data = &data[1..];
        }
        assert!(data.starts_with(b"{"));
        let (data, body) = get_next_matching(data, b'{', b'}');
        (
            data,
            TraitDefinition{
                name: type_name,
                value: type_value,
                visibility: visibility,
                doc: String::new(),
                attributes: Vec::new(),
                body: String::from_utf8(body.to_vec()).unwrap(),
            }
        )
    }
}
