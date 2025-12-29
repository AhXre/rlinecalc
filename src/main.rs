mod tokenizer;

use clap::Parser;
use tokenizer::tokenize;

// reads the input line and parses it to the arguments
#[derive(Parser)]
struct Cli {
    operation: String,
}

fn main() {
    let args = Cli::parse();

    // Tokenize the line gotted from the command call
    let tokens = tokenize(args.operation);

    // Then it prints the tokens
    for token in tokens {
        println!("{:?}", token)
    }
}
