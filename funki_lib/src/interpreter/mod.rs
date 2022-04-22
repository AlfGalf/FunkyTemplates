use std::borrow::Borrow;
use std::collections::HashMap;
use std::ops::Add;

use itertools::Itertools;

use crate::ast::*;
use crate::data_types::*;
use crate::external_operators::CustomBuiltIn;
use crate::interpreter::builtins::built_in;
use crate::{Argument, CustomBinOp, CustomType, CustomUnaryOp, OperatorChars};

mod builtins;
mod test;

// Stores the current custom operators in the language
pub struct Customs<C: CustomType> {
  bin_ops: HashMap<OperatorChars, CustomBinOp<C>>,
  unary_ops: HashMap<OperatorChars, CustomUnaryOp<C>>,
  built_ins: HashMap<String, CustomBuiltIn<C>>,
}

impl<C: CustomType> Customs<C> {
  #[cfg(test)]
  fn new() -> Self {
    Self {
      bin_ops: Default::default(),
      unary_ops: Default::default(),
      built_ins: Default::default(),
    }
  }

  pub fn new_from_hash(
    bin: HashMap<OperatorChars, CustomBinOp<C>>,
    unary: HashMap<OperatorChars, CustomUnaryOp<C>>,
    builtins: HashMap<String, CustomBuiltIn<C>>,
  ) -> Self {
    Self {
      bin_ops: bin,
      unary_ops: unary,
      built_ins: builtins,
    }
  }
}

// Interprets a specific top-level function in a template
pub fn interpret<C: CustomType>(
  temp: &Program,
  name: &str,
  arg: InterpretVal<C>,
  customs: &Customs<C>,
) -> Result<Argument<C>, InterpretError> {
  let res = {
    let frame = Frame::<C>::from_template(temp);
    if let Ok(func) = frame.find(name) {
      if let InterpretVal::Function(p) = func {
        interpret_function(&p, &mut Frame::from_template(temp), arg, customs)
      } else {
        panic!("Should be impossible to have top level expr with non function expr");
      }
    } else {
      return Err(InterpretError::new(
        format!("Could not find {}", name).as_str(),
      ));
    }
  }?;

  res.to_return_val()
}

// Recursive function for evaluating an expression
fn interpret_recurse<C: CustomType>(
  expr: &Expr,
  env: &mut Frame<C>,
  customs: &Customs<C>,
) -> Result<InterpretVal<C>, InterpretError> {
  use crate::ast::ExprInner::*;
  match &expr.val {
    Str(s) => Ok(InterpretVal::String(s.to_string())),
    Number(n) => Ok(InterpretVal::Int(*n)),
    Unary(o, e) => match o {
      UnaryOp::Not => {
        let res = interpret_recurse(&*e, env, customs)?;
        if let InterpretVal::Bool(e) = res {
          Ok(InterpretVal::Bool(!e))
        } else if let InterpretVal::Custom(c) = res {
          c.pre_not()
            .map(|v| InterpretVal::from_arg(&v))
            .map_err(|e| InterpretError::new(&e.to_string()))
        } else {
          Err(InterpretError::new(
            "Tried to apply '!' to a non boolean value.",
          ))
        }
      }
      UnaryOp::Neg => {
        let res = interpret_recurse(&*e, env, customs)?;
        if let InterpretVal::Int(i) = res {
          Ok(InterpretVal::Int(-i))
        } else if let InterpretVal::Custom(c) = res {
          c.pre_neg()
            .map(|v| InterpretVal::from_arg(&v))
            .map_err(|e| InterpretError::new(&e.to_string()))
        } else {
          Err(InterpretError::new(
            "Tried to apply '-' to a non int value.",
          ))
        }
      }
    },
    FuncCall(f, a) => {
      let arg = interpret_recurse(a, env, customs)?;
      {
        if let ExprInner::Var(n) = f.val.clone() {
          built_in(&n, arg.clone(), env, customs)
        } else {
          None
        }
      }
      .unwrap_or_else(|| {
        let val = interpret_recurse(f, env, customs)?;

        match val {
          InterpretVal::Function(p) => interpret_function(&p, env, arg, customs),
          InterpretVal::Lambda(p, mut e) => interpret_lambda(p, &mut e, arg, customs),
          _ => Err(InterpretError::new("Called value that is not a function.")),
        }
      })
    }
    Var(s) => env.find(s),
    InterpolationString(vs) => Ok(InterpretVal::String(
      vs.iter()
        .map(|p| match p {
          InterpolationPart::String(s) => Ok(s.to_string()),
          InterpolationPart::Expr(e) => Ok(interpret_recurse(e, env, customs)?.to_string()),
        })
        .fold_ok(String::new(), |s, p| s.add(p.as_str()))?,
    )),
    Op(l, o, r) => eval_op(&*l, o, &*r, env, customs),
    Tuple(v) => Ok(InterpretVal::Tuple(
      v.iter()
        .map(|e| interpret_recurse(e, env, customs))
        .collect::<Result<Vec<InterpretVal<C>>, InterpretError>>()?,
    )),
    Lambda(p) => Ok(InterpretVal::Lambda(*p.clone(), env.clone())),
    CustomBinOp(l, o, r) => {
      // Operator must exist otherwise wouldn't parse, so okay to unwrap
      let op = customs.bin_ops.get(o).unwrap();

      let l = interpret_recurse(l, env, customs)?;
      let r = interpret_recurse(r, env, customs)?;

      op.call_func(&l, &r)
    }
    CustomUnaryOp(o, r) => {
      // Operator must exist otherwise wouldn't parse, so okay to unwrap
      let op = customs.unary_ops.get(o).unwrap();

      let r = interpret_recurse(r, env, customs)?;

      op.call_func(&r)
    }
  }
  .map_err(|mut e| {
    e.add_loc(expr.start, expr.end);
    e
  })
}

