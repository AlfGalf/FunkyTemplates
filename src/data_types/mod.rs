use std::fmt::{Debug, Display, Formatter};

use crate::ast::Pattern;

#[derive(Debug, PartialEq)]
pub enum ReturnVal {
    String(String),
    Int(i32),
    Bool(bool),
    Tuple(Vec<ReturnVal>),
}

pub struct InterpretError {
    message: String,
}

impl InterpretError {
    pub fn new(name: &str) -> Self {
        Self {
            message: name.to_string(),
        }
    }
}

pub enum InterpretVal {
    Int(i32),
    Bool(bool),
    String(String),
    Function(Vec<Pattern>),
}

impl InterpretVal {
    pub fn print(&self) -> String {
        match self {
            InterpretVal::Int(i) => i.to_string(),
            _ => panic!("Type not found"),
        }
    }
}

impl Display for ReturnVal {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ReturnVal::Bool(b) => write!(fmt, "Bool({})", b),
            ReturnVal::Int(i) => write!(fmt, "Int({})", i),
            ReturnVal::String(s) => write!(fmt, "String({})", s),
            _ => write!(fmt, "Unrecognised type"),
        }
    }
}

impl Debug for InterpretError {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> std::fmt::Result {
        write!(fmt, "Interpret Error: {}", self.message)
    }
}
