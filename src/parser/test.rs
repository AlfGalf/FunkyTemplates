#[test]
fn test_string_parsers() {
    use crate::parser::language_definition;
    assert_eq!(
        language_definition::FunctionNameStringParser::new()
            .parse("#asASD_879")
            .unwrap(),
        "asASD_879"
    );

    assert!(language_definition::FunctionNameStringParser::new()
        .parse("#2asASD_879")
        .is_err());

    assert_eq!(
        language_definition::StringTermParser::new()
            .parse("\"hello, world\"")
            .unwrap(),
        "hello, world"
    );

    assert!(language_definition::FunctionNameStringParser::new()
        .parse("\"hello \n world\"")
        .is_err());

    assert!(language_definition::FunctionNameStringParser::new()
        .parse("\"hello \" world\"")
        .is_err());
}

#[test]
fn test_function_parser() {
    use crate::parser::language_definition;
    assert_eq!(
        format!(
            "{:?}",
            language_definition::PatternParser::new()
                .parse("x -> 5 + 4;\n")
                .unwrap()
        ),
        "x -> (5 + 4)"
    );

    assert_eq!(
        format!(
            "{:?}",
            language_definition::FunctionParser::new()
                .parse("#main \n    x -> 5 + 4;\n")
                .unwrap()
        ),
        "(\"main\", |x -> (5 + 4)|)"
    );
    assert_eq!(
        format!(
            "{:?}",
            language_definition::FunctionParser::new()
                .parse("#main \n x -> 5 + 4 ;\n y -> 5-2;\n")
                .unwrap()
        ),
        "(\"main\", |x -> (5 + 4)\ny -> (5 - 2)|)"
    );
    assert_eq!(
        format!(
            "{:?}",
            language_definition::FunctionParser::new()
                .parse("#main \n (a, b) -> a + 4;\n")
                .unwrap()
        ),
        "(\"main\", |{a, b} -> (a + 4)|)"
    );
    assert_eq!(
        format!(
            "{:?}",
            language_definition::FunctionParser::new()
                .parse("#main \n (a, (c, true)) -> a + c;\n")
                .unwrap()
        ),
        "(\"main\", |{a, {c, true}} -> (a + c)|)"
    );
}

#[test]
fn test_function_guards() {
    use crate::parser::language_definition;
    assert_eq!(
        format!(
            "{:?}",
            language_definition::PatternParser::new()
                .parse("x -> 5 + 4\n| test();\n")
                .unwrap()
        ),
        "x -> (5 + 4)"
    );
}

