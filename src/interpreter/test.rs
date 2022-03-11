use std::collections::HashMap;

use crate::ast::ExprInner::Op;
use crate::interpreter::CustomOps;
#[cfg(test)]
use crate::InterpretVal;
use crate::OperatorChars::Carat;
use crate::{Argument, CustBinOp, ReturnVal};

// Creates a empty tuple, helper function for tests
#[cfg(test)]
fn blank() -> InterpretVal {
  InterpretVal::Tuple(vec![])
}

// Tests the interpreter works at all
#[test]
fn test_interpret() {
  use crate::interpreter::interpret;
  use crate::{ParserState, TemplateParser};
  let temp = TemplateParser::new()
    .parse(&ParserState::new(), "#main\n5;")
    .unwrap();
  let res = interpret(&temp, "main", blank(), &CustomOps::new());
  assert!(res.is_ok());
  assert_eq!(format!("{:?}", res.ok().unwrap()), "Int(5)");
}

// Tests lambda functions
#[test]
fn test_lambda() {
  use crate::interpreter::interpret;
  use crate::{ParserState, TemplateParser};
  let temp = TemplateParser::new()
    .parse(&ParserState::new(), "#main\n|x => 5|();")
    .unwrap();
  let res = interpret(&temp, "main", blank(), &CustomOps::new());
  // println!("{:?}", res);
  assert!(res.is_ok());
  assert_eq!(format!("{:?}", res.ok().unwrap()), "Int(5)");
}

// Tests functions work
#[test]
fn test_func() {
  use crate::interpreter::interpret;
  use crate::{ParserState, TemplateParser};
  let temp = TemplateParser::new()
    .parse(&ParserState::new(), "#one 1;#main\none();")
    .unwrap();
  let res = interpret(&temp, "main", blank(), &CustomOps::new());
  // println!("{:?}", res);
  assert!(res.is_ok());
  assert_eq!(format!("{:?}", res.ok().unwrap()), "Int(1)");
}

// Tests string interpolation
#[test]
fn test_interpolation() {
  use crate::interpreter::interpret;
  use crate::{ParserState, TemplateParser};
  let temp = TemplateParser::new()
    .parse(&ParserState::new(), "#main\nf\"test{2}test{5}\"f;")
    .unwrap();
  let res = interpret(&temp, "main", blank(), &CustomOps::new());
  // println!("{:?}", res);
  assert!(res.is_ok());
  assert_eq!(format!("{:?}", res.ok().unwrap()), "String(test2test5)");
}

// Tests the addition and subtraction operators
#[test]
fn test_add_sub() {
  use crate::interpreter::interpret;
  use crate::{ParserState, TemplateParser};
  let temp = TemplateParser::new()
    .parse(
      &ParserState::new(),
      "#main\nf\"test{2+2} {4-3} {2--1}\"f + \"test\";",
    )
    .unwrap();
  let res = interpret(&temp, "main", blank(), &CustomOps::new());
  // println!("{:?}", res);
  assert!(res.is_ok());
  assert_eq!(format!("{:?}", res.ok().unwrap()), "String(test4 1 3test)");
}

// Tests the multiply and divide operators
#[test]
fn test_mult_div() {
  use crate::interpreter::interpret;
  use crate::{ParserState, TemplateParser};
  let temp = TemplateParser::new()
    .parse(
      &ParserState::new(),
      "#main\n f\"test{2*3} {10/3} {\"test\" * 2}\"f;",
    )
    .unwrap();
  let res = interpret(&temp, "main", blank(), &CustomOps::new());
  // println!("{:?}", res);
  assert!(res.is_ok());
  assert_eq!(
    format!("{:?}", res.ok().unwrap()),
    "String(test6 3 testtest)"
  );
}

