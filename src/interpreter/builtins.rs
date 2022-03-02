use crate::interpreter::{interpret_function, interpret_lambda, Frame};
use crate::{InterpretError, InterpretVal};

pub fn built_in(
  name: String,
  arg: InterpretVal,
  frame: &mut Frame,
) -> Option<Result<InterpretVal, InterpretError>> {
  match name.as_str() {
    "list" => Some(list_func(arg)),
    "get" => Some(get_func(arg)),
    "map" => Some(map_func(arg, frame)),
    "filter" => Some(filter_func(arg, frame)),
    "len" => Some(length_func(arg)),
    "any" => Some(any_func(arg, frame)),
    "all" => Some(all_func(arg, frame)),
    "fold" => Some(fold_func(arg, frame)),
    _ => None,
  }
}

// Executes the builtin list function, which converts a tuple into a list.
fn list_func(arg: InterpretVal) -> Result<InterpretVal, InterpretError> {
  let a = arg.unwrap_tuple();

  if let InterpretVal::Tuple(v) = a {
    Ok(InterpretVal::List(v))
  } else {
    Ok(InterpretVal::List(vec![a]))
  }
}

// Executes the builtin len function
fn length_func(arg: InterpretVal) -> Result<InterpretVal, InterpretError> {
  if let InterpretVal::List(t) = arg.unwrap_tuple() {
    Ok(InterpretVal::Int(t.len() as i32))
  } else {
    Err(InterpretError::new(
      "Wrong argument type for `len` function.",
    ))
  }
}

// Executes the builtin get function, which gets an item at a specific index in a list
fn get_func(arg: InterpretVal) -> Result<InterpretVal, InterpretError> {
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
fn map_func(arg: InterpretVal, frame: &mut Frame) -> Result<InterpretVal, InterpretError> {
  if let InterpretVal::Tuple(t) = arg {
    if t.len() == 2 {
      match (t.get(0).unwrap(), t.get(1).unwrap()) {
        (InterpretVal::List(v), InterpretVal::Function(f)) => Ok(InterpretVal::List(
          v.iter()
            .map(|i| interpret_function(f, frame, i.clone()))
            .collect::<Result<Vec<InterpretVal>, InterpretError>>()?,
        )),
        (InterpretVal::List(v), InterpretVal::Lambda(f, e)) => {
          let mut env = e.clone();
          env.set_next(frame);
          Ok(InterpretVal::List(
            v.iter()
              .map(|i| interpret_lambda(f.clone(), &mut env, i.clone()))
              .collect::<Result<Vec<InterpretVal>, InterpretError>>()?,
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
fn filter_func(arg: InterpretVal, frame: &mut Frame) -> Result<InterpretVal, InterpretError> {
  if let InterpretVal::Tuple(t) = arg {
    if t.len() == 2 {
      match (t.get(0).unwrap(), t.get(1).unwrap()) {
        (InterpretVal::List(v), InterpretVal::Function(f)) => Ok(InterpretVal::List(
          v.iter()
            .map(|v| {
              if let InterpretVal::Bool(b) = interpret_function(f, frame, v.clone())? {
                Ok((v.clone(), b))
              } else {
                Err(InterpretError::new("Filter function was not a bool."))
              }
            })
            .collect::<Result<Vec<(InterpretVal, bool)>, InterpretError>>()?
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
                if let InterpretVal::Bool(b) = interpret_lambda(f.clone(), &mut env, v.clone())? {
                  Ok((v.clone(), b))
                } else {
                  Err(InterpretError::new("Filter function was not a bool."))
                }
              })
              .collect::<Result<Vec<(InterpretVal, bool)>, InterpretError>>()?
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
fn any_func(arg: InterpretVal, frame: &mut Frame) -> Result<InterpretVal, InterpretError> {
  if let InterpretVal::Tuple(t) = arg {
    if t.len() == 2 {
      match (t.get(0).unwrap(), t.get(1).unwrap()) {
        (InterpretVal::List(v), InterpretVal::Function(f)) => Ok(InterpretVal::Bool(
          v.iter()
            .map(|v| {
              if let InterpretVal::Bool(b) = interpret_function(f, frame, v.clone())? {
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
                if let InterpretVal::Bool(b) = interpret_lambda(f.clone(), &mut env, v.clone())? {
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
fn all_func(arg: InterpretVal, frame: &mut Frame) -> Result<InterpretVal, InterpretError> {
  if let InterpretVal::Tuple(t) = arg {
    if t.len() == 2 {
      match (t.get(0).unwrap(), t.get(1).unwrap()) {
        (InterpretVal::List(v), InterpretVal::Function(f)) => Ok(InterpretVal::Bool(
          v.iter()
            .map(|v| {
              if let InterpretVal::Bool(b) = interpret_function(f, frame, v.clone())? {
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
                if let InterpretVal::Bool(b) = interpret_lambda(f.clone(), &mut env, v.clone())? {
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
fn fold_func(arg: InterpretVal, frame: &mut Frame) -> Result<InterpretVal, InterpretError> {
  if let InterpretVal::Tuple(t) = arg {
    if t.len() == 3 {
      match (t.get(0).unwrap(), t.get(1).unwrap(), t.get(2).unwrap()) {
        (InterpretVal::List(v), s, InterpretVal::Function(f)) => {
          v.iter().fold(Ok(s.clone()), |acc, x| {
            interpret_function(f, frame, InterpretVal::Tuple(vec![acc?, x.clone()]))
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
