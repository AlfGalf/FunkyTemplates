use crate::interpreter::{interpret_function, interpret_lambda, Customs, Frame};
use crate::{CustomType, InterpretError, InterpretVal};

// Checks if the token refers to an inbuilt function
// If it does, executes that function and returns Some() with the result of the function
// Otherwise, returns none
pub fn built_in<C: CustomType>(name: &str, customs: &Customs<C>) -> Option<InterpretVal<C>> {
  match name {
    "list" => Some(InterpretVal::BuiltIn(name.to_string(), list_func)),
    "get" => Some(InterpretVal::BuiltIn(name.to_string(), get_func)),
    "map" => Some(InterpretVal::BuiltIn(name.to_string(), map_func)),
    "filter" => Some(InterpretVal::BuiltIn(name.to_string(), filter_func)),
    "len" => Some(InterpretVal::BuiltIn(name.to_string(), length_func)),
    "any" => Some(InterpretVal::BuiltIn(name.to_string(), any_func)),
    "all" => Some(InterpretVal::BuiltIn(name.to_string(), all_func)),
    "fold" => Some(InterpretVal::BuiltIn(name.to_string(), fold_func)),
    n => {
      if customs.built_ins.contains_key(n) {
        Some(InterpretVal::BuiltIn(n.to_string(), eval_custom))
      } else {
        None
      }
    }
  }
}

// Executes the builtin list function, which converts a tuple into a list.
fn eval_custom<C: CustomType>(
  arg: InterpretVal<C>,
  _: &mut Frame<C>,
  customs: &Customs<C>,
  n: String,
) -> Result<InterpretVal<C>, InterpretError> {
  let a = arg.unwrap_tuple();
  customs.built_ins.get(&n).unwrap().call_func(&a)
}

// Executes the builtin list function, which converts a tuple into a list.
fn list_func<C: CustomType>(
  arg: InterpretVal<C>,
  _: &mut Frame<C>,
  _: &Customs<C>,
  _: String,
) -> Result<InterpretVal<C>, InterpretError> {
  let a = arg.unwrap_tuple();

  if let InterpretVal::Tuple(v) = a {
    Ok(InterpretVal::List(v))
  } else {
    Ok(InterpretVal::List(vec![a]))
  }
}

// Executes the builtin len function
fn length_func<C: CustomType>(
  arg: InterpretVal<C>,
  _: &mut Frame<C>,
  _: &Customs<C>,
  _: String,
) -> Result<InterpretVal<C>, InterpretError> {
  if let InterpretVal::List(t) = arg.unwrap_tuple() {
    Ok(InterpretVal::Int(t.len() as i32))
  } else {
    Err(InterpretError::new(
      "Wrong argument type for `len` function.",
    ))
  }
}

// Executes the builtin get function, which gets an item at a specific index in a list
fn get_func<C: CustomType>(
  arg: InterpretVal<C>,
  _: &mut Frame<C>,
  _: &Customs<C>,
  _: String,
) -> Result<InterpretVal<C>, InterpretError> {
  let a = arg.unwrap_tuple();

  if let InterpretVal::Tuple(v) = a {
    if v.len() == 2 {
      if let (InterpretVal::List(l), InterpretVal::Int(i)) = (v[0].clone(), v[1].clone()) {
        if i < l.len() as i32 || i < 0 {
          Ok(l[i as usize].clone())
        } else {
          Err(InterpretError::new("Index out of range."))
        }
      } else {
        Err(InterpretError::new(
          format!("Wrong arguments for `get` ({:?}, {:?})", v[0], v[1]).as_str(),
        ))
      }
    } else {
      Err(InterpretError::new(
        "Wrong number of arguments provided for `get`.",
      ))
    }
  } else {
    Err(InterpretError::new(
      "Wrong number of arguments provided for `get`.",
    ))
  }
}

