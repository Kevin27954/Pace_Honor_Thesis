use std::fmt::Display;

#[derive(Debug, Clone, Copy)]
pub enum Value {
    // We can make this more efficent by making it f16 or f32 instead. However for simplicity we
    // won't do this
    Number(f64),
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut str = String::new();

        let concat_str = match self {
            Self::Number(num) => {
                format!("{}", num)
            }
        };

        str.push_str(&concat_str);
        write!(f, "{}", str)
    }
}