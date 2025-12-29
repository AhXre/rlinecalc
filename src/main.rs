mod tokenizer;
mod calculator;

use clap::Parser;
use tokenizer::tokenize;
use calculator::calculate;

// reads the input line and parses it to the arguments
#[derive(Parser)]
struct Cli {
    operation: String,
}

fn main() {
    let args = Cli::parse();

    // Tokenize the line gotted from the command call
    let tokens = tokenize(args.operation);

    let result = calculate(tokens);

    println!("Result: {:?}", result);
}
