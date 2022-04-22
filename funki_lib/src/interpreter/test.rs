#[cfg(test)]
use crate::CustomType;
#[cfg(test)]
use crate::InterpretVal;

// Creates a empty tuple, helper function for tests
#[cfg(test)]
fn blank<C: CustomType>() -> InterpretVal<C> {
  InterpretVal::Tuple(vec![])
}

// Tests the interpreter works at all
#[test]
fn test_interpret() {
  use crate::interpreter::interpret;
  use crate::{BlankCustom, Customs, ParserState, ProgramParser};
  let temp = ProgramParser::new()
    .parse(&ParserState::new(), "#main\n5;")
    .unwrap();
  let res = interpret::<BlankCustom>(&temp, "main", blank(), &Customs::new());
  assert!(res.is_ok());
  assert_eq!(format!("{:?}", res.ok().unwrap()), "Int(5)");
}

// Tests lambda functions
#[test]
fn test_lambda() {
  use crate::interpreter::interpret;
  use crate::{BlankCustom, Customs, ParserState, ProgramParser};
  let temp = ProgramParser::new()
    .parse(&ParserState::new(), "#main\n|x => 5|();")
    .unwrap();
  let res = interpret::<BlankCustom>(&temp, "main", blank(), &Customs::new());
  // println!("{:?}", res);
  assert!(res.is_ok());
  assert_eq!(format!("{:?}", res.ok().unwrap()), "Int(5)");
}

// Tests functions work
#[test]
fn test_func() {
  use crate::interpreter::interpret;
  use crate::{BlankCustom, Customs, ParserState, ProgramParser};
  let temp = ProgramParser::new()
    .parse(&ParserState::new(), "#one 1;#main\none();")
    .unwrap();
  let res = interpret::<BlankCustom>(&temp, "main", blank(), &Customs::new());
  // println!("{:?}", res);
  assert!(res.is_ok());
  assert_eq!(format!("{:?}", res.ok().unwrap()), "Int(1)");
}

// Tests string interpolation
#[test]
fn test_interpolation() {
  use crate::interpreter::interpret;
  use crate::{BlankCustom, Customs, ParserState, ProgramParser};
  let temp = ProgramParser::new()
    .parse(&ParserState::new(), "#main\nf\"test{2}test{5}\"f;")
    .unwrap();
  let res = interpret::<BlankCustom>(&temp, "main", blank(), &Customs::new());
  // println!("{:?}", res);
  assert!(res.is_ok());
  assert_eq!(format!("{:?}", res.ok().unwrap()), "String(test2test5)");
}

// Tests the addition and subtraction operators
#[test]
fn test_add_sub() {
  use crate::interpreter::interpret;
  use crate::{BlankCustom, Customs, ParserState, ProgramParser};
  let temp = ProgramParser::new()
    .parse(
      &ParserState::new(),
      "#main\nf\"test{2+2} {4-3} {2--1}\"f + \"test\";",
    )
    .unwrap();
  let res = interpret::<BlankCustom>(&temp, "main", blank(), &Customs::new());
  // println!("{:?}", res);
  assert!(res.is_ok());
  assert_eq!(format!("{:?}", res.ok().unwrap()), "String(test4 1 3test)");
}

// Tests the multiply and divide operators
#[test]
fn test_mult_div() {
  use crate::interpreter::interpret;
  use crate::{BlankCustom, Customs, ParserState, ProgramParser};
  let temp = ProgramParser::new()
    .parse(
      &ParserState::new(),
      "#main\n f\"test{2*3} {10/3} {\"test\" * 2}\"f;",
    )
    .unwrap();
  let res = interpret::<BlankCustom>(&temp, "main", blank(), &Customs::new());
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
  use crate::interpreter::{pattern_match, Customs, Frame};
  use crate::BlankCustom;
  use std::collections::HashMap;
  assert_eq!(
    format!(
      "{:?}",
      pattern_match::<BlankCustom>(
        Expr::var(0, "x".to_string(), 0),
        InterpretVal::Int(5),
        &mut Frame::new(),
        &Customs::new(),
      )
      .unwrap()
      .unwrap()
      .frame
    ),
    format!("{:?}", {
      let mut h = HashMap::new();
      h.insert("x".to_string(), InterpretVal::<BlankCustom>::Int(5));
      h
    })
  );

  assert!(pattern_match::<BlankCustom>(
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
    &Customs::new(),
  )
  .unwrap()
  .is_none());

  assert_eq!(
    format!(
      "{:?}",
      pattern_match::<BlankCustom>(
        Expr::tuple(
          0,
          vec![Expr::number(0, 5, 0), Expr::var(0, "y".to_string(), 0)],
          0,
        ),
        InterpretVal::Tuple(vec![InterpretVal::Int(5), InterpretVal::Int(4)]),
        &mut Frame::new(),
        &Customs::new(),
      )
      .unwrap()
      .unwrap()
      .frame
    ),
    format!("{:?}", {
      let mut h = HashMap::new();
      h.insert("y".to_string(), InterpretVal::<BlankCustom>::Int(4));
      h
    })
  );

  assert!(pattern_match::<BlankCustom>(
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
    &Customs::new(),
  )
  .is_err())
}

