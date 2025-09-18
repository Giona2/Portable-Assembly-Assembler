use std::panic;

use super::tokens::*;
use super::types::*;
use crate::lexer;


pub fn get_given_value_here(operator_config: &OperatorConfig, lexing_token_stream: &[lexer::LexingToken], current_token_index: &usize) -> GivenValueType {
    println!("Looking for given value here: {:?}", &lexing_token_stream[*current_token_index]);
    let used_bytes_value: GivenValueType;

    if operator_config.is_direct {
        let numeric_value: usize = lexing_token_stream[*current_token_index].to_number().unwrap()
            .parse().unwrap();
        let full_bytes_value: [u8; 8] = numeric_value.to_le_bytes();
        used_bytes_value = GivenValueType::DirectValue((&full_bytes_value[0..operator_config.size as usize]).to_vec());
    } else {
        let from_variable_index: VarFrameType = lexing_token_stream[*current_token_index].to_number().unwrap()
            .parse().unwrap();

        used_bytes_value = GivenValueType::VariableIndex(from_variable_index);
    }

    return used_bytes_value;
}

pub fn get_denotations_here(lexing_token_stream: &[lexer::LexingToken], current_token_index: &mut usize) -> OperatorConfig {
    let mut operator_config = OperatorConfig::new();

    while let lexer::LexingToken::OperatingSizeDenotator(denotator) = &lexing_token_stream[*current_token_index] {
        match denotator {
            lexer::OperationSizeDenotation::Direct => operator_config.is_direct = true,
            lexer::OperationSizeDenotation::Float => operator_config.is_float = true,
            lexer::OperationSizeDenotation::Pointer => operator_config.is_pointer = true,
        }

        *current_token_index += 1;
    }

    *current_token_index -= 1;

    return operator_config;
}

pub fn look_for_arg_seperator(lexing_token_stream: &[lexer::LexingToken], current_token_index: &usize) {
    if let lexer::LexingToken::ArgSeperator = lexing_token_stream[*current_token_index] {
    } else {
        panic!("Arguments of an instruction should be seperated");
    }
}

pub fn look_for_eoi(lexing_token_stream: &[lexer::LexingToken], current_token_index: &usize) {
    if let lexer::LexingToken::EndOfInstruction = lexing_token_stream[*current_token_index] {
    } else {
        panic!("Instruction did not terminate correctly")
    }
}
