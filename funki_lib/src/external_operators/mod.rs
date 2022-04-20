use std::fmt::Debug;

use crate::{Argument, ReturnVal};

/// The available characters for custom operators to be assigned.
#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub enum OperatorChars {
  /// At: `@`
  At,
  /// Carat: `^`
  Carat,
  /// And: `&`
  And,
  /// Dollar: `$`
  Dollar,
  /// Section: `§`
  Section,
  /// QuestionMark: `?`
  QuestionMark,
  /// Backslash: `\`
  Backslash,
  /// Tilda: `~`
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

/// A custom binary operator.
#[derive(Clone, Debug)]
pub struct CustomBinOp<C: CustomType> {
  /// The function that gets called to evaluate this operator.
  /// Both the arguments are evaluated before the function is called.
  pub function: fn(ReturnVal<C>, ReturnVal<C>) -> Result<Argument<C>, Box<dyn ToString>>,
}

/// A custom unary operator.
#[derive(Clone, Debug)]
pub struct CustomUnaryOp<C: CustomType> {
  /// The function that gets called to evaluate this operator.
  /// The argument is evaluated before the function is called.
  pub function: fn(ReturnVal<C>) -> Result<Argument<C>, Box<dyn ToString>>,
}

/// A custom builtin function.
#[derive(Clone, Debug)]
pub struct CustomBuiltIn<C: CustomType> {
  /// The function that gets called to evaluate a function call with this builtin.
  /// The argument is evaluated before the function is called.
  /// If multiple arguments are provided they are wrapped in a tuple.
  pub function: fn(ReturnVal<C>) -> Result<Argument<C>, Box<dyn ToString>>,
}

// Helper function
fn not_defined_err<C: CustomType>() -> Result<Argument<C>, Box<dyn ToString>> {
  Err(Box::new("Not defined."))
}

// Helper function
fn bool_not_defined_err() -> Result<bool, Box<dyn ToString>> {
  Err(Box::new("Not defined."))
}

/// A custom type for interpretation must implement this trait
/// Defines its behaviour in operations
pub trait CustomType: Clone + Debug + ToString + PartialEq {
  /// Behaviour of `this + arg`
  fn pre_add(&self, _: ReturnVal<Self>) -> Result<Argument<Self>, Box<dyn ToString>> {
    not_defined_err()
  }
  /// Behaviour of `arg + this`
  fn post_add(&self, _: ReturnVal<Self>) -> Result<Argument<Self>, Box<dyn ToString>> {
    not_defined_err()
  }
  /// Behaviour of `this - arg`
  fn pre_sub(&self, _: ReturnVal<Self>) -> Result<Argument<Self>, Box<dyn ToString>> {
    not_defined_err()
  }
  /// Behaviour of `arg - this`
  fn post_sub(&self, _: ReturnVal<Self>) -> Result<Argument<Self>, Box<dyn ToString>> {
    not_defined_err()
  }
  /// Behaviour of `this * arg`
  fn pre_mult(&self, _: ReturnVal<Self>) -> Result<Argument<Self>, Box<dyn ToString>> {
    not_defined_err()
  }
  /// Behaviour of `arg * this`
  fn post_mult(&self, _: ReturnVal<Self>) -> Result<Argument<Self>, Box<dyn ToString>> {
    not_defined_err()
  }
  /// Behaviour of `this / arg`
  fn pre_div(&self, _: ReturnVal<Self>) -> Result<Argument<Self>, Box<dyn ToString>> {
    not_defined_err()
  }
  /// Behaviour of `arg / this`
  fn post_div(&self, _: ReturnVal<Self>) -> Result<Argument<Self>, Box<dyn ToString>> {
    not_defined_err()
  }
  /// Behaviour of `this % arg`
  fn pre_mod(&self, _: ReturnVal<Self>) -> Result<Argument<Self>, Box<dyn ToString>> {
    not_defined_err()
  }
  /// Behaviour of `arg % this`
  fn post_mod(&self, _: ReturnVal<Self>) -> Result<Argument<Self>, Box<dyn ToString>> {
    not_defined_err()
  }
  /// Behaviour of `this == arg`
  fn pre_eq(&self, _: ReturnVal<Self>) -> Result<bool, Box<dyn ToString>> {
    bool_not_defined_err()
  }
  /// Behaviour of `arg == this`
  fn post_eq(&self, _: ReturnVal<Self>) -> Result<bool, Box<dyn ToString>> {
    bool_not_defined_err()
  }
  /// Behaviour of `this != arg`
  fn pre_neq(&self, _: ReturnVal<Self>) -> Result<bool, Box<dyn ToString>> {
    bool_not_defined_err()
  }
  /// Behaviour of `arg != this`
  fn post_neq(&self, _: ReturnVal<Self>) -> Result<bool, Box<dyn ToString>> {
    bool_not_defined_err()
  }
  /// Behaviour of `this < arg`
  fn pre_lt(&self, _: ReturnVal<Self>) -> Result<bool, Box<dyn ToString>> {
    bool_not_defined_err()
  }
  /// Behaviour of `arg < this`
  fn post_lt(&self, _: ReturnVal<Self>) -> Result<bool, Box<dyn ToString>> {
    bool_not_defined_err()
  }
  /// Behaviour of `this > arg`
  fn pre_gt(&self, _: ReturnVal<Self>) -> Result<bool, Box<dyn ToString>> {
    bool_not_defined_err()
  }
  /// Behaviour of `arg > this`
  fn post_gt(&self, _: ReturnVal<Self>) -> Result<bool, Box<dyn ToString>> {
    bool_not_defined_err()
  }
  /// Behaviour of `this <= arg`
  fn pre_leq(&self, _: ReturnVal<Self>) -> Result<bool, Box<dyn ToString>> {
    bool_not_defined_err()
  }
  /// Behaviour of `arg <= this`
  fn post_leq(&self, _: ReturnVal<Self>) -> Result<bool, Box<dyn ToString>> {
    bool_not_defined_err()
  }
  /// Behaviour of `this >= arg`
  fn pre_geq(&self, _: ReturnVal<Self>) -> Result<bool, Box<dyn ToString>> {
    bool_not_defined_err()
  }
  /// Behaviour of `arg >= this`
  fn post_geq(&self, _: ReturnVal<Self>) -> Result<bool, Box<dyn ToString>> {
    bool_not_defined_err()
  }
  /// Behaviour of `this && arg`
  fn pre_and(&self, _: ReturnVal<Self>) -> Result<bool, Box<dyn ToString>> {
    bool_not_defined_err()
  }
  /// Behaviour of `arg && this`
  fn post_and(&self, _: ReturnVal<Self>) -> Result<bool, Box<dyn ToString>> {
    bool_not_defined_err()
  }
  /// Behaviour of `this || arg`
  fn pre_or(&self, _: ReturnVal<Self>) -> Result<bool, Box<dyn ToString>> {
    bool_not_defined_err()
  }
  /// Behaviour of `arg || this`
  fn post_or(&self, _: ReturnVal<Self>) -> Result<bool, Box<dyn ToString>> {
    bool_not_defined_err()
  }
  /// Behaviour of `!this`
  fn pre_not(&self) -> Result<Argument<Self>, Box<dyn ToString>> {
    not_defined_err()
  }
  /// Behaviour of `-this`
  fn pre_neg(&self) -> Result<Argument<Self>, Box<dyn ToString>> {
    not_defined_err()
  }
}
