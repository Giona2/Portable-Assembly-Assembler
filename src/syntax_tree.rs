use std::{panic, sync::Arc};


use crate::lexer::{
    self, LexingToken, OperationSizeDenotation
};


type VAR_FRAME_TYPE = i16;


pub fn generate_syntax_tree(lexing_token_stream: &[LexingToken]) -> Vec<SyntaxTreeToken> {
    let mut constructed_syntax_tree: Vec<SyntaxTreeToken> = Vec::new();

    let mut current_token_index: usize = 0;
    while current_token_index < lexing_token_stream.len() { match &lexing_token_stream[current_token_index] {
        LexingToken::VariableInstruction(instruction) => { match instruction {
            lexer::VariableInstruction::STT => {
                // Get the var frame size
                current_token_index += 1;
                let var_frame_size: i16 = lexing_token_stream[current_token_index].to_number().unwrap()
                    .parse().unwrap();

                // Check for EOI
                current_token_index += 1;
                look_for_eoi(lexing_token_stream, &current_token_index);

                // Construct the token
                constructed_syntax_tree.push(SyntaxTreeToken::VariableInstruction(VariableInstruction::STT(var_frame_size)));
            }

            lexer::VariableInstruction::SET => {
                // Get the denotations
                current_token_index += 1;
                let mut operator_config = get_denotations_here(lexing_token_stream, &mut current_token_index);

                // Get the byte size
                current_token_index += 1;
                operator_config.size = lexing_token_stream[current_token_index].to_number().unwrap()
                    .parse().unwrap();

                // Look for the seperator
                current_token_index += 1;
                look_for_arg_seperator(lexing_token_stream, &current_token_index);

                // Look for the variable index
                current_token_index += 1;
                let variable_index: VAR_FRAME_TYPE = lexing_token_stream[current_token_index].to_number().unwrap()
                    .parse().unwrap();

                // Look for the seperator
                current_token_index += 1;
                look_for_arg_seperator(lexing_token_stream, &current_token_index);

                // Look for the given value. Parse depending on the given denotations
                current_token_index += 1;
                let used_bytes_value = get_given_value_here(&operator_config, lexing_token_stream, &current_token_index);

                // Look for EOI
                current_token_index += 1;
                look_for_eoi(lexing_token_stream, &current_token_index);

                // Construct the token
                let constructed_token = VariableInstruction::SET(operator_config, variable_index, used_bytes_value);
                constructed_syntax_tree.push(SyntaxTreeToken::VariableInstruction(constructed_token));
            }

            lexer::VariableInstruction::LOD => {
                // Get the denotations
                current_token_index += 1;
                let mut operator_config = get_denotations_here(lexing_token_stream, &mut current_token_index);

                // Get the byte size
                current_token_index += 1;
                operator_config.size = (&lexing_token_stream[current_token_index]).to_number().unwrap()
                    .parse().unwrap();

                // Look for the arg seperator
                current_token_index += 1;
                look_for_arg_seperator(lexing_token_stream, &current_token_index);

                // Look for the given value. Parse depending on the given denotations
                current_token_index += 1;
                let used_bytes_value = get_given_value_here(&operator_config, lexing_token_stream, &current_token_index);

                // Look for the EOI
                current_token_index += 1;
                look_for_eoi(lexing_token_stream, &current_token_index);

                // Construct the token
                let constructed_token = VariableInstruction::LOD(operator_config, used_bytes_value);
                constructed_syntax_tree.push(SyntaxTreeToken::VariableInstruction(constructed_token));
            }

            lexer::VariableInstruction::RET => {
                // Get the donominations
                current_token_index += 1;
                let mut operator_config = get_denotations_here(lexing_token_stream, &mut current_token_index);

                // Get the byte size
                current_token_index += 1;
                operator_config.size = (&lexing_token_stream[current_token_index]).to_number().unwrap()
                    .parse().unwrap();

                // Look for the arg seporator
                current_token_index += 1;
                look_for_arg_seperator(lexing_token_stream, &current_token_index);

                // Get the variable index
                current_token_index += 1;
                let variable_index: VAR_FRAME_TYPE = (&lexing_token_stream[current_token_index]).to_number().unwrap()
                    .parse().unwrap();

                // Look for the EOI
                current_token_index += 1;
                look_for_eoi(lexing_token_stream, &current_token_index);

                // Construct the token
                let constructed_token = VariableInstruction::RET(operator_config, variable_index);
                constructed_syntax_tree.push(SyntaxTreeToken::VariableInstruction(constructed_token));
            }

            _ => {}
        }}

        LexingToken::ArithmeticInstruction(instruction) => { match instruction {
            lexer::ArithmeticInstruction::ADD => {
                // Get the operation config
                current_token_index += 1;
                let mut operator_config = get_denotations_here(lexing_token_stream, &mut current_token_index);

                // Get the operation size
                current_token_index += 1;
                operator_config.size = (&lexing_token_stream[current_token_index]).to_number().unwrap()
                    .parse().unwrap();

                // Look for the arg seperator
                current_token_index += 1;
                look_for_arg_seperator(lexing_token_stream, &current_token_index);

                // Look for the given value. Parse depending on the given denotations
                current_token_index += 1;
                let used_bytes_value = get_given_value_here(&operator_config, lexing_token_stream, &current_token_index);

                // Look for EOI
                current_token_index += 1;
                look_for_eoi(lexing_token_stream, &current_token_index);

                // Construct token
                let constructed_token = ArithmeticInstruction::ADD(operator_config, used_bytes_value);
                constructed_syntax_tree.push(SyntaxTreeToken::ArithmeticInstruction(constructed_token));
            }
        }}

        _ => {}

    } current_token_index += 1; }

    return constructed_syntax_tree;
}


pub fn get_given_value_here(operator_config: &OperatorConfig, lexing_token_stream: &[LexingToken], current_token_index: &usize) -> GivenValueType {
    println!("Looking for given value here: {:?}", &lexing_token_stream[*current_token_index]);
    let used_bytes_value: GivenValueType;

    if operator_config.is_direct {
        let numeric_value: usize = lexing_token_stream[*current_token_index].to_number().unwrap()
            .parse().unwrap();
        let full_bytes_value: [u8; 8] = numeric_value.to_le_bytes();
        used_bytes_value = GivenValueType::DirectValue((&full_bytes_value[0..operator_config.size as usize]).to_vec());
    } else {
        let from_variable_index: VAR_FRAME_TYPE = lexing_token_stream[*current_token_index].to_number().unwrap()
            .parse().unwrap();

        used_bytes_value = GivenValueType::VariableIndex(from_variable_index);
    }

    return used_bytes_value;
}

pub fn get_denotations_here(lexing_token_stream: &[LexingToken], current_token_index: &mut usize) -> OperatorConfig {
    let mut operator_config = OperatorConfig::new();

    while let LexingToken::OperatingSizeDenotator(denotator) = &lexing_token_stream[*current_token_index] {
        match denotator {
            OperationSizeDenotation::Direct => operator_config.is_direct = true,
            OperationSizeDenotation::Float => operator_config.is_float = true,
            OperationSizeDenotation::Pointer => operator_config.is_pointer = true,
        }

        *current_token_index += 1;
    }

    *current_token_index -= 1;

    return operator_config;
}

pub fn look_for_arg_seperator(lexing_token_stream: &[LexingToken], current_token_index: &usize) {
    if let LexingToken::ArgSeperator = lexing_token_stream[*current_token_index] {
    } else {
        panic!("Arguments of an instruction should be seperated");
    }
}

pub fn look_for_eoi(lexing_token_stream: &[LexingToken], current_token_index: &usize) {
    if let LexingToken::EndOfInstruction = lexing_token_stream[*current_token_index] {
    } else {
        panic!("Instruction did not terminate correctly")
    }
}


#[derive(Debug, Clone)]
pub enum SyntaxTreeToken {
    VariableInstruction(VariableInstruction),
    ArithmeticInstruction(ArithmeticInstruction),
}

#[derive(Debug, Clone)]
pub enum VariableInstruction {
    // Stack size
    STT(i16),
    SET(OperatorConfig, VAR_FRAME_TYPE, GivenValueType),
    LOD(OperatorConfig, GivenValueType),
    RET(OperatorConfig, VAR_FRAME_TYPE),
    END,
}

#[derive(Debug, Clone)]
pub enum ArithmeticInstruction {
    ADD(OperatorConfig, GivenValueType),
}

#[derive(Debug, Clone, Copy)]
pub struct OperatorConfig {
    pub size: i8,
    pub is_float: bool,
    pub is_direct: bool,
    pub is_pointer: bool,
} impl OperatorConfig {
    pub fn new() -> Self { return Self {
        size: 0,
        is_float: false,
        is_direct: false,
        is_pointer: false,
    }}
}

#[derive(Debug, Clone)]
pub enum GivenValueType {
    VariableIndex(VAR_FRAME_TYPE),
    DirectValue(Vec<u8>)
}
