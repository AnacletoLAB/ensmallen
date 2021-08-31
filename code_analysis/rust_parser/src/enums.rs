use super::*;

#[derive(Debug, Clone, PartialEq)]
pub struct Enum {
    pub visibility: Visibility,
    pub doc: String,
    pub attributes: Vec<Attribute>,
    pub body: Vec<u8>,
    pub enum_type: Type,
}

impl CanParse for Enum {
    fn can_parse(mut data: &[u8]) -> bool {
        let _visibility = parse!(data, Visibility);
        data.starts_with(b"enum")
    }
}

impl Parse for Enum {
    fn parse(mut data: &[u8]) -> (&[u8], Self) {
        let visibility = parse!(data, Visibility);

        assert!(data.starts_with(b"enum"));
        data = skip_whitespace(&data[4..]);

        let enum_type = parse!(data, Type);

        let (data, body) = get_next_matching(data, b'{', b'}');

        (
            data,
            Enum{
                body: body.to_vec(),
                enum_type,
                visibility,
                doc: String::new(),
                attributes: Vec::new(),
            }
        )

    }
}