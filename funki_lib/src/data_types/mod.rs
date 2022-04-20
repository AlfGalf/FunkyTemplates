use std::cell::RefCell;
use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::fmt::{Debug, Formatter};
use std::ops::Add;

use itertools::Itertools;

use crate::ast::Pattern;
use crate::external_operators::CustomType;
use crate::{Argument, Program, ReturnVal};

/// Errors from the interpreter, can optionally have location information added
#[derive(Clone)]
pub struct InterpretError {
  pub message: String,
  pub location: Option<(usize, usize)>,
}

impl InterpretError {
  // Creates an interpret error with no location
  pub fn new(name: &str) -> Self {
    Self {
      message: name.to_string(),
      location: None,
    }
  }

  // Errors when they come from custom string
  pub fn from_custom(name: Box<dyn ToString>) -> Self {
    Self {
      message: name.to_string(),
      location: None,
    }
  }

  // Adds location data
  pub fn add_loc(&mut self, start: usize, end: usize) {
    if self.location.is_none() {
      self.location = Some((start, end));
    }
  }
}

// Values within the interpreter
// Cant use default implementations as CustomType cannot implement those types
#[derive(Clone, PartialEq)]
pub enum InterpretVal<C: CustomType> {
  Int(i32),
  Bool(bool),
  String(String),
  Function(Vec<Pattern>),
  Tuple(Vec<InterpretVal<C>>),
  List(Vec<InterpretVal<C>>),
  Lambda(Pattern, Frame<C>),
  Custom(C),
}

impl<C: CustomType> Debug for InterpretVal<C> {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    match &self {
      InterpretVal::Int(n) => write!(f, "Int({:?})", n),
      InterpretVal::Bool(b) => write!(f, "Bool({:?})", b),
      InterpretVal::String(s) => write!(f, "String({:?})", s),
      InterpretVal::Function(fun) => write!(f, "Function({:?})", fun),
      InterpretVal::Tuple(t) => write!(f, "Tuple({:?})", t),
      InterpretVal::List(l) => write!(f, "List({:?})", l),
      InterpretVal::Lambda(l, _) => write!(f, "Lambda({:?})", l),
      InterpretVal::Custom(c) => write!(f, "Custom({:?})", c),
    }
  }
}

impl<C: CustomType> ToString for InterpretVal<C> {
  // Used to convert values into strings for when they are added in interpolation strings
  fn to_string(&self) -> String {
    match self {
      InterpretVal::Int(i) => i.to_string(),
      InterpretVal::String(s) => s.to_string(),
      InterpretVal::Bool(t) => t.to_string(),
      InterpretVal::Tuple(t) => format!("({})", t.iter().map(|v| v.to_string()).join(", ")),
      InterpretVal::List(t) => format!("[{}]", t.iter().map(|v| v.to_string()).join(", ")),
      _ => panic!("Type not found"),
    }
  }
}

impl<C: CustomType> InterpretVal<C> {
  // Unwraps a tuple of length 1 to its enclosed value
  pub fn unwrap_tuple(self) -> InterpretVal<C> {
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

  // Creates a Interpret val from a interpret val
  pub fn from_arg(arg: &Argument<C>) -> Self {
    match arg {
      Argument::Int(x) => InterpretVal::Int(*x),
      Argument::String(s) => InterpretVal::String(s.clone()),
      Argument::Tuple(v) => InterpretVal::Tuple(v.iter().map(InterpretVal::from_arg).collect()),
      Argument::List(v) => InterpretVal::List(v.iter().map(InterpretVal::from_arg).collect()),
      Argument::Custom(c) => InterpretVal::Custom(c.clone()),
    }
  }

  // Adds two interpret values together
  pub fn add_op(&self, v: &InterpretVal<C>) -> Result<InterpretVal<C>, InterpretError> {
    match (self, v) {
      (InterpretVal::String(l), r) => {
        Ok(InterpretVal::String(l.clone().add(r.to_string().as_str())))
      }
      (InterpretVal::Int(l), InterpretVal::Int(r)) => Ok(InterpretVal::Int(l + r)),
      (InterpretVal::Custom(l), r) => l
        .pre_add(r.to_return_val()?)
        .map(|v| InterpretVal::from_arg(&v))
        .map_err(InterpretError::from_custom),
      (l, InterpretVal::Custom(r)) => r
        .post_add(l.to_return_val()?)
        .map(|v| InterpretVal::from_arg(&v))
        .map_err(InterpretError::from_custom),
      (l, r) => Err(InterpretError::new(
        format!("Add operator not defined for {:?} + {:?}.", l, r).as_str(),
      )),
    }
  }

  // Subtracts v from this value
  pub fn sub_op(&self, v: &InterpretVal<C>) -> Result<InterpretVal<C>, InterpretError> {
    match (self, v) {
      (InterpretVal::Int(l), InterpretVal::Int(r)) => Ok(InterpretVal::Int(l - r)),
      (InterpretVal::Custom(l), r) => l
        .pre_sub(r.to_return_val()?)
        .map(|v| InterpretVal::from_arg(&v))
        .map_err(InterpretError::from_custom),
      (l, InterpretVal::Custom(r)) => r
        .post_sub(l.to_return_val()?)
        .map(|v| InterpretVal::from_arg(&v))
        .map_err(InterpretError::from_custom),
      (l, r) => Err(InterpretError::new(
        format!("Subtract operator not defined for {:?} - {:?}.", l, r).as_str(),
      )),
    }
  }

  // Multiplies this value by v
  pub fn mult_op(&self, v: &InterpretVal<C>) -> Result<InterpretVal<C>, InterpretError> {
    match (self, v) {
      (InterpretVal::Int(l), InterpretVal::Int(r)) => Ok(InterpretVal::Int(l * r)),
      (InterpretVal::String(l), InterpretVal::Int(r)) => {
        Ok(InterpretVal::String(l.repeat(*r as usize)))
      }
      (InterpretVal::Custom(l), r) => l
        .pre_mult(r.to_return_val()?)
        .map(|v| InterpretVal::from_arg(&v))
        .map_err(InterpretError::from_custom),
      (l, InterpretVal::Custom(r)) => r
        .post_mult(l.to_return_val()?)
        .map(|v| InterpretVal::from_arg(&v))
        .map_err(InterpretError::from_custom),
      (l, r) => Err(InterpretError::new(
        format!("Multiplication operator not defined for {:?} * {:?}.", l, r).as_str(),
      )),
    }
  }

  pub fn div_op(&self, v: &InterpretVal<C>) -> Result<InterpretVal<C>, InterpretError> {
    match (self, v) {
      (InterpretVal::Int(l), InterpretVal::Int(r)) => Ok(InterpretVal::Int(l / r)),
      (InterpretVal::Custom(l), r) => l
        .pre_div(r.to_return_val()?)
        .map(|v| InterpretVal::from_arg(&v))
        .map_err(InterpretError::from_custom),
      (l, InterpretVal::Custom(r)) => r
        .post_div(l.to_return_val()?)
        .map(|v| InterpretVal::from_arg(&v))
        .map_err(InterpretError::from_custom),
      (l, r) => Err(InterpretError::new(
        format!("Division operator not defined for {:?} / {:?}.", l, r).as_str(),
      )),
    }
  }

  // Finds the value of this value modulo v
  pub fn modulo_op(&self, v: &InterpretVal<C>) -> Result<InterpretVal<C>, InterpretError> {
    match (self, v) {
      (InterpretVal::Int(l), InterpretVal::Int(r)) => Ok(InterpretVal::Int(l % r)),
      (InterpretVal::Custom(l), r) => l
        .pre_mod(r.to_return_val()?)
        .map(|v| InterpretVal::from_arg(&v))
        .map_err(InterpretError::from_custom),
      (l, InterpretVal::Custom(r)) => r
        .post_mod(l.to_return_val()?)
        .map(|v| InterpretVal::from_arg(&v))
        .map_err(InterpretError::from_custom),
      (l, r) => Err(InterpretError::new(
        format!("Modulo operator not defined for {:?} % {:?}.", l, r).as_str(),
      )),
    }
  }

  // Checks for equivalence of this value and other
  fn eq(&self, other: &Self) -> Result<bool, InterpretError> {
    match (self.clone().unwrap_tuple(), other.clone().unwrap_tuple()) {
      (InterpretVal::Int(l), InterpretVal::Int(r)) => Ok(l == r),
      (InterpretVal::Bool(l), InterpretVal::Bool(r)) => Ok(l == r),
      (InterpretVal::String(l), InterpretVal::String(r)) => Ok(l == r),
      (InterpretVal::Tuple(l), InterpretVal::Tuple(r)) => Ok(
        l.len() == r.len()
          && l
            .into_iter()
            .zip(r)
            .map(|(l, r)| l.eq(&r))
            .fold_ok(true, |l, r| l && r)?,
      ),
      (InterpretVal::Function(_), InterpretVal::Function(_)) => {
        Err(InterpretError::new("Cannot compare functions."))
      }
      (InterpretVal::List(l), InterpretVal::List(r)) => Ok(
        l.len() == r.len()
          && l
            .into_iter()
            .zip(r)
            .map(|(l, r)| l.eq(&r))
            .fold_ok(true, |l, r| l && r)?,
      ),
      (InterpretVal::Custom(l), r) => l
        .pre_eq(r.to_return_val()?)
        .map_err(|s| InterpretError::from_custom(s)),
      (l, InterpretVal::Custom(r)) => r
        .post_eq(l.to_return_val()?)
        .map_err(|s| InterpretError::from_custom(s)),
      (l, r) => Err(InterpretError::new(
        format!("Non matching types for equality: {:?} == {:?}", l, r).as_str(),
      )),
    }
  }

  // Checks for equivalence of this value and other, wrapper for other function to simplify
  pub fn eq_op(&self, right: &Self) -> Result<bool, InterpretError> {
    self.eq(right)
  }

  // Checks for inverse of the eq_op function
  pub fn neq_op(&self, v: &InterpretVal<C>) -> Result<bool, InterpretError> {
    Ok(!self.eq(v)?)
  }

  // Checks if this value can be considered less than v
  pub fn lt_op(&self, v: &InterpretVal<C>) -> Result<bool, InterpretError> {
    match (self, v) {
      (InterpretVal::Int(l), InterpretVal::Int(r)) => Ok(l < r),
      (InterpretVal::Custom(l), r) => l
        .pre_lt(r.to_return_val()?)
        .map_err(InterpretError::from_custom),
      (l, InterpretVal::Custom(r)) => r
        .post_lt(l.to_return_val()?)
        .map_err(InterpretError::from_custom),
      (l, r) => Err(InterpretError::new(
        format!("Comparison of types not supported {:?} < {:?}.", l, r).as_str(),
      )),
    }
  }

  // Checks if this value can be considered greater than v
  pub fn gt_op(&self, v: &InterpretVal<C>) -> Result<bool, InterpretError> {
    match (self, v) {
      (InterpretVal::Int(l), InterpretVal::Int(r)) => Ok(l > r),
      (InterpretVal::Custom(l), r) => l
        .pre_gt(r.to_return_val()?)
        .map_err(InterpretError::from_custom),
      (l, InterpretVal::Custom(r)) => r
        .post_gt(l.to_return_val()?)
        .map_err(InterpretError::from_custom),
      (l, r) => Err(InterpretError::new(
        format!("Comparison of types not supported {:?} > {:?}.", l, r).as_str(),
      )),
    }
  }

  // Checks if this value can be considered less than or equal to v
  pub fn leq_op(&self, v: &InterpretVal<C>) -> Result<bool, InterpretError> {
    match (self, v) {
      (InterpretVal::Int(l), InterpretVal::Int(r)) => Ok(l <= r),
      (InterpretVal::Custom(l), r) => l
        .pre_leq(r.to_return_val()?)
        .map_err(InterpretError::from_custom),
      (l, InterpretVal::Custom(r)) => r
        .post_leq(l.to_return_val()?)
        .map_err(InterpretError::from_custom),
      (l, r) => Err(InterpretError::new(
        format!("Comparison of types not supported {:?} <= {:?}.", l, r).as_str(),
      )),
    }
  }

  // Checks if this value can be considered greater than or equal to v
  pub fn geq_op(&self, v: &InterpretVal<C>) -> Result<bool, InterpretError> {
    match (self, v) {
      (InterpretVal::Int(l), InterpretVal::Int(r)) => Ok(l >= r),
      (InterpretVal::Custom(l), r) => l
        .pre_geq(r.to_return_val()?)
        .map_err(InterpretError::from_custom),
      (l, InterpretVal::Custom(r)) => r
        .post_geq(l.to_return_val()?)
        .map_err(InterpretError::from_custom),
      (l, r) => Err(InterpretError::new(
        format!("Comparison of types not supported {:?} >= {:?}.", l, r).as_str(),
      )),
    }
  }

  // Finds the result of this value and v under the logical and operator
  pub fn and_op(&self, v: &InterpretVal<C>) -> Result<bool, InterpretError> {
    match (self, v) {
      (InterpretVal::Bool(l), InterpretVal::Bool(r)) => Ok(*l && *r),
      (InterpretVal::Custom(l), r) => l
        .pre_and(r.to_return_val()?)
        .map_err(InterpretError::from_custom),
      (l, InterpretVal::Custom(r)) => r
        .post_and(l.to_return_val()?)
        .map_err(InterpretError::from_custom),
      (l, r) => Err(InterpretError::new(
        format!("And operator not supported for {:?} && {:?}.", l, r).as_str(),
      )),
    }
  }

  // Finds the result of this value and v under the logical or operator
  pub fn or_op(&self, v: &InterpretVal<C>) -> Result<bool, InterpretError> {
    match (self, v) {
      (InterpretVal::Bool(l), InterpretVal::Bool(r)) => Ok(*l || *r),
      (InterpretVal::Custom(l), r) => l
        .pre_or(r.to_return_val()?)
        .map_err(InterpretError::from_custom),
      (l, InterpretVal::Custom(r)) => r
        .post_or(l.to_return_val()?)
        .map_err(InterpretError::from_custom),
      (l, r) => Err(InterpretError::new(
        format!("And operator not supported for {:?} && {:?}.", l, r).as_str(),
      )),
    }
  }

  // Converts an interpret value to a return val that can be returned through the API
  pub fn to_return_val(&self) -> Result<ReturnVal<C>, InterpretError> {
    match self {
      InterpretVal::Int(i) => Ok(ReturnVal::Int(*i)),
      InterpretVal::Bool(b) => Ok(ReturnVal::Bool(*b)),
      InterpretVal::String(s) => Ok(ReturnVal::String(s.clone())),
      InterpretVal::Tuple(v) => Ok(ReturnVal::Tuple(
        v.iter()
          .map(|x| x.to_return_val())
          .collect::<Result<Vec<ReturnVal<C>>, InterpretError>>()?,
      )),
      InterpretVal::List(v) => Ok(ReturnVal::List(
        v.iter()
          .map(|x| x.to_return_val())
          .collect::<Result<Vec<ReturnVal<C>>, InterpretError>>()?,
      )),
      InterpretVal::Function(_) => Err(InterpretError::new(
        "Cannot have function return type to root.",
      )),
      InterpretVal::Lambda(_, _) => Err(InterpretError::new(
        "Cannot have lambda return type to root.",
      )),
      InterpretVal::Custom(c) => Ok(ReturnVal::Custom((*c).clone())),
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

// Frame for holding the environment in an execution of a program
#[derive(Debug, Clone, PartialEq)]
pub struct Frame<C: CustomType> {
  pub(crate) frame: HashMap<String, InterpretVal<C>>,
  next: Option<RefCell<Box<Frame<C>>>>,
}

impl<C: CustomType> Frame<C> {
  // Creates a new blank frame
  pub fn new() -> Self {
    Self {
      frame: HashMap::new(),
      next: None,
    }
  }

  // Builds a new frame from a template
  pub fn from_template(t: &Program) -> Self {
    Self {
      frame: t
        .env
        .iter()
        .map(|(a, b)| (a.clone(), InterpretVal::Function(b.clone())))
        .collect(),
      next: None,
    }
  }

  // Adds a new value to the frame
  pub fn add_val(&mut self, name: String, expr: &InterpretVal<C>) -> Result<(), InterpretError> {
    if let Entry::Vacant(e) = self.frame.entry(name) {
      e.insert(expr.clone());
      Ok(())
    } else {
      Err(InterpretError::new(
        "Multiple variables within the same frame.",
      ))
    }
  }

  // finds the value associates with a token in this frame
  pub fn find(&self, name: &str) -> Result<InterpretVal<C>, InterpretError> {
    match name {
      "true" => Ok(InterpretVal::Bool(true)),
      "false" => Ok(InterpretVal::Bool(false)),
      _ => {
        if let Some(r) = self.frame.get(name) {
          Ok(r.clone())
        } else if let Some(n) = &self.next {
          n.borrow().find(name)
        } else {
          Err(InterpretError::new(&*format!(
            "Cannot find value {}.",
            name
          )))
        }
      }
    }
  }

  // Sets the next frame in the linked list of frames
  // Note the clone, this can be done as the pure functional nature of the language prevents the
  //  higher frames being mutated while values in a lower function are modified
  // Could be replaced with a Rc for less data copying
  pub fn set_next(&mut self, next: &Frame<C>) {
    self.next = Some(RefCell::new(Box::new((*next).clone())))
  }
}
