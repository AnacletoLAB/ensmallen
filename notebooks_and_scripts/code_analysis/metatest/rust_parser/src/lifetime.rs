use super::*;

#[derive(Debug, Clone, PartialEq)]
pub struct Lifetime(pub String);

impl Parse for Lifetime {
    fn parse(mut data: &[u8]) -> (&[u8], Self) {
        data = &data[1..];
        let identifier = parse!(data, Identifier);
        (
            data, 
            Lifetime(String::from(identifier))
        )
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_lifetime() {
        let data = "'lifetime".to_string();
        let ptr = data.as_bytes();
        let (data, lt) = Lifetime::parse(ptr);
        assert_eq!(lt.0, "lifetime");
    }

    #[test]
    fn test_anonymous_lifetime() {
        let data = "'_".to_string();
        let ptr = data.as_bytes();
        let (data, lt) = Lifetime::parse(ptr);
        assert_eq!(lt.0, "_");
    }
}