// Tests arguments being passed into functions
#[test]
fn test_args() {
  use crate::interpreter::interpret;
  use crate::{BlankCustom, Customs, ParserState, ProgramParser};

  let temp = ProgramParser::new()
    .parse(&ParserState::new(), "#one x -> x + 1;#main\none(2);")
    .unwrap();
  let res = interpret::<BlankCustom>(&temp, "main", InterpretVal::Tuple(vec![]), &Customs::new());
  // println!("{:?}", res);
  assert!(res.is_ok());
  assert_eq!(format!("{:?}", res.ok().unwrap()), "Int(3)");
}

// Tests teh pattern match functionality
#[test]
fn test_pattern_match_func() {
  use crate::interpreter::interpret;
  use crate::{BlankCustom, Customs, ParserState, ProgramParser};

  let temp = ProgramParser::new()
    .parse(
      &ParserState::new(),
      "#main (x, 1) -> x - 1; (x, y) -> x + y;x -> x + 1; ",
    )
    .unwrap();
  let res = interpret::<BlankCustom>(&temp, "main", InterpretVal::Int(1), &Customs::new());
  assert!(res.is_ok());
  assert_eq!(format!("{:?}", res.ok().unwrap()), "Int(2)");

  let res = interpret::<BlankCustom>(
    &temp,
    "main",
    InterpretVal::Tuple(vec![InterpretVal::Int(4), InterpretVal::Int(1)]),
    &Customs::new(),
  );
  assert!(res.is_ok());
  assert_eq!(format!("{:?}", res.ok().unwrap()), "Int(3)");

  let res = interpret::<BlankCustom>(
    &temp,
    "main",
    InterpretVal::Tuple(vec![InterpretVal::Int(4), InterpretVal::Int(2)]),
    &Customs::new(),
  );
  assert!(res.is_ok());
  assert_eq!(format!("{:?}", res.ok().unwrap()), "Int(6)");
}

// Tests teh equality operator
#[test]
fn test_eq() {
  use crate::interpreter::interpret;
  use crate::{BlankCustom, Customs, ParserState, ProgramParser};

  let temp = ProgramParser::new()
    .parse(&ParserState::new(), "#one x -> x == 1;#main\none(1);")
    .unwrap();
  let res = interpret::<BlankCustom>(&temp, "main", InterpretVal::Tuple(vec![]), &Customs::new());
  // println!("{:?}", res);
  assert!(res.is_ok());
  assert_eq!(format!("{:?}", res.ok().unwrap()), "Bool(true)");

  let temp = ProgramParser::new()
    .parse(
      &ParserState::new(),
      "#one x -> x == (1, 2); #main\none(1, 2);",
    )
    .unwrap();
  let res = interpret::<BlankCustom>(&temp, "main", InterpretVal::Tuple(vec![]), &Customs::new());
  // println!("{:?}", res);
  assert!(res.is_ok());
  assert_eq!(format!("{:?}", res.ok().unwrap()), "Bool(true)");

  let temp = ProgramParser::new()
    .parse(
      &ParserState::new(),
      "#one x -> x == (1, 2); #main\none(1, 3);",
    )
    .unwrap();
  let res = interpret::<BlankCustom>(&temp, "main", InterpretVal::Tuple(vec![]), &Customs::new());
  // println!("{:?}", res);
  assert!(res.is_ok());
  assert_eq!(format!("{:?}", res.ok().unwrap()), "Bool(false)");
}

