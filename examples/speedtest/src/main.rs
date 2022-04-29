use std::time;

use funki_lang::*;

fn main() {
  let lang = Language::<BlankCustom>::new();

  let code = "\
  #square
  n -> n * n;
  _ -> \"Error\";

  #main\
  (name, i) -> f\"{name} is {square(i)} years\"f;\
  _ -> \"Error\";";

  let temp = lang.parse(code.to_string()).unwrap();

  let start = time::Instant::now();

  for i in 1..1000000 {
    let func = temp.function("main").unwrap();

    let func = func.arg(Argument::Tuple(vec![
      Argument::String("Alfie".to_string()),
      Argument::Int(i % 1000),
    ]));

    let res = func.call().unwrap();
    if i % 10000 == 0 {
      println!("{res:?}");
    }
  }

  let funki_time = start.elapsed();

  let start = time::Instant::now();

  for i in 1..1000000 {
    let res = format_equiv("Alfie".to_string(), i % 1000);
    if i % 10000 == 0 {
      println!("{res:?}");
    }
  }

  let rust_time = start.elapsed();

  println!("Funki: {:?}", funki_time);
  println!("Rust: {:?}", rust_time);
}

fn format_equiv(name: String, age: i32) -> String {
  format!("{name} is {} years", square(age))
}

fn square(i: i32) -> i32 {
  i * i
}

// MacBook Pro 2022
// Funki: 6.962078291s
// Rust: 141.6265ms
// python time: 0.2861509323120117
