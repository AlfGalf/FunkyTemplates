extern crate core;
#[macro_use]
extern crate lalrpop_util;

use std::cmp;
use std::collections::HashMap;
use std::fmt::{Debug, Formatter};

use itertools::Itertools;
use lalrpop_util::ParseError;

use crate::ast::{ParserState, Program};
use crate::data_types::{InterpretError, InterpretVal};
use crate::external_operators::{
  CustomBinOp, CustomBuiltIn, CustomType, CustomUnaryOp, OperatorChars,
};
use crate::interpreter::{interpret, Customs};
use crate::parser::language_definition::ProgramParser;

mod ast;
mod data_types;
mod interpreter;
mod parser;
mod test;

pub mod external_operators;

/// Represents a language to be parsed
pub struct Language<C: CustomType> {
  unary_operators: HashMap<OperatorChars, CustomUnaryOp<C>>,
  binary_operators: HashMap<OperatorChars, CustomBinOp<C>>,
  built_ins: HashMap<String, CustomBuiltIn<C>>,
}

/// Represents a set of template functions
#[derive(Debug)]
pub struct Script<C: CustomType> {
  lang: String,
  temp: Program,
  unary_operators: HashMap<OperatorChars, CustomUnaryOp<C>>,
  binary_operators: HashMap<OperatorChars, CustomBinOp<C>>,
  built_ins: HashMap<String, CustomBuiltIn<C>>,
}

/// Represents an argument being parsed in to a function call
pub enum Argument<C: CustomType> {
  /// Basic integer type
  Int(i32),
  /// Basic String type
  String(String),
  /// Tuple type
  Tuple(Vec<Argument<C>>),
  /// List type
  List(Vec<Argument<C>>),
  /// Custom data types.
  /// If multiple data types are required use aan enum type for the custom type.
  Custom(C),
}

impl<C: CustomType> Clone for Argument<C> {
  fn clone(&self) -> Self {
    match &self {
      Argument::Int(i) => Argument::Int(*i),
      Argument::String(s) => Argument::String(s.clone()),
      Argument::Tuple(t) => Argument::Tuple(t.clone()),
      Argument::List(t) => Argument::List(t.clone()),
      Argument::Custom(c) => Argument::Custom(c.clone()),
    }
  }
}

/// Represents a function from a template
/// Can contain an argument also
pub struct LangFunc<'a, C: CustomType> {
  lang: &'a Script<C>,
  name: String,
  arg: Option<Argument<C>>,
  text: String,
}

impl<C: CustomType> Default for Language<C> {
  fn default() -> Self {
    Self::new()
  }
}

impl<C: CustomType> Language<C> {
  pub fn new() -> Self {
    Self {
      unary_operators: Default::default(),
      binary_operators: Default::default(),
      built_ins: Default::default(),
    }
  }

  /// Adds a custom binary operator to the Language
  pub fn add_bin_op(&mut self, char: OperatorChars, op: CustomBinOp<C>) -> &Self {
    self.binary_operators.entry(char).or_insert(op);
    self
  }

  /// Adds a custom unary operator to the Language
  pub fn add_unary_op(&mut self, char: OperatorChars, op: CustomUnaryOp<C>) -> &Self {
    self.unary_operators.entry(char).or_insert(op);
    self
  }

  /// Adds a custom builtin function to the Language
  pub fn add_custom_function(&mut self, name: String, func: CustomBuiltIn<C>) -> &Self {
    self.built_ins.entry(name).or_insert(func);
    self
  }

  /// Parses a set of code into a template
  pub fn parse(&self, code: String) -> Result<Script<C>, LanguageErr> {
    let parser = ProgramParser::new();
    let parser_state = ParserState {
      unary_ops: self.unary_operators.keys().cloned().collect(),
      binary_ops: self.binary_operators.keys().cloned().collect(),
    };
    let res: Result<Program, ParseError<usize, _, (usize, String, usize)>> =
      parser.parse(&parser_state, &code);

    match res {
      Ok(l) => Ok(Script {
        temp: l,
        lang: code,
        unary_operators: self.unary_operators.clone(),
        binary_operators: self.binary_operators.clone(),
        built_ins: self.built_ins.clone(),
      }),
      Err(e) => Err(LanguageErr::new_from_parser_err(
        e.map_token(|_| "".to_string()),
        code,
      )),
    }
  }
}

impl Script<BlankCustom> {
  /// Builds a language from a code string
  ///
  /// ## Example
  /// ```
  /// use funki_lang::{Script, BlankCustom};
  /// let x = Script::<BlankCustom>::from_text("#main x -> x + 1;");
  /// ```
  pub fn from_text(lang: &str) -> Result<Self, LanguageErr> {
    let parser: ProgramParser = ProgramParser::new();
    let res = parser.parse(&ParserState::new(), lang);
    match res {
      Ok(l) => Ok(Self {
        temp: l,
        lang: lang.to_string(),
        unary_operators: Default::default(),
        binary_operators: Default::default(),
        built_ins: Default::default(),
      }),
      Err(e) => {
        // println!("{}", e);
        Err(LanguageErr::new_from_parser_err(
          e.map_token(|_| "".to_string()),
          lang.to_string(),
        ))
      }
    }
  }
}

