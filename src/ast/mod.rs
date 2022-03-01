use std::collections::HashMap;
use std::fmt::{Debug, Error, Formatter};

use itertools::Itertools;

pub struct Template {
  pub env: HashMap<String, Vec<Pattern>>,
}

#[derive(Clone, PartialEq)]
pub struct Pattern {
  pub start: Expr,
  pub result: Expr,
  pub guards: Vec<Guard>,
}

#[derive(Clone, PartialEq)]
pub struct Guard {
  pub expr: Expr,
}

#[derive(Clone, PartialEq)]
pub enum InterpolationPart {
  String(String),
  Expr(Expr),
}

#[derive(Clone, PartialEq)]
pub enum ExprInner {
  Number(i32),
  Op(Box<Expr>, Opcode, Box<Expr>),
  Unary(UnaryOp, Box<Expr>),
  FuncCall(Box<Expr>, Box<Expr>),
  Var(String),
  Tuple(Vec<Expr>),
  Str(String),
  InterpolationString(Vec<InterpolationPart>),
  Function(Vec<Pattern>),
}

#[derive(Clone, PartialEq)]
pub struct Expr {
  pub val: ExprInner,
  pub start: usize,
  pub end: usize,
}

impl Expr {
  fn new(start: usize, val: ExprInner, end: usize) -> Self {
    Self { val, start, end }
  }

  pub fn number(start: usize, v: i32, end: usize) -> Self {
    Self::new(start, ExprInner::Number(v), end)
  }

  pub fn op(start: usize, v1: Expr, v2: Opcode, v3: Expr, end: usize) -> Self {
    Self::new(start, ExprInner::Op(Box::new(v1), v2, Box::new(v3)), end)
  }

  pub fn unary(start: usize, v1: UnaryOp, v2: Expr, end: usize) -> Self {
    Self::new(start, ExprInner::Unary(v1, Box::new(v2)), end)
  }

  pub fn func_call(start: usize, v1: Expr, v2: Expr, end: usize) -> Self {
    Self::new(start, ExprInner::FuncCall(Box::new(v1), Box::new(v2)), end)
  }

  pub fn var(start: usize, v1: String, end: usize) -> Self {
    Self::new(start, ExprInner::Var(v1), end)
  }

  pub fn tuple(start: usize, v1: Vec<Expr>, end: usize) -> Self {
    Self::new(start, ExprInner::Tuple(v1), end)
  }

  pub fn string(start: usize, v1: String, end: usize) -> Self {
    Self::new(start, ExprInner::Str(v1), end)
  }

  pub fn interpolation_string(start: usize, v1: Vec<InterpolationPart>, end: usize) -> Self {
    Self::new(start, ExprInner::InterpolationString(v1), end)
  }

  pub fn function(start: usize, v1: Vec<Pattern>, end: usize) -> Self {
    Self::new(start, ExprInner::Function(v1), end)
  }

  pub fn unwrap_tuple(self) -> Self {
    match self {
      Expr {
        val: ExprInner::Tuple(s),
        start: l,
        end: r,
      } => {
        if s.len() == 1 {
          s.get(0).unwrap().clone()
        } else {
          Expr {
            val: ExprInner::Tuple(s),
            start: l,
            end: r,
          }
        }
      }
      _ => self,
    }
  }
}

#[derive(Copy, Clone, PartialEq)]
pub enum Opcode {
  Mul,
  Div,
  Mod,
  Add,
  Sub,
  Eq,
  Leq,
  Lt,
  Geq,
  Gt,
  And,
  Or,
  Neq,
}

#[derive(Copy, Clone, PartialEq)]
pub enum UnaryOp {
  Not,
  Neg,
}

impl Debug for Expr {
  fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
    match self.val {
      ExprInner::Var(ref s) => write!(fmt, "{}", s),
      ExprInner::FuncCall(ref n, ref v) => write!(fmt, "{:?}({:?})", n, v),
      ExprInner::Number(n) => write!(fmt, "{:?}", n),
      ExprInner::Op(ref l, op, ref r) => write!(fmt, "({:?} {:?} {:?})", l, op, r),
      ExprInner::Tuple(ref l) => write!(
        fmt,
        "{{{}}}",
        l.iter()
          .map(|i| format!("{:?}", i))
          .collect::<Vec<String>>()
          .join(", ")
      ),
      ExprInner::Unary(o, ref t) => write!(fmt, "{:?}({:?})", o, t),
      ExprInner::Str(ref s) => write!(fmt, "\"{}\"", s),
      ExprInner::InterpolationString(ref s) => write!(
        fmt,
        "stringInt({})",
        s.iter()
          .map(|i| format!("{:?}", i))
          .collect::<Vec<String>>()
          .join(" + ")
      ),
      ExprInner::Function(ref p) => write!(
        fmt,
        "|{}|",
        p.iter()
          .map(|i| format!("{:?}", i))
          .collect::<Vec<String>>()
          .join("\n"),
      ),
    }
  }
}

impl Debug for InterpolationPart {
  fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
    use self::InterpolationPart::*;
    match self {
      String(s) => write!(fmt, "\"{}\"", s),
      Expr(e) => write!(fmt, "{:?}", e),
    }
  }
}

impl Debug for Opcode {
  fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
    use self::Opcode::*;
    match *self {
      Mul => write!(fmt, "*"),
      Div => write!(fmt, "/"),
      Mod => write!(fmt, "%"),
      Add => write!(fmt, "+"),
      Sub => write!(fmt, "-"),
      Eq => write!(fmt, "=="),
      Neq => write!(fmt, "!="),
      Leq => write!(fmt, "<="),
      Lt => write!(fmt, "<"),
      Geq => write!(fmt, ">="),
      Gt => write!(fmt, ">"),
      And => write!(fmt, "&&"),
      Or => write!(fmt, "||"),
    }
  }
}

impl Debug for UnaryOp {
  fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
    use self::UnaryOp::*;
    match *self {
      Not => write!(fmt, "!"),
      Neg => write!(fmt, "-"),
    }
  }
}

impl Debug for Pattern {
  fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
    write!(fmt, "{:?} -> {:?}", self.start, self.result)
  }
}

impl Debug for Template {
  fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
    let mut funcs: Vec<&String> = self.env.keys().into_iter().collect();
    funcs.sort();
    write!(
      fmt,
      "{}",
      funcs
        .iter()
        .map(|f| format!(
          "#{} {}",
          *f,
          self
            .env
            .get(*f)
            .unwrap()
            .iter()
            .map(|p| format!("{:?}", p))
            .join("\n")
        ))
        .collect::<Vec<String>>()
        .join("\n")
    )
  }
}