// Tests pattern matching code specifically
#[test]
fn test_pattern_match() {
  use crate::ast::*;
  use crate::data_types::*;
  use crate::interpreter::{pattern_match, Frame};
  use std::collections::HashMap;
  assert_eq!(
    pattern_match(
      Expr::var(0, "x".to_string(), 0),
      InterpretVal::Int(5),
      &mut Frame::new(),
      &CustomOps::new(),
    )
    .unwrap()
    .unwrap()
    .frame,
    {
      let mut h = HashMap::new();
      h.insert("x".to_string(), InterpretVal::Int(5));
      h
    }
  );

  assert!(pattern_match(
    Expr::tuple(
      0,
      vec![
        Expr::var(0, "x".to_string(), 0),
        Expr::var(0, "y".to_string(), 0),
      ],
      0,
    ),
    InterpretVal::Int(5),
    &mut Frame::new(),
    &CustomOps::new(),
  )
  .unwrap()
  .is_none());

  assert_eq!(
    pattern_match(
      Expr::tuple(
        0,
        vec![Expr::number(0, 5, 0), Expr::var(0, "y".to_string(), 0)],
        0,
      ),
      InterpretVal::Tuple(vec![InterpretVal::Int(5), InterpretVal::Int(4)]),
      &mut Frame::new(),
      &CustomOps::new(),
    )
    .unwrap()
    .unwrap()
    .frame,
    {
      let mut h = HashMap::new();
      h.insert("y".to_string(), InterpretVal::Int(4));
      h
    }
  );

  assert!(pattern_match(
    Expr::tuple(
      0,
      vec![
        Expr::var(0, "x".to_string(), 0),
        Expr::var(0, "x".to_string(), 0),
      ],
      0,
    ),
    InterpretVal::Tuple(vec![InterpretVal::Int(5), InterpretVal::Int(6)]),
    &mut Frame::new(),
    &CustomOps::new(),
  )
  .is_err())
}

// Tests arguments being passed into functions
#[test]
fn test_args() {
  use crate::interpreter::interpret;
  use crate::{ParserState, TemplateParser};

  let temp = TemplateParser::new()
    .parse(&ParserState::new(), "#one x -> x + 1;#main\none(2);")
    .unwrap();
  let res = interpret(
    &temp,
    "main",
    InterpretVal::Tuple(vec![]),
    &CustomOps::new(),
  );
  // println!("{:?}", res);
  assert!(res.is_ok());
  assert_eq!(format!("{:?}", res.ok().unwrap()), "Int(3)");
}

// Tests teh pattern match functionality
#[test]
fn test_pattern_match_func() {
  use crate::interpreter::interpret;
  use crate::{ParserState, TemplateParser};

  let temp = TemplateParser::new()
    .parse(
      &ParserState::new(),
      "#main (x, 1) -> x - 1; (x, y) -> x + y;x -> x + 1; ",
    )
    .unwrap();
  let res = interpret(&temp, "main", InterpretVal::Int(1), &CustomOps::new());
  assert!(res.is_ok());
  assert_eq!(format!("{:?}", res.ok().unwrap()), "Int(2)");

  let res = interpret(
    &temp,
    "main",
    InterpretVal::Tuple(vec![InterpretVal::Int(4), InterpretVal::Int(1)]),
    &CustomOps::new(),
  );
  assert!(res.is_ok());
  assert_eq!(format!("{:?}", res.ok().unwrap()), "Int(3)");

  let res = interpret(
    &temp,
    "main",
    InterpretVal::Tuple(vec![InterpretVal::Int(4), InterpretVal::Int(2)]),
    &CustomOps::new(),
  );
  assert!(res.is_ok());
  assert_eq!(format!("{:?}", res.ok().unwrap()), "Int(6)");
}

// Tests teh equality operator
#[test]
fn test_eq() {
  use crate::interpreter::interpret;
  use crate::{ParserState, TemplateParser};

  let temp = TemplateParser::new()
    .parse(&ParserState::new(), "#one x -> x == 1;#main\none(1);")
    .unwrap();
  let res = interpret(
    &temp,
    "main",
    InterpretVal::Tuple(vec![]),
    &CustomOps::new(),
  );
  // println!("{:?}", res);
  assert!(res.is_ok());
  assert_eq!(format!("{:?}", res.ok().unwrap()), "Bool(true)");

  let temp = TemplateParser::new()
    .parse(
      &ParserState::new(),
      "#one x -> x == (1, 2); #main\none(1, 2);",
    )
    .unwrap();
  let res = interpret(
    &temp,
    "main",
    InterpretVal::Tuple(vec![]),
    &CustomOps::new(),
  );
  // println!("{:?}", res);
  assert!(res.is_ok());
  assert_eq!(format!("{:?}", res.ok().unwrap()), "Bool(true)");

  let temp = TemplateParser::new()
    .parse(
      &ParserState::new(),
      "#one x -> x == (1, 2); #main\none(1, 3);",
    )
    .unwrap();
  let res = interpret(
    &temp,
    "main",
    InterpretVal::Tuple(vec![]),
    &CustomOps::new(),
  );
  // println!("{:?}", res);
  assert!(res.is_ok());
  assert_eq!(format!("{:?}", res.ok().unwrap()), "Bool(false)");
}