impl<C: CustomType> Script<C> {
  /// Lists the available functions
  ///
  /// ## Example
  /// ```
  /// use funki_lang::{Script, BlankCustom};
  /// let x = Script::<BlankCustom>::from_text("#main x -> x + 1;").unwrap();
  /// x.list(); // -> ["main"]
  /// ```
  pub fn list(&self) -> Vec<String> {
    return self.temp.env.keys().map(|s| s.to_string()).collect();
  }

  /// Selects a function from the ParsedTemplatete
  ///
  /// ## ExamParsedTemplate/// ```
  /// use funki_templates::{Script, BlankCustom};
  /// let x = Script::<BlankCustom>::from_text("#main x -> x + 1;").unwrap();
  /// let f = x.function("main");
  /// ```
  pub fn function(&self, name: &str) -> Result<LangFunc<C>, LanguageErr> {
    if self.temp.env.contains_key(name) {
      Ok(LangFunc {
        lang: self,
        name: name.to_string(),
        arg: None,
        text: self.lang.clone(),
      })
    } else {
      Err(LanguageErr::new_no_loc(format!(
        "Cannot find function \"{}\".",
        name
      )))
    }
  }
}

/// Type for the values returned from the interpretation
pub enum ReturnVal<T: CustomType> {
  String(String),
  Int(i32),
  Bool(bool),
  Tuple(Vec<ReturnVal<T>>),
  List(Vec<ReturnVal<T>>),
  Custom(T),
}

impl<'a, C: CustomType> LangFunc<'a, C> {
  /// Adds an argument for aParsedTemplateion call
  ///
  /// #ParsedTemplateple
  /// ```
  /// use funki_lang::{Argument, Script, BlankCustom};
  /// let x = Script::<BlankCustom>::from_text("#main x -> x + 4;").unwrap();
  /// let f = x.function("main").unwrap();
  /// let f = f.arg(Argument::Int(5));
  /// f.call().unwrap(); // -> ReturnVal::Int(9)
  /// ```
  pub fn arg(mut self, arg: Argument<C>) -> Self {
    self.arg = Some(arg);
    self
  }
  /// Interprets this function
  /// Can return a language errParsedTemplatethe interpretation faiParsedTemplate//
  /// ## Example
  /// ```
  /// use funki_lang::{Language, Script, BlankCustom};
  /// let x = Script::<BlankCustom>::from_text("#main 5;").unwrap();
  /// let f = x.function("main").unwrap();
  /// f.call().unwrap(); // -> ReturnVal::Int(5)
  /// ```
  pub fn call(&self) -> Result<ReturnVal<C>, LanguageErr> {
    if let Some(x) = &self.arg {
      interpret(
        &self.lang.temp,
        self.name.as_str(),
        InterpretVal::from_arg(x),
        &Customs::new_from_hash(
          self.lang.binary_operators.clone(),
          self.lang.unary_operators.clone(),
          self.lang.built_ins.clone(),
        ),
      )
    } else {
      interpret(
        &self.lang.temp,
        self.name.as_str(),
        InterpretVal::Tuple(vec![]),
        &Customs::new_from_hash(Default::default(), Default::default(), Default::default()),
      )
    }
    .map_err(|e| LanguageErr::new_from_int_err(e, self.text.clone()))
  }
}

/// A language error with a location
pub struct LocationLangErr {
  message: String,
  lines: (usize, usize),
  char: (usize, usize),
  section: String,
}

/// An enum for the possible types of error that can result from interpretation
pub enum LanguageErr {
  NoLoc(String),
  Loc(LocationLangErr),
}

impl LanguageErr {
  /// Creates a language error with location information
  /// Adds in the original language string so the line numbers and string section can be found
  fn new_loc(message: String, location: (usize, usize), lang: String) -> Self {
    let (start_line, start_char) = get_lang_pos(&lang, location.0);
    let (end_line, end_char) = get_lang_pos(&lang, location.1);
    LanguageErr::Loc(LocationLangErr {
      lines: (start_line, end_line),
      section: lang[location.0..location.1].to_string(),
      char: (start_char, end_char),
      message,
    })
  }

  /// Creates a location error with no location data
  fn new_no_loc(message: String) -> Self {
    LanguageErr::NoLoc(message)
  }

  /// Creates a location error from an interpretation error
  fn new_from_int_err(err: InterpretError, lang: String) -> Self {
    if err.location.is_some() {
      Self::new_loc(err.message, err.location.unwrap(), lang)
    } else {
      Self::new_no_loc(err.message)
    }
  }

