#[derive(Copy, Clone)]
pub enum OperatorType {
    Addition,
    Substraction,
}

impl OperatorType {
    pub fn to_string(&self) -> String {
        return String::from(match self {
            Self::Addition => "+",
            Self::Substraction => "-"
        })
    }
}

pub enum Token {
    Operator(OperatorType),
    Value(i32)
}

impl Token {
    pub fn to_string(&self) -> String {
        return match self {
            Token::Operator(op) => op.to_string(),
            Token::Value(val) => val.to_string()
        }
    }
}