// Tests pattern guards
#[test]
fn test_guards() {
  use crate::interpreter::interpret;
  use crate::{ParserState, TemplateParser};

  let temp = TemplateParser::new()
    .parse(&ParserState::new(), "#main\nx -> 2|x==3;y -> 5;")
    .unwrap();
  let res = interpret(&temp, "main", InterpretVal::Int(2), &CustomOps::new());
  // println!("{:?}", res);
  assert!(res.is_ok());
  assert_eq!(format!("{:?}", res.ok().unwrap()), "Int(5)");

  let res = interpret(&temp, "main", InterpretVal::Int(3), &CustomOps::new());
  // println!("{:?}", res);
  assert!(res.is_ok());
  assert_eq!(format!("{:?}", res.ok().unwrap()), "Int(2)");
}

// Tests string escape sequences
#[test]
fn test_escapes() {
  use crate::interpreter::interpret;
  use crate::{ParserState, TemplateParser};

  let temp = TemplateParser::new()
    .parse(&ParserState::new(), "#main\n\"\\{\\}\\\\\";")
    .unwrap();
  let res = interpret(
    &temp,
    "main",
    InterpretVal::Tuple(vec![]),
    &CustomOps::new(),
  );
  // println!("{:?}", res);
  assert!(res.is_ok());
  assert_eq!(format!("{:?}", res.ok().unwrap()), "String({}\\)");

  let temp = TemplateParser::new()
    .parse(&ParserState::new(), "#main\nx -> f\"\\{\\} {x} \\\\\"f;")
    .unwrap();
  let res = interpret(&temp, "main", InterpretVal::Int(5), &CustomOps::new());
  // println!("{:?}", res);
  assert!(res.is_ok());
  assert_eq!(format!("{:?}", res.ok().unwrap()), "String({} 5 \\)");
}

// Tests that errors capture location successfully and the error is correct
#[test]
fn test_error() {
  use crate::interpreter::interpret;
  use crate::{ParserState, TemplateParser};

  let temp = TemplateParser::new()
    .parse(&ParserState::new(), "#main\n 5 + \"hi\";")
    .unwrap();
  let res = interpret(
    &temp,
    "main",
    InterpretVal::Tuple(vec![]),
    &CustomOps::new(),
  );
  // println!("{:?}", res);
  assert!(res.is_err());
  assert_eq!(
    format!("{:?}", res.err().unwrap()),
    "Interpret Error: \"Add operator not defined for Int(5) + String(\"hi\").\" loc: 7 - 15"
  );
}

// Tests that builtin functions work at all and further that the list function works
#[test]
fn test_builtin() {
  use crate::interpreter::interpret;
  use crate::{ParserState, TemplateParser};

  let temp = TemplateParser::new()
    .parse(&ParserState::new(), "#main\nget(list(1, 4, 9, 11), 2);")
    .unwrap();
  let res = interpret(
    &temp,
    "main",
    InterpretVal::Tuple(vec![]),
    &CustomOps::new(),
  );
  // println!("{:?}", res);
  // assert!(res.is_ok());
  assert_eq!(format!("{:?}", res.unwrap()), "Int(9)");

  let temp = TemplateParser::new()
    .parse(&ParserState::new(), "#main\nget(list(1, 4), 2);")
    .unwrap();
  let res = interpret(
    &temp,
    "main",
    InterpretVal::Tuple(vec![]),
    &CustomOps::new(),
  );
  // println!("{:?}", res);
  assert!(res.is_err());
  assert_eq!(
    format!(
      "{:?}",
      crate::LanguageErr::new_from_int_err(
        res.err().unwrap(),
        "#main\nget(list(1, 4), 2);".to_string(),
      )
    ),
    "Error: \"Index out of range.\"\nAt lines: 2:1 - 2:19\nCode: `get(list(1, 4), 2)`"
  );
}

