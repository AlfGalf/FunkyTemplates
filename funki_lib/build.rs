extern crate lalrpop;

/// Generates the parser and lexer code from the `.lalrpop` file at compile time
fn main() {
  lalrpop::process_root().unwrap();
}
