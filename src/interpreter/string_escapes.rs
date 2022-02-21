use crate::InterpretError;

struct EscapedString<'a> {
    s: std::str::Chars<'a>,
}

impl<'a> Iterator for EscapedString<'a> {
    type Item = Result<char, InterpretError>;

    fn next(&mut self) -> Option<Self::Item> {
        self.s.next().map(|c| match c {
            '\\' => match self.s.next() {
                None => Err(InterpretError::new("Escape char at end of str.")),
                Some('n') => Ok('\n'),
                Some('\\') => Ok('\\'),
                Some('{') => Ok('{'),
                Some('}') => Ok('}'),
                // etc.
                Some(c) => Err(InterpretError::new(
                    format!("Unknown escape char {}", c).as_str(),
                )),
            },
            c => Ok(c),
        })
    }
}

pub fn process_string(str: &str) -> Result<String, InterpretError> {
    let s = EscapedString { s: str.chars() };
    s.collect()
}
