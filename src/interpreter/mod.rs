use std::collections::HashMap;

use crate::ast::*;
use crate::data_types::*;

mod test;

struct Frame {
    frame: HashMap<String, Box<Expr>>,
    next: Option<Box<Frame>>,
}

impl Frame {
    fn new(t: &Template) -> Self {
        Self {
            frame: t.env.clone(),
            next: None,
        }
    }

    fn add(&mut self, name: String, expr: Box<Expr>) {
        self.frame.insert(name, expr);
    }
}

pub fn interpret(temp: &Template, name: &str) -> Result<ReturnVal, InterpretError> {
    let res = {
        if let Some(func) = temp.env.get(name) {
            if let Expr::Function(f) = &**func {
                interpret_function(&f, &mut Frame::new(temp))
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
            _ => panic!("Unrecognised type"),
        }
    })
}

fn interpret_recurse(expr: &Box<Expr>, env: &mut Frame) -> Result<InterpretVal, InterpretError> {
    match &**expr {
        Expr::Str(s) => Ok(InterpretVal::String(s.clone())),
        Expr::Number(n) => Ok(InterpretVal::Int(n.clone())),
        Expr::Unary(o, e) => match o {
            UnaryOp::Not => {
                let res = interpret_recurse(&e, env);
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
        Expr::FuncCall(f, p) => {
            let val = interpret_recurse(&f, env)?;
            if let InterpretVal::Function(p) = val {
                interpret_function(&p, env)
            } else {
                Err(InterpretError::new(
                    "Called something that was not a function",
                ))
            }
        }
        _ => Err(InterpretError::new("Unrecognised node type.")),
    }
}

fn interpret_function(
    func: &Vec<Pattern>,
    env: &mut Frame,
) -> Result<InterpretVal, InterpretError> {
    // TODO: Match pattern
    // TODO: use params
    let pat = func
        .first()
        .ok_or(InterpretError::new("Cannot find applicable pattern"))?;
    return interpret_recurse(&pat.result, env);
}
