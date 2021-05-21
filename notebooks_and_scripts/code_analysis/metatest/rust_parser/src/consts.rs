use super::*;

#[derive(Debug, Clone, PartialEq)]
pub struct Const {
    pub doc: String,
    pub name: String,
    pub const_type: Type,
    pub value: String,
    pub attributes: Vec<String>,
    pub visibility: Visibility,
}

impl Default for Const {
    fn default() -> Self {
        Const {
            doc: String::new(),
            name: String::new(),
            const_type: Type::default(),
            value: String::new(),
            attributes: Vec::new(),
            visibility: Visibility::Private,
        }
    }
}

impl CanParse for Const {
    fn can_parse(mut data: &[u8]) -> bool {
        let _visibility = parse!(data, Visibility);
        data.starts_with(b"const")
    }
}

impl Parse for Const {
    /// If the line starts with "use " parse everything until the cloumn.
    fn parse(mut data: &[u8]) -> (&[u8], Self) {
        let mut result = Const::default();
        result.visibility = parse!(data, Visibility);
        // skip "const"
        data = &data[6..];

        // parse the name
        result.name = parse!(data, Identifier).into();
        
        assert_eq!(data[0], b':', "Const statement without column");
        data = &data[1..];

        result.const_type = parse!(data, Type);

        assert_eq!(data[0], b'=', "Const statement without equal sign");
        data = skip_whitespace(&data[1..]);

        while data[0] != b';' {
            result.value.push(next_char!(data) as char);
        }

        data = skip_whitespace(&data[1..]);

        (data, result)
    }
}