use crate::{InterpretError, InterpretVal};

pub fn built_in(name: String, arg: InterpretVal) -> Option<Result<InterpretVal, InterpretError>> {
    match name.as_str() {
        "list" => Some(list_func(arg)),
        _ => None,
    }
}

fn list_func(arg: InterpretVal) -> Result<InterpretVal, InterpretError> {
    let a = arg.unwrap_tuple();

    if let InterpretVal::Tuple(v) = a {
        Ok(InterpretVal::List(v))
    } else {
        Ok(InterpretVal::List(vec![a]))
    }
}
