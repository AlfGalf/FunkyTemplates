use lalrpop_util;

mod test;

#[cfg(not(feature = "test"))]
mod language_definition;

#[cfg(feature = "test")]
lalrpop_mod!(language_definition, "src/parser/language_definition.rs");

