use super::*;


#[derive(Debug, Clone, PartialEq)]
pub enum DocSection{
    Introduction(String),
    Example{
        prologue: String,
        code: String,
        epilogue: String,
    },
    Arguments{
        prologue: String,
        arguments: Vec<Argument>,
        epilogue: String,
    },
    Raises{
        prologue: String,
        exceptions: Vec<String>,
        epilogue: String,
    },
    References{
        text: String,
    },
    Unsafe{
        text: String,
    },
    CustomSection{
        title: String,
        content: String,
    }
}

impl DocSection {
    pub fn new(title: Vec<u8>, content: Vec<u8>) -> DocSection {
        let title   = String::from_utf8(title).unwrap();
        match title.as_str() {
            "Introduction" => {
                DocSection::Introduction(
                    bytes_to_string(
                        &content
                    ).split("\n")
                    .map(|x| trim_str(x.to_string()))
                    .collect::<Vec<_>>()
                    .join("\n")
                )
            }
            "Safety" => {
                DocSection::Unsafe{text:trim_str(bytes_to_string(&content))}
            }
            "References" => {
                DocSection::References{text:trim_str(bytes_to_string(&content))}
            }
            "Arguments" => {
                let mut data = &content[..];
                let mut prologue = Vec::new();
                let mut args = Vec::new();
                let mut epilogue = Vec::new();

                while !data.is_empty() && !data.starts_with(b"*") {
                    prologue.push(data[0]);
                    data = &data[1..];
                }
                
                while !data.is_empty() && data.starts_with(b"*") {
                    let mut arg = Vec::new();
                    while !data.is_empty() && data[0] != b'\n' {
                        arg.push(data[0]);
                        data = &data[1..];
                    }
                    data = skip_whitespace(&data[1..]);
                    args.push(Argument::new(arg));
                }

                while !data.is_empty(){
                    epilogue.push(data[0]);
                    data = &data[1..];
                }

                DocSection::Arguments{
                    prologue: bytes_to_string(trim(&prologue)),
                    arguments: args,
                    epilogue: bytes_to_string(trim(&epilogue)),
                }
            }
            "Example" => {
                let mut data = &content[..];
                let mut prologue = Vec::new();
                let mut code = Vec::new();
                let mut epilogue = Vec::new();

                while !data.is_empty() && !data.starts_with(b"```") {
                    prologue.push(data[0]);
                    data = &data[1..];
                }
            
                // skip the ```
                data = &data[3..];
                while !data.is_empty() && !data.starts_with(b"```") {
                    code.push(data[0]);
                    data = &data[1..];
                }
                // skip the ```
                data = &data[3..];

                while !data.is_empty(){
                    epilogue.push(data[0]);
                    data = &data[1..];
                }

                DocSection::Example{
                    prologue: bytes_to_string(trim(&prologue)),
                    code: bytes_to_string(trim(&code)),
                    epilogue: bytes_to_string(trim(&epilogue)),
                }
            }
            "Raises" => {
                let mut data = skip_whitespace(&content[..]);
                let mut prologue = Vec::new();
                let mut lines = Vec::new();
                let mut epilogue = Vec::new();

                while !data.is_empty() && !data.starts_with(b"*") {
                    prologue.push(data[0]);
                    data = &data[1..];
                }
                
                while !data.is_empty() && data.starts_with(b"*") {
                    // skip the *
                    data = skip_whitespace(&data[1..]);
                    let mut line = Vec::new();
                    while !data.is_empty() && data[0] != b'\n' {
                        line.push(data[0]);
                        data = &data[1..];
                    }
                    data = skip_whitespace(&data[1..]);
                    lines.push(bytes_to_string(&line));
                }

                while !data.is_empty(){
                    epilogue.push(data[0]);
                    data = &data[1..];
                }

                DocSection::Raises{
                    prologue: bytes_to_string(trim(&prologue)),
                    exceptions: lines,
                    epilogue: bytes_to_string(trim(&epilogue)),
                }
            }
            _ => {
                DocSection::CustomSection{
                    title: title,
                    content: bytes_to_string(&content),
                }
            }
        }
    }
}