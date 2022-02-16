extern crate core;
#[macro_use]
extern crate lalrpop_util;

use std::fmt::{Debug, Formatter};
use std::rc::Rc;

use crate::ast::Template;
use crate::data_types::{InterpretError, ReturnVal};
use crate::interpreter::interpret;
use crate::parser::language_definition::TemplateParser;

mod ast;
mod data_types;
mod interpreter;
mod parser;
mod test;

pub struct Language {
    temp: Rc<Template>,
}

pub struct LangFunc {
    lang: Rc<Template>,
    name: String,
}

impl Language {
    pub fn from_text(lang: &str) -> Result<Self, LanguageErr> {
        let parser: TemplateParser = TemplateParser::new();
        let res = parser.parse(lang);
        match res {
            Ok(l) => Ok(Self { temp: Rc::new(l) }),
            Err(e) => {
                println!("{}", e);
                Err(LanguageErr {})
            }
        }
    }

    pub fn function(&self, name: &str) -> LangFunc {
        LangFunc {
            lang: Rc::clone(&self.temp),
            name: name.to_string(),
        }
    }
}

impl LangFunc {
    pub fn call(&self) -> Result<ReturnVal, InterpretError> {
        interpret(self.lang.as_ref(), self.name.as_str())
    }
}

pub struct LanguageErr {}

impl Debug for LanguageErr {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> std::fmt::Result {
        write!(fmt, "")
    }
}
