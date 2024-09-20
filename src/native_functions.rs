use std::time::{SystemTime, UNIX_EPOCH};

use crate::compiler::values::{NativeFn, Value};

fn make_native(
    name: &str,
    native_fn: fn(usize, &[Value]) -> Result<Value, &str>,
    arity: u8,
) -> NativeFn {
    NativeFn {
        name: name.to_string(),
        native_fn,
        arity,
    }
}

pub fn get_all_natives() -> Vec<NativeFn> {
    vec![
        make_native("clock", clock, 0),
        make_native("print", print, 255),
    ]
}

fn print(_args: usize, values: &[Value]) -> Result<Value, &str> {
    let output_str = values
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<_>>()
        .join(", ");
    println!("{output_str}");

    Ok(Value::None)
}

fn clock(_args: usize, _values: &[Value]) -> Result<Value, &str> {
    let secs = SystemTime::now().duration_since(UNIX_EPOCH);
    match secs {
        Ok(sec) => Ok(Value::Number(sec.as_secs_f64())),
        Err(_err) => Err("Error getting seconds"),
    }
}
