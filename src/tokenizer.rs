
fn is_operator(ch: char) -> bool {
    if ch == '+' || ch == '-' {
        return true
    } else {
        return false
    }
}

// Tokenizes the string to fit the standart of the program
pub fn tokenize(line: String) -> Vec<String> {
    let mut tokens: Vec<String> = Vec::new();
    let mut token: String = "".into();

    for ch in line.chars() {
        if ch.is_ascii_digit() {
            token = token + &ch.to_string();
        } else if is_operator(ch) {
            if token.len() != 0 {
                tokens.push(token);
                token = String::from("");
            }
            tokens.push(ch.to_string());
        }
        // ignore space characters
        // ignore not acknowledge characters
    }

    // pushes the token if it was not finalized
    if token.len() != 0 {
        tokens.push(token);
    }

    return tokens;
}
