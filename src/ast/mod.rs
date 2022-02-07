use std::collections::HashMap;
use std::fmt::{write, Debug, Error, Formatter, Pointer};

pub struct Template {
    pub env: HashMap<String, Box<Expr>>,
}

#[derive(Clone)]
pub struct Pattern {
    pub start: Box<Expr>,
    pub result: Box<Expr>,
    pub guards: Vec<Guard>,
}

#[derive(Clone)]
pub struct Guard {
    pub expr: Box<Expr>,
}

#[derive(Clone)]
pub enum InterpolationPart {
    String(String),
    Expr(Box<Expr>),
}

#[derive(Clone)]
pub enum Expr {
    Number(i32),
    Op(Box<Expr>, Opcode, Box<Expr>),
    Unary(UnaryOp, Box<Expr>),
    FuncCall(Box<Expr>, Vec<Box<Expr>>),
    Var(String),
    Tuple(Vec<Box<Expr>>),
    Str(String),
    InterpolationString(Vec<InterpolationPart>),
    Function(Vec<Pattern>),
    Error,
}

#[derive(Copy, Clone)]
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
}

#[derive(Copy, Clone)]
pub enum UnaryOp {
    Not,
}

impl Debug for Expr {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        use self::Expr::*;
        match *self {
            Var(ref s) => write!(fmt, "{}", s),
            FuncCall(ref n, ref v) => write!(
                fmt,
                "{:?}({})",
                n,
                v.iter()
                    .map(|i| format!("{:?}", i))
                    .collect::<Vec<String>>()
                    .join(", ")
            ),
            Number(n) => write!(fmt, "{:?}", n),
            Op(ref l, op, ref r) => write!(fmt, "({:?} {:?} {:?})", l, op, r),
            Tuple(ref l) => write!(
                fmt,
                "{{{}}}",
                l.iter()
                    .map(|i| format!("{:?}", i))
                    .collect::<Vec<String>>()
                    .join(", ")
            ),
            Unary(o, ref t) => write!(fmt, "{:?}({:?})", o, t),
            Str(ref s) => write!(fmt, "\"{}\"", s),
            Error => write!(fmt, "error"),
            InterpolationString(ref s) => write!(
                fmt,
                "stringInt({})",
                s.iter()
                    .map(|i| format!("{:?}", i))
                    .collect::<Vec<String>>()
                    .join(" + ")
            ),
            Function(ref p) => write!(
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
                .map(|f| format!("#{} {:?}", *f, self.env.get(*f).unwrap()))
                .collect::<Vec<String>>()
                .join("\n")
        )
    }
}
