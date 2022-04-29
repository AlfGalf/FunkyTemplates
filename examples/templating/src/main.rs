use std::io;
use std::process::exit;

use funki_lang::*;

fn main() {
  let lang = Language::<BlankCustom>::new();

  let code = {
    let mut buffer = String::new();
    let stdin = io::stdin(); // We get `Stdin` here.

    loop {
      let res = stdin.read_line(&mut buffer).unwrap_or_else(|e| {
        println!("Failed to read in code. Error: {e}");
        exit(1)
      });

      if res == 0 {
        break buffer;
      }
    }
  };

  let temp = lang.parse(code).unwrap_or_else(|e| {
    println!("Parse error: \n{:?}", e);
    exit(2)
  });

  let func = temp.function("main").unwrap_or_else(|_| {
    println!("Cannot find function.");
    exit(3)
  });

  let func = func.arg(Argument::Tuple(vec![
    Argument::String("Alfie".to_string()),
    Argument::Int(5),
  ]));

  let res = func.call().unwrap_or_else(|e| {
    println!("Execution Error: \n{e:?}");
    exit(4)
  });

  if let ReturnVal::String(s) = res {
    println!("{s}");
  } else {
    println!("Wrong type returned.");
  }
}
