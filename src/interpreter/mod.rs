use std::cell::RefCell;
use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::ops::Add;

use itertools::Itertools;

use crate::ast::*;
use crate::data_types::*;

mod test;

/// Frame for holding the environment in an execution of a program
#[derive(Clone)]
struct Frame {
    frame: HashMap<String, InterpretVal>,
    next: Option<RefCell<Box<Frame>>>,
}

impl Frame {
    fn new() -> Self {
        Self {
            frame: HashMap::new(),
            next: None,
        }
    }

    fn from_template(t: &Template) -> Self {
        Self {
            frame: t
                .env
                .iter()
                .map(|(a, b)| (a.clone(), InterpretVal::Function(b.clone())))
                .collect(),
            next: None,
        }
    }

    fn add_val(&mut self, name: String, expr: &InterpretVal) -> Result<(), InterpretError> {
        if let Entry::Vacant(e) = self.frame.entry(name) {
            e.insert(expr.clone());
            Ok(())
        } else {
            Err(InterpretError::new(
                "Multiple variables within the same frame.",
            ))
        }
    }

    fn find(&self, name: &str) -> Result<InterpretVal, InterpretError> {
        if let Some(r) = self.frame.get(name) {
            Ok(r.clone())
        } else if let Some(n) = &self.next {
            n.borrow().find(name)
        } else {
            Err(InterpretError::new("Cannot find value."))
        }
    }
}

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
                let res = interpret_recurse(e, env)?;
                if let InterpretVal::Bool(e) = res {
                    Ok(InterpretVal::Bool(!e))
                } else {
                    Err(InterpretError::new(
                        "Tried to apply '!' to a non boolean value.",
                    ))
                }
            }
            UnaryOp::Neg => {
                let res = interpret_recurse(e, env)?;
                if let InterpretVal::Int(i) = res {
                    Ok(InterpretVal::Int(-i))
                } else {
                    Err(InterpretError::new(
                        "Tried to apply '-' to a non int value.",
                    ))
                }
            }
        },
        Expr::Function(p) => Ok(InterpretVal::Function(p.clone())),
        Expr::FuncCall(f, a) => {
            let val = interpret_recurse(f, env)?;
            let arg = interpret_recurse(&Expr::Tuple(a.clone()), env)?;
            if let InterpretVal::Function(p) = val {
                interpret_function(&p, env, arg)
            } else {
                Err(InterpretError::new(
                    "Called something that was not a function",
                ))
            }
        }
        Expr::Var(s) => env.find(s).clone(),
        Expr::InterpolationString(vs) => Ok(InterpretVal::String(
            vs.iter()
                .map(|p| match p {
                    InterpolationPart::String(s) => Ok(s.to_string()),
                    InterpolationPart::Expr(e) => Ok(interpret_recurse(e, env)?.print()),
                })
                .fold_ok(String::new(), |s, p| s.add(p.as_str()))?,
        )),
        Expr::Op(l, o, r) => eval_op(l, o, r, env),
        Expr::Tuple(v) => Ok(InterpretVal::Tuple(
            v.iter()
                .map(|e| interpret_recurse(e, env))
                .collect::<Result<Vec<InterpretVal>, InterpretError>>()?,
        )),

        e => Err(InterpretError::new(
            format!("Unrecognised node type {:?}.", e).as_str(),
        )),
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
                format!("Add operator not defined for {:?}.", t).as_str(),
            )),
        },
        Opcode::Sub => {
            if let (InterpretVal::Int(l), InterpretVal::Int(r)) = (left, right) {
                Ok(InterpretVal::Int(l - r))
            } else {
                Err(InterpretError::new("Cannot subtract non string."))
            }
        }
        Opcode::Mul => match (left, right) {
            (InterpretVal::Int(l), InterpretVal::Int(r)) => Ok(InterpretVal::Int(l * r)),
            (InterpretVal::String(l), InterpretVal::Int(r)) => {
                if r >= 0 {
                    Ok(InterpretVal::String(l.repeat(r as usize)))
                } else {
                    Err(InterpretError::new(
                        "Cannot repeat a string a negative number of times",
                    ))
                }
            }
            (l, r) => Err(InterpretError::new(
                format!("Invalid operators for multiplication: {:?} * {:?}", l, r).as_str(),
            )),
        },
        Opcode::Div => match (left, right) {
            (InterpretVal::Int(l), InterpretVal::Int(r)) => Ok(InterpretVal::Int(l / r)),
            (l, r) => Err(InterpretError::new(
                format!("Invalid operators for division: {:?} * {:?}", l, r).as_str(),
            )),
        },
        _ => todo!("Other op types"),
    }
}

fn pattern_match(
    param: Box<Expr>,
    arg: InterpretVal,
    env: &mut Frame,
) -> Result<Option<Frame>, InterpretError> {
    let mut res = Frame::new();

    let mut stack = vec![(param, arg.unwrap_tuple())];
    //     if let InterpretVal::Tuple(s) = arg {
    //         if s.len() == 1 {
    //             vec![(&param, s.first().unwrap().clone())]
    //         } else {
    //             vec![(&param, InterpretVal::Tuple(s))]
    //         }
    //     } else {
    //         vec![(&param, arg)]
    //     }
    // };

    while !stack.is_empty() {
        let (cur_param, cur_arg) = {
            let (p1, c1) = stack.pop().unwrap();
            (p1.clone().unwrap_tuple(), c1.unwrap_tuple())
        };

        match cur_param {
            Expr::Var(s) => {
                res.add_val(s.clone(), &cur_arg)?;
            }
            Expr::Tuple(s) => {
                if let InterpretVal::Tuple(v) = cur_arg {
                    if s.len() == v.len() {
                        for (p, a) in s
                            .into_iter()
                            .zip(v)
                            .collect::<Vec<(Box<Expr>, InterpretVal)>>()
                        {
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
    res.next = Some(RefCell::new(Box::new(env.clone())));

    Ok(Some(res))
}

fn interpret_function(
    func: &[Pattern],
    env: &mut Frame,
    arg: InterpretVal,
) -> Result<InterpretVal, InterpretError> {
    for p in func {
        if let Some(mut r) = pattern_match(p.start.clone(), arg.clone(), env)? {
            return interpret_recurse(&*p.result, &mut r);
        }
    }

    Err(InterpretError::new("Cannot find applicable pattern"))
}

fn unwrap_expr(e: Box<Expr>) -> Box<Expr> {
    if let Expr::Tuple(s) = *e {
        if s.len() == 1 {
            s[0].clone()
        } else {
            Box::new(Expr::Tuple(s))
        }
    } else {
        e
    }
}
