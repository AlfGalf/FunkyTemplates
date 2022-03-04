use std::borrow::Borrow;
use std::ops::Add;

use itertools::Itertools;

use crate::ast::*;
use crate::data_types::*;
use crate::interpreter::builtins::built_in;
use crate::ReturnVal;

mod builtins;
mod test;

// Interprets a specific top-level function in a template
pub fn interpret(
  temp: &Template,
  name: &str,
  arg: InterpretVal,
) -> Result<ReturnVal, InterpretError> {
  let res = {
    let frame = Frame::from_template(temp);
    if let Ok(func) = frame.find(name) {
      if let InterpretVal::Function(p) = func {
        interpret_function(&p, &mut Frame::from_template(temp), arg)
      } else {
        panic!("Should be impossible to have top level expr with non function expr");
      }
    } else {
      return Err(InterpretError::new(
        format!("Could not find {}", name).as_str(),
      ));
    }
  }?;

  interpret_val_to_return(&res)
}

// Converts an interpret value to a return val that can be returned through the API
fn interpret_val_to_return(i: &InterpretVal) -> Result<ReturnVal, InterpretError> {
  match i {
    InterpretVal::Int(i) => Ok(ReturnVal::Int(*i)),
    InterpretVal::Bool(b) => Ok(ReturnVal::Bool(*b)),
    InterpretVal::String(s) => Ok(ReturnVal::String(s.clone())),
    InterpretVal::Tuple(v) => Ok(ReturnVal::Tuple(
      v.iter()
        .map(interpret_val_to_return)
        .collect::<Result<Vec<ReturnVal>, InterpretError>>()?,
    )),
    InterpretVal::List(v) => Ok(ReturnVal::List(
      v.iter()
        .map(interpret_val_to_return)
        .collect::<Result<Vec<ReturnVal>, InterpretError>>()?,
    )),
    InterpretVal::Function(_) => Err(InterpretError::new(
      "Cannot have function return type to root.",
    )),
    InterpretVal::Lambda(_, _) => Err(InterpretError::new(
      "Cannot have lambda return type to root.",
    )),
  }
}

// Recursive function for evaluating an expression
fn interpret_recurse(expr: &Expr, env: &mut Frame) -> Result<InterpretVal, InterpretError> {
  use crate::ast::ExprInner::*;
  match &expr.val {
    Str(s) => Ok(InterpretVal::String(s.to_string())),
    Number(n) => Ok(InterpretVal::Int(*n)),
    Unary(o, e) => match o {
      UnaryOp::Not => {
        let res = interpret_recurse(&*e, env)?;
        if let InterpretVal::Bool(e) = res {
          Ok(InterpretVal::Bool(!e))
        } else {
          Err(InterpretError::new(
            "Tried to apply '!' to a non boolean value.",
          ))
        }
      }
      UnaryOp::Neg => {
        let res = interpret_recurse(&*e, env)?;
        if let InterpretVal::Int(i) = res {
          Ok(InterpretVal::Int(-i))
        } else {
          Err(InterpretError::new(
            "Tried to apply '-' to a non int value.",
          ))
        }
      }
    },
    FuncCall(f, a) => {
      let arg = interpret_recurse(a, env)?;
      {
        if let ExprInner::Var(n) = f.val.clone() {
          built_in(n, arg.clone(), env)
        } else {
          None
        }
      }
      .unwrap_or_else(|| {
        let val = interpret_recurse(f, env)?;

        match val {
          InterpretVal::Function(p) => interpret_function(&p, env, arg),
          InterpretVal::Lambda(p, mut e) => interpret_lambda(p, &mut e, arg),
          _ => Err(InterpretError::new("Called value that is not a function.")),
        }
      })
    }
    Var(s) => env.find(s),
    InterpolationString(vs) => Ok(InterpretVal::String(
      vs.iter()
        .map(|p| match p {
          InterpolationPart::String(s) => Ok(s.to_string()),
          InterpolationPart::Expr(e) => Ok(interpret_recurse(e, env)?.to_string()),
        })
        .fold_ok(String::new(), |s, p| s.add(p.as_str()))?,
    )),
    Op(l, o, r) => eval_op(&*l, o, &*r, env),
    Tuple(v) => Ok(InterpretVal::Tuple(
      v.iter()
        .map(|e| interpret_recurse(e, env))
        .collect::<Result<Vec<InterpretVal>, InterpretError>>()?,
    )),
    Lambda(p) => Ok(InterpretVal::Lambda(*p.clone(), env.clone())),
    CustomBinOp(_, _, _) => todo!(),
    CustomUnaryOp(_, _) => todo!(),
  }
  .map_err(|mut e| {
    e.add_loc(expr.start, expr.end);
    e
  })
}

// Function for evaluating a binary operation
fn eval_op(
  l: &Expr,
  op: &Opcode,
  r: &Expr,
  env: &mut Frame,
) -> Result<InterpretVal, InterpretError> {
  let left = interpret_recurse(l, env)?;
  let right = interpret_recurse(r, env)?;

  match op {
    Opcode::Add => left.add_op(&right),
    Opcode::Sub => left.sub_op(&right),
    Opcode::Mul => left.mult_op(&right),
    Opcode::Div => left.div_op(&right),
    Opcode::Mod => left.modulo_op(&right),
    Opcode::Eq => left.eq_op(&right),
    Opcode::Neq => left.neq_op(&right),
    Opcode::Lt => left.lt_op(&right),
    Opcode::Gt => left.gt_op(&right),
    Opcode::Leq => left.leq_op(&right),
    Opcode::Geq => left.geq_op(&right),
    Opcode::And => left.and_op(&right),
    Opcode::Or => left.or_op(&right),
  }
}

// Function for checking if a pattern matches a set of arguments.
// If it does, returns an Ok with a filled frame
// Otherwise returns None
// Returns an error if a variable with the same name is assigned to twice
fn pattern_match(
  param: Expr,
  arg: InterpretVal,
  env: &mut Frame,
) -> Result<Option<Frame>, InterpretError> {
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
            for (p, a) in s.into_iter().zip(v).collect::<Vec<(Expr, InterpretVal)>>() {
              stack.push((p, a).clone())
            }
          }
        } else {
          return Ok(None);
        }
      }
      e => {
        let res = interpret_recurse(&e, env)?;
        if !(res == cur_arg) {
          return Ok(None);
        }
      }
    }
  }
  res.set_next(env);

  Ok(Some(res))
}

// Interprets a function
fn interpret_function(
  func: &[Pattern],
  env: &mut Frame,
  arg: InterpretVal,
) -> Result<InterpretVal, InterpretError> {
  for p in func {
    if let Some(mut r) = pattern_match(p.start.clone(), arg.clone(), env)? {
      if p
        .guards
        .iter()
        .map(|p| {
          let res = interpret_recurse(p.expr.borrow(), &mut r)?;
          Ok(res == InterpretVal::Bool(true))
        })
        .fold_ok(true, |l, r| l && r)?
      {
        return interpret_recurse(&p.result, &mut r);
      }
    }
  }

  Err(InterpretError::new("Cannot find applicable pattern."))
}

// Interprets a lambda function
fn interpret_lambda(
  func: Pattern,
  env: &mut Frame,
  arg: InterpretVal,
) -> Result<InterpretVal, InterpretError> {
  if let Some(mut r) = pattern_match(func.start, arg, env)? {
    r.set_next(env);
    interpret_recurse(&func.result, &mut r)
  } else {
    Err(InterpretError::new(
      "Lambda function did not match pattern.",
    ))
  }
}
