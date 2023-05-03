use std::fs;

use protols::parser::{tokenize, ParseError, Parser};

fn main() {
    let file = fs::read_to_string("examples/example.proto").unwrap();

    let tokens = tokenize(&file);
    let parse_result = Parser::new(tokens).parse("example.proto");

    println!("{:#?}", parse_result.root);
    println!("{:#?}", parse_result.errors);

    print_errors(&parse_result.errors, &file);
}

fn print_errors(errors: &[ParseError], input: &str) {
    for error in errors {
        println!();
        println!("{:?}", error);
        let context = &input[error.position - 25..error.position + 25];
        println!("{}", context);
        println!();
    }
}
