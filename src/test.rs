#[test]
fn test_string_parsers() {
    use crate::{Language, ReturnVal};
    let lang_op = Language::from_text("#main t -> \"Hello\";");
    assert!(lang_op.is_ok());
    let lang = lang_op.unwrap();
    assert_eq!(
        lang.function("main").call().unwrap(),
        ReturnVal::String("Hello".to_string())
    );
}
