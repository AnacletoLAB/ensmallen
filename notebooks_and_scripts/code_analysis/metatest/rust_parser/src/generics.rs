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

impl CmpWithoutModifiers for GenericValue {
    fn cmp_without_modifiers(&self, other: &GenericValue) -> bool {
        match (self, other) {
            (GenericValue::Type(t1), GenericValue::Type(t2)) => t1.cmp_without_modifiers(t2),
            (GenericValue::TypeAssignement(t1, t2), GenericValue::TypeAssignement(o1, o2)) => {
                t1.cmp_without_modifiers(o1) && t2.cmp_without_modifiers(o2)
            }
            _ => false,
        }
    }
}

impl PartialEq<&str> for GenericValue {
    fn eq(&self, other:&&str) -> bool {
        match self {
            GenericValue::Type(t) => t == other,
            GenericValue::TypeAssignement(t1, t2) => {
                match other.split_once("=") {
                    Some((v1, v2)) => {
                        t1 == v1 && t2 == v2
                    }
                    None => false
                }
            }
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

impl CmpWithoutModifiers for Generics {
    fn cmp_without_modifiers(&self, other:&Generics) -> bool {
        if self.0.len() != other.0.len() {
            return false;
        }

        for i in 0..self.0.len(){
            if !self.0[i].cmp_without_modifiers(&other.0[i]){
                return false; 
            }
        }

        true
    }
}