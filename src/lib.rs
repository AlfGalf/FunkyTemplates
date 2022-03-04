extern crate core;
#[macro_use]
extern crate lalrpop_util;

use std::collections::HashSet;
use std::fmt::{Debug, Formatter};

use itertools::Itertools;

use crate::ast::{ParserState, Template};
use crate::data_types::{InterpretError, InterpretVal};
use crate::external_operators::CustomOperator;
use crate::interpreter::interpret;
use crate::parser::language_definition::TemplateParser;

mod ast;
mod data_types;
mod interpreter;
mod parser;
mod test;

pub mod external_operators;

/// Represents a language to be parsed
pub struct Language {
  operators: HashSet<CustomOperator>,
}

/// Represents a set of template functions
pub struct ParsedTemplate {
  lang: String,
  temp: Template,
  operators: HashSet<CustomOperator>,
}

/// Represents an argument being parsed in to a function call
pub enum Argument {
  Int(i32),
  String(String),
  Tuple(Vec<Argument>),
}

/// Represents a function from a template
/// Can contain an argument also
pub struct LangFunc<'a> {
  lang: &'a ParsedTemplate,
  name: String,
  arg: Option<Argument>,
  text: String,
}

impl ParsedTemplate {
  /// Builds a language from a code string
  ///
  /// ## Example
  /// ```
  /// use funki_templates::ParsedTemplate;
  /// let x = ParsedTemplate::from_text("#main x -> x + 1;");
  /// ```
  pub fn from_text(lang: &str) -> Result<Self, LanguageErr> {
    let parser: TemplateParser = TemplateParser::new();
    let res = parser.parse(&ParserState::new(), lang);
    match res {
      Ok(l) => Ok(Self {
        temp: l,
        lang: lang.to_string(),
        operators: HashSet::new(),
      }),
      Err(e) => {
        // println!("{}", e);
        Err(LanguageErr::new_no_loc(format!("Parsing error: {}", e)))
      }
    }
  }

  /// Lists the available functions
  ///
  /// ## Example
  /// ```
  /// use funki_templates::ParsedTemplate;
  /// let x = ParsedTemplate::from_text("#main x -> x + 1;").unwrap();
  /// x.list(); // -> ["main"]
  /// ```
  pub fn list(&self) -> Vec<String> {
    return self.temp.env.keys().map(|s| s.to_string()).collect();
  }

  /// Selects a function from the template
  ///
  /// ## Example
  /// ```
  /// use funki_templates::ParsedTemplate;
  /// let x = ParsedTemplate::from_text("#main x -> x + 1;").unwrap();
  /// let f = x.function("main");
  /// ```
  pub fn function(&self, name: &str) -> Result<LangFunc, LanguageErr> {
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
#[derive(PartialEq)]
pub enum ReturnVal {
  String(String),
  Int(i32),
  Bool(bool),
  Tuple(Vec<ReturnVal>),
  List(Vec<ReturnVal>),
}

impl<'a> LangFunc<'a> {
  /// Adds an argument for a function call
  ///
  /// ## Example
  /// ```
  /// use funki_templates::{Argument, ParsedTemplate};
  /// let x = ParsedTemplate::from_text("#main x -> x + 4;").unwrap();
  /// let f = x.function("main").unwrap();
  /// let f = f.arg(Argument::Int(5));
  /// f.call().unwrap(); // -> ReturnVal::Int(9)
  /// ```
  pub fn arg(mut self, arg: Argument) -> Self {
    self.arg = Some(arg);
    self
  }
  /// Interprets this function
  /// Can return a language error if the interpretation fails
  ///
  /// ## Example
  /// ```
  /// use funki_templates::{Language, ParsedTemplate};
  /// let x = ParsedTemplate::from_text("#main 5;").unwrap();
  /// let f = x.function("main").unwrap();
  /// f.call().unwrap(); // -> ReturnVal::Int(5)
  /// ```
  pub fn call(&self) -> Result<ReturnVal, LanguageErr> {
    if let Some(x) = &self.arg {
      interpret(
        &self.lang.temp,
        self.name.as_str(),
        InterpretVal::from_arg(x),
      )
    } else {
      interpret(
        &self.lang.temp,
        self.name.as_str(),
        InterpretVal::Tuple(vec![]),
      )
    }
    .map_err(|e| LanguageErr::from_int_err(e, self.text.clone()))
  }
}

/// A language error with a location
pub struct LocationLangErr {
  message: String,
  lines: (usize, usize),
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
    let lines_to_start = lang[0..location.0]
      .as_bytes()
      .iter()
      .filter(|&&c| c == b'\n')
      .count();
    let lines_to_end = lines_to_start
      + lang[location.0..location.1]
        .as_bytes()
        .iter()
        .filter(|&&c| c == b'\n')
        .count();

    LanguageErr::Loc(LocationLangErr {
      lines: (lines_to_start, lines_to_end),
      section: lang[location.0..location.1].to_string(),
      message,
    })
  }

  /// Creates a location error with no location data
  fn new_no_loc(message: String) -> Self {
    LanguageErr::NoLoc(message)
  }

  /// Creates a location error from an interpretation error
  fn from_int_err(err: InterpretError, lang: String) -> Self {
    if err.location.is_some() {
      Self::new_loc(err.message, err.location.unwrap(), lang)
    } else {
      Self::new_no_loc(err.message)
    }
  }
}

impl Debug for LanguageErr {
  fn fmt(&self, fmt: &mut Formatter<'_>) -> std::fmt::Result {
    match self {
      LanguageErr::Loc(l) => {
        write!(
          fmt,
          "Error: \"{}\"\nAt lines: {} - {}\nCode: `{}`",
          l.message,
          l.lines.0 + 1,
          l.lines.1 + 1,
          l.section
        )
      }
      LanguageErr::NoLoc(l) => {
        write!(fmt, "Error: {}", l)
      }
    }
  }
}

impl Debug for ReturnVal {
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
    }
  }
}

impl ToString for ReturnVal {
  fn to_string(&self) -> String {
    match self {
      ReturnVal::Bool(b) => b.to_string(),
      ReturnVal::Int(i) => i.to_string(),
      ReturnVal::String(s) => s.to_string(),
      ReturnVal::Tuple(v) => format!("({})", v.iter().map(|i| i.to_string()).join(", ")),
      ReturnVal::List(v) => format!("[{}]", v.iter().map(|i| i.to_string()).join(", ")),
    }
  }
}
