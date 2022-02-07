use crate::interpreter::interpret;
use crate::TemplateParser;

#[test]
fn test_interpret() {
    let temp = TemplateParser::new().parse("#main\n5;").unwrap();
    let res = interpret(&temp, "main");
    assert!(res.is_ok());
    assert_eq!(format!("{}", res.ok().unwrap()), "Int(5)");
}

#[test]
fn test_lambda() {
    let temp = TemplateParser::new().parse("#main\n|x => 5|();").unwrap();
    let res = interpret(&temp, "main");
    println!("{:?}", res);
    assert!(res.is_ok());
    assert_eq!(format!("{}", res.ok().unwrap()), "Int(5)");
}
