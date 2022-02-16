use std::any::Any;
use std::collections::HashMap;
use std::ops::Add;

use itertools::Itertools;

use crate::ast::*;
use crate::data_types::*;

mod test;

struct Frame {
    frame: HashMap<String, Expr>,
    next: Option<Box<Frame>>,
}

impl Frame {
    fn new(t: &Template) -> Self {
        Self {
            frame: t.env.clone(),
            next: None,
        }
    }

    fn add(&mut self, name: String, expr: &Expr) {
        self.frame.insert(name, expr.clone());
    }

    fn find(&self, name: &str) -> Result<&Expr, InterpretError> {
        self.frame
            .get(name)
            .ok_or_else(|| InterpretError::new("Could not find variable."))
    }
}

fn print_type_of<T>(_: &T) -> String {
    std::any::type_name::<T>().to_string()
}

pub fn interpret(temp: &Template, name: &str) -> Result<ReturnVal, InterpretError> {
    let res = {
        if let Some(func) = temp.env.get(name) {
            if let Expr::Function(f) = func {
                interpret_function(f, &mut Frame::new(temp))
            } else {
                panic!("Should be impossible to have top level expr with non function expr");
            }
        } else {
            return Err(InterpretError::new(
                format!("Could not find {}", name).as_str(),
            ));
        }
    }?;

    Ok({
        match res {
            InterpretVal::Int(i) => ReturnVal::Int(i),
            InterpretVal::Bool(b) => ReturnVal::Bool(b),
            InterpretVal::String(s) => ReturnVal::String(s),
            _ => todo!("Other types"),
        }
    })
}

fn interpret_recurse(expr: &Expr, env: &mut Frame) -> Result<InterpretVal, InterpretError> {
    match expr {
        Expr::Str(s) => Ok(InterpretVal::String(s.clone())),
        Expr::Number(n) => Ok(InterpretVal::Int(*n)),
        Expr::Unary(o, e) => match o {
            UnaryOp::Not => {
                let res = interpret_recurse(e, env);
                if let Ok(InterpretVal::Bool(e)) = res {
                    Ok(InterpretVal::Bool(!e))
                } else {
                    Err(InterpretError::new(
                        "Tried to apply '!' to a non boolean value.",
                    ))
                }
            }
        },
        Expr::Function(p) => Ok(InterpretVal::Function(p.clone())),
        Expr::FuncCall(f, _) => {
            let val = interpret_recurse(f, env)?;
            if let InterpretVal::Function(p) = val {
                interpret_function(&p, env)
            } else {
                Err(InterpretError::new(
                    "Called something that was not a function",
                ))
            }
        }
        Expr::Var(s) => interpret_recurse(&env.find(s)?.clone(), env),
        Expr::InterpolationString(vs) => Ok(InterpretVal::String(
            vs.iter()
                .map(|p| match p {
                    InterpolationPart::String(s) => Ok(s.to_string()),
                    InterpolationPart::Expr(e) => Ok(interpret_recurse(e, env)?.print()),
                })
                .fold_ok(String::new(), |s, p| s.add(p.as_str()))?,
        )),
        Expr::Op(l, o, r) => eval_op(l, o, r, env),

        _ => Err(InterpretError::new("Unrecognised node type.")),
    }
}

fn eval_op(
    l: &Box<Expr>,
    op: &Opcode,
    r: &Box<Expr>,
    env: &mut Frame,
) -> Result<InterpretVal, InterpretError> {
    let left = interpret_recurse(l, env)?;
    let right = interpret_recurse(r, env)?;

    match op {
        Opcode::Add => match left {
            InterpretVal::String(s) => {
                if let InterpretVal::String(r) = right {
                    Ok(InterpretVal::String(s.add(r.as_str())))
                } else {
                    Err(InterpretError::new("Cannot add non string to sting."))
                }
            }
            InterpretVal::Int(i) => {
                if let InterpretVal::Int(r) = right {
                    Ok(InterpretVal::Int(i + r))
                } else {
                    Err(InterpretError::new("Cannot add non int to int."))
                }
            }
            t => Err(InterpretError::new(
                format!("Add operator not defined for {}.", print_type_of(&t)).as_str(),
            )),
        },
        _ => todo!("Other op types"),
    }
}

fn interpret_function(func: &[Pattern], env: &mut Frame) -> Result<InterpretVal, InterpretError> {
    // TODO: Match pattern
    // TODO: use params

    // for p in func {
    // }

    let pat = func
        .first()
        .ok_or_else(|| InterpretError::new("Cannot find applicable pattern"))?;
    interpret_recurse(&pat.result, env)
}
