use crate::{Argument, ReturnVal};

#[derive(Clone, PartialEq)]
pub enum OperatorChars {
  At,
  Carat,
  And,
  Dollar,
  Section,
  QuestionMark,
  Backslash,
  ForwardSlash,
  Tilda,
}

pub struct CustomFunction {}

#[derive(Clone)]
pub struct CustomOperator {
  operator: OperatorChars,
  function: fn(ReturnVal, ReturnVal) -> Result<Argument, Box<dyn ToString>>,
}

// pub trait CustomOperator {}
//
// pub trait CustomType {}
//
// pub trait CustomBuiltinFunction {}