// Tests pattern guards
#[test]
fn test_guards() {
  use crate::interpreter::interpret;
  use crate::{BlankCustom, Customs, ParserState, ProgramParser};

  let temp = ProgramParser::new()
    .parse(&ParserState::new(), "#main\nx -> 2|x==3;y -> 5;")
    .unwrap();
  let res = interpret::<BlankCustom>(&temp, "main", InterpretVal::Int(2), &Customs::new());
  // println!("{:?}", res);
  assert!(res.is_ok());
  assert_eq!(format!("{:?}", res.ok().unwrap()), "Int(5)");

  let res = interpret::<BlankCustom>(&temp, "main", InterpretVal::Int(3), &Customs::new());
  // println!("{:?}", res);
  assert!(res.is_ok());
  assert_eq!(format!("{:?}", res.ok().unwrap()), "Int(2)");
}

// Tests string escape sequences
#[test]
fn test_escapes() {
  use crate::interpreter::interpret;
  use crate::{BlankCustom, Customs, ParserState, ProgramParser};

  let temp = ProgramParser::new()
    .parse(&ParserState::new(), "#main\n\"\\{\\}\\\\\";")
    .unwrap();
  let res = interpret::<BlankCustom>(&temp, "main", InterpretVal::Tuple(vec![]), &Customs::new());
  // println!("{:?}", res);
  assert!(res.is_ok());
  assert_eq!(format!("{:?}", res.ok().unwrap()), "String({}\\)");

  let temp = ProgramParser::new()
    .parse(&ParserState::new(), "#main\nx -> f\"\\{\\} {x} \\\\\"f;")
    .unwrap();
  let res = interpret::<BlankCustom>(&temp, "main", InterpretVal::Int(5), &Customs::new());
  // println!("{:?}", res);
  assert!(res.is_ok());
  assert_eq!(format!("{:?}", res.ok().unwrap()), "String({} 5 \\)");
}

