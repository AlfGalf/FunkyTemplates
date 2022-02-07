use lalrpop_util;

mod test;

#[cfg(not(feature = "test"))]
pub(crate) mod language_definition;

#[cfg(feature = "test")]
lalrpop_mod!(language_definition, "src/PARSER/language_definition.rs");
