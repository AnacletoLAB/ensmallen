use super::*;

#[derive(Debug, Clone, PartialEq)]
pub struct Extern{
    pub body: String
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
            return (data, Extern{body: String::from_utf8(body.to_vec()).unwrap()});
        }

        assert!(data.starts_with(b"\"C\""));
        data = skip_whitespace(&data[3..]);
        let (data, body) = get_next_matching(data, b'{', b'}');

        (data, Extern{body: String::from_utf8(body.to_vec()).unwrap()})
    }
}