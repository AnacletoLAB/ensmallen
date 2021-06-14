

use super::*;

#[derive(Debug, Clone, PartialEq)]
pub struct Macro {
    pub doc: String,
    pub name: String,
    pub attributes: Vec<Attribute>,
    pub visibility: Visibility,
    pub content: String,
}

impl Default for Macro {
    fn default() -> Self {
        Macro {
            doc: String::new(),
            name: String::new(),
            attributes: Vec::new(),
            visibility: Visibility::Private,
            content: String::new(),
        }
    }
}

impl CanParse for Macro {
    fn can_parse(mut data: &[u8]) -> bool {
        let _visibility = parse!(data, Visibility);
        data.starts_with(b"macro_rules!")
    }
}

impl Parse for Macro {
    /// If the line starts with "use " parse everything until the cloumn.
    fn parse(mut data: &[u8]) -> (&[u8], Self) {
        let mut result = Macro::default();
        result.visibility = parse!(data, Visibility);

        // skip "macro_rules"
        data = &data[12..];

        // parse the name
        result.name = parse!(data, Identifier).into();

        data = skip_whitespace(data);
        
        let (mut data, mut macro_body) = get_next_matching(data, b'{', b'}');

        result.content = bytes_to_string(macro_body);

        data = skip_whitespace(data);
        (data, result)
    }
}



#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_macro() {
        let data = r#"
macro_rules! next_char {
    ($data:ident) => {{
        let result = $data[0];
        $data = &$data[1..];
        result
    }};
}

pub(crate) use std::fs;"#.to_string();
        let ptr = data.as_bytes();
        let (reminder, res) = Macro::parse(ptr);
        assert_eq!(reminder, "pub(crate) use std::fs;".as_bytes());
    }

}