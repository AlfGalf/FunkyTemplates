extern crate core;
#[macro_use]
extern crate lalrpop_util;

use std::fmt::{Debug, Formatter};
use std::rc::Rc;

use crate::ast::Template;
use crate::data_types::{InterpretError, InterpretVal, ReturnVal};
use crate::interpreter::interpret;
use crate::parser::language_definition::TemplateParser;

mod ast;
mod data_types
mod interpreter;
mod parser;
mod test;

/// Represents a set of template functions
pub struct Language {
    temp: Rc<Template>,
}

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
}

impl Language {
    /// Builds a language from a code string
    pub fn from_text(lang: &str) -> Result<Self, LanguageErr> {
        let parser: TemplateParser = TemplateParser::new();
        let res = parser.parse(lang);
        match res {
            Ok(l) => Ok(Self { temp: Rc::new(l) }),
            Err(_) => {
                // println!("{}", e);
                Err(LanguageErr {})
            }
        }
    }

    /// Selects a function from the template
    pub fn function(&self, name: &str) -> LangFunc {
        LangFunc {
            lang: Rc::clone(&self.temp),
            name: name.to_string(),
            arg: None,
        }
    }
}

impl LangFunc {
    pub fn arg(mut self, arg: Argument) -> Self {
        self.arg = Some(arg);
        self
    }
}

impl LangFunc {
    pub fn call(&self) -> Result<ReturnVal, InterpretError> {
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
    }
}

pub struct LanguageErr {}

impl Debug for LanguageErr {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> std::fmt::Result {
        write!(fmt, "")
    }
}