  /// Creates a location error from a parser error
  fn new_from_parser_err(
    err: ParseError<usize, String, (usize, String, usize)>,
    lang: String,
  ) -> Self {
    match err {
      ParseError::InvalidToken { location } => {
        let (line, char) = get_lang_pos(&lang, location);
        Self::Loc(LocationLangErr {
          message: "Invalid token".to_string(),
          lines: (line, line),
          char: (char, char),
          section: lang[location..location + 10].to_string(),
        })
      }
      ParseError::UnrecognizedEOF { location, .. } => {
        let (line, char) = get_lang_pos(&lang, location);
        Self::Loc(LocationLangErr {
          message: "Unexpected End of File".to_string(),
          lines: (line, line),
          char: (char, char),
          section: lang[cmp::min(location - 10, 0)..].to_string(),
        })
      }
      ParseError::UnrecognizedToken {
        token: (l, _, r), ..
      } => {
        let (start_line, start_char) = get_lang_pos(&lang, l);
        let (end_line, end_char) = get_lang_pos(&lang, r);
        Self::Loc(LocationLangErr {
          message: "Unrecognised token".to_string(),
          lines: (start_line, end_line),
          char: (start_char, end_char),
          section: lang[l..r].to_string(),
        })
      }
      ParseError::ExtraToken {
        token: (l, _, r), ..
      } => {
        let (start_line, start_char) = get_lang_pos(&lang, l);
        let (end_line, end_char) = get_lang_pos(&lang, r);
        Self::Loc(LocationLangErr {
          message: "Extra token".to_string(),
          lines: (start_line, end_line),
          char: (start_char, end_char),
          section: lang[l..r].to_string(),
        })
      }
      ParseError::User { error: (l, m, r) } => {
        let (start_line, start_char) = get_lang_pos(&lang, l);
        let (end_line, end_char) = get_lang_pos(&lang, r);
        Self::Loc(LocationLangErr {
          message: m,
          lines: (start_line, end_line),
          char: (start_char, end_char),
          section: lang[l..r].to_string(),
        })
      }
    }
  }
}

fn get_lang_pos(lang: &str, pos: usize) -> (usize, usize) {
  let new_lines = lang[0..pos]
    .as_bytes()
    .iter()
    .enumerate()
    .filter(|(_, c)| **c == b'\n');
  let line_num = new_lines.clone().count();
  let char = pos - new_lines.last().unwrap_or((0, &b'x')).0;

  (line_num, char)
}

impl Debug for LanguageErr {
  fn fmt(&self, fmt: &mut Formatter<'_>) -> std::fmt::Result {
    match self {
      LanguageErr::Loc(l) => {
        write!(
          fmt,
          "Error: \"{}\"\nAt lines: {}:{} - {}:{}\nCode: `{}`",
          l.message,
          l.lines.0 + 1,
          l.char.0,
          l.lines.1 + 1,
          l.char.1,
          l.section
        )
      }
      LanguageErr::NoLoc(l) => {
        write!(fmt, "Error: {}", l)
      }
    }
  }
}

impl<C: CustomType> Debug for ReturnVal<C> {
  fn fmt(&self, fmt: &mut Formatter<'_>) -> std::fmt::Result {
    match self {
      ReturnVal::Bool(b) => write!(fmt, "Bool({})", b),
      ReturnVal::Int(i) => write!(fmt, "Int({})", i),
      ReturnVal::String(s) => write!(fmt, "String({})", s),
      ReturnVal::Tuple(v) => write!(
        fmt,
        "Tuple({})",
        v.iter().map(|i| format!("{:?}", i)).join(", ")
      ),
      ReturnVal::List(v) => write!(
        fmt,
        "List({})",
        v.iter().map(|i| format!("{:?}", i)).join(", ")
      ),
      ReturnVal::Custom(v) => write!(fmt, "Custom({:?})", v),
    }
  }
}

impl<C: CustomType> ToString for ReturnVal<C> {
  fn to_string(&self) -> String {
    match self {
      ReturnVal::Bool(b) => b.to_string(),
      ReturnVal::Int(i) => i.to_string(),
      ReturnVal::String(s) => s.to_string(),
      ReturnVal::Tuple(v) => format!("({})", v.iter().map(|i| i.to_string()).join(", ")),
      ReturnVal::List(v) => format!("[{}]", v.iter().map(|i| i.to_string()).join(", ")),
      ReturnVal::Custom(v) => v.to_string(),
    }
  }
}

/// A blank custom type to use if no custom types are required
#[derive(Clone, Debug, PartialEq)]
pub struct BlankCustom {}

impl ToString for BlankCustom {
  fn to_string(&self) -> String {
    "Blank".to_string()
  }
}

impl CustomType for BlankCustom {}
