use crate::InterpretError;

// Struct for iterating
struct EscapedString<'a> {
  s: std::str::Chars<'a>,
}

// Iterates over an escaped string and removes escapes
// This code is based off code from:
// https://stackoverflow.com/questions/58551211/how-do-i-interpret-escaped-characters-in-a-string
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
        Some('"') => Ok('"'),
        Some(c) => Err(InterpretError::new(
          format!("Unknown escape char `{}`", c).as_str(),
        )),
      },
      c => Ok(c),
    })
  }
}

// Processes a string and returns a Result with a String if everything is fine,
// Otherwise returns an InterpretError
pub fn process_string(str: &str) -> Result<String, InterpretError> {
  let s = EscapedString { s: str.chars() };
  s.collect()
}
