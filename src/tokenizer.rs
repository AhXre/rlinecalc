use crate::models::OperatorType;
use crate::models::DelimiterType;
use crate::models::Token;

/**
 * Function to get the operator
 */
fn get_token(ch: char) -> Option<Token> {
    return match ch {
        '+' => Some(Token::Operator(OperatorType::Addition)),
        '-' => Some(Token::Operator(OperatorType::Substraction)),
        '*' => Some(Token::Operator(OperatorType::Multiplication)),
        '/' => Some(Token::Operator(OperatorType::Division)),
        '(' => Some(Token::Delimiter(DelimiterType::OpenBraket)),
        ')' => Some(Token::Delimiter(DelimiterType::CloseBraket)),
        _ => None,
    }
}

/**
 * Tokenizes the string to fit the standart of the program
 */
pub fn tokenize(line: String) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new();
    let mut token: String = String::from("");

    for ch in line.chars() {
        if ch.is_ascii_digit() {
            token = token + &ch.to_string();
        } else if let Some(value) = get_token(ch) {
            // If there was a token read, then it saves it
            if token.len() != 0 {
                tokens.push(Token::Value(token.parse().expect("Error: This token is not a number")));
                token = String::from("");
            }
            tokens.push(value);
        }
        // ignore space characters
        // ignore unknown characters
    }

    // pushes the token if it was not finalized
    if token.len() != 0 {
        tokens.push(Token::Value(token.parse().expect("This token is not a number")));
    }

    return tokens;
}
