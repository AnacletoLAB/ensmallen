use super::*;

#[derive(Debug, Clone, PartialEq)]
pub struct Attribute(String);

impl CanParse for Attribute {
    fn can_parse(data: &[u8]) -> bool {
        data.starts_with(b"#")
    }
}

impl Parse for Attribute {
    fn parse(mut data: &[u8]) -> (&[u8], Self) {
        assert!(data.starts_with(b"#"));
        data = skip_whitespace(&data[1..]);
        let (mut data, attr_body) = get_next_matching(data, b'[', b']');
        data = skip_whitespace(data);
        (data, Attribute(String::from_utf8(attr_body.to_vec()).unwrap()))
    }
}

impl From<Attribute> for String {
    fn from(data: Attribute) -> String {
        data.0
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_attribute() {
        let data = "#[inline(always)]\nNEXTLINE".to_string();
        let ptr = data.as_bytes();
        let (reminder, res) = Attribute::parse(ptr);
        assert_eq!(reminder, "NEXTLINE".as_bytes());
        assert_eq!(String::from(res), "inline(always)");
    }
}