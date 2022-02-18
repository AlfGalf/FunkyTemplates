use std::fmt::{Debug, Display, Formatter};

use itertools::Itertools;

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
            Argument::Int(x) => InterpretVal::Int(*x),
            Argument::String(s) => InterpretVal::String(s.clone()),
            _ => todo!(),
        }
    }

    pub fn eq(&self, other: &Self) -> Result<bool, InterpretError> {
        match (self.clone().unwrap_tuple(), other.clone().unwrap_tuple()) {
            (InterpretVal::Int(l), InterpretVal::Int(r)) => Ok(l == r),
            (InterpretVal::Bool(l), InterpretVal::Bool(r)) => Ok(l == r),
            (InterpretVal::String(l), InterpretVal::String(r)) => Ok(l == r),
            (InterpretVal::Tuple(l), InterpretVal::Tuple(r)) => Ok(l.len() == r.len()
                && l.into_iter()
                    .zip(r)
                    .map(|(l, r)| l.eq(&r))
                    .fold_ok(true, |l, r| l && r)?),
            (InterpretVal::Function(_), InterpretVal::Function(_)) => {
                Err(InterpretError::new("Cannot compare functions."))
            }
            (l, r) => Err(InterpretError::new(
                format!("Non matching types for equality: {:?} == {:?}", l, r).as_str(),
            )),
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