// Executes the builtin map function
fn map_func<C: CustomType>(
  arg: InterpretVal<C>,
  frame: &mut Frame<C>,
  customs: &Customs<C>,
  _: String,
) -> Result<InterpretVal<C>, InterpretError> {
  if let InterpretVal::Tuple(t) = arg {
    if t.len() == 2 {
      match (t.get(0).unwrap(), t.get(1).unwrap()) {
        (InterpretVal::List(v), InterpretVal::Function(f)) => Ok(InterpretVal::List(
          v.iter()
            .map(|i| interpret_function(f, frame, i.clone(), customs))
            .collect::<Result<Vec<InterpretVal<C>>, InterpretError>>()?,
        )),
        (InterpretVal::List(v), InterpretVal::Lambda(f, e)) => {
          let mut env = e.clone();
          env.set_next(frame);
          Ok(InterpretVal::List(
            v.iter()
              .map(|i| interpret_lambda(f.clone(), &mut env, i.clone(), customs))
              .collect::<Result<Vec<InterpretVal<C>>, InterpretError>>()?,
          ))
        }
        _ => Err(InterpretError::new(
          format!(
            "Wrong argument types provided to map: {:?}, {:?}",
            t.get(0),
            t.get(1)
          )
          .as_str(),
        )),
      }
    } else {
      Err(InterpretError::new(
        "Wrong number of arguments provided to map.",
      ))
    }
  } else {
    Err(InterpretError::new(
      "Wrong number of arguments provided to map.",
    ))
  }
}

// Executes the builtin filter function
fn filter_func<C: CustomType>(
  arg: InterpretVal<C>,
  frame: &mut Frame<C>,
  customs: &Customs<C>,
  _: String,
) -> Result<InterpretVal<C>, InterpretError> {
  if let InterpretVal::Tuple(t) = arg {
    if t.len() == 2 {
      match (t.get(0).unwrap(), t.get(1).unwrap()) {
        (InterpretVal::List(v), InterpretVal::Function(f)) => Ok(InterpretVal::List(
          v.iter()
            .map(|v| {
              if let InterpretVal::Bool(b) = interpret_function(f, frame, v.clone(), customs)? {
                Ok((v.clone(), b))
              } else {
                Err(InterpretError::new("Filter function was not a bool."))
              }
            })
            .collect::<Result<Vec<(InterpretVal<C>, bool)>, InterpretError>>()?
            .iter()
            .filter(|(_, b)| *b)
            .map(|(v, _)| v.clone())
            .collect(),
        )),
        (InterpretVal::List(v), InterpretVal::Lambda(f, e)) => {
          let mut env = e.clone();
          env.set_next(frame);

          Ok(InterpretVal::List(
            v.iter()
              .map(|v| {
                if let InterpretVal::Bool(b) =
                  interpret_lambda(f.clone(), &mut env, v.clone(), customs)?
                {
                  Ok((v.clone(), b))
                } else {
                  Err(InterpretError::new("Filter function was not a bool."))
                }
              })
              .collect::<Result<Vec<(InterpretVal<C>, bool)>, InterpretError>>()?
              .iter()
              .filter(|(_, b)| *b)
              .map(|(v, _)| v.clone())
              .collect(),
          ))
        }
        _ => Err(InterpretError::new(
          format!(
            "Wrong argument types provided to filter: {:?}, {:?}",
            t.get(0),
            t.get(1)
          )
          .as_str(),
        )),
      }
    } else {
      Err(InterpretError::new(
        "Wrong number of arguments provided to filter.",
      ))
    }
  } else {
    Err(InterpretError::new(
      "Wrong number of arguments provided to filter.",
    ))
  }
}

// Executes the builtin any function
fn any_func<C: CustomType>(
  arg: InterpretVal<C>,
  frame: &mut Frame<C>,
  customs: &Customs<C>,
  _: String,
) -> Result<InterpretVal<C>, InterpretError> {
  if let InterpretVal::Tuple(t) = arg {
    if t.len() == 2 {
      match (t.get(0).unwrap(), t.get(1).unwrap()) {
        (InterpretVal::List(v), InterpretVal::Function(f)) => Ok(InterpretVal::Bool(
          v.iter()
            .map(|v| {
              if let InterpretVal::Bool(b) = interpret_function(f, frame, v.clone(), customs)? {
                Ok(b)
              } else {
                Err(InterpretError::new("Any function result was not a bool."))
              }
            })
            .collect::<Result<Vec<bool>, InterpretError>>()?
            .iter()
            .any(|v| *v),
        )),
        (InterpretVal::List(v), InterpretVal::Lambda(f, e)) => {
          let mut env = e.clone();
          env.set_next(frame);
          Ok(InterpretVal::Bool(
            v.iter()
              .map(|v| {
                if let InterpretVal::Bool(b) =
                  interpret_lambda(f.clone(), &mut env, v.clone(), customs)?
                {
                  Ok(b)
                } else {
                  Err(InterpretError::new("Any function result was not a bool."))
                }
              })
              .collect::<Result<Vec<bool>, InterpretError>>()?
              .iter()
              .any(|v| *v),
          ))
        }
        _ => Err(InterpretError::new(
          format!(
            "Wrong argument types provided to any: {:?}, {:?}",
            t.get(0),
            t.get(1)
          )
          .as_str(),
        )),
      }
    } else {
      Err(InterpretError::new(
        "Wrong number of arguments provided to any.",
      ))
    }
  } else {
    Err(InterpretError::new(
      "Wrong number of arguments provided to any.",
    ))
  }
}

