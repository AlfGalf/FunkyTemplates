use crate::{Argument, InterpretError, InterpretVal, ReturnVal};

#[derive(Clone, Eq, PartialEq, Hash)]
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

pub struct CustomFunction {}

#[derive(Clone)]
pub struct CustBinOp {
  pub function: fn(ReturnVal, ReturnVal) -> Result<Argument, Box<dyn ToString>>,
}

impl CustBinOp {
  pub fn call_func(
    &self,
    val1: &InterpretVal,
    val2: &InterpretVal,
  ) -> Result<InterpretVal, InterpretError> {
    let arg1 = val1.to_return_val()?;
    let arg2 = val2.to_return_val()?;

    (self.function)(arg1, arg2)
      .map_err(|e| InterpretError::new("e"))
      .map(|v| InterpretVal::from_arg(&v))
  }
}

// pub trait CustomOperator {}
//
// pub trait CustomType {}
//
// pub trait CustomBuiltinFunction {}
