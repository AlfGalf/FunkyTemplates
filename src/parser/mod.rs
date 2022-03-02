mod test;

// Macro to import the lalrpop parser library
lalrpop_mod!(
    #[allow(clippy::all)]
    #[allow(unused)]
    pub language_definition,
    "/parser/language_definition.rs"
);