#[test]
fn test_term_parser() {
    use crate::parser::language_definition;
    assert_eq!(
        language_definition::NameParser::new()
            .parse("name")
            .unwrap(),
        "name"
    );
    assert_eq!(
        format!(
            "{:?}",
            language_definition::ExprParser::new().parse("5").unwrap()
        ),
        "5"
    );
    assert_eq!(
        format!(
            "{:?}",
            language_definition::ExprParser::new()
                .parse("5 + 2 * 3")
                .unwrap()
        ),
        "(5 + (2 * 3))"
    );
    assert_eq!(
        format!(
            "{:?}",
            language_definition::ExprParser::new()
                .parse("5 - 2 % 3")
                .unwrap()
        ),
        "(5 - (2 % 3))"
    );
    assert_eq!(
        format!(
            "{:?}",
            language_definition::ExprParser::new()
                .parse("func(1, 2)")
                .unwrap()
        ),
        "func(1, 2)"
    );
    assert_eq!(
        format!(
            "{:?}",
            language_definition::ExprParser::new()
                .parse("func()")
                .unwrap()
        ),
        "func()"
    );
    assert_eq!(
        format!(
            "{:?}",
            language_definition::ExprParser::new()
                .parse("func")
                .unwrap()
        ),
        "func"
    );
    assert_eq!(
        format!(
            "{:?}",
            language_definition::ExprParser::new()
                .parse("func(1, 2 + 3)")
                .unwrap()
        ),
        "func(1, (2 + 3))"
    );
    assert_eq!(
        format!(
            "{:?}",
            language_definition::ExprParser::new()
                .parse("2 + 3 == 5")
                .unwrap()
        ),
        "((2 + 3) == 5)"
    );
    assert_eq!(
        format!(
            "{:?}",
            language_definition::ExprParser::new()
                .parse("2 + 3 <= 5")
                .unwrap()
        ),
        "((2 + 3) <= 5)"
    );
    assert_eq!(
        format!(
            "{:?}",
            language_definition::ExprParser::new()
                .parse("2 + 3 >= 5")
                .unwrap()
        ),
        "((2 + 3) >= 5)"
    );
    assert_eq!(
        format!(
            "{:?}",
            language_definition::ExprParser::new()
                .parse("2 + 3 < 5")
                .unwrap()
        ),
        "((2 + 3) < 5)"
    );
    assert_eq!(
        format!(
            "{:?}",
            language_definition::ExprParser::new()
                .parse("2 + 3 > 5")
                .unwrap()
        ),
        "((2 + 3) > 5)"
    );
    assert_eq!(
        format!(
            "{:?}",
            language_definition::ExprParser::new()
                .parse("(5, 6, 7)")
                .unwrap()
        ),
        "{5, 6, 7}"
    );
    assert_eq!(
        format!(
            "{:?}",
            language_definition::ExprParser::new()
                .parse("(5, func(), 7)")
                .unwrap()
        ),
        "{5, func(), 7}"
    );
    assert_eq!(
        format!(
            "{:?}",
            language_definition::ExprParser::new()
                .parse("!true")
                .unwrap()
        ),
        "!(true)"
    );
    assert_eq!(
        format!(
            "{:?}",
            language_definition::ExprParser::new()
                .parse("!!true")
                .unwrap()
        ),
        "!(!(true))"
    );
    assert_eq!(
        format!(
            "{:?}",
            language_definition::ExprParser::new()
                .parse("true && !true")
                .unwrap()
        ),
        "(true && !(true))"
    );
    assert_eq!(
        format!(
            "{:?}",
            language_definition::ExprParser::new()
                .parse("\"Hello\"")
                .unwrap()
        ),
        "\"Hello\""
    );
    assert_eq!(
        format!(
            "{:?}",
            language_definition::ExprParser::new()
                .parse("\"Hello\nhello\"")
                .unwrap()
        ),
        "\"Hello\nhello\""
    );
    assert_eq!(
        format!(
            "{:?}",
            language_definition::ExprParser::new()
                .parse("f\"Hellohello\"f")
                .unwrap()
        ),
        "stringInt(\"Hellohello\")"
    );
    assert_eq!(
        format!(
            "{:?}",
            language_definition::ExprParser::new()
                .parse("f\"Hello{a}hello\"f")
                .unwrap()
        ),
        "stringInt(\"Hello\" + a + \"hello\")"
    );
    assert_eq!(
        format!(
            "{:?}",
            language_definition::ExprParser::new()
                .parse("f\"Hello{a}hello{b}\"f")
                .unwrap()
        ),
        "stringInt(\"Hello\" + a + \"hello\" + b + \"\")"
    );
    assert_eq!(
        format!(
            "{:?}",
            language_definition::ExprParser::new()
                .parse("f\"Hello{f\"test \n{b} test\"f}hello{b}\"f")
                .unwrap()
        ),
        "stringInt(\"Hello\" + stringInt(\"test \n\" + b + \" test\") + \"hello\" + b + \"\")"
    );
    assert_eq!(
        format!(
            "{:?}",
            language_definition::ExprParser::new()
                .parse("|a, b => c|")
                .unwrap()
        ),
        "|{a, b} -> c|"
    );
}

#[test]
fn test_template() {
    use crate::parser::language_definition;
    assert_eq!(
        format!(
            "{:?}",
            language_definition::TemplateParser::new()
                .parse("#main\n  x -> true;\n")
                .unwrap()
        ),
        "#main |x -> true|"
    );
    assert_eq!(
        format!(
            "{:?}",
            language_definition::TemplateParser::new()
                .parse("#main\n x -> true;\n #second\n y -> false;\n")
                .unwrap()
        ),
        "#main |x -> true|\n#second |y -> false|"
    );
    assert_eq!(
        format!(
            "{:?}",
            language_definition::TemplateParser::new()
                .parse("#main\n x -> true ;\n\n\n #second\n y -> false;\n")
                .unwrap()
        ),
        "#main |x -> true|\n#second |y -> false|"
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
#main |test -> 1
{a, b} -> 2
{a, (5 + (2 * 3))} -> 3
{a, true} -> 4
{a, true} -> test
\"test\" -> 5|
#second |test -> 6|";

        assert_eq!(
            format!(
                "{:?}",
                language_definition::TemplateParser::new()
                    .parse(test_str)
                    .unwrap()
            ),
            res_str
        );
    }
}