// Function for evaluating a binary operation
fn eval_op<C: CustomType>(
  l: &Expr,
  op: &Opcode,
  r: &Expr,
  env: &mut Frame<C>,
  customs: &Customs<C>,
) -> Result<InterpretVal<C>, InterpretError> {
  let left = interpret_recurse(l, env, customs)?;
  let right = interpret_recurse(r, env, customs)?;

  match op {
    Opcode::Add => left.add_op(&right),
    Opcode::Sub => left.sub_op(&right),
    Opcode::Mul => left.mult_op(&right),
    Opcode::Div => left.div_op(&right),
    Opcode::Mod => left.modulo_op(&right),
    Opcode::Eq => Ok(InterpretVal::Bool(left.eq_op(&right)?)),
    Opcode::Neq => Ok(InterpretVal::Bool(left.neq_op(&right)?)),
    Opcode::Lt => Ok(InterpretVal::Bool(left.lt_op(&right)?)),
    Opcode::Gt => Ok(InterpretVal::Bool(left.gt_op(&right)?)),
    Opcode::Leq => Ok(InterpretVal::Bool(left.leq_op(&right)?)),
    Opcode::Geq => Ok(InterpretVal::Bool(left.geq_op(&right)?)),
    Opcode::And => Ok(InterpretVal::Bool(left.and_op(&right)?)),
    Opcode::Or => Ok(InterpretVal::Bool(left.or_op(&right)?)),
  }
}

// Function for checking if a pattern matches a set of arguments.
// If it does, returns an Ok with a filled frame
// Otherwise returns None
// Returns an error if a variable with the same name is assigned to twice
fn pattern_match<C: CustomType>(
  param: Expr,
  arg: InterpretVal<C>,
  env: &mut Frame<C>,
  customs: &Customs<C>,
) -> Result<Option<Frame<C>>, InterpretError> {
  let mut res = Frame::new();

  let mut stack = vec![(param.unwrap_tuple(), arg.unwrap_tuple())];

  while !stack.is_empty() {
    use crate::ast::ExprInner::*;
    let (cur_param, cur_arg) = {
      let (p1, c1) = stack.pop().unwrap();
      (p1.clone().unwrap_tuple(), c1.unwrap_tuple())
    };

    match cur_param {
      Expr {
        val: Var(s),
        start: _,
        end: _,
      } => {
        res.add_val(s.clone(), &cur_arg)?;
      }
      Expr {
        val: Tuple(s),
        start: _,
        end: _,
      } => {
        if let InterpretVal::Tuple(v) = cur_arg {
          if s.len() == v.len() {
            for (p, a) in s
              .into_iter()
              .zip(v)
              .collect::<Vec<(Expr, InterpretVal<C>)>>()
            {
              stack.push((p, a).clone())
            }
          }
        } else {
          return Ok(None);
        }
      }
      e => {
        let res = interpret_recurse(&e, env, customs)?;
        if !(res.eq_op(&cur_arg)?) {
          return Ok(None);
        }
      }
    }
  }
  res.set_next(env);

  Ok(Some(res))
}

// Interprets a function
pub fn interpret_function<C: CustomType>(
  func: &[Pattern],
  env: &mut Frame<C>,
  arg: InterpretVal<C>,
  customs: &Customs<C>,
) -> Result<InterpretVal<C>, InterpretError> {
  for p in func {
    if let Some(mut r) = pattern_match(p.start.clone(), arg.clone(), env, customs)? {
      if p
        .guards
        .iter()
        .map(|p| {
          let res = interpret_recurse(p.expr.borrow(), &mut r, customs)?;
          res.eq_op(&InterpretVal::Bool(true))
        })
        .fold_ok(true, |l, r| l && r)?
      {
        return interpret_recurse(&p.result, &mut r, customs);
      }
    }
  }

  Err(InterpretError::new("Cannot find applicable pattern."))
}

// Interprets a lambda function
fn interpret_lambda<C: CustomType>(
  func: Pattern,
  env: &mut Frame<C>,
  arg: InterpretVal<C>,
  customs: &Customs<C>,
) -> Result<InterpretVal<C>, InterpretError> {
  if let Some(mut r) = pattern_match(func.start, arg, env, customs)? {
    r.set_next(env);
    interpret_recurse(&func.result, &mut r, customs)
  } else {
    Err(InterpretError::new(
      "Lambda function did not match pattern.",
    ))
  }
}

impl<C: CustomType> CustomBinOp<C> {
  fn call_func(
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

impl<C: CustomType> CustomUnaryOp<C> {
  fn call_func(&self, val1: &InterpretVal<C>) -> Result<InterpretVal<C>, InterpretError> {
    let arg1 = val1.clone().unwrap_tuple().to_return_val()?;

    (self.function)(arg1)
      .map_err(|e| InterpretError::new(&e.to_string()))
      .map(|v| InterpretVal::from_arg(&v))
  }
}

impl<C: CustomType> CustomBuiltIn<C> {
  fn call_func(&self, val1: &InterpretVal<C>) -> Result<InterpretVal<C>, InterpretError> {
    let arg1 = val1.clone().unwrap_tuple().to_return_val()?;

    (self.function)(arg1)
      .map_err(|e| InterpretError::new(&e.to_string()))
      .map(|v| InterpretVal::from_arg(&v))
  }
}