// Tests the builtin map function
#[test]
fn test_map() {
  use crate::interpreter::interpret;
  use crate::{ParserState, TemplateParser};

  let temp = TemplateParser::new()
    .parse(&ParserState::new(), "#main\n x -> map(x, |i => i + 1|);")
    .unwrap();
  let res = interpret(
    &temp,
    "main",
    InterpretVal::List(vec![
      InterpretVal::Int(3),
      InterpretVal::Int(123),
      InterpretVal::Int(-123),
    ]),
    &CustomOps::new(),
  );
  // println!("{:?}", res);
  assert!(res.is_ok());
  assert_eq!(
    format!("{:?}", res.unwrap()),
    "List(Int(4), Int(124), Int(-122))"
  );
}

// Tests teh builtin filter function
#[test]
fn test_filter() {
  use crate::interpreter::interpret;
  use crate::{ParserState, TemplateParser};

  let temp = TemplateParser::new()
    .parse(
      &ParserState::new(),
      "#main\n x -> filter(x, |i => i % 3 == 0|);",
    )
    .unwrap();
  let res = interpret(
    &temp,
    "main",
    InterpretVal::List(vec![
      InterpretVal::Int(3),
      InterpretVal::Int(4),
      InterpretVal::Int(5),
      InterpretVal::Int(6),
      InterpretVal::Int(12),
      InterpretVal::Int(13),
      InterpretVal::Int(1236),
      InterpretVal::Int(1237),
    ]),
    &CustomOps::new(),
  );
  // println!("{:?}", res);
  assert!(res.is_ok());
  assert_eq!(
    format!("{:?}", res.unwrap()),
    "List(Int(3), Int(6), Int(12), Int(1236))"
  );
}

// Tests the builtin len function
#[test]
fn test_length() {
  use crate::interpreter::interpret;
  use crate::{ParserState, TemplateParser};

  let temp = TemplateParser::new()
    .parse(&ParserState::new(), "#main\n x -> len(x);")
    .unwrap();
  let res = interpret(
    &temp,
    "main",
    InterpretVal::List(vec![
      InterpretVal::Int(3),
      InterpretVal::Int(4),
      InterpretVal::Int(5),
      InterpretVal::Int(6),
      InterpretVal::Int(12),
      InterpretVal::Int(13),
      InterpretVal::Int(1236),
      InterpretVal::Int(1237),
    ]),
    &CustomOps::new(),
  );
  // println!("{:?}", res);
  assert!(res.is_ok());
  assert_eq!(format!("{:?}", res.unwrap()), "Int(8)");
}

// Tests the builtin any function
#[test]
fn test_any() {
  use crate::interpreter::interpret;
  use crate::{ParserState, TemplateParser};

  let temp = TemplateParser::new()
    .parse(
      &ParserState::new(),
      "#main\n x -> any(x, |i => i % 3 == 0|);",
    )
    .unwrap();
  let res = interpret(
    &temp,
    "main",
    InterpretVal::List(vec![
      InterpretVal::Int(3),
      InterpretVal::Int(4),
      InterpretVal::Int(5),
      InterpretVal::Int(6),
      InterpretVal::Int(12),
      InterpretVal::Int(13),
      InterpretVal::Int(1236),
      InterpretVal::Int(1237),
    ]),
    &CustomOps::new(),
  );
  // println!("{:?}", res);
  assert!(res.is_ok());
  assert_eq!(format!("{:?}", res.unwrap()), "Bool(true)");

  let temp = TemplateParser::new()
    .parse(
      &ParserState::new(),
      "#main\n x -> any(x, |i => i % 3 == 0|);",
    )
    .unwrap();
  let res = interpret(
    &temp,
    "main",
    InterpretVal::List(vec![
      InterpretVal::Int(4),
      InterpretVal::Int(5),
      InterpretVal::Int(13),
      InterpretVal::Int(1237),
    ]),
    &CustomOps::new(),
  );
  // println!("{:?}", res);
  assert!(res.is_ok());
  assert_eq!(format!("{:?}", res.unwrap()), "Bool(false)");
}

