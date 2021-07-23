use super::*;

pub fn translate_doc(doc: &str) -> String {
    let mut result = String::new();

    // parse the documentation into sections
    let (_, doc) = Doc::parse(doc.as_bytes());
    let sections = doc.sections;

    for section in sections {
        match section {
            DocSection::Introduction(intro) => {
                result.push_str(&bytes_to_string(trim(intro.as_bytes())));
            }
            DocSection::Arguments {
                arguments,
                ..
            } => {
                result.push_str("\n\nParameters\n----------\n");

                //args_sec.extend(prologue.chars());

                for argument in arguments {
                    match argument {
                        Argument::Parsable(DocArg {
                            name,
                            arg_type,
                            description,
                        }) => result.push_str(
                            &format!(
                                "{name}: {arg_type},\n    {description}\n",
                                name = name,
                                arg_type = translate_type_str(arg_type),
                                description = description,
                            )
                        ),
                        Argument::NotParsable(_) => {}
                    }
                }

                //args_sec.extend(epilogue.chars());
            }
            DocSection::Raises {
                exceptions,
                ..
            } => {
                result.push_str("\n\nRaises\n-------\n");

                for excp in exceptions {
                    result.push_str(&format!("ValueError\n    {}\n", excp));
                }
            }
            DocSection::Unsafe { text } => {
                result.push_str("\n\nSafety\n------\n");
                result.push_str(&text);
            }
            _ => {}
        }
    }

    result
        .split('\n')
        .map(|x| format!("    /// {}", x))
        .collect::<Vec<_>>()
        .join("\n")
}