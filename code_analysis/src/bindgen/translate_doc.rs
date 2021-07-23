use super::*;

pub fn translate_doc(doc: String) -> String {
    let mut result = String::new();

    // parse the documentation into sections
    let (_, doc) = Doc::parse(doc.as_bytes());
    let sections = doc.sections;

    for section in sections {
        match section {
            DocSection::Introduction(intro) => {
                result.extend(bytes_to_string(trim(intro.as_bytes())).chars());
            }
            DocSection::Arguments {
                arguments,
                ..
            } => {
                result.extend("\n\nParameters\n----------\n".chars());

                //args_sec.extend(prologue.chars());

                for argument in arguments {
                    match argument {
                        Argument::Parsable(DocArg {
                            name,
                            arg_type,
                            description,
                        }) => result.extend(
                            format!(
                                "{name}: {arg_type},\n    {description}\n",
                                name = name,
                                arg_type = translate_type_str(arg_type),
                                description = description,
                            )
                            .chars(),
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
                result.extend("\n\nRaises\n-------\n".chars());

                for excp in exceptions {
                    result.extend(format!("ValueError\n    {}\n", excp).chars());
                }
            }
            DocSection::Unsafe { text } => {
                result.extend("\n\nSafety\n------\n".chars());
                result.extend(text.chars());
            }
            _ => {}
        }
    }

    result
        .split("\n")
        .map(|x| format!("    /// {}", x))
        .collect::<Vec<_>>()
        .join("\n")
}