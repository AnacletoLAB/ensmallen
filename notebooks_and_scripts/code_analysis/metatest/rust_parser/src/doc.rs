use super::*;

#[derive(Debug, Clone, PartialEq)]
pub struct Doc{
    pub sections: Vec<DocSection>,
}

impl Parse for Doc {
    fn parse(mut data: &[u8]) -> (&[u8], Self) {
        let mut result = Vec::new();
        
        let mut section_title = b"Introduction".to_vec();
        let mut section_content = Vec::new();
        while !data.is_empty() {
            if data.starts_with(b"```") {
                let mut code_section = b"```".to_vec();
                // skip the ```
                data = &data[3..];

                while !data.is_empty() && !data.starts_with(b"```") {
                    code_section.push(data[0]);
                    data = &data[1..];
                }
                // skip the ```
                data = &data[3..];
                code_section.extend(b"```");

                section_content.extend(code_section);
            }


            if data[0] == b'#' {
                result.push(
                    DocSection::new(section_title, section_content)
                );
                section_title = Vec::new();
                section_content = Vec::new();
                // skip the #
                data = skip_whitespace(&data[1..]);
                while !data.is_empty() && data[0] != b'\n' {
                    section_title.push(data[0]);
                    data = &data[1..];
                }
            } else {
                section_content.push(data[0]);
                data = &data[1..];
            }
        }
        if !section_title.is_empty() {
            result.push(
                DocSection::new(section_title, section_content)
            );
        }

        (data, Doc{sections: result})
    }
}