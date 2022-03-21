use std::fmt::Debug;

use crate::{Argument, InterpretError, InterpretVal, ReturnVal};

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub enum OperatorChars {
  At,
  Carat,
  And,
  Dollar,
  Section,
  QuestionMark,
  Backslash,
  Tilda,
}

impl ToString for OperatorChars {
  fn to_string(&self) -> String {
    match self {
      OperatorChars::At => "@",
      OperatorChars::Carat => "^",
      OperatorChars::And => "&",
      OperatorChars::Dollar => "$",
      OperatorChars::Section => "§",
      OperatorChars::QuestionMark => "?",
      OperatorChars::Backslash => "\\",
      OperatorChars::Tilda => "~",
    }
    .to_string()
  }
}

#[derive(Clone, Debug)]
pub struct CustomBinOp<C: CustomType> {
  pub function: fn(ReturnVal<C>, ReturnVal<C>) -> Result<Argument<C>, Box<dyn ToString>>,
}

impl<C: CustomType> CustomBinOp<C> {
  pub fn call_func(
    &self,
    val1: &InterpretVal<C>,
    val2: &InterpretVal<C>,
  ) -> Result<InterpretVal<C>, InterpretError> {
    let arg1 = val1.clone().unwrap_tuple().to_return_val()?;
    let arg2 = val2.clone().unwrap_tuple().to_return_val()?;

    (self.function)(arg1, arg2)
      .map_err(|e| InterpretError::new(&e.to_string()))
      .map(|v| InterpretVal::from_arg(&v))
  }
}

#[derive(Clone, Debug)]
pub struct CustomUnaryOp<C: CustomType> {
  pub function: fn(ReturnVal<C>) -> Result<Argument<C>, Box<dyn ToString>>,
}

impl<C: CustomType> CustomUnaryOp<C> {
  pub fn call_func(&self, val1: &InterpretVal<C>) -> Result<InterpretVal<C>, InterpretError> {
    let arg1 = val1.clone().unwrap_tuple().to_return_val()?;

    (self.function)(arg1)
      .map_err(|e| InterpretError::new(&e.to_string()))
      .map(|v| InterpretVal::from_arg(&v))
  }
}

#[derive(Clone, Debug)]
pub struct CustomBuiltIn<C: CustomType> {
  pub function: fn(ReturnVal<C>) -> Result<Argument<C>, Box<dyn ToString>>,
}

impl<C: CustomType> CustomBuiltIn<C> {
  pub fn call_func(&self, val1: &InterpretVal<C>) -> Result<InterpretVal<C>, InterpretError> {
    let arg1 = val1.clone().unwrap_tuple().to_return_val()?;

    (self.function)(arg1)
      .map_err(|e| InterpretError::new(&e.to_string()))
      .map(|v| InterpretVal::from_arg(&v))
  }
}

fn not_defined_err<C: CustomType>() -> Result<Argument<C>, Box<dyn ToString>> {
  Err(Box::new("Not defined."))
}

fn bool_not_defined_err() -> Result<bool, Box<dyn ToString>> {
  Err(Box::new("Not defined."))
}

pub trait CustomType: Clone + Debug + ToString + PartialEq {
  fn pre_add(&self, _: ReturnVal<Self>) -> Result<Argument<Self>, Box<dyn ToString>> {
    not_defined_err()
  }
  fn post_add(&self, _: ReturnVal<Self>) -> Result<Argument<Self>, Box<dyn ToString>> {
    not_defined_err()
  }
  fn pre_sub(&self, _: ReturnVal<Self>) -> Result<Argument<Self>, Box<dyn ToString>> {
    not_defined_err()
  }
  fn post_sub(&self, _: ReturnVal<Self>) -> Result<Argument<Self>, Box<dyn ToString>> {
    not_defined_err()
  }
  fn pre_mult(&self, _: ReturnVal<Self>) -> Result<Argument<Self>, Box<dyn ToString>> {
    not_defined_err()
  }
  fn post_mult(&self, _: ReturnVal<Self>) -> Result<Argument<Self>, Box<dyn ToString>> {
    not_defined_err()
  }
  fn pre_div(&self, _: ReturnVal<Self>) -> Result<Argument<Self>, Box<dyn ToString>> {
    not_defined_err()
  }
  fn post_div(&self, _: ReturnVal<Self>) -> Result<Argument<Self>, Box<dyn ToString>> {
    not_defined_err()
  }
  fn pre_mod(&self, _: ReturnVal<Self>) -> Result<Argument<Self>, Box<dyn ToString>> {
    not_defined_err()
  }
  fn post_mod(&self, _: ReturnVal<Self>) -> Result<Argument<Self>, Box<dyn ToString>> {
    not_defined_err()
  }
  fn pre_eq(&self, _: ReturnVal<Self>) -> Result<bool, Box<dyn ToString>> {
    bool_not_defined_err()
  }
  fn post_eq(&self, _: ReturnVal<Self>) -> Result<bool, Box<dyn ToString>> {
    bool_not_defined_err()
  }
  fn pre_neq(&self, _: ReturnVal<Self>) -> Result<bool, Box<dyn ToString>> {
    bool_not_defined_err()
  }
  fn post_neq(&self, _: ReturnVal<Self>) -> Result<bool, Box<dyn ToString>> {
    bool_not_defined_err()
  }
  fn pre_lt(&self, _: ReturnVal<Self>) -> Result<bool, Box<dyn ToString>> {
    bool_not_defined_err()
  }
  fn post_lt(&self, _: ReturnVal<Self>) -> Result<bool, Box<dyn ToString>> {
    bool_not_defined_err()
  }
  fn pre_gt(&self, _: ReturnVal<Self>) -> Result<bool, Box<dyn ToString>> {
    bool_not_defined_err()
  }
  fn post_gt(&self, _: ReturnVal<Self>) -> Result<bool, Box<dyn ToString>> {
    bool_not_defined_err()
  }
  fn pre_leq(&self, _: ReturnVal<Self>) -> Result<bool, Box<dyn ToString>> {
    bool_not_defined_err()
  }
  fn post_leq(&self, _: ReturnVal<Self>) -> Result<bool, Box<dyn ToString>> {
    bool_not_defined_err()
  }
  fn pre_geq(&self, _: ReturnVal<Self>) -> Result<bool, Box<dyn ToString>> {
    bool_not_defined_err()
  }
  fn post_geq(&self, _: ReturnVal<Self>) -> Result<bool, Box<dyn ToString>> {
    bool_not_defined_err()
  }
  fn pre_and(&self, _: ReturnVal<Self>) -> Result<bool, Box<dyn ToString>> {
    bool_not_defined_err()
  }
  fn post_and(&self, _: ReturnVal<Self>) -> Result<bool, Box<dyn ToString>> {
    bool_not_defined_err()
  }
  fn pre_or(&self, _: ReturnVal<Self>) -> Result<bool, Box<dyn ToString>> {
    bool_not_defined_err()
  }
  fn post_or(&self, _: ReturnVal<Self>) -> Result<bool, Box<dyn ToString>> {
    bool_not_defined_err()
  }
  fn pre_not(&self) -> Result<Argument<Self>, Box<dyn ToString>> {
    not_defined_err()
  }
  fn pre_neg(&self) -> Result<Argument<Self>, Box<dyn ToString>> {
    not_defined_err()
  }
}
