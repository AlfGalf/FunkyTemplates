use std::borrow::Borrow;
use std::cell::RefCell;
use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::ops::Add;

use itertools::Itertools;

use crate::ast::*;
use crate::data_types::*;
use crate::interpreter::builtins::built_in;
use crate::interpreter::string_escapes::process_string;
use crate::ReturnVal;

mod builtins;
mod string_escapes;
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
        match name {
            "true" => Ok(InterpretVal::Bool(true)),
            "false" => Ok(InterpretVal::Bool(false)),
            _ => {
                if let Some(r) = self.frame.get(name) {
                    Ok(r.clone())
                } else if let Some(n) = &self.next {
                    n.borrow().find(name)
                } else {
                    Err(InterpretError::new("Cannot find value."))
                }
            }
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
    use crate::ast::ExprInner::*;
    match &expr.val {
        Str(s) => Ok(InterpretVal::String(process_string(s)?)),
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
        Function(p) => Ok(InterpretVal::Function(p.clone())),
        FuncCall(f, a) => {
            let arg = interpret_recurse(a, env)?;
            if let ExprInner::Var(n) = f.val.clone() {
                if let Some(v) = built_in(n, arg.clone()) {
                    return v;
                }
            }
            let val = interpret_recurse(f, env)?;

            if let InterpretVal::Function(p) = val {
                interpret_function(&p, env, arg)
            } else {
                Err(InterpretError::new("Called value that is not a function."))
            }
        }
        Var(s) => env.find(s),
        InterpolationString(vs) => Ok(InterpretVal::String(
            vs.iter()
                .map(|p| match p {
                    InterpolationPart::String(s) => Ok(process_string(s)?),
                    InterpolationPart::Expr(e) => Ok(interpret_recurse(e, env)?.print()),
                })
                .fold_ok(String::new(), |s, p| s.add(p.as_str()))?,
        )),
        Op(l, o, r) => eval_op(&*l, o, &*r, env),
        Tuple(v) => Ok(InterpretVal::Tuple(
            v.iter()
                .map(|e| interpret_recurse(e, env))
                .collect::<Result<Vec<InterpretVal>, InterpretError>>()?,
        )),
    }
    .map_err(|mut e| {
        e.add_loc(expr.start, expr.end);
        e
    })
}

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
            if p.guards
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
