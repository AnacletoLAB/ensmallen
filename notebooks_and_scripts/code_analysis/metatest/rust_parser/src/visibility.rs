use super::*;

#[derive(Debug, Clone, PartialEq)]
pub enum Visibility{
    Private,
    Public,
    PublicCrate, 
    PublicSuper,
}

impl Parse for Visibility {
    fn parse(mut data: &[u8]) -> (&[u8], Self) {
        match data {
            _x if data.starts_with(b"pub(super)") => (&data[10..], Visibility::PublicSuper),
            _x if data.starts_with(b"pub(crate)") => (&data[10..], Visibility::PublicCrate),
            _x if data.starts_with(b"pub") => (&data[3..], Visibility::Public),
            _ => (data, Visibility::Private),
        }
    }
}
