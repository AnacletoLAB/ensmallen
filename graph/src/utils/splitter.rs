pub struct SplitUnquotedChar<'s> {
    line: &'s str,
    separator: char,
}

impl<'s> SplitUnquotedChar<'s> {
    pub fn new(line: &'s str, separator: char) -> Self {
        Self {
            line,
            separator,
        }
    }
}

impl<'s> Iterator for SplitUnquotedChar<'s> {
    type Item = &'s str;

    fn next(&mut self) -> Option<&'s str> {
        let mut char_indices = self.line.char_indices();
        if let Some((_, c0)) = char_indices.next() {
            let mut previous = c0;
            for (bi, c) in self.line.char_indices() {
                if c == self.separator {
                    if c0 == '"' {
                        if bi == 1 || previous != '"' {
                            previous = c;
                            continue;
                        }
                        // the first and last quotes aren't part of the
                        // returned token
                        let token = &self.line[1..bi - 1];
                        self.line = &self.line[bi..];
                        return Some(token);
                    }
                    let token = &self.line[..bi];
                    self.line = &self.line[bi..];
                    return Some(token);
                }
                previous = c;
            }
            let unwrap = c0 == '"' && previous == '"' && self.line.len() > 1;
            let token = if unwrap {
                &self.line[1..self.line.len() - 1]
            } else {
                self.line
            };
            self.line = &self.line[0..0];
            Some(token)
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
