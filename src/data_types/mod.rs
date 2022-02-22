use std::fmt::{Debug, Formatter};
use std::ops::Add;

use itertools::Itertools;

use crate::ast::Pattern;
use crate::Argument;

#[derive(Clone)]
pub struct InterpretError {
    pub message: String,
    pub location: Option<(usize, usize)>,
}

impl InterpretError {
    pub fn new(name: &str) -> Self {
        Self {
            message: name.to_string(),
            location: None,
        }
    }
    pub fn add_loc(&mut self, start: usize, end: usize) {
        if self.location.is_none() {
            self.location = Some((start, end));
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
    List(Vec<InterpretVal>),
}

impl InterpretVal {
    pub fn print(&self) -> String {
        match self {
            InterpretVal::Int(i) => i.to_string(),
            InterpretVal::String(s) => s.to_string(),
            InterpretVal::Bool(t) => t.to_string(),
            InterpretVal::Tuple(t) => format!("({})", t.iter().map(|v| v.print()).join(", ")),
            InterpretVal::List(t) => format!("[{}]", t.iter().map(|v| v.print()).join(", ")),
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
            Argument::Tuple(v) => {
                InterpretVal::Tuple(v.iter().map(InterpretVal::from_arg).collect())
            }
        }
    }

    pub fn add_op(&self, v: &InterpretVal) -> Result<InterpretVal, InterpretError> {
        match (self, v) {
            (InterpretVal::String(l), r) => {
                Ok(InterpretVal::String(l.clone().add(r.print().as_str())))
            }
            (InterpretVal::Int(l), InterpretVal::Int(r)) => Ok(InterpretVal::Int(l + r)),
            (l, r) => Err(InterpretError::new(
                format!("Add operator not defined for {:?} + {:?}.", l, r).as_str(),
            )),
        }
    }

    pub fn sub_op(&self, v: &InterpretVal) -> Result<InterpretVal, InterpretError> {
        match (self, v) {
            (InterpretVal::Int(l), InterpretVal::Int(r)) => Ok(InterpretVal::Int(l - r)),
            (l, r) => Err(InterpretError::new(
                format!("Subtract operator not defined for {:?} - {:?}.", l, r).as_str(),
            )),
        }
    }

    pub fn mult_op(&self, v: &InterpretVal) -> Result<InterpretVal, InterpretError> {
        match (self, v) {
            (InterpretVal::Int(l), InterpretVal::Int(r)) => Ok(InterpretVal::Int(l * r)),
            (InterpretVal::String(l), InterpretVal::Int(r)) => {
                Ok(InterpretVal::String(l.repeat(*r as usize)))
            }
            (l, r) => Err(InterpretError::new(
                format!("Multiplication operator not defined for {:?} * {:?}.", l, r).as_str(),
            )),
        }
    }

    pub fn div_op(&self, v: &InterpretVal) -> Result<InterpretVal, InterpretError> {
        match (self, v) {
            (InterpretVal::Int(l), InterpretVal::Int(r)) => Ok(InterpretVal::Int(l / r)),
            (l, r) => Err(InterpretError::new(
                format!("Division operator not defined for {:?} / {:?}.", l, r).as_str(),
            )),
        }
    }

    pub fn modulo_op(&self, v: &InterpretVal) -> Result<InterpretVal, InterpretError> {
        match (self, v) {
            (InterpretVal::Int(l), InterpretVal::Int(r)) => Ok(InterpretVal::Int(l % r)),
            (l, r) => Err(InterpretError::new(
                format!("Modulo operator not defined for {:?} % {:?}.", l, r).as_str(),
            )),
        }
    }

    fn eq(&self, other: &Self) -> Result<bool, InterpretError> {
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
            (InterpretVal::List(l), InterpretVal::List(r)) => Ok(l.len() == r.len()
                && l.into_iter()
                    .zip(r)
                    .map(|(l, r)| l.eq(&r))
                    .fold_ok(true, |l, r| l && r)?),
            (l, r) => Err(InterpretError::new(
                format!("Non matching types for equality: {:?} == {:?}", l, r).as_str(),
            )),
        }
    }

    pub fn eq_op(&self, right: &Self) -> Result<InterpretVal, InterpretError> {
        Ok(InterpretVal::Bool(self.eq(right)?))
    }

    pub fn neq_op(&self, v: &InterpretVal) -> Result<InterpretVal, InterpretError> {
        Ok(InterpretVal::Bool(!self.eq(v)?))
    }

    pub fn lt_op(&self, v: &InterpretVal) -> Result<InterpretVal, InterpretError> {
        match (self, v) {
            (InterpretVal::Int(l), InterpretVal::Int(r)) => Ok(InterpretVal::Bool(l < r)),
            (l, r) => Err(InterpretError::new(
                format!("Comparison of types not supported {:?} < {:?}.", l, r).as_str(),
            )),
        }
    }

    pub fn gt_op(&self, v: &InterpretVal) -> Result<InterpretVal, InterpretError> {
        match (self, v) {
            (InterpretVal::Int(l), InterpretVal::Int(r)) => Ok(InterpretVal::Bool(l > r)),
            (l, r) => Err(InterpretError::new(
                format!("Comparison of types not supported {:?} > {:?}.", l, r).as_str(),
            )),
        }
    }

    pub fn leq_op(&self, v: &InterpretVal) -> Result<InterpretVal, InterpretError> {
        match (self, v) {
            (InterpretVal::Int(l), InterpretVal::Int(r)) => Ok(InterpretVal::Bool(l <= r)),
            (l, r) => Err(InterpretError::new(
                format!("Comparison of types not supported {:?} <= {:?}.", l, r).as_str(),
            )),
        }
    }

    pub fn geq_op(&self, v: &InterpretVal) -> Result<InterpretVal, InterpretError> {
        match (self, v) {
            (InterpretVal::Int(l), InterpretVal::Int(r)) => Ok(InterpretVal::Bool(l >= r)),
            (l, r) => Err(InterpretError::new(
                format!("Comparison of types not supported {:?} >= {:?}.", l, r).as_str(),
            )),
        }
    }

    pub fn and_op(&self, v: &InterpretVal) -> Result<InterpretVal, InterpretError> {
        match (self, v) {
            (InterpretVal::Bool(l), InterpretVal::Bool(r)) => Ok(InterpretVal::Bool(*l && *r)),
            (l, r) => Err(InterpretError::new(
                format!("And operator not supported for {:?} && {:?}.", l, r).as_str(),
            )),
        }
    }

    pub fn or_op(&self, v: &InterpretVal) -> Result<InterpretVal, InterpretError> {
        match (self, v) {
            (InterpretVal::Bool(l), InterpretVal::Bool(r)) => Ok(InterpretVal::Bool(*l || *r)),
            (l, r) => Err(InterpretError::new(
                format!("And operator not supported for {:?} && {:?}.", l, r).as_str(),
            )),
        }
    }
}

impl Debug for InterpretError {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> std::fmt::Result {
        if let Some((s, e)) = self.location {
            write!(
                fmt,
                "Interpret Error: \"{}\" loc: {} - {}",
                self.message, s, e
            )
        } else {
            write!(fmt, "Interpret Error: {}", self.message)
        }
    }
}
