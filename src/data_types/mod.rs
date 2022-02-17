use std::fmt::{Debug, Display, Formatter};

use crate::ast::Pattern;
use crate::Argument;

#[derive(Debug, PartialEq)]
pub enum ReturnVal {
    String(String),
    Int(i32),
    Bool(bool),
    Tuple(Vec<ReturnVal>),
}

#[derive(Clone)]
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

#[derive(Debug, Clone, PartialEq)]
pub enum InterpretVal {
    Int(i32),
    Bool(bool),
    String(String),
    Function(Vec<Pattern>),
    Tuple(Vec<InterpretVal>),
}

impl InterpretVal {
    pub fn print(&self) -> String {
        match self {
            InterpretVal::Int(i) => i.to_string(),
            InterpretVal::String(s) => s.to_string(),
            _ => panic!("Type not found"),
        }
    }

    pub fn blank() -> Self {
        InterpretVal::Tuple(vec![])
    }

    pub fn unwrap_tuple(self) -> InterpretVal {
        if let InterpretVal::Tuple(s) = self {
            if s.len() == 1 {
                s[0].clone()
            } else {
                InterpretVal::Tuple(s)
            }
        } else {
            self
        }
    }

    pub fn from_arg(arg: &Argument) -> Self {
        match arg {
            Argument::Int(x) => InterpretVal::Int(x.clone()),
            Argument::String(s) => InterpretVal::String(s.clone()),
            _ => todo!(),
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
