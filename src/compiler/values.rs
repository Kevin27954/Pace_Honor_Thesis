use std::fmt::Display;

/*
* Enum is 4 byte (Number, Boolean, None - each one is a TAG represented by int (u32) )
* Current max memory is f64 -> 8 bytes
* Padding due to wanting same size bytes, so + 4 byte to TAG
* Total = 16 Byte
*/

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    // We can make this more efficent by making it f16 or f32 instead. However for simplicity we
    // won't do this
    Number(f64),
    Boolean(bool),
    None,
    //String(String),
    ValueObj(ValueObj),
}

#[derive(Debug, PartialEq, Clone)]
pub enum ValueObj {
    String(Box<String>),
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut str = String::new();

        let concat_str = match self {
            Self::Number(num) => {
                format!("{}", num)
            }
            Self::Boolean(bool) => {
                format!("{}", bool)
            }
            Self::None => {
                format!("none")
            }
            Self::ValueObj(value_obj) => match value_obj {
                ValueObj::String(string) => {
                    format!("{}", string)
                }
            },
        };

        str.push_str(&concat_str);
        write!(f, "{}", str)
    }
}
