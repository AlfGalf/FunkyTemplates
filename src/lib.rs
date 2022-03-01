extern crate core;
#[macro_use]
extern crate lalrpop_util;

use std::fmt::{Debug, Display, Formatter};
use std::rc::Rc;

use itertools::Itertools;

use crate::ast::Template;
use crate::data_types::{InterpretError, InterpretVal};
use crate::interpreter::interpret;
use crate::parser::language_definition::TemplateParser;

mod ast;
mod data_types;
mod interpreter;
mod parser;
mod test;

/// Represents a set of template functions
pub struct Language {
  temp: Rc<Template>,
  lang: String,
}

// Represents an argument being parsed in to a function call
pub enum Argument {
  Int(i32),
  String(String),
  Tuple(Vec<Argument>),
}

/// Represents a function within a template
pub struct LangFunc {
  lang: Rc<Template>,
  name: String,
  arg: Option<Argument>,
  text: String,
}

impl Language {
  /// Builds a language from a code string
  pub fn from_text(lang: &str) -> Result<Self, LanguageErr> {
    let parser: TemplateParser = TemplateParser::new();
    let res = parser.parse(lang);
    match res {
      Ok(l) => Ok(Self {
        temp: Rc::new(l),
        lang: lang.to_string(),
      }),
      Err(e) => {
        // println!("{}", e);
        Err(LanguageErr::new_no_loc(format!("Parsing error: {}", e)))
      }
    }
  }

  pub fn list(&self) -> Vec<String> {
    return self.temp.env.keys().map(|s| s.to_string()).collect();
  }

  /// Selects a function from the template
  pub fn function(&self, name: &str) -> Result<LangFunc, LanguageErr> {
    if self.temp.env.contains_key(name) {
      Ok(LangFunc {
        lang: Rc::clone(&self.temp),
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

impl LangFunc {
  pub fn arg(mut self, arg: Argument) -> Self {
    self.arg = Some(arg);
    self
  }
}

#[derive(Debug, PartialEq)]
pub enum ReturnVal {
  String(String),
  Int(i32),
  Bool(bool),
  Tuple(Vec<ReturnVal>),
  List(Vec<ReturnVal>),
}

impl LangFunc {
  pub fn call(&self) -> Result<ReturnVal, LanguageErr> {
    if let Some(x) = &self.arg {
      interpret(
        self.lang.as_ref(),
        self.name.as_str(),
        InterpretVal::from_arg(x),
      )
    } else {
      interpret(
        self.lang.as_ref(),
        self.name.as_str(),
        InterpretVal::Tuple(vec![]),
      )
    }
    .map_err(|e| LanguageErr::from_int_err(e, self.text.clone()))
  }
}

pub struct LocationLangErr {
  message: String,
  lines: (usize, usize),
  section: String,
}

pub enum LanguageErr {
  NoLoc(String),
  Loc(LocationLangErr),
}

impl LanguageErr {
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

  fn new_no_loc(message: String) -> Self {
    LanguageErr::NoLoc(message)
  }

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

impl Display for ReturnVal {
  fn fmt(&self, fmt: &mut Formatter<'_>) -> std::fmt::Result {
    match self {
      ReturnVal::Bool(b) => write!(fmt, "Bool({})", b),
      ReturnVal::Int(i) => write!(fmt, "Int({})", i),
      ReturnVal::String(s) => write!(fmt, "String({})", s),
      ReturnVal::Tuple(v) => write!(
        fmt,
        "Tuple({})",
        v.iter().map(|i| format!("{}", i)).join(", ")
      ),
      ReturnVal::List(v) => write!(
        fmt,
        "List({})",
        v.iter().map(|i| format!("{}", i)).join(", ")
      ),
    }
  }
}
