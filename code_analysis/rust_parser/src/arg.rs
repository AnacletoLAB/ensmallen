use super::*;
use regex::Regex;

pub const ARGUMENT_REGEX: &str = r"\* `([^`]+)`: ([^-\n]+) - (.+)";

#[derive(Debug, Clone, PartialEq)]
pub struct DocArg {
    pub name: String,
    pub arg_type: String,
    pub description: String,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Argument {
    Parsable(DocArg),
    NotParsable(String),
}

impl Argument {
    pub fn new(data: Vec<u8>) -> Argument {
        let re = Regex::new(ARGUMENT_REGEX).unwrap();
        let data = bytes_to_string(&data);

        if re.is_match(&data) {
            let caps = re.captures(&data).unwrap();
            return Argument::Parsable(DocArg{
                name: caps.get(1).unwrap().as_str().to_string(),
                arg_type: caps.get(2).unwrap().as_str().to_string(),
                description: caps.get(3).unwrap().as_str().to_string(),
            });
        }

        Argument::NotParsable(data)
    }
}