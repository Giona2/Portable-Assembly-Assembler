use std::{
    fs,
    sync::Arc,
};

use clap::Parser;

mod lexer;
mod error;


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

    let token_stream: Vec<lexer::LexingToken> = lexer::generate_lexing_token_stream(Arc::new(file_content));

    println!("{:?}", token_stream);
}
