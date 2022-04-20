use std::io;
use std::process::exit;

use funki_templates::external_operators::{CustomBuiltIn, CustomType};
use funki_templates::*;
use gcd::Gcd;

#[derive(Clone, PartialEq, Debug)]
struct Fraction {
  num: isize,
  denom: usize,
}

impl Fraction {
  fn simplify(&self) -> Self {
    let div = self.denom.gcd(self.num.abs() as usize);

    Self {
      num: self.num / div as isize,
      denom: self.denom / div,
    }
  }

  fn invert(&self) -> Result<Self, Box<dyn ToString>> {
    if self.num == 0 {
      Err(Box::new("Cannot invert fraction with 0 numerator."))
    } else if self.num >= 0 {
      Ok(Self {
        num: self.denom as isize,
        denom: self.num as usize,
      })
    } else {
      Ok(Self {
        num: -(self.denom as isize),
        denom: (-self.num) as usize,
      })
    }
  }
}

impl ToString for Fraction {
  fn to_string(&self) -> String {
    format!("{} / {}", self.num, self.denom)
  }
}

impl CustomType for Fraction {
  fn pre_add(&self, a: ReturnVal<Self>) -> Result<Argument<Self>, Box<dyn ToString>> {
    match a {
      ReturnVal::Int(i) => Ok(Argument::Custom(
        Self {
          num: self.num + self.denom as isize * i as isize,
          denom: self.denom,
        }
        .simplify(),
      )),
      ReturnVal::Custom(c) => Ok(Argument::Custom(
        Self {
          num: self.num * c.denom as isize + self.denom as isize * c.num as isize,
          denom: self.denom * c.denom,
        }
        .simplify(),
      )),
      _ => Err(Box::new("Not defined")),
    }
  }

  fn post_add(&self, v: ReturnVal<Self>) -> Result<Argument<Self>, Box<dyn ToString>> {
    self.pre_add(v)
  }

  fn pre_sub(&self, a: ReturnVal<Self>) -> Result<Argument<Self>, Box<dyn ToString>> {
    match a {
      ReturnVal::Int(i) => Ok(Argument::Custom(
        Self {
          num: self.num - self.denom as isize * i as isize,
          denom: self.denom,
        }
        .simplify(),
      )),
      ReturnVal::Custom(c) => Ok(Argument::Custom(
        Self {
          num: self.num * c.denom as isize - self.denom as isize * c.num,
          denom: self.denom * c.denom,
        }
        .simplify(),
      )),
      _ => Err(Box::new("Not defined")),
    }
  }

  fn post_sub(&self, a: ReturnVal<Self>) -> Result<Argument<Self>, Box<dyn ToString>> {
    match a {
      ReturnVal::Int(i) => Ok(Argument::Custom(
        Self {
          num: self.denom as isize * i as isize - self.num,
          denom: self.denom,
        }
        .simplify(),
      )),
      _ => Err(Box::new("Not defined")),
    }
  }

  fn pre_mult(&self, a: ReturnVal<Self>) -> Result<Argument<Self>, Box<dyn ToString>> {
    match a {
      ReturnVal::Int(i) => Ok(Argument::Custom(
        Self {
          num: self.num * i as isize,
          denom: self.denom,
        }
        .simplify(),
      )),
      ReturnVal::Custom(c) => Ok(Argument::Custom(
        Self {
          num: self.num * c.num,
          denom: self.denom * c.denom,
        }
        .simplify(),
      )),
      _ => Err(Box::new("Not defined")),
    }
  }

  fn post_mult(&self, a: ReturnVal<Self>) -> Result<Argument<Self>, Box<dyn ToString>> {
    self.pre_mult(a)
  }

  fn pre_div(&self, a: ReturnVal<Self>) -> Result<Argument<Self>, Box<dyn ToString>> {
    match a {
      ReturnVal::Int(i) => Ok(Argument::Custom(Self {
        num: self.num,
        denom: self.denom * {
          if i >= 0 {
            i
          } else {
            -i
          }
        } as usize,
      })),
      ReturnVal::Custom(c) => self.pre_mult(ReturnVal::Custom(c.invert()?)),
      _ => Err(Box::new("Not defined")),
    }
  }

  fn post_div(&self, a: ReturnVal<Self>) -> Result<Argument<Self>, Box<dyn ToString>> {
    match a {
      ReturnVal::Int(i) => {
        if self.num == 0 {
          Err(Box::new("Cannot divide by 0"))
        } else {
          Ok(Argument::Custom(Self {
            num: self.denom as isize * i as isize,
            denom: self.num as usize,
          }))
        }
      }
      _ => Err(Box::new("Not defined")),
    }
  }

  fn pre_not(&self) -> Result<Argument<Self>, Box<dyn ToString>> {
    Ok(Argument::Custom(self.invert()?))
  }

  fn pre_neg(&self) -> Result<Argument<Self>, Box<dyn ToString>> {
    Ok(Argument::Custom(Self {
      num: -self.num,
      denom: self.denom,
    }))
  }
}

fn main() {
  let mut lang = Language::<Fraction>::new();

  lang.add_custom_function(
    "new_frac".to_string(),
    CustomBuiltIn {
      function: |f| {
        if let ReturnVal::Tuple(v) = f {
          if v.len() == 2 {
            if let (ReturnVal::Int(n), ReturnVal::Int(d)) = (v.get(0).unwrap(), v.get(1).unwrap()) {
              if d >= &0 {
                Ok(Argument::Custom(
                  Fraction {
                    num: *n as isize,
                    denom: *d as usize,
                  }
                  .simplify(),
                ))
              } else {
                Ok(Argument::Custom(
                  Fraction {
                    num: (-*n) as isize,
                    denom: (-*d) as usize,
                  }
                  .simplify(),
                ))
              }
            } else {
              Err(Box::new("Wrong arg type."))
            }
          } else {
            Err(Box::new("Wrong arg type."))
          }
        } else {
          Err(Box::new("Wrong arg type."))
        }
      },
    },
  );

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
    Argument::Custom(Fraction { num: 3, denom: 5 }),
    Argument::Custom(Fraction { num: 8, denom: 3 }),
  ]));

  let res = func.call().unwrap_or_else(|e| {
    println!("Execution Error: \n{e:?}");
    exit(4)
  });

  println!("{}", res.to_string());
}
