#[cfg(test)]
use crate::InterpretVal;

#[cfg(test)]
fn blank() -> InterpretVal {
  InterpretVal::Tuple(vec![])
}

#[test]
fn test_interpret() {
  use crate::interpreter::interpret;
  use crate::TemplateParser;
  let temp = TemplateParser::new().parse("#main\n5;").unwrap();
  let res = interpret(&temp, "main", blank());
  assert!(res.is_ok());
  assert_eq!(format!("{}", res.ok().unwrap()), "Int(5)");
}

#[test]
fn test_lambda() {
  use crate::interpreter::interpret;
  use crate::TemplateParser;
  let temp = TemplateParser::new().parse("#main\n|x => 5|();").unwrap();
  let res = interpret(&temp, "main", blank());
  // println!("{:?}", res);
  assert!(res.is_ok());
  assert_eq!(format!("{}", res.ok().unwrap()), "Int(5)");
}

#[test]
fn test_func() {
  use crate::interpreter::interpret;
  use crate::TemplateParser;
  let temp = TemplateParser::new().parse("#one 1;#main\none();").unwrap();
  let res = interpret(&temp, "main", blank());
  // println!("{:?}", res);
  assert!(res.is_ok());
  assert_eq!(format!("{}", res.ok().unwrap()), "Int(1)");
}

#[test]
fn test_interpolation() {
  use crate::interpreter::interpret;
  use crate::TemplateParser;
  let temp = TemplateParser::new()
    .parse("#main\nf\"test{2}test{5}\"f;")
    .unwrap();
  let res = interpret(&temp, "main", blank());
  // println!("{:?}", res);
  assert!(res.is_ok());
  assert_eq!(format!("{}", res.ok().unwrap()), "String(test2test5)");
}

#[test]
fn test_ass_sub() {
  use crate::interpreter::interpret;
  use crate::TemplateParser;
  let temp = TemplateParser::new()
    .parse("#main\nf\"test{2+2} {4-3} {2--1}\"f + \"test\";")
    .unwrap();
  let res = interpret(&temp, "main", blank());
  // println!("{:?}", res);
  assert!(res.is_ok());
  assert_eq!(format!("{}", res.ok().unwrap()), "String(test4 1 3test)");
}

#[test]
fn test_mult_div() {
  use crate::interpreter::interpret;
  use crate::TemplateParser;
  let temp = TemplateParser::new()
    .parse("#main\n f\"test{2*3} {10/3} {\"test\" * 2}\"f;")
    .unwrap();
  let res = interpret(&temp, "main", blank());
  // println!("{:?}", res);
  assert!(res.is_ok());
  assert_eq!(format!("{}", res.ok().unwrap()), "String(test6 3 testtest)");
}

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
  )
  .is_err())
}

#[test]
fn test_args() {
  use crate::interpreter::interpret;
  use crate::TemplateParser;

  let temp = TemplateParser::new()
    .parse("#one x -> x + 1;#main\none(2);")
    .unwrap();
  let res = interpret(&temp, "main", InterpretVal::Tuple(vec![]));
  // println!("{:?}", res);
  assert!(res.is_ok());
  assert_eq!(format!("{}", res.ok().unwrap()), "Int(3)");
}

#[test]
fn test_pattern_match_func() {
  use crate::interpreter::interpret;
  use crate::TemplateParser;

  let temp = TemplateParser::new()
    .parse("#main (x, 1) -> x - 1; (x, y) -> x + y;x -> x + 1; ")
    .unwrap();
  let res = interpret(&temp, "main", InterpretVal::Int(1));
  assert!(res.is_ok());
  assert_eq!(format!("{}", res.ok().unwrap()), "Int(2)");

  let res = interpret(
    &temp,
    "main",
    InterpretVal::Tuple(vec![InterpretVal::Int(4), InterpretVal::Int(1)]),
  );
  assert!(res.is_ok());
  assert_eq!(format!("{}", res.ok().unwrap()), "Int(3)");

  let res = interpret(
    &temp,
    "main",
    InterpretVal::Tuple(vec![InterpretVal::Int(4), InterpretVal::Int(2)]),
  );
  assert!(res.is_ok());
  assert_eq!(format!("{}", res.ok().unwrap()), "Int(6)");
}

#[test]
fn test_eq() {
  use crate::interpreter::interpret;
  use crate::TemplateParser;

  let temp = TemplateParser::new()
    .parse("#one x -> x == 1;#main\none(1);")
    .unwrap();
  let res = interpret(&temp, "main", InterpretVal::Tuple(vec![]));
  // println!("{:?}", res);
  assert!(res.is_ok());
  assert_eq!(format!("{}", res.ok().unwrap()), "Bool(true)");

  let temp = TemplateParser::new()
    .parse("#one x -> x == (1, 2); #main\none(1, 2);")
    .unwrap();
  let res = interpret(&temp, "main", InterpretVal::Tuple(vec![]));
  // println!("{:?}", res);
  assert!(res.is_ok());
  assert_eq!(format!("{}", res.ok().unwrap()), "Bool(true)");

  let temp = TemplateParser::new()
    .parse("#one x -> x == (1, 2); #main\none(1, 3);")
    .unwrap();
  let res = interpret(&temp, "main", InterpretVal::Tuple(vec![]));
  // println!("{:?}", res);
  assert!(res.is_ok());
  assert_eq!(format!("{}", res.ok().unwrap()), "Bool(false)");
}

#[test]
fn test_guards() {
  use crate::interpreter::interpret;
  use crate::TemplateParser;

  let temp = TemplateParser::new()
    .parse("#main\nx -> 2|x==3;y -> 5;")
    .unwrap();
  let res = interpret(&temp, "main", InterpretVal::Int(2));
  // println!("{:?}", res);
  assert!(res.is_ok());
  assert_eq!(format!("{}", res.ok().unwrap()), "Int(5)");

  let res = interpret(&temp, "main", InterpretVal::Int(3));
  // println!("{:?}", res);
  assert!(res.is_ok());
  assert_eq!(format!("{}", res.ok().unwrap()), "Int(2)");
}

#[test]
fn test_escapes() {
  use crate::interpreter::interpret;
  use crate::TemplateParser;

  let temp = TemplateParser::new()
    .parse("#main\n\"\\{\\}\\\\\";")
    .unwrap();
  let res = interpret(&temp, "main", InterpretVal::Tuple(vec![]));
  // println!("{:?}", res);
  assert!(res.is_ok());
  assert_eq!(format!("{}", res.ok().unwrap()), "String({}\\)");

  let temp = TemplateParser::new()
    .parse("#main\nx -> f\"\\{\\} {x} \\\\\"f;")
    .unwrap();
  let res = interpret(&temp, "main", InterpretVal::Int(5));
  // println!("{:?}", res);
  assert!(res.is_ok());
  assert_eq!(format!("{}", res.ok().unwrap()), "String({} 5 \\)");
}

#[test]
fn test_error() {
  use crate::interpreter::interpret;
  use crate::TemplateParser;

  let temp = TemplateParser::new().parse("#main\n 5 + \"hi\";").unwrap();
  let res = interpret(&temp, "main", InterpretVal::Tuple(vec![]));
  // println!("{:?}", res);
  assert!(res.is_err());
  assert_eq!(
    format!("{:?}", res.err().unwrap()),
    "Interpret Error: \"Add operator not defined for Int(5) + String(\"hi\").\" loc: 7 - 15"
  );
}

#[test]
fn test_builtin() {
  use crate::interpreter::interpret;
  use crate::TemplateParser;

  let temp = TemplateParser::new()
    .parse("#main\nget(list(1, 4, 9, 11), 2);")
    .unwrap();
  let res = interpret(&temp, "main", InterpretVal::Tuple(vec![]));
  // println!("{:?}", res);
  // assert!(res.is_ok());
  assert_eq!(format!("{:?}", res.unwrap()), "Int(9)");

  let temp = TemplateParser::new()
    .parse("#main\nget(list(1, 4), 2);")
    .unwrap();
  let res = interpret(&temp, "main", InterpretVal::Tuple(vec![]));
  // println!("{:?}", res);
  assert!(res.is_err());
  assert_eq!(
    format!(
      "{:?}",
      crate::LanguageErr::from_int_err(
        res.err().unwrap(),
        "#main\nget(list(1, 4), 2);".to_string(),
      )
    ),
    "Error: \"Index out of range.\"\nAt lines: 2 - 2\nCode: `get(list(1, 4), 2)`"
  );
}

#[test]
fn test_list() {
  use crate::interpreter::interpret;
  use crate::TemplateParser;

  let temp = TemplateParser::new()
    .parse("#main\n x -> map(x, |i => i + 1|);")
    .unwrap();
  let res = interpret(
    &temp,
    "main",
    InterpretVal::List(vec![
      InterpretVal::Int(3),
      InterpretVal::Int(123),
      InterpretVal::Int(-123),
    ]),
  );
  // println!("{:?}", res);
  assert!(res.is_ok());
  assert_eq!(
    format!("{}", res.unwrap()),
    "List(Int(4), Int(124), Int(-122))"
  );
}

#[test]
fn test_filter() {
  use crate::interpreter::interpret;
  use crate::TemplateParser;

  let temp = TemplateParser::new()
    .parse("#main\n x -> filter(x, |i => i % 3 == 0|);")
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
  );
  // println!("{:?}", res);
  assert!(res.is_ok());
  assert_eq!(
    format!("{}", res.unwrap()),
    "List(Int(3), Int(6), Int(12), Int(1236))"
  );
}

#[test]
fn test_length() {
  use crate::interpreter::interpret;
  use crate::TemplateParser;

  let temp = TemplateParser::new().parse("#main\n x -> len(x);").unwrap();
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
  );
  // println!("{:?}", res);
  assert!(res.is_ok());
  assert_eq!(format!("{}", res.unwrap()), "Int(8)");
}

#[test]
fn test_any() {
  use crate::interpreter::interpret;
  use crate::TemplateParser;

  let temp = TemplateParser::new()
    .parse("#main\n x -> any(x, |i => i % 3 == 0|);")
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
  );
  // println!("{:?}", res);
  assert!(res.is_ok());
  assert_eq!(format!("{}", res.unwrap()), "Bool(true)");

  let temp = TemplateParser::new()
    .parse("#main\n x -> any(x, |i => i % 3 == 0|);")
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
  );
  // println!("{:?}", res);
  assert!(res.is_ok());
  assert_eq!(format!("{}", res.unwrap()), "Bool(false)");
}

#[test]
fn test_all() {
  use crate::interpreter::interpret;
  use crate::TemplateParser;

  let temp = TemplateParser::new()
    .parse("#main\n x -> all(x, |i => i % 3 == 0|);")
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
  );
  // println!("{:?}", res);
  assert!(res.is_ok());
  assert_eq!(format!("{}", res.unwrap()), "Bool(false)");

  let temp = TemplateParser::new()
    .parse("#main\n x -> any(x, |i => i % 3 == 0|);")
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
  );
  // println!("{:?}", res);
  assert!(res.is_ok());
  assert_eq!(format!("{}", res.unwrap()), "Bool(true)");
}