// Executes the builtin all function
fn all_func<C: CustomType>(
  arg: InterpretVal<C>,
  frame: &mut Frame<C>,
  customs: &Customs<C>,
  _: String,
) -> Result<InterpretVal<C>, InterpretError> {
  if let InterpretVal::Tuple(t) = arg {
    if t.len() == 2 {
      match (t.get(0).unwrap(), t.get(1).unwrap()) {
        (InterpretVal::List(v), InterpretVal::Function(f)) => Ok(InterpretVal::Bool(
          v.iter()
            .map(|v| {
              if let InterpretVal::Bool(b) = interpret_function(f, frame, v.clone(), customs)? {
                Ok(b)
              } else {
                Err(InterpretError::new("Any function result was not a bool."))
              }
            })
            .collect::<Result<Vec<bool>, InterpretError>>()?
            .iter()
            .all(|v| *v),
        )),
        (InterpretVal::List(v), InterpretVal::Lambda(f, e)) => {
          let mut env = e.clone();
          env.set_next(frame);

          Ok(InterpretVal::Bool(
            v.iter()
              .map(|v| {
                if let InterpretVal::Bool(b) =
                  interpret_lambda(f.clone(), &mut env, v.clone(), customs)?
                {
                  Ok(b)
                } else {
                  Err(InterpretError::new("Any function result was not a bool."))
                }
              })
              .collect::<Result<Vec<bool>, InterpretError>>()?
              .iter()
              .all(|v| *v),
          ))
        }
        _ => Err(InterpretError::new(
          format!(
            "Wrong argument types provided to any: {:?}, {:?}",
            t.get(0),
            t.get(1)
          )
          .as_str(),
        )),
      }
    } else {
      Err(InterpretError::new(
        "Wrong number of arguments provided to any.",
      ))
    }
  } else {
    Err(InterpretError::new(
      "Wrong number of arguments provided to any.",
    ))
  }
}

// Executes the builtin fold function
fn fold_func<C: CustomType>(
  arg: InterpretVal<C>,
  frame: &mut Frame<C>,
  customs: &Customs<C>,
  _: String,
) -> Result<InterpretVal<C>, InterpretError> {
  if let InterpretVal::Tuple(t) = arg {
    if t.len() == 3 {
      match (t.get(0).unwrap(), t.get(1).unwrap(), t.get(2).unwrap()) {
        (InterpretVal::List(v), s, InterpretVal::Function(f)) => {
          v.iter().fold(Ok(s.clone()), |acc, x| {
            interpret_function(
              f,
              frame,
              InterpretVal::Tuple(vec![acc?, x.clone()]),
              customs,
            )
          })
        }
        (InterpretVal::List(v), s, InterpretVal::Lambda(f, e)) => {
          v.iter().fold(Ok(s.clone()), |acc, x| {
            let mut l = e.clone();
            l.set_next(frame);
            interpret_lambda(
              f.clone(),
              &mut l,
              InterpretVal::Tuple(vec![acc?, x.clone()]),
              customs,
            )
          })
        }
        _ => Err(InterpretError::new(
          "Wrong arguments types provided to fold.",
        )),
      }
    } else {
      Err(InterpretError::new(
        "Wrong number of arguments provided to fold.",
      ))
    }
  } else {
    Err(InterpretError::new(
      "Wrong number of arguments provided to fold.",
    ))
  }
}
