use super::*;

#[derive(Debug, Clone, PartialEq)]
pub struct Use {
    pub visibility: Visibility,
    pub doc: String,
    pub attributes: Vec<Attribute>,
    pub content: String,
}

impl Default for Use {
    fn default() -> Self {
        Use {
            doc: String::new(),
            attributes: Vec::new(),
            visibility: Visibility::Private,
            content: String::new(),
        }
    }
}

impl CanParse for Use{
    fn can_parse(data: &[u8]) -> bool {
        data.starts_with(b"use")
    }
}

impl Parse for Use {
    /// If the line starts with "use " parse everything until the cloumn.
    fn parse(mut data: &[u8]) -> (&[u8], Self) {
        let mut content = String::new();
        // skip "use " and the whitespace
        data = skip_whitespace(&data[4..]);
    
        while data[0] != b';' {
            content.push(next_char!(data) as char);
        }

        // Skip the column
        data = skip_whitespace(&data[1..]);

        let mut result = Use::default();
        result.content = content;
        (data, result)
    }
}

impl From<Use> for String {
    fn from(data: Use) -> String {
        // TODO! update viz to include visibility and other sutff
        data.content
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_use() {
        let data = "use test::{this, is::a::{use, statement}};\nNEXTLINE".to_string();
        let ptr = data.as_bytes();
        let (reminder, res) = Use::parse(ptr);
        assert_eq!(reminder, "NEXTLINE".as_bytes());
        assert_eq!(String::from(res), "test::{this, is::a::{use, statement}}");
    }
}