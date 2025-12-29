fn is_operator_str(str: &String) -> bool {
    if str == "+" || str == "-" {
        return true;
    } else {
        return false;
    }
}

fn calculate_with(operant: &String, operants: &Vec<i32>) -> i32 {
    if operant == "+" {
        return operants[0] + operants[1];
    }
    if operant == "-" {
        return operants[0] - operants[1];
    }
    return 0;
}

pub fn calculate(tokens: Vec<String>) -> i32 {
    let mut operants: Vec<i32> = Vec::new();
    let mut operator: String = String::from("");

    let mut total: i32 = 0;

    for token in tokens {
        if is_operator_str(&token) {
            // the "+" and the "-" operator needs an operant before itself
            if operants.len() == 1 {
                operator = token;
            } else {
                // TODO: This should be an error
                println!("There is not operants for the operator");
                return 0;
            }
        } else {
            let value: i32 = token.parse().expect("The string is not a numeric value");
            operants.push(value);
            // if there are enough operators, then execute the operation
            if operants.len() == 2 && operator != "" {
                total = calculate_with(&operator, &operants);
                operants = vec![total];
            }
        }
    }
    return total;
}
