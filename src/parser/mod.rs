mod test;

lalrpop_mod!(
    #[allow(clippy::all)]
    #[allow(unused)]
    pub language_definition,
    "/parser/language_definition.rs"
);
