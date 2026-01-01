#[derive(Copy, Clone)]
pub enum DelimiterType {
    OpenBraket,
    CloseBraket
}

#[derive(Copy, Clone)]
pub enum OperatorType {
    Addition,
    Substraction,
}

pub enum Token {
    Operator(OperatorType),
    Delimiter(DelimiterType),
    Value(i32)
}

// Implementations

impl DelimiterType {
    pub fn to_string(&self) -> String {
        return String::from(match self {
            Self::OpenBraket => '(',
            Self::CloseBraket => ')',
        })
    }
}

impl OperatorType {//
    pub fn to_string(&self) -> String {
        return String::from(match self {
            Self::Addition => "+",
            Self::Substraction => "-"
        })
    }
}

impl Token {
    pub fn to_string(&self) -> String {
        return match self {
            Token::Operator(op) => op.to_string(),
            Token::Value(val) => val.to_string(),
            Token::Delimiter(del) => del.to_string(),
        }
    }
}
