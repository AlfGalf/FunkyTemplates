use std::fmt::{Debug, Error, Formatter};

pub struct Top {
    pub functions: Vec<Function>,
}

pub struct Function {
    pub name: String,
    pub patterns: Vec<Pattern>,
}

pub struct Pattern {
    pub start: String,
    pub result: Box<Expr>,
    pub guards: Vec<Guard>,
}

pub struct Guard {}

pub enum Expr {
    Number(i32),
    Op(Box<Expr>, Opcode, Box<Expr>),
    FuncCall(String, Vec<Box<Expr>>),
    Var(String),
    Tuple(Vec<Box<Expr>>),
    Error,
}

pub enum ExprSymbol {
    Op(Box<ExprSymbol>, Opcode, Box<ExprSymbol>),
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
}

impl Debug for Expr {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        use self::Expr::*;
        match *self {
            Var(ref s) => write!(fmt, "{}", s),
            FuncCall(ref n, ref v) => write!(fmt, "{}({})", n, v.iter().map(|i| format!("{:?}", i)).collect::<Vec<String>>().join(", ")),
            Number(n) => write!(fmt, "{:?}", n),
            Op(ref l, op, ref r) => write!(fmt, "({:?} {:?} {:?})", l, op, r),
            Tuple(ref l) => write!(fmt, "{{{}}}", l.iter().map(|i| format!("{:?}", i)).collect::<Vec<String>>().join(", ")),
            Error => write!(fmt, "error"),
        }
    }
}

impl<'input> Debug for ExprSymbol {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        use self::ExprSymbol::*;
        match *self {
            Op(ref l, op, ref r) => write!(fmt, "({:?} {:?} {:?})", l, op, r),
            Error => write!(fmt, "error"),
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
        }
    }
}

impl Debug for Pattern {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        write!(fmt, "{} -> {:?}", self.start, self.result)
    }
}

impl Debug for Function {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        write!(fmt, "#{}\n{}", self.name, self.patterns.iter()
            .map(|p| format!("{:?}", p)).collect::<Vec<String>>().join("\n"))
    }
}
