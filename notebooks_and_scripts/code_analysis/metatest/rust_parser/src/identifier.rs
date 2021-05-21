use super::*;

const IDENTIFIER_ALPHABET: &[u8] = b"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789_";

#[derive(Debug, Clone)]
pub struct Identifier(String);

impl Parse for Identifier {
    fn parse(mut data: &[u8]) -> (&[u8], Self) {
        let mut result = String::new();

        while !data.is_empty() && IDENTIFIER_ALPHABET.contains(&data[0]) {
            result.push(next_char!(data) as char);
        }

        (data, Identifier(result))
    }
}

impl From<Identifier> for String {
    fn from(data: Identifier) -> String {
        data.0
    }
}