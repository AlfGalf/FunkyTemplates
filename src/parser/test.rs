use crate::parser::language_definition;

#[test]
fn test_string_parsers() {
    assert_eq!(language_definition::FunctionNameStringParser::new()
                   .parse("#asASD_879").unwrap(), "asASD_879");

    assert!(language_definition::FunctionNameStringParser::new()
        .parse("#2asASD_879").is_err());

    assert_eq!(language_definition::StringTermParser::new()
                   .parse("\"hello, world\"").unwrap(), "hello, world");

    assert!(language_definition::FunctionNameStringParser::new()
        .parse("\"hello \n world\"").is_err());

    assert!(language_definition::FunctionNameStringParser::new()
        .parse("\"hello \" world\"").is_err());
}

#[test]
fn test_function_parser() {
    assert_eq!(format!("{:?}", language_definition::PatternParser::new()
        .parse("x -> 5 + 4\n").unwrap()), "x -> (5 + 4)");

    assert_eq!(format!("{:?}", language_definition::FunctionParser::new()
        .parse("#main \n   x -> 5 + 4\n").unwrap()), "#main\nx -> (5 + 4)");

    assert_eq!(format!("{:?}", language_definition::FunctionParser::new()
        .parse("#main \nx -> 5 + 4 \n y -> 5-2\n").unwrap()), "#main\nx -> (5 + 4)\ny -> (5 - 2)");
}

#[test]
fn test_function_guards() {
    assert_eq!(format!("{:?}", language_definition::PatternParser::new()
        .parse("x -> 5 + 4\n| test()\n").unwrap()), "x -> (5 + 4)");
}

#[test]
fn test_term_parser() {
    assert_eq!(language_definition::NameParser::new()
                   .parse("name").unwrap(), "name");
    assert_eq!(format!("{:?}", language_definition::ExprParser::new()
        .parse("5 + 2 * 3").unwrap()), "(5 + (2 * 3))");
    assert_eq!(format!("{:?}", language_definition::ExprParser::new()
        .parse("5").unwrap()), "5");
    assert_eq!(format!("{:?}", language_definition::ExprParser::new()
        .parse("func(1)").unwrap()), "func(1)");
    assert_eq!(format!("{:?}", language_definition::ExprParser::new()
        .parse("func()").unwrap()), "func()");
    assert_eq!(format!("{:?}", language_definition::ExprParser::new()
        .parse("func(1, 2 + 3)").unwrap()), "func(1, (2 + 3))");
}