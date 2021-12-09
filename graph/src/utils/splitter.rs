pub struct SplitUnquotedChar<'s> {
    line: &'s str,
    separator: char,
}

impl<'s> SplitUnquotedChar<'s> {
    pub fn new(line: &'s str, separator: char) -> Self {
        Self { line, separator }
    }
}

/// parse a line of a csv spearated by `separator` which
/// can has quotes and escaped quotes
impl<'s> Iterator for SplitUnquotedChar<'s> {
    type Item = &'s str;

    fn next(&mut self) -> Option<&'s str> {
        let line_start = self.line;
        let mut counter = 0;
        let mut inside_double_quotes = false;
        let mut previous_char_is_backslash = false;
        while let Some(current_char) = self.line.chars().next() {
            match current_char {
                '\\' => {
                    previous_char_is_backslash ^= true;
                }
                '"' => {
                    if !previous_char_is_backslash {
                        inside_double_quotes ^= true;
                    }
                    previous_char_is_backslash = false;
                }
                x if x == self.separator => {
                    // if we are not escaped and we are not inside quotes
                    // return the current val
                    if !previous_char_is_backslash && !inside_double_quotes {
                        // skip the separator
                        self.line = &self.line[1..];
                        // reutrn the current line
                        return Some(&line_start[..counter]);
                    }
                    previous_char_is_backslash = false;
                }
                _ => {
                    previous_char_is_backslash = false;
                }
            }
            // skip one character
            let char_size = current_char.len_utf8();
            self.line = &self.line[char_size..];
            counter += char_size;
        }

        // if the line is empty then we are finished
        if !line_start.is_empty() {
            Some(line_start)
        } else {
            None
        }
    }
}

/// Returns iterator over given line to split.
///
/// # Arguments
/// `line`: &str - The line to split.
/// `separator`: char - The separator to use to split the lines.
/// `support_balanced_quotes`: bool - Whether to support balanced quotes.
pub(crate) fn splitter<'a>(
    line: &'a str,
    separator: char,
    support_balanced_quotes: bool,
) -> impl Iterator<Item = &'a str> {
    let iterator: Box<dyn Iterator<Item = &'a str>> = if support_balanced_quotes {
        Box::new(SplitUnquotedChar::new(line, separator).into_iter())
    } else {
        Box::new(line.split(separator).into_iter())
    };
    iterator
}
