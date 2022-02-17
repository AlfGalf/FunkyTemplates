#[test]
fn test_string_parsers() {
    use crate::Argument;
    use crate::{Language, ReturnVal};
    let lang_op = Language::from_text("#main t -> f\"Hello {t + 3}\"f;");
    assert!(lang_op.is_ok());
    let lang = lang_op.unwrap();
    assert_eq!(
        lang.function("main").arg(Argument::Int(1)).call().unwrap(),
        ReturnVal::String("Hello 4".to_string())
    );
}
