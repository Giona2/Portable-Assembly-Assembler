use std::{
    fs,
    sync::Arc,
};

use clap::Parser;

use crate::lexer::tokens::LexingToken;

mod assembler;
mod lexer;
//mod error;
mod syntax_tree;


const DEFAULT_OUTPUT_PATH: &str = "./a.pbin";


#[derive(Parser)]
#[command(
    name = "Portable Assembly Compiler Collection",
    version = "0.1.1",
    about = "Collection for Portable Assembly utilities",
)]
struct CmdArgs {
    #[arg(index = 1, help = "Input pasm file")]
    target_file_path: String,

    #[arg(long = "out", short = 'o', required = false, help = "Output binary file")]
    target_dir: Option<String>,
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

    // Construct the syntax tree
    println!("\n\n=== Syntax Tree ===");
    let syntax_token_tree = syntax_tree::generate_syntax_tree(&token_stream);
    for token in syntax_token_tree.clone() {
        println!("{:?}", token);
    }

    // Construct the binary
    println!("\n\n=== Result ===");
    let binary_file = assembler::generate_binary(&syntax_token_tree);
    for byte in binary_file.clone() {
        print!("{:02x} ", byte);
    }
    println!();

    // Write to the output file
    if let Some(target_dir) = cmd_args.target_dir {
        fs::write(target_dir, binary_file).unwrap();
    } else {
        fs::write(DEFAULT_OUTPUT_PATH, binary_file).unwrap();
    }
}
