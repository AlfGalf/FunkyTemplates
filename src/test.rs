// Tests the library works and the values are returned correctly
#[test]
fn test_library() {
  use crate::Argument;
  use crate::{ParsedTemplate, ReturnVal};
  let lang_op = ParsedTemplate::from_text("#main t -> f\"Hello {t + 3}\"f;");
  assert!(lang_op.is_ok());
  let lang = lang_op.unwrap();
  assert_eq!(
    lang
      .function("main")
      .unwrap()
      .arg(Argument::Int(1))
      .call()
      .unwrap(),
    ReturnVal::String("Hello 4".to_string())
  );
}

// Tests teh error messages are correct.
#[test]
fn test_interpret_errors() {
  use crate::Argument;
  use crate::ParsedTemplate;
  let lang_op = ParsedTemplate::from_text("#main t -> \nf\"Hello {t + \n \"hi\"}\"f;");
  assert!(lang_op.is_ok());
  let lang = lang_op.unwrap();
  assert_eq!(
    format!(
      "{:?}",
      lang.function("main")
          .unwrap()
          .arg(Argument::Int(1))
          .call()
          .err()
          .unwrap()
    ),
    "Error: \"Add operator not defined for Int(1) + String(\"hi\").\"\nAt lines: 2:10 - 3:6\nCode: `t + \n \"hi\"`"
  );
}

// Tests the parse errors are correct
#[test]
fn test_parse_errors() {
  use crate::ParsedTemplate;
  let lang_op = ParsedTemplate::from_text("#main t -> Hello");
  assert!(lang_op.is_err());
  assert_eq!(
    "Error: \"Unexpected End of File\"\nAt lines: 1:16 - 1:16\nCode: `#main t -> Hello`",
    format!("{:?}", lang_op.err().unwrap())
  )
}
