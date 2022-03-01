use crate::interpreter::{interpret_function, Frame};
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
    _ => None,
  }
}

// Executes the builtin list funciton, which converts a tuple into a list.
fn list_func(arg: InterpretVal) -> Result<InterpretVal, InterpretError> {
  let a = arg.unwrap_tuple();

  if let InterpretVal::Tuple(v) = a {
    Ok(InterpretVal::List(v))
  } else {
    Ok(InterpretVal::List(vec![a]))
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
      if let (InterpretVal::List(v), InterpretVal::Function(f)) =
        (t.get(0).unwrap(), t.get(1).unwrap())
      {
        Ok(InterpretVal::List(
          v.iter()
            .map(|i| interpret_function(f, frame, i.clone()))
            .collect::<Result<Vec<InterpretVal>, InterpretError>>()?,
        ))
      } else {
        Err(InterpretError::new(
          format!(
            "Wrong argument types provided to map: {:?}, {:?}",
            t.get(0),
            t.get(1)
          )
          .as_str(),
        ))
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
      if let (InterpretVal::List(v), InterpretVal::Function(f)) =
        (t.get(0).unwrap(), t.get(1).unwrap())
      {
        Ok(InterpretVal::List(
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
        ))
      } else {
        Err(InterpretError::new(
          format!(
            "Wrong argument types provided to filter: {:?}, {:?}",
            t.get(0),
            t.get(1)
          )
          .as_str(),
        ))
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

// Executes the builtin any function
fn any_func(arg: InterpretVal, frame: &mut Frame) -> Result<InterpretVal, InterpretError> {
  if let InterpretVal::Tuple(t) = arg {
    if t.len() == 2 {
      if let (InterpretVal::List(v), InterpretVal::Function(f)) =
        (t.get(0).unwrap(), t.get(1).unwrap())
      {
        Ok(InterpretVal::Bool(
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
        ))
      } else {
        Err(InterpretError::new(
          format!(
            "Wrong argument types provided to any: {:?}, {:?}",
            t.get(0),
            t.get(1)
          )
          .as_str(),
        ))
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
      if let (InterpretVal::List(v), InterpretVal::Function(f)) =
        (t.get(0).unwrap(), t.get(1).unwrap())
      {
        Ok(InterpretVal::Bool(
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
        ))
      } else {
        Err(InterpretError::new(
          format!(
            "Wrong argument types provided to any: {:?}, {:?}",
            t.get(0),
            t.get(1)
          )
          .as_str(),
        ))
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