// Tests the builtin all function
#[test]
fn test_all() {
  use crate::interpreter::interpret;
  use crate::{ParserState, TemplateParser};

  let temp = TemplateParser::new()
    .parse(
      &ParserState::new(),
      "#main\n x -> all(x, |i => i % 3 == 0|);",
    )
    .unwrap();
  let res = interpret(
    &temp,
    "main",
    InterpretVal::List(vec![
      InterpretVal::Int(3),
      InterpretVal::Int(4),
      InterpretVal::Int(5),
      InterpretVal::Int(6),
      InterpretVal::Int(12),
      InterpretVal::Int(13),
      InterpretVal::Int(1236),
      InterpretVal::Int(1237),
    ]),
    &CustomOps::new(),
  );
  // println!("{:?}", res);
  assert!(res.is_ok());
  assert_eq!(format!("{:?}", res.unwrap()), "Bool(false)");

  let temp = TemplateParser::new()
    .parse(
      &ParserState::new(),
      "#main\n x -> all(x, |i => i % 3 == 0|);",
    )
    .unwrap();
  let res = interpret(
    &temp,
    "main",
    InterpretVal::List(vec![
      InterpretVal::Int(3),
      InterpretVal::Int(6),
      InterpretVal::Int(12),
      InterpretVal::Int(1236),
    ]),
    &CustomOps::new(),
  );
  // println!("{:?}", res);
  assert!(res.is_ok());
  assert_eq!(format!("{:?}", res.unwrap()), "Bool(true)");
}

// Tests the builtin fold function
#[test]
fn test_fold() {
  use crate::interpreter::interpret;
  use crate::{ParserState, TemplateParser};

  let temp = TemplateParser::new()
    .parse(
      &ParserState::new(),
      "#main\n x -> fold(x, 0, |a, i => a + i|);",
    )
    .unwrap();
  let res = interpret(
    &temp,
    "main",
    InterpretVal::List(vec![
      InterpretVal::Int(3),
      InterpretVal::Int(4),
      InterpretVal::Int(5),
      InterpretVal::Int(6),
      InterpretVal::Int(12),
      InterpretVal::Int(13),
      InterpretVal::Int(1236),
      InterpretVal::Int(1237),
    ]),
    &CustomOps::new(),
  );
  // println!("{:?}", res);
  assert!(res.is_ok());
  assert_eq!(format!("{:?}", res.unwrap()), "Int(2516)");
}

// Tests closures and environment capture
#[test]
fn test_closure() {
  use crate::interpreter::interpret;
  use crate::{ParserState, TemplateParser};

  let temp = TemplateParser::new()
    .parse(
      &ParserState::new(),
      "#closure y -> |a => a + y|; #main\n x -> closure(3)(x);",
    )
    .unwrap();
  let res = interpret(&temp, "main", InterpretVal::Int(5), &CustomOps::new());
  // println!("{:?}", res);
  assert!(res.is_ok());
  assert_eq!(format!("{:?}", res.unwrap()), "Int(8)");
}

// Tests builtin operators
#[test]
fn test_builtin_binary_operators() {
  use crate::interpreter::interpret;
  use crate::OperatorChars;
  use crate::{ParserState, TemplateParser};

  let temp = TemplateParser::new()
    .parse(
      &ParserState {
        unary_ops: vec![],
        binary_ops: vec![OperatorChars::Carat],
      },
      "#main 2 ^ 3;",
    )
    .unwrap();
  let res = interpret(
    &temp,
    "main",
    blank(),
    &CustomOps::new_from_hash(
      HashMap::from([(
        OperatorChars::Carat,
        CustBinOp {
          function: |l, r| {
            if let (ReturnVal::Int(l), ReturnVal::Int(r)) = (l, r) {
              Ok(Argument::Int(l.pow(r as u32)))
            } else {
              panic!()
            }
          },
        },
      )]),
      Default::default(),
    ),
  );
  println!("{:?}", res);
  assert!(res.is_ok());
  assert_eq!(format!("{:?}", res.unwrap()), "Int(8)");
}
