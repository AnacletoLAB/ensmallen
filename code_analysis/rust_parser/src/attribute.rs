use super::*;

#[derive(Debug, Clone)]
pub struct Attribute(pub String);

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

impl PartialEq for Attribute {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl PartialEq<&str> for Attribute {
    fn eq(&self, other: &&str) -> bool {
        &self.0.as_str() == other
    }
}

impl PartialEq<str> for Attribute {
    fn eq(&self, other: &str) -> bool {
        self.0.as_str() == other
    }
}

impl Attribute {
    /// Parse attribute for fuzz_type
    /// The goal of this attribute is to limit the range of arguments when
    /// the meta-test harness is generated.
    /// This way we can avoid timeouts and Out of memory errors which are 
    ///  an user responsability.
    ///
    /// The intended syntax is:
    /// ```ignore
    /// #[fuzz_type(iteration: Option<u8>)]
    /// fn test(iterations: Option<usize>) {
    ///    
    /// }
    /// ```
    ///
    pub fn parse_fuzz_type(&self) -> Option<(String, Type)> {
        if !self.0.starts_with("fuzz_type") {
            return None;
        }
        let mut data = skip_whitespace(&self.0.as_bytes()[10..]);

        let arg_name: String = parse!(data, Identifier).into();
        
        assert_eq!(data[0], b':', "Fuzz type statement without the column :");
        data = skip_whitespace(&data[1..]);

        let arg_type = parse!(data, Type);
        let _  = data;

        Some((arg_name, arg_type))

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

    #[test]
    fn test_fuzz_type_attribute() {
        let data = "#[fuzz_type(iterations: Option<u8>)]\nNEXTLINE".to_string();
        let ptr = data.as_bytes();
        let (reminder, res) = Attribute::parse(ptr);
        assert_eq!(reminder, "NEXTLINE".as_bytes());
        assert_eq!(String::from(res.clone()), "fuzz_type(iterations: Option<u8>)");

        match res.parse_fuzz_type() {
            None => panic!("The parse_fuzz_type is none. This is supposed to be Some."),
            Some((arg_name, arg_type)) => {
                assert_eq!(arg_name, "iterations");
                assert_eq!(arg_type, "Option<u8>");
            } 
        }
    }
}