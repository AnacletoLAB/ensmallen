use super::*;

#[derive(Debug, Clone)]
pub struct Static {
    pub doc: String,
    pub name: String,
    pub static_type: Type,
    pub value: String,
    pub attributes: Vec<String>,
    pub visibility: Visibility,
}

impl Default for Static {
    fn default() -> Self {
        Static {
            doc: String::new(),
            name: String::new(),
            static_type: Type::default(),
            value: String::new(),
            attributes: Vec::new(),
            visibility: Visibility::Private,
        }
    }
}

impl CanParse for Static {
    fn can_parse(mut data: &[u8]) -> bool {
        let _visibility = parse!(data, Visibility);
        data.starts_with(b"static")
    }
}

impl Parse for Static {
    /// If the line starts with "use " parse everything until the cloumn.
    fn parse(mut data: &[u8]) -> (&[u8], Self) {
        let mut result = Static::default();
        result.visibility = parse!(data, Visibility);
        // skip "Static"
        data = &data[6..];

        // parse the name
        result.name = parse!(data, Identifier).into();
        
        assert_eq!(data[0], b':', "Static statement without column");
        data = &data[1..];

        result.static_type = parse!(data, Type);

        assert_eq!(data[0], b'=', "Static statement without equal sign");
        data = skip_whitespace(&data[1..]);

        while data[0] != b';' {
            result.value.push(next_char!(data) as char);
        }

        data = skip_whitespace(&data[1..]);

        (data, result)
    }
}