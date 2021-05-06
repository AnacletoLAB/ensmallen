use super::*;
use super::*;

#[derive(Debug, Clone, PartialEq)]
pub enum GenericValue{
    Type(Type),
    Lifetime(Lifetime),
    TypeAssignement(Type, Type),
    TypeInheritance(Type, Type),
}

impl Parse for GenericValue {
    fn parse(mut data: &[u8]) -> (&[u8], Self) {
        if data.starts_with(b"'") {
            let res = GenericValue::Lifetime(parse!(data, Lifetime));
            return (data, res);
        }

        let start = parse!(data, Type);

        if data.starts_with(b":") {
            data = &data[1..];
            let end = parse!(data, Type);
            return (data, GenericValue::TypeInheritance(start, end));
        }
        if data.starts_with(b"=") {
            data = &data[1..];
            let end = parse!(data, Type);
            return (data, GenericValue::TypeAssignement(start, end));
        }

        (data, GenericValue::Type(start))
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Generics(pub Vec<GenericValue>);

impl CanParse for Generics {
    fn can_parse(mut data: &[u8]) -> bool {
        data = skip_whitespace(data);
        data.starts_with(b"<")
    }
}

impl Parse for Generics {
    fn parse(mut data: &[u8]) -> (&[u8], Self) {
        let mut generics = Vec::new();
        let (remainder, mut matching) = get_next_matching(data, b'<', b'>');
        data = remainder;

        while !matching.is_empty() {
            generics.push(parse!(matching, GenericValue));
            // skip the comma
            if matching.starts_with(b",") {
                matching = &matching[1..];
            }
        }
        (data, Generics(generics))
    }
}

impl Default for Generics {
    fn default() -> Self {
        Generics(Vec::new())
    }
}