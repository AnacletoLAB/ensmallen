use super::*;

#[derive(Debug, Clone, PartialEq)]
pub struct Use(pub String);

impl CanParse for Use{
    fn can_parse(data: &[u8]) -> bool {
        data.starts_with(b"use")
    }
}

impl Parse for Use {
    /// If the line starts with "use " parse everything until the cloumn.
    fn parse(mut data: &[u8]) -> (&[u8], Self) {
        let mut result = String::new();
        // skip "use " and the whitespace
        data = skip_whitespace(&data[4..]);
    
        while data[0] != b';' {
            result.push(next_char!(data) as char);
        }

        // Skip the column
        data = skip_whitespace(&data[1..]);

        (data, Use(result))
    }
}

impl From<Use> for String {
    fn from(data: Use) -> String {
        data.0
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