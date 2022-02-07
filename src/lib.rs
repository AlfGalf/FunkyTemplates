extern crate core;
#[macro_use]
extern crate lalrpop_util;

use std::cell::RefCell;
use std::fmt::{Debug, Formatter};
use std::rc::Rc;
use std::sync::Mutex;

use crate::ast::Template;
use crate::parser::language_definition;
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
    pub fn new(lang: &str) -> Result<Self, LanguageErr> {
        let parser: TemplateParser = TemplateParser::new();
        let res = parser.parse(lang);
        match res {
            Ok(l) => Ok(Self { temp: Rc::new(l) }),
            Err(e) => {
                println!("{}", e.to_string());
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
    pub fn call(&self) -> String {
        "test".to_string()
    }
}

pub struct LanguageErr {}

impl Debug for LanguageErr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}
