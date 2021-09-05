use super::*;

#[derive(Debug, Clone, PartialEq)]
pub struct Extern{
    pub visibility: Visibility,
    pub doc: String,
    pub attributes: Vec<Attribute>,
    pub content: String
}

impl Default for Extern {
    fn default() -> Self {
        Extern {
            doc: String::new(),
            attributes: Vec::new(),
            visibility: Visibility::Private,
            content: String::new(),
        }
    }
}

impl CanParse for Extern {
    fn can_parse(mut data: &[u8]) -> bool {
        data = skip_whitespace(data);
        data.starts_with(b"extern")
    }
}

impl Parse for Extern {
    fn parse(mut data: &[u8]) -> (&[u8], Self) {
        data = skip_whitespace(&data[6..]);
        if data.starts_with(b"crate"){
            data = skip_whitespace(&data[5..]);
            let (data, body) = split_at(data, b';');
            let mut result = Extern::default();
            result.content = bytes_to_string(body);
            return (data, result);
        }

        assert!(data.starts_with(b"\"C\""));
        data = skip_whitespace(&data[3..]);
        let (data, body) = get_next_matching(data, b'{', b'}');

        let mut result = Extern::default();
        result.content = bytes_to_string(body);
        (data, result)
    }
}