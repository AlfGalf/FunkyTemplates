// Tests the library works and the values are returned correctly
#[test]
fn test_library() {
  use crate::Argument;
  use crate::{BlankCustom, Script};
  let lang_op = Script::<BlankCustom>::from_text("#main t -> f\"Hello {t + 3}\"f;");
  assert!(lang_op.is_ok());
  let lang = lang_op.unwrap();
  assert_eq!(
    lang
      .function("main")
      .unwrap()
      .arg(Argument::Int(1))
      .call()
      .unwrap()
      .to_string(),
    Argument::<BlankCustom>::String("Hello 4".to_string()).to_string()
  );
}

// Tests the error messages are correct.
#[test]
fn test_interpret_errors() {
  use crate::Argument;
  use crate::{BlankCustom, Script};
  let lang_op = Script::<BlankCustom>::from_text("#main t -> \nf\"Hello {t + \n \"hi\"}\"f;");
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
  use crate::{BlankCustom, Script};
  let lang_op = Script::<BlankCustom>::from_text("#main t -> Hello");
  assert!(lang_op.is_err());
  assert_eq!(
    "Error: \"Unexpected End of File\"\nAt lines: 1:16 - 1:16\nCode: `#main t -> Hello`",
    format!("{:?}", lang_op.err().unwrap())
  )
}

// Tests passing in bin ops
#[test]
fn test_binary_ops() {
  use crate::*;
  let mut lang = Language::new();
  lang.add_bin_op(
    OperatorChars::Carat,
    CustomBinOp {
      function: |l: Argument<BlankCustom>, r| {
        if let (Argument::Int(l), Argument::Int(r)) = (l, r) {
          Ok(Argument::Int(l.pow(r as u32)))
        } else {
          panic!()
        }
      },
    },
  );

  let parsed = lang.parse("#main (x, y) -> x ^ y;".to_string());

  assert!(parsed.is_ok());
  let parsed = parsed.unwrap();

  let func = parsed.function("main");
  assert!(func.is_ok());
  let func = func.unwrap();

  let arg = func.arg(Argument::Tuple(vec![Argument::Int(2), Argument::Int(3)]));
  let res = arg.call();

  assert!(res.is_ok());
  let res = res.unwrap();

  assert_eq!(format!("{:?}", res), "Int(8)");
}
