use crate::{
    compiler::{chunk::Chunk, values::Value},
    vm::VM,
};

fn assert_eq_value(source_str: String, expected_value: Value) {
    let mut vm = VM::new(Chunk::new());
    match vm.interpret(source_str) {
        Ok(value) => {
            assert_eq!(value, expected_value)
        }
        Err(_err) => {}
    }
}

#[test]
fn numeric_expr() {
    let source_str = String::from("(-1 + 2) * 3 - -4 ");
    assert_eq_value(source_str, Value::Number(7.0));
    let source_str = String::from("(5 - (3 - 1)) + -1");
    assert_eq_value(source_str, Value::Number(2.0));
    let source_str = String::from("((8 * 2) - (15 / (3 + 2)))");
    assert_eq_value(source_str, Value::Number(13.0));
}

#[test]
fn equality_expr() {
    let source_str = String::from("!(5 - 4 > true == !none)");
    assert_eq_value(source_str, Value::Boolean(false));
    let source_str = String::from("((5 > 3) == (10 < 20)) == ((true != false) == (1 == 1)");
    assert_eq_value(source_str, Value::Boolean(true));
}

#[test]
fn greater_expr() {
    let source_str = String::from("3 > 3");
    assert_eq_value(source_str, Value::Boolean(false));
    let source_str = String::from("9 > 3");
    assert_eq_value(source_str, Value::Boolean(true));
    let source_str = String::from("1 > 3");
    assert_eq_value(source_str, Value::Boolean(false));
    let source_str = String::from("2 > 8");
    assert_eq_value(source_str, Value::Boolean(false));
}

#[test]
fn less_expr() {
    let source_str = String::from("3 < 3");
    assert_eq_value(source_str, Value::Boolean(false));
    let source_str = String::from("9 < 3");
    assert_eq_value(source_str, Value::Boolean(false));
    let source_str = String::from("1 < 3");
    assert_eq_value(source_str, Value::Boolean(true));
    let source_str = String::from("2 < 8");
    assert_eq_value(source_str, Value::Boolean(true));
}

#[test]
fn greater_eq_expr() {
    let source_str = String::from("3 >= 3");
    assert_eq_value(source_str, Value::Boolean(true));
    let source_str = String::from("9 >= 3");
    assert_eq_value(source_str, Value::Boolean(true));
    let source_str = String::from("1 >= 3");
    assert_eq_value(source_str, Value::Boolean(false));
    let source_str = String::from("2 >= 8");
    assert_eq_value(source_str, Value::Boolean(false));
}

#[test]
fn less_eq_expr() {
    let source_str = String::from("3 <= 3");
    assert_eq_value(source_str, Value::Boolean(true));
    let source_str = String::from("9 <= 3");
    assert_eq_value(source_str, Value::Boolean(false));
    let source_str = String::from("1 <= 3");
    assert_eq_value(source_str, Value::Boolean(true));
    let source_str = String::from("2 <= 8");
    assert_eq_value(source_str, Value::Boolean(true));
}
