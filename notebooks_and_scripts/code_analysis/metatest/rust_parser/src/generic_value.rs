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
        data = skip_whitespace(data);

        if data.starts_with(b"'") {
            let res = GenericValue::Lifetime(parse!(data, Lifetime));
            return (data, res);
        }

        let start = parse!(data, Type);
        data = skip_whitespace(data);

        if data.starts_with(b":") {
            data = skip_whitespace(&data[1..]);
            let end = parse!(data, Type);
            return (data, GenericValue::TypeInheritance(start, end));
        }
        if data.starts_with(b"=") {
            data = skip_whitespace(&data[1..]);
            let end = parse!(data, Type);
            return (data, GenericValue::TypeAssignement(start, end));
            
        }

        (data, GenericValue::Type(start))
    }
}