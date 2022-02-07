use std::env::var;

use crate::Language;

#[test]
fn test_string_parsers() {
    let lang_op = Language::new("#main t -> \"Hello\";");
    assert!(lang_op.is_ok());
    let lang = lang_op.unwrap();
    assert_eq!(lang.function("main").call(), "test");
}
