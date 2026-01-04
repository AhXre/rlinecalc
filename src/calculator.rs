use std::collections::VecDeque;
use std::iter::Peekable;
use std::slice::Iter;

use crate::models::DelimiterType;
use crate::models::Token;
use crate::models::OperatorType;

/**
 * This function helps us determine what to do with each operator
 */
fn execute_operator(operator: &OperatorType, operants: &VecDeque<i32>) -> i32 {
    return match operator {
        OperatorType::Addition => operants[0] + operants[1],
        OperatorType::Substraction => operants[0] - operants[1],
        OperatorType::Multiplication => operants[0] * operants[1],
        OperatorType::Division => operants[0] / operants[1]
    }
}

/** Function that makes the three */
fn make_tree(iter: &mut Peekable<Iter<Token>>, dq: &mut VecDeque<Token>) {

    let priority: i32 = if let Some(Token::Operator(operator)) = dq.get(0) { operator.get_priority() } else {0};

    // Now there is no tree but a ordered list (deque) that will contain the order in which one must process the operands
    //
    while let Some(token) = iter.peek() {
        match token {
            Token::Delimiter(delimiter) => match delimiter {
                // the one you must detect first is the close braket as we are in reverse
                DelimiterType::CloseBraket => {
                    // maybe return
                    iter.next();
                    return;
                },
                DelimiterType::OpenBraket => {
                    iter.next();
                    let mut sub_dq: VecDeque<Token> = VecDeque::new();
                    make_tree(iter, &mut sub_dq);
                    dq.append(&mut sub_dq);
                }
            },
            Token::Operator(operator) => {
                if operator.get_priority() < priority {
                    return;
                } else if operator.get_priority() > priority {
                    if let Some(last_token) = dq.pop_back() {
                        let mut sub_dq: VecDeque<Token> = VecDeque::new();
                        sub_dq.push_back(Token::Operator(*operator));
                        if let Token::Value(_) = last_token {
                            sub_dq.push_back(last_token);
                        }
                        iter.next();
                        make_tree(iter, &mut sub_dq);
                        dq.append(&mut sub_dq);
                    } else {
                        println!("the sintaxis is wrong, maybe the operator is before the operand");
                        return;
                    }
                } else {
                    dq.push_front(Token::Operator(*operator));
                    iter.next();
                }
            },
            Token::Value(value) => {
                dq.push_back(Token::Value(*value));
                iter.next();
            }
        }
    }
}

fn calculate_with_deque(dq: &VecDeque<Token>) -> i32 {
    let mut operands: VecDeque<i32> = VecDeque::new();
    let mut iter_rev = dq.iter().rev();
    while let Some(token) = iter_rev.next() {
        match token {
            Token::Operator(op) => {
                let mut r_operands = operands.split_off(2);
                r_operands.push_front(execute_operator(op, &operands));
                operands = r_operands;
            },
            Token::Value(val) => {
                operands.push_front(*val);
            },
            _ => {
                println!("What's doing a delimiter here?");
                return 0;
            }
        }
    }
    if let Some(value) = operands.get(0) {
        return *value;
    } else {
        return 0;
    }
}

pub fn calculate(tokens: &Vec<Token>) -> i32 {
    let mut deque: VecDeque<Token> = VecDeque::new();
    make_tree(&mut tokens.iter().peekable(), &mut deque);

    println!("Tree: ");
    for token in &deque {
        print!("{:?},", token.to_string());
    }
    println!("\n--------");

    return calculate_with_deque(&deque);
}
