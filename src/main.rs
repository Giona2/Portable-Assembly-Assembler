use std::{
    fs,
    sync::Arc,
};

use clap::Parser;

mod assembler;
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
    let token_stream: Vec<lexer::LexingToken> = lexer::generate_lexing_token_stream(Arc::new(file_content));
    println!("{:?}", token_stream);

    // Construct the syntax tree
    let syntax_tree: Vec<syntax_tree::SyntaxTreeToken> = syntax_tree::generate_syntax_tree(&token_stream);
    println!("{:?}", syntax_tree);

    // Assemble the program
    let binary: Vec<u8> = assembler::generate_binary(&syntax_tree);
    println!("{:?}", binary);
}
