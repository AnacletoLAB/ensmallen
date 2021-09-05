use super::*;

#[derive(Debug, Clone, PartialEq)]
pub struct MacroCall {
    pub doc: String,
    pub name: String,
    pub attributes: Vec<Attribute>,
    pub content: String,
}

impl Default for MacroCall {
    fn default() -> Self {
        MacroCall {
            doc: String::new(),
            name: String::new(),
            attributes: Vec::new(),
            content: String::new(),
        }
    }
}

impl CanParse for MacroCall {
    fn can_parse(mut data: &[u8]) -> bool {
        let _name = parse!(data, Identifier);
        data.starts_with(b"!")
    }
}

impl Parse for MacroCall {
    /// If the line starts with "use " parse everything until the cloumn.
    fn parse(mut data: &[u8]) -> (&[u8], Self) {
        let mut result = MacroCall::default();

        // parse the name
        result.name = parse!(data, Identifier).into();

        // skip "!"
        data = &data[1..];
 
        data = skip_whitespace(data);
        
        let (mut data, macro_body) = match data[0] {
            b'{' => {
                get_next_matching(data, b'{', b'}')
            },
            b'[' => {
                get_next_matching(data, b'[', b']')
            },
            b'(' => {
                get_next_matching(data, b'(', b')')
            },
            _ => {
                panic!("macro call without parenthesis!");
            }
        };

        result.content = bytes_to_string(macro_body);

        data = skip_whitespace(data);

        if data[0] == b';' {
            data = &data[1..];
        }

        (data, result)
    }
}



#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_macro() {
        let data = "implement_my_method!{a}\npub(crate) use std::fs;".to_string();
        let ptr = data.as_bytes();
        let (reminder, res) = MacroCall::parse(ptr);
        assert_eq!(reminder, "pub(crate) use std::fs;".as_bytes());
    }

}