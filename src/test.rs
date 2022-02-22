#[test]
fn test_string_parsers() {
    use crate::Argument;
    use crate::{Language, ReturnVal};
    let lang_op = Language::from_text("#main t -> f\"Hello {t + 3}\"f;");
    assert!(lang_op.is_ok());
    let lang = lang_op.unwrap();
    assert_eq!(
        lang.function("main")
            .unwrap()
            .arg(Argument::Int(1))
            .call()
            .unwrap(),
        ReturnVal::String("Hello 4".to_string())
    );
}

#[test]
fn test_errors() {
    use crate::Argument;
    use crate::Language;
    let lang_op = Language::from_text("#main t -> \nf\"Hello {t + \n \"hi\"}\"f;");
    assert!(lang_op.is_ok());
    let lang = lang_op.unwrap();
    // println!(
    //     "{:?}",
    //     lang.function("main").unwrap().arg(Argument::Int(1)).call()
    // );
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
        "Error: \"Add operator not defined for Int(1) + String(\"hi\").\"\nAt lines: 2 - 3\nCode: `t + \n \"hi\"`"
    );
}
