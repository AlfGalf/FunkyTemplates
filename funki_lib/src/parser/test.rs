// Test to check strings, and name strings parse correctly
#[test]
fn test_string_parsers() {
  use crate::parser::language_definition;
  use crate::ParserState;
  assert_eq!(
    language_definition::FunctionNameStringParser::new()
      .parse(&ParserState::new(), "#asASD_879")
      .unwrap(),
    "asASD_879"
  );

  assert!(language_definition::FunctionNameStringParser::new()
    .parse(&ParserState::new(), "#2asASD_879")
    .is_err());

  assert_eq!(
    language_definition::StringTermParser::new()
      .parse(&ParserState::new(), "\"hello, world\"")
      .unwrap(),
    "hello, world"
  );

  assert!(language_definition::FunctionNameStringParser::new()
    .parse(&ParserState::new(), "\"hello \n world\"")
    .is_err());

  assert!(language_definition::FunctionNameStringParser::new()
    .parse(&ParserState::new(), "\"hello \" world\"")
    .is_err());
}

// Test to check functions parse correctly
#[test]
fn test_function_parser() {
  use crate::parser::language_definition;
  use crate::ParserState;
  assert_eq!(
    format!(
      "{:?}",
      language_definition::PatternParser::new()
        .parse(&ParserState::new(), "x -> 5 + 4;\n")
        .unwrap()
    ),
    "x -> (5 + 4)"
  );

  assert_eq!(
    format!(
      "{:?}",
      language_definition::FunctionParser::new()
        .parse(&ParserState::new(), "#main \n    x -> 5 + 4;\n")
        .unwrap()
    ),
    "(\"main\", [x -> (5 + 4)])"
  );
  assert_eq!(
    format!(
      "{:?}",
      language_definition::FunctionParser::new()
        .parse(&ParserState::new(), "#main \n x -> 5 + 4 ;\n y -> 5-2;\n")
        .unwrap()
    ),
    "(\"main\", [x -> (5 + 4), y -> (5 - 2)])"
  );
  assert_eq!(
    format!(
      "{:?}",
      language_definition::FunctionParser::new()
        .parse(&ParserState::new(), "#main \n (a, b) -> a + 4;\n")
        .unwrap()
    ),
    "(\"main\", [{a, b} -> (a + 4)])"
  );
  assert_eq!(
    format!(
      "{:?}",
      language_definition::FunctionParser::new()
        .parse(&ParserState::new(), "#main \n (a, (c, true)) -> a + c;\n")
        .unwrap()
    ),
    "(\"main\", [{a, {c, true}} -> (a + c)])"
  );
}

// Test to check guards parse correctly
#[test]
fn test_function_guards() {
  use crate::parser::language_definition;
  use crate::ParserState;
  assert_eq!(
    format!(
      "{:?}",
      language_definition::PatternParser::new()
        .parse(&ParserState::new(), "x -> 5 + 4\n| test();\n")
        .unwrap()
    ),
    "x -> (5 + 4)"
  );
}

// Test to check all types of term parse correctly
#[test]
fn test_term_parser() {
  use crate::parser::language_definition;
  use crate::ParserState;

  assert_eq!(
    language_definition::NameParser::new()
      .parse(&ParserState::new(), "name")
      .unwrap(),
    "name"
  );
  assert_eq!(
    format!(
      "{:?}",
      language_definition::ExprParser::new()
        .parse(&ParserState::new(), "5")
        .unwrap()
    ),
    "5"
  );
  assert_eq!(
    format!(
      "{:?}",
      language_definition::ExprParser::new()
        .parse(&ParserState::new(), "5 + 2 * 3")
        .unwrap()
    ),
    "(5 + (2 * 3))"
  );
  assert_eq!(
    format!(
      "{:?}",
      language_definition::ExprParser::new()
        .parse(&ParserState::new(), "5 - 2 % 3")
        .unwrap()
    ),
    "(5 - (2 % 3))"
  );
  assert_eq!(
    format!(
      "{:?}",
      language_definition::ExprParser::new()
        .parse(&ParserState::new(), "func(1, 2)")
        .unwrap()
    ),
    "func({1, 2})"
  );
  assert_eq!(
    format!(
      "{:?}",
      language_definition::ExprParser::new()
        .parse(&ParserState::new(), "func()")
        .unwrap()
    ),
    "func({})"
  );
  assert_eq!(
    format!(
      "{:?}",
      language_definition::ExprParser::new()
        .parse(&ParserState::new(), "func")
        .unwrap()
    ),
    "func"
  );
  assert_eq!(
    format!(
      "{:?}",
      language_definition::ExprParser::new()
        .parse(&ParserState::new(), "func(1, 2 + 3)")
        .unwrap()
    ),
    "func({1, (2 + 3)})"
  );
  assert_eq!(
    format!(
      "{:?}",
      language_definition::ExprParser::new()
        .parse(&ParserState::new(), "2 + 3 == 5")
        .unwrap()
    ),
    "((2 + 3) == 5)"
  );
  assert_eq!(
    format!(
      "{:?}",
      language_definition::ExprParser::new()
        .parse(&ParserState::new(), "2 + 3 != 5")
        .unwrap()
    ),
    "((2 + 3) != 5)"
  );
  assert_eq!(
    format!(
      "{:?}",
      language_definition::ExprParser::new()
        .parse(&ParserState::new(), "2 + 3 <= 5")
        .unwrap()
    ),
    "((2 + 3) <= 5)"
  );
  assert_eq!(
    format!(
      "{:?}",
      language_definition::ExprParser::new()
        .parse(&ParserState::new(), "2 + 3 >= 5")
        .unwrap()
    ),
    "((2 + 3) >= 5)"
  );
  assert_eq!(
    format!(
      "{:?}",
      language_definition::ExprParser::new()
        .parse(&ParserState::new(), "2 + 3 < 5")
        .unwrap()
    ),
    "((2 + 3) < 5)"
  );
  assert_eq!(
    format!(
      "{:?}",
      language_definition::ExprParser::new()
        .parse(&ParserState::new(), "2 + 3 > 5")
        .unwrap()
    ),
    "((2 + 3) > 5)"
  );
  assert_eq!(
    format!(
      "{:?}",
      language_definition::ExprParser::new()
        .parse(&ParserState::new(), "(5, 6, 7)")
        .unwrap()
    ),
    "{5, 6, 7}"
  );
  assert_eq!(
    format!(
      "{:?}",
      language_definition::ExprParser::new()
        .parse(&ParserState::new(), "(5, func(), 7)")
        .unwrap()
    ),
    "{5, func({}), 7}"
  );
  assert_eq!(
    format!(
      "{:?}",
      language_definition::ExprParser::new()
        .parse(&ParserState::new(), "!true")
        .unwrap()
    ),
    "!(true)"
  );
  assert_eq!(
    format!(
      "{:?}",
      language_definition::ExprParser::new()
        .parse(&ParserState::new(), "-1")
        .unwrap()
    ),
    "-(1)"
  );
  assert_eq!(
    format!(
      "{:?}",
      language_definition::ExprParser::new()
        .parse(&ParserState::new(), "true && !true")
        .unwrap()
    ),
    "(true && !(true))"
  );
  assert_eq!(
    format!(
      "{:?}",
      language_definition::ExprParser::new()
        .parse(&ParserState::new(), "\"Hello\"")
        .unwrap()
    ),
    "\"Hello\""
  );
  assert_eq!(
    format!(
      "{:?}",
      language_definition::ExprParser::new()
        .parse(&ParserState::new(), "\"Hello\nhello\"")
        .unwrap()
    ),
    "\"Hello\nhello\""
  );
  assert_eq!(
    format!(
      "{:?}",
      language_definition::ExprParser::new()
        .parse(&ParserState::new(), "f\"Hellohello\"f")
        .unwrap()
    ),
    "stringInt(\"Hellohello\")"
  );
  assert_eq!(
    format!(
      "{:?}",
      language_definition::ExprParser::new()
        .parse(&ParserState::new(), "f\"Hello{a}hello\"f")
        .unwrap()
    ),
    "stringInt(\"Hello\" + a + \"hello\")"
  );
  assert_eq!(
    format!(
      "{:?}",
      language_definition::ExprParser::new()
        .parse(&ParserState::new(), "f\"Hello{a}hello{b}\"f")
        .unwrap()
    ),
    "stringInt(\"Hello\" + a + \"hello\" + b + \"\")"
  );
  assert_eq!(
    format!(
      "{:?}",
      language_definition::ExprParser::new()
        .parse(
          &ParserState::new(),
          "f\"Hello{f\"test \n{b} test\"f}hello{b}\"f",
        )
        .unwrap()
    ),
    "stringInt(\"Hello\" + stringInt(\"test \n\" + b + \" test\") + \"hello\" + b + \"\")"
  );
  assert_eq!(
    format!(
      "{:?}",
      language_definition::ExprParser::new()
        .parse(&ParserState::new(), "f\"\\{\\} {x} \\\"\"f")
        .unwrap()
    ),
    "stringInt(\"{} \" + x + \" \"\")"
  );
  assert!(language_definition::ExprParser::new()
    .parse(&ParserState::new(), "f\"{\\}\"f")
    .is_err());
  assert_eq!(
    format!(
      "{:?}",
      language_definition::ExprParser::new()
        .parse(&ParserState::new(), "|a, b => c|")
        .unwrap()
    ),
    "|{a, b} -> c|"
  );
}

// Test to check entire templates parse correctly
#[test]
fn test_template() {
  use crate::parser::language_definition;
  use crate::ParserState;

  assert_eq!(
    format!(
      "{:?}",
      language_definition::ProgramParser::new()
        .parse(&ParserState::new(), "#main\n  x -> true;\n")
        .unwrap()
    ),
    "#main x -> true"
  );
  assert_eq!(
    format!(
      "{:?}",
      language_definition::ProgramParser::new()
        .parse(
          &ParserState::new(),
          "#main\n x -> true;\n #second\n y -> false;\n",
        )
        .unwrap()
    ),
    "#main x -> true\n#second y -> false"
  );
  assert_eq!(
    format!(
      "{:?}",
      language_definition::ProgramParser::new()
        .parse(
          &ParserState::new(),
          "#main\n x -> true ;\n\n\n #second\n y -> false;\n",
        )
        .unwrap()
    ),
    "#main x -> true\n#second y -> false"
  );
  {
    let test_str = "\
#main
test -> 1;
(a, b) -> 2;
(a, (5 + 2 * 3)) -> 3;
(a, true) -> 4;
(a, true) -> (test);
\"test\" -> 5;

#second
test -> 6;";
    let res_str = "\
#main test -> 1
{a, b} -> 2
{a, (5 + (2 * 3))} -> 3
{a, true} -> 4
{a, true} -> test
\"test\" -> 5
#second test -> 6";

    assert_eq!(
      format!(
        "{:?}",
        language_definition::ProgramParser::new()
          .parse(&ParserState::new(), test_str)
          .unwrap()
      ),
      res_str
    );
  }
}

// Test for parsing with custom binary operators
#[test]
fn test_custom_bin_operators() {
  use crate::parser::language_definition;
  use crate::OperatorChars;
  use crate::ParserState;

  let test_str = "\
#main
(a, b) -> a ? b;
";

  let parser = language_definition::ProgramParser::new();
  let res_1 = parser.parse(&ParserState::new(), test_str);
  assert!(res_1.is_err());
  assert_eq!(
    format!("{:?}", res_1),
    "Err(User { error: (16, \"This binary operator is not defined\", 21) })"
  );

  let res_2 = parser.parse(
    &ParserState {
      unary_ops: vec![],
      binary_ops: vec![OperatorChars::QuestionMark],
    },
    test_str,
  );
  assert!(res_2.is_ok());
  assert_eq!(
    format!("{:?}", res_2),
    "Ok(#main {a, b} -> CustomOp(a ? b))"
  )
}

// Test for parsing with custom unary operators
#[test]
fn test_custom_unary_operators() {
  use crate::parser::language_definition;
  use crate::OperatorChars;
  use crate::ParserState;

  let test_str = "\
#main
a -> ?a;
";

  let parser = language_definition::ProgramParser::new();
  let res_1 = parser.parse(&ParserState::new(), test_str);
  assert!(res_1.is_err());
  assert_eq!(
    format!("{:?}", res_1),
    "Err(User { error: (11, \"This unary operator is not defined\", 13) })"
  );

  let res_2 = parser.parse(
    &ParserState {
      unary_ops: vec![OperatorChars::QuestionMark],
      binary_ops: vec![],
    },
    test_str,
  );
  assert!(res_2.is_ok());
  assert_eq!(format!("{:?}", res_2), "Ok(#main a -> CustomOp(? a))")
}

// Test for custom operator evaluation order
#[test]
fn test_custom_op_order() {
  use crate::parser::language_definition;
  use crate::OperatorChars;
  use crate::ParserState;

  let test_str = "\
#main
a -> a & ?b * ^d;
";

  let parser = language_definition::ProgramParser::new();
  let res = parser.parse(
    &ParserState {
      unary_ops: vec![OperatorChars::QuestionMark, OperatorChars::Carat],
      binary_ops: vec![OperatorChars::And],
    },
    test_str,
  );
  assert!(dbg!(&res).is_ok());
  assert_eq!(
    format!("{:?}", res),
    "Ok(#main a -> (CustomOp(a & CustomOp(? b)) * CustomOp(^ d)))"
  )
}
