use std::sync::Arc;


use crate::lexer::{
    LexingToken, LexingTokenInstruction,
};


pub fn generate_syntax_tree(lexing_token_stream: &[LexingToken]) -> Vec<SyntaxTreeToken> {
    let mut constructed_syntax_tree: Vec<SyntaxTreeToken> = Vec::new();

    let mut current_token_index: usize = 0;
    while current_token_index < lexing_token_stream.len() { let current_token = lexing_token_stream.get(current_token_index).unwrap();
        println!("current token: {:?}", current_token);

        match current_token {
            LexingToken::Instruction(instruction) => { match instruction {
                LexingTokenInstruction::STT => {
                    constructed_syntax_tree.push(SyntaxTreeToken::Instruction(Instruction::STT));
                }

                LexingTokenInstruction::NEW => {
                    // Get the variable index
                    current_token_index += 1;
                    let index = lexing_token_stream.get(current_token_index).unwrap().get_number().unwrap();

                    // Add the instruction
                    constructed_syntax_tree.push(SyntaxTreeToken::Instruction(Instruction::NEW(index as i8)));
                }

                LexingTokenInstruction::SET => {
                    // Get the variable index
                    current_token_index += 1;
                    let variable_index = lexing_token_stream.get(current_token_index).unwrap().get_number().unwrap();

                    // Find the seperator
                    current_token_index += 1;
                    _ = lexing_token_stream.get(current_token_index).unwrap().get_arg_seperator().unwrap();

                    // Get the variable value
                    current_token_index += 1;
                    let variable_value = lexing_token_stream.get(current_token_index).unwrap().get_number().unwrap();

                    // Construction the token
                    constructed_syntax_tree.push(SyntaxTreeToken::Instruction(Instruction::SET(variable_index as i8, variable_value)));
                }

                LexingTokenInstruction::DRP => {
                    // Get the variable index
                    current_token_index += 1;
                    let variable_index = lexing_token_stream.get(current_token_index).unwrap().get_number().unwrap();

                    // Construction the token
                    constructed_syntax_tree.push(SyntaxTreeToken::Instruction(Instruction::DRP(variable_index as i8)));
                }

                LexingTokenInstruction::LOD => {
                    // Get the variable index
                    current_token_index += 1;
                    let variable_index = lexing_token_stream.get(current_token_index).unwrap().get_number().unwrap();

                    // Construction the token
                    constructed_syntax_tree.push(SyntaxTreeToken::Instruction(Instruction::LOD(variable_index as i8)));
                }

                LexingTokenInstruction::RET => {
                    constructed_syntax_tree.push(SyntaxTreeToken::Instruction(Instruction::RET));
                }

                LexingTokenInstruction::END => {
                    constructed_syntax_tree.push(SyntaxTreeToken::Instruction(Instruction::END));
                }

                LexingTokenInstruction::ADD => {
                    // Get the adding value
                    current_token_index += 1;
                    let add_value = lexing_token_stream.get(current_token_index).unwrap().get_number().unwrap();

                    // Construct the token
                    constructed_syntax_tree.push(SyntaxTreeToken::Instruction(Instruction::ADD(add_value)));
                }
            }}
            _ => {}
        }

        current_token_index += 1;
    }

    return constructed_syntax_tree;
}


#[derive(Debug, Clone, Copy)]
pub enum SyntaxTreeToken {
    Instruction(Instruction)
}

#[derive(Debug, Clone, Copy)]
pub enum Instruction {
    STT,
    NEW(i8),
    SET(i8, usize),
    DRP(i8),
    LOD(i8),
    RET,
    END,

    ADD(usize),
}
