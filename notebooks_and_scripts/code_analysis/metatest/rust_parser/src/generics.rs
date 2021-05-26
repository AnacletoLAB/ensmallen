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

impl PartialEq<&str> for GenericValue {
    fn eq(&self, other:&&str) -> bool {
        match self {
            GenericValue::Type(t) => t == other,
            _ => false,
        }
    }
}

impl From<GenericValue> for String {
    fn from(x: GenericValue) -> String {
        match x {
            GenericValue::Lifetime(lt) => {
                format!("'{}", lt.0)
            }
            GenericValue::Type(t) => {
                String::from(t)
            }
            GenericValue::TypeAssignement(t1, t2) => {
                format!("{} = {}", String::from(t1), String::from(t2))
            }
            GenericValue::TypeInheritance(t1, t2) => {
                format!("{} : {}", String::from(t1), String::from(t2))
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Generics(pub Vec<GenericValue>);

impl std::ops::Index<usize> for Generics {
    type Output = GenericValue;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

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

impl From<Generics> for String {
    fn from(x: Generics) -> String {
        if x.0.is_empty(){
            return String::new();
        }
        let mut result = "<".to_string();

        for gen_val in x.0 {
            result.push_str(&String::from(gen_val));
            result.push_str(", ");
        }
        result = result.trim_end_matches(&", ").to_string();
        result.push('>');
        result
    }
}