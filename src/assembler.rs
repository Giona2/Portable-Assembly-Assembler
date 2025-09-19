use crate::syntax_tree;


pub fn translate_instruction(instruction: &syntax_tree::tokens::SyntaxTreeToken) -> u8 { match instruction {
    syntax_tree::tokens::SyntaxTreeToken::VariableInstruction(instruction) => { match instruction {
        syntax_tree::tokens::VariableInstruction::STT(_) => return 0x01,

        syntax_tree::tokens::VariableInstruction::SET(_,_ , _) => return 0x02,

        syntax_tree::tokens::VariableInstruction::LOD(_, _) => return 0x03,

        syntax_tree::tokens::VariableInstruction::RET(_, _) => return 0x04,

        syntax_tree::tokens::VariableInstruction::END => return 0x05,
    }}

    syntax_tree::tokens::SyntaxTreeToken::ArithmeticInstruction(instruction) => { match instruction {
        syntax_tree::tokens::ArithmeticInstruction::ADD(_, _) => return 0x0d,
    }}
}}


pub fn generate_binary(syntax_tree: &[syntax_tree::tokens::SyntaxTreeToken]) -> Vec<u8> {
    let mut constructed_binary: Vec<u8> = Vec::new();

    for token in syntax_tree { match token {
        syntax_tree::tokens::SyntaxTreeToken::VariableInstruction(instruction) => {

        }

        _ => { unimplemented!() }
    }}

    return constructed_binary;
}
