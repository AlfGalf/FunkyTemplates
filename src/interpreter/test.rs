#[test]
fn test_interpret() {
    use crate::interpreter::interpret;
    use crate::TemplateParser;
    let temp = TemplateParser::new().parse("#main\n5;").unwrap();
    let res = interpret(&temp, "main");
    assert!(res.is_ok());
    assert_eq!(format!("{}", res.ok().unwrap()), "Int(5)");
}

#[test]
fn test_lambda() {
    use crate::interpreter::interpret;
    use crate::TemplateParser;
    let temp = TemplateParser::new().parse("#main\n|x => 5|();").unwrap();
    let res = interpret(&temp, "main");
    println!("{:?}", res);
    assert!(res.is_ok());
    assert_eq!(format!("{}", res.ok().unwrap()), "Int(5)");
}

#[test]
fn test_func() {
    use crate::interpreter::interpret;
    use crate::TemplateParser;
    let temp = TemplateParser::new().parse("#one 1;#main\none();").unwrap();
    let res = interpret(&temp, "main");
    println!("{:?}", res);
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
    let res = interpret(&temp, "main");
    println!("{:?}", res);
    assert!(res.is_ok());
    assert_eq!(format!("{}", res.ok().unwrap()), "String(test2test5)");
}

#[test]
fn test_addition() {
    use crate::interpreter::interpret;
    use crate::TemplateParser;
    let temp = TemplateParser::new()
        .parse("#main\nf\"test{2+2}\"f;")
        .unwrap();
    let res = interpret(&temp, "main");
    println!("{:?}", res);
    assert!(res.is_ok());
    assert_eq!(format!("{}", res.ok().unwrap()), "String(test4)");
}
