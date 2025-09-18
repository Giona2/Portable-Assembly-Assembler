use std::{
    fs,
    sync::Arc,
};

use clap::Parser;

use crate::lexer::tokens::LexingToken;

//mod assembler;
mod lexer;
mod error;
mod syntax_tree;


#[derive(Parser)]
struct CmdArgs {
    #[arg(index=1)]
    target_file_path: String,
}


fn main() {
    // Parse the command line arguments
    let cmd_args = CmdArgs::parse();

    // Open the given soruce file
    let file_content: String = fs::read_to_string(cmd_args.target_file_path).unwrap();

    // Construct the token stream
    let token_stream: Vec<LexingToken> = lexer::generate_lexing_token_stream(Arc::new(file_content));
    println!("\n\n=== Lexer ===");
    for token in token_stream.clone() {
        print!("{:?} ", token);
        if token == LexingToken::EndOfInstruction {
            println!("\n");
        }
    }

    println!("\n\n=== Syntax Tree ===");
    let syntax_token_tree = syntax_tree::generate_syntax_tree(&token_stream);
    for token in syntax_token_tree.clone() {
        println!("{:?}", token);
    }
}
