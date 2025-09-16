use crate::syntax_tree::{
    SyntaxTreeToken, Instruction
}


pub fn generate_binary(syntax_tree: &[SyntaxTreeToken]) -> Vec<u8> {
    let mut constructed_binary: Vec<u8> = Vec::new();

    for token in syntax_tree { match token {
        SyntaxTreeToken::Instruction(instruction) => { match instruction.clone() {
            Instruction::STT => {
                constructed_binary.push(translate_instruction(instruction));
            }

            Instruction::NEW(size) => {
                constructed_binary.push(translate_instruction(instruction));
                constructed_binary.push(size as u8);
            }

            Instruction::SET(index, value) => {
                constructed_binary.push(translate_instruction(instruction));
                constructed_binary.push(index as u8);
            }
        }}
    }}

    todo!()
}


pub fn translate_instruction(instruction: &Instruction) -> u8 { match instruction {
    Instruction::STT       => 0x01,
    Instruction::NEW(_)    => 0x02,
    Instruction::SET(_, _) => 0x03,
    Instruction::DRP(_)    => 0x04,
    Instruction::LOD(_)    => 0x07,
    Instruction::RET       => 0x06,
    Instruction::END       => 0x07,
    Instruction::ADD(_)    => 0x10,
}}
