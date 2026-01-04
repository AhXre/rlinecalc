use std::iter::Peekable;
use std::iter::Rev;
use std::slice::Iter;

use crate::models::DelimiterType;
use crate::models::Token;
use crate::models::OperatorType;

const MAX_OPERANTS_SIZE: usize = 3;

struct OperationStruct {
    operator: Option<OperatorType>,
    operants: Vec<Operant>
}

enum Operant {
    Value(i32),
    Operation(OperationStruct)
}

/**
 * This function helps us determine what to do with each operator
 */
fn execute_operator(operator: &OperatorType, operants: &Vec<i32>) -> i32 {
    return match operator {
        OperatorType::Addition => operants[0] + operants[1],
        OperatorType::Substraction => operants[0] - operants[1],
        OperatorType::Multiplication => operants[0] * operants[1],
        OperatorType::Division => operants[0] / operants[1]
    }
}

/** Function that makes the three */
fn make_tree(iter: &mut Peekable<Rev<Iter<Token>>>) -> Operant {

    // this could be a single operant and still work
    let mut operant: Option<Operant> = None;

    while let Some(token) = iter.peek() {
        match token {
            Token::Operator(op) => {
                let mut operation = OperationStruct {
                    operator: Some(*op),
                    operants: Vec::with_capacity(MAX_OPERANTS_SIZE),
                };
                iter.next();
                operation.operants.push(make_tree(iter));
                if let Some(value) = operant {
                    operation.operants.push(value);
                }
                return Operant::Operation(operation);
            },
            Token::Delimiter(del) => match del {
                DelimiterType::CloseBraket => {
                    // we are analizing the tokens in reverse, so this is like opening
                    iter.next();
                    operant = Some(make_tree(iter));
                },
                DelimiterType::OpenBraket => {
                    // i hope this breaks the while and not the match
                    iter.next();
                    break;
                }
            },
            Token::Value(val) => {
                iter.next();
                operant = Some(Operant::Value(*val));
            }
        }
    }
    if let Some(value) = operant {
        return value;
    } else {
        return Operant::Value(0);
    }
}

fn print_tree(root: &Operant) {
    match root {
        Operant::Operation(value) => {
            if let Some(operator) = value.operator {
                println!("{:?}", operator.to_string());
            } else {
                println!("no operator");
            }
            for operant in &value.operants {
                print_tree(operant);
            }
        },
        Operant::Value(value) => {
            println!("{:?}", value);
        }
    }
}

fn solve_tree(root: &Operant) -> i32 {
    match root {
        Operant::Operation(value) => {
            let mut operants: Vec<i32> = Vec::with_capacity(MAX_OPERANTS_SIZE);
            for operant in &value.operants {
                operants.push(solve_tree(operant));
            }
            if let Some(operator) = value.operator {
                return execute_operator(&operator, &operants);
            } else {
                println!("error ahhh: No hay operador");
                return 0;
            }
        },
        Operant::Value(value) => {
            return *value;
        }
    }
}

pub fn calculate(tokens: &Vec<Token>) -> i32 {
    let root: Operant = make_tree(&mut tokens.iter().rev().peekable());

    println!("Tree: ");
    print_tree(&root);
    println!("--------");

    return solve_tree(&root);
}
