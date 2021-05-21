use super::*;

#[derive(Debug, Clone)]
pub struct DocLine(pub String);

impl CanParse for DocLine {
    fn can_parse(data: &[u8]) -> bool {
        data.starts_with(b"///")
    }
}

impl Parse for DocLine {
    fn parse(data: &[u8]) -> (&[u8], Self) {
        let (data, doc_line) = split_at(data, b'\n');
        let doc_line = &doc_line[3..];
        let res = String::from_utf8(doc_line.to_vec()).unwrap();
        (data, DocLine(res))
    }
}

impl From<DocLine> for String {
    fn from(data: DocLine) -> String {
        data.0
    }
}

#[derive(Debug, Clone)]
pub struct ModuleDocLine(pub String);

impl CanParse for ModuleDocLine {
    fn can_parse(data: &[u8]) -> bool {
        data.starts_with(b"//!")
    }
}

impl Parse for ModuleDocLine {
    fn parse(data: &[u8]) -> (&[u8], Self) {
        let (data, doc_line) = split_at(data, b'\n');
        let doc_line = &doc_line[3..];
        let res = String::from_utf8(doc_line.to_vec()).unwrap();
        (data, ModuleDocLine(res))
    }
}

impl From<ModuleDocLine> for String {
    fn from(data: ModuleDocLine) -> String {
        data.0
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_doc_line() {
        let data = "/// TEST DOC\nNEXTLINE".to_string();
        let ptr = data.as_bytes();
        let (reminder, res) = DocLine::parse(ptr);
        assert_eq!(reminder, "NEXTLINE".as_bytes());
        assert_eq!(String::from(res), " TEST DOC");
    }
}