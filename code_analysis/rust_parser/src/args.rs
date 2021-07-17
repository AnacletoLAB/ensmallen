use super::*;

#[derive(Debug, Clone, PartialEq)]
pub struct Arg{
    pub name: String,
    pub arg_modifier: TypeModifiers,
    pub arg_type: Type,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Args(pub Vec<Arg>);

impl Default for Args {
    fn default() -> Self {
        Args(Vec::new())
    }
}

impl Parse for Args {
    fn parse(data: &[u8]) -> (&[u8], Self) {
        let (data, mut args_content) = get_next_matching(data, b'(', b')');
        let mut result = Vec::new();
        loop {
            args_content = skip_whitespace(args_content);
            if args_content.starts_with(b",") {
                args_content = skip_whitespace(&args_content[1..]);
            }

            if args_content.is_empty() {
                break;
            }

            let modifier = parse!(args_content, TypeModifiers);
            let name: String = parse!(args_content, Identifier).into();

            if name == "self" {
                result.push(Arg{
                    name: "self".to_string(),
                    arg_modifier: modifier,
                    arg_type: Type::SelfType,
                });
                continue;
            }
            
            args_content = skip_whitespace(args_content);
            assert!(args_content.starts_with(b":"), "Missing column in arg definition at '{}'", String::from_utf8(args_content.to_vec()).unwrap());
            args_content = skip_whitespace(&args_content[1..]);
            
            let arg_type = parse!(args_content, Type);
            result.push(Arg{
                name: name,
                arg_modifier: modifier,
                arg_type: arg_type,
            });
        }

        (data, Args(result))
    }
}
