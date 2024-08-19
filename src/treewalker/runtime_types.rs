#[derive(PartialEq)]
pub enum RuntimeValue {
    String(String),
    Number(f64),
    None,
    Boolean(bool),
}
