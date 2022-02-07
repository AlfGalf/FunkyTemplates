use lalrpop_util;

mod test;

lalrpop_mod!(pub language_definition, "/parser/language_definition.rs");
