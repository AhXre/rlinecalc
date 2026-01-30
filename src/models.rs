#[derive(Copy, Clone)]
pub enum DelimiterType {
    OpenBraket,
    CloseBraket
}

#[derive(Copy, Clone)]
pub enum OperatorType {
    Addition,
    Substraction,
    Multiplication,
    Division,
}

pub enum Token {
    Operator(OperatorType),
    Delimiter(DelimiterType),
    Value(f64)
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
            Self::Substraction => "-",
            Self::Multiplication => "*",
            Self::Division => "/",
        })
    }
    pub fn get_priority(&self) -> i32 {
        return match self {
            Self::Addition => 0,
            Self::Substraction => 0,
            Self::Multiplication => 1,
            Self::Division => 1
        }
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