// Tests that errors capture location successfully and the error is correct
#[test]
fn test_error() {
  use crate::interpreter::interpret;
  use crate::{BlankCustom, Customs, ParserState, ProgramParser};

  let temp = ProgramParser::new()
    .parse(&ParserState::new(), "#main\n 5 + \"hi\";")
    .unwrap();
  let res = interpret::<BlankCustom>(&temp, "main", InterpretVal::Tuple(vec![]), &Customs::new());
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
  use crate::{BlankCustom, Customs, ParserState, ProgramParser};

  let temp = ProgramParser::new()
    .parse(&ParserState::new(), "#main\nget(list(1, 4, 9, 11), 2);")
    .unwrap();
  let res = interpret::<BlankCustom>(&temp, "main", InterpretVal::Tuple(vec![]), &Customs::new());
  // println!("{:?}", res);
  // assert!(res.is_ok());
  assert_eq!(format!("{:?}", res.unwrap()), "Int(9)");

  let temp = ProgramParser::new()
    .parse(&ParserState::new(), "#main\nget(list(1, 4), 2);")
    .unwrap();
  let res = interpret::<BlankCustom>(&temp, "main", InterpretVal::Tuple(vec![]), &Customs::new());
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
  use crate::{BlankCustom, Customs, ParserState, ProgramParser};

  let temp = ProgramParser::new()
    .parse(&ParserState::new(), "#main\n x -> map(x, |i => i + 1|);")
    .unwrap();
  let res = interpret::<BlankCustom>(
    &temp,
    "main",
    InterpretVal::List(vec![
      InterpretVal::Int(3),
      InterpretVal::Int(123),
      InterpretVal::Int(-123),
    ]),
    &Customs::new(),
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
  use crate::{BlankCustom, Customs, ParserState, ProgramParser};

  let temp = ProgramParser::new()
    .parse(
      &ParserState::new(),
      "#main\n x -> filter(x, |i => i % 3 == 0|);",
    )
    .unwrap();
  let res = interpret::<BlankCustom>(
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
    &Customs::new(),
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
  use crate::{BlankCustom, Customs, ParserState, ProgramParser};

  let temp = ProgramParser::new()
    .parse(&ParserState::new(), "#main\n x -> len(x);")
    .unwrap();
  let res = interpret::<BlankCustom>(
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
    &Customs::new(),
  );
  // println!("{:?}", res);
  assert!(res.is_ok());
  assert_eq!(format!("{:?}", res.unwrap()), "Int(8)");
}

// Tests the builtin any function
#[test]
fn test_any() {
  use crate::interpreter::interpret;
  use crate::{BlankCustom, Customs, ParserState, ProgramParser};

  let temp = ProgramParser::new()
    .parse(
      &ParserState::new(),
      "#main\n x -> any(x, |i => i % 3 == 0|);",
    )
    .unwrap();
  let res = interpret::<BlankCustom>(
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
    &Customs::new(),
  );
  // println!("{:?}", res);
  assert!(res.is_ok());
  assert_eq!(format!("{:?}", res.unwrap()), "Bool(true)");

  let temp = ProgramParser::new()
    .parse(
      &ParserState::new(),
      "#main\n x -> any(x, |i => i % 3 == 0|);",
    )
    .unwrap();
  let res = interpret::<BlankCustom>(
    &temp,
    "main",
    InterpretVal::List(vec![
      InterpretVal::Int(4),
      InterpretVal::Int(5),
      InterpretVal::Int(13),
      InterpretVal::Int(1237),
    ]),
    &Customs::new(),
  );
  // println!("{:?}", res);
  assert!(res.is_ok());
  assert_eq!(format!("{:?}", res.unwrap()), "Bool(false)");
}

// Tests the builtin all function
#[test]
fn test_all() {
  use crate::interpreter::interpret;
  use crate::{BlankCustom, Customs, ParserState, ProgramParser};

  let temp = ProgramParser::new()
    .parse(
      &ParserState::new(),
      "#main\n x -> all(x, |i => i % 3 == 0|);",
    )
    .unwrap();
  let res = interpret::<BlankCustom>(
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
    &Customs::new(),
  );
  // println!("{:?}", res);
  assert!(res.is_ok());
  assert_eq!(format!("{:?}", res.unwrap()), "Bool(false)");

  let temp = ProgramParser::new()
    .parse(
      &ParserState::new(),
      "#main\n x -> all(x, |i => i % 3 == 0|);",
    )
    .unwrap();
  let res = interpret::<BlankCustom>(
    &temp,
    "main",
    InterpretVal::List(vec![
      InterpretVal::Int(3),
      InterpretVal::Int(6),
      InterpretVal::Int(12),
      InterpretVal::Int(1236),
    ]),
    &Customs::new(),
  );
  // println!("{:?}", res);
  assert!(res.is_ok());
  assert_eq!(format!("{:?}", res.unwrap()), "Bool(true)");
}

// Tests the builtin fold function
#[test]
fn test_fold() {
  use crate::interpreter::interpret;
  use crate::{BlankCustom, Customs, ParserState, ProgramParser};

  let temp = ProgramParser::new()
    .parse(
      &ParserState::new(),
      "#main\n x -> fold(x, 0, |a, i => a + i|);",
    )
    .unwrap();
  let res = interpret::<BlankCustom>(
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
    &Customs::new(),
  );
  // println!("{:?}", res);
  assert!(res.is_ok());
  assert_eq!(format!("{:?}", res.unwrap()), "Int(2516)");
}

// Tests closures and environment capture
#[test]
fn test_closure() {
  use crate::interpreter::interpret;
  use crate::{BlankCustom, Customs, ParserState, ProgramParser};

  let temp = ProgramParser::new()
    .parse(
      &ParserState::new(),
      "#closure y -> |a => a + y|; #main\n x -> closure(3)(x);",
    )
    .unwrap();
  let res = interpret::<BlankCustom>(&temp, "main", InterpretVal::Int(5), &Customs::new());
  // println!("{:?}", res);
  assert!(res.is_ok());
  assert_eq!(format!("{:?}", res.unwrap()), "Int(8)");
}

// Tests builtin operators
#[test]
fn test_custom_builtin_binary_operators() {
  use crate::interpreter::interpret;
  use crate::OperatorChars;
  use crate::{Argument, BlankCustom, CustomBinOp, Customs, ParserState, ProgramParser};
  use std::collections::HashMap;

  let temp = ProgramParser::new()
    .parse(
      &ParserState {
        unary_ops: vec![],
        binary_ops: vec![OperatorChars::Carat],
      },
      "#main 2 ^ 3;",
    )
    .unwrap();
  let res = interpret::<BlankCustom>(
    &temp,
    "main",
    blank(),
    &Customs::new_from_hash(
      HashMap::from([(
        OperatorChars::Carat,
        CustomBinOp {
          function: |l, r| {
            if let (Argument::Int(l), Argument::Int(r)) = (l, r) {
              Ok(Argument::Int(l.pow(r as u32)))
            } else {
              panic!()
            }
          },
        },
      )]),
      Default::default(),
      Default::default(),
    ),
  );
  assert!(res.is_ok());
  assert_eq!(format!("{:?}", res.unwrap()), "Int(8)");
}

// Tests builtin unary operators
#[test]
fn test_custom_builtin_unary_operators() {
  use crate::interpreter::interpret;
  use crate::OperatorChars;
  use crate::{Argument, BlankCustom, CustomUnaryOp, Customs, ParserState, ProgramParser};
  use std::collections::HashMap;

  let temp = ProgramParser::new()
    .parse(
      &ParserState {
        unary_ops: vec![OperatorChars::Carat],
        binary_ops: vec![],
      },
      "#main ^4;",
    )
    .unwrap();
  let res = interpret::<BlankCustom>(
    &temp,
    "main",
    blank(),
    &Customs::new_from_hash(
      Default::default(),
      HashMap::from([(
        OperatorChars::Carat,
        CustomUnaryOp {
          function: |l| {
            if let Argument::Int(l) = l {
              Ok(Argument::Int(l + 5))
            } else {
              panic!()
            }
          },
        },
      )]),
      Default::default(),
    ),
  );
  assert!(res.is_ok());
  assert_eq!(format!("{:?}", res.unwrap()), "Int(9)");
}

// Tests builtin functions
#[test]
fn test_custom_builtin_functions() {
  use crate::interpreter::interpret;
  use crate::{Argument, BlankCustom, CustomBuiltIn, Customs, ParserState, ProgramParser};
  use std::collections::HashMap;

  let temp = ProgramParser::new()
    .parse(
      &ParserState {
        unary_ops: vec![],
        binary_ops: vec![],
      },
      "#main test(4);",
    )
    .unwrap();
  let res = interpret::<BlankCustom>(
    &temp,
    "main",
    blank(),
    &Customs::new_from_hash(
      Default::default(),
      Default::default(),
      HashMap::from([(
        "test".to_string(),
        CustomBuiltIn {
          function: |a| {
            if let Argument::Int(a) = a {
              Ok(Argument::Int(a + 5))
            } else {
              panic!()
            }
          },
        },
      )]),
    ),
  );
  assert!(res.is_ok());
  assert_eq!(format!("{:?}", res.unwrap()), "Int(9)");
}

// Tests custom types
#[test]
fn test_custom_types() {
  use crate::interpreter::interpret;
  use crate::{Argument, CustomBuiltIn, Customs, ParserState, ProgramParser};
  use std::collections::HashMap;

  #[derive(Clone, Debug, PartialEq)]
  struct Custom {
    num: i32,
    denom: i32,
  }

  impl ToString for Custom {
    fn to_string(&self) -> String {
      "custom".to_string()
    }
  }

  impl CustomType for Custom {
    fn pre_add(&self, r: Argument<Custom>) -> Result<Argument<Custom>, Box<dyn ToString>> {
      if let Argument::Int(i) = r {
        Ok(Argument::Custom(Self {
          num: self.num + self.denom * i,
          denom: self.denom,
        }))
      } else {
        Err(Box::new("Err 4"))
      }
    }
  }

  let temp = ProgramParser::new()
    .parse(
      &ParserState {
        unary_ops: vec![],
        binary_ops: vec![],
      },
      "#main frac(4, 3) + 2;",
    )
    .unwrap();
  let res = interpret::<Custom>(
    &temp,
    "main",
    blank(),
    &Customs::new_from_hash(
      Default::default(),
      Default::default(),
      HashMap::from([(
        "frac".to_string(),
        CustomBuiltIn {
          function: |a| {
            if let Argument::Tuple(v) = a {
              if v.len() == 2 {
                let v1 = v.get(0).unwrap();
                let v2 = v.get(1).unwrap();

                if let (Argument::Int(n), Argument::Int(d)) = (v1, v2) {
                  Ok(Argument::Custom(Custom { num: *n, denom: *d }))
                } else {
                  Err(Box::new("Err 1"))
                }
              } else {
                Err(Box::new("Err 2"))
              }
            } else {
              Err(Box::new("Err 3"))
            }
          },
        },
      )]),
    ),
  );
  assert!(res.is_ok());
  assert_eq!(
    format!("{:?}", res.unwrap()),
    "Custom(Custom { num: 10, denom: 3 })"
  );
}
