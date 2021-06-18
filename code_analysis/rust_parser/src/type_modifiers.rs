use super::*;

#[derive(Debug, Clone, PartialEq)]
pub struct TypeModifiers {
    pub pointer: bool,
    pub reference: bool,
    pub mutable: bool,
    pub constant: bool,
    pub lifetime: Option<Lifetime>,
}

impl Default for TypeModifiers {
    fn default() -> Self {
        TypeModifiers{
            pointer: false,
            reference: false,
            mutable: false,
            constant: false,
            lifetime: None,
        }
    }
}

impl std::fmt::Display for TypeModifiers {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

impl TypeModifiers {
    fn to_string(&self) -> String {
        String::from(self.clone())
    }
}

impl Parse for TypeModifiers {
    fn parse(mut data: &[u8]) -> (&[u8], Self) {
        let mut mutable = false;
        let mut pointer = false;
        let mut constant = false;
        let mut reference = false;
        let mut lifetime = None;

        if data.starts_with(b"mut") {
            mutable = true;
            data = skip_whitespace(&data[4..]);
        }

        if data.starts_with(b"&") {
            reference = true;
            data = skip_whitespace(&data[1..]);
        }

        if data.starts_with(b"*") {
            pointer = true;
            data = skip_whitespace(&data[1..]);
        }

        if data.starts_with(b"mut") {
            mutable = true;
            data = skip_whitespace(&data[4..]);
        }

        if data.starts_with(b"const") {
            constant = true;
            data = skip_whitespace(&data[5..]);
        }

        if data.starts_with(b"'") {
            lifetime = Some(parse!(data, Lifetime));
        }

        if data.starts_with(b"mut") {
            mutable = true;
            data = skip_whitespace(&data[4..]);
        }

        data = skip_whitespace(data);
        (
            data, 
            TypeModifiers{
                reference,
                pointer,
                constant,
                mutable,
                lifetime,
            }
        )
    }
}

impl From<TypeModifiers> for String {
    fn from(x: TypeModifiers) -> String{
        if x.reference {
            let mut result = "&".to_string();
            if x.mutable {
                result.push_str(" mut ");
            }
            if let Some(lt) = x.lifetime {
                result.push_str("'");
                result.push_str(&lt.0);
                result.push_str(" ");
            }
            return result;
        }
        if x.pointer {
            let mut result = "*".to_string();
            if x.mutable {
                result.push_str(" mut ");
            }
            if x.constant {
                result.push_str(" const ");
            }
            if let Some(lt) = x.lifetime {
                result.push_str(" '");
                result.push_str(&lt.0);
                result.push_str(" ");
            }
            return result;
        }

        let mut result = "".to_string();
        if x.mutable{
            result.push_str(" mut ");
        }
        result
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_type_modifier() {
        let data = "& mut 'a u64".to_string();
        let ptr = data.as_bytes();
        let (data, modif) = TypeModifiers::parse(ptr);
        assert_eq!(modif.pointer, false);
        assert_eq!(modif.reference, true);
        assert_eq!(modif.mutable, true);
        assert_eq!(modif.constant, false);
        assert!(modif.lifetime.is_some());  
        assert_eq!(modif.lifetime.unwrap().0, "a");
        assert_eq!(data, "u64".as_bytes());
    }

    #[test]
    fn test_type_modifier2() {
        let data = "mut u64".to_string();
        let ptr = data.as_bytes();
        let (data, modif) = TypeModifiers::parse(ptr);
        assert_eq!(modif.pointer, false);
        assert_eq!(modif.reference, false);
        assert_eq!(modif.mutable, true);  
        assert_eq!(modif.constant, false);
        assert_eq!(modif.lifetime, None);
        assert_eq!(data, "u64".as_bytes());
    }

    #[test]
    fn test_type_modifier3() {
        let data = "& 'a u64".to_string();
        let ptr = data.as_bytes();
        let (data, modif) = TypeModifiers::parse(ptr);
        assert_eq!(modif.pointer, false);
        assert_eq!(modif.reference, true);
        assert_eq!(modif.mutable, false);
        assert_eq!(modif.constant, false);
        assert!(modif.lifetime.is_some());  
        assert_eq!(modif.lifetime.unwrap().0, "a");
        assert_eq!(data, "u64".as_bytes());
    }
    
    #[test]
    fn test_type_modifier4() {
        let data = "& mut u64".to_string();
        let ptr = data.as_bytes();
        let (data, modif) = TypeModifiers::parse(ptr);
        assert_eq!(modif.pointer, false);
        assert_eq!(modif.reference, true);
        assert_eq!(modif.mutable, true);
        assert_eq!(modif.constant, false);
        assert_eq!(modif.lifetime, None);
        assert_eq!(data, "u64".as_bytes());
    }

    #[test]
    fn test_type_modifier5() {
        let data = "u64".to_string();
        let ptr = data.as_bytes();
        let (data, modif) = TypeModifiers::parse(ptr);
        assert_eq!(modif.pointer, false);
        assert_eq!(modif.reference, false);
        assert_eq!(modif.constant, false);
        assert_eq!(modif.mutable, false);
        assert_eq!(modif.lifetime, None);
        assert_eq!(data, "u64".as_bytes());
    }

    #[test]
    fn test_type_modifier6() {
        let data = "* mut u64".to_string();
        let ptr = data.as_bytes();
        let (data, modif) = TypeModifiers::parse(ptr);
        assert_eq!(modif.pointer, true);
        assert_eq!(modif.reference, false);
        assert_eq!(modif.constant, false);
        assert_eq!(modif.mutable, true);
        assert_eq!(modif.lifetime, None);
        assert_eq!(data, "u64".as_bytes());
    }

    #[test]
    fn test_type_modifier7() {
        let data = "* const u64".to_string();
        let ptr = data.as_bytes();
        let (data, modif) = TypeModifiers::parse(ptr);
        assert_eq!(modif.pointer, true);
        assert_eq!(modif.reference, false);
        assert_eq!(modif.mutable, false);
        assert_eq!(modif.constant, true);
        assert_eq!(modif.lifetime, None);
        assert_eq!(data, "u64".as_bytes());
    }
}