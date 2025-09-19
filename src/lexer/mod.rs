use std::sync::Arc;

use indexmap::{indexmap, IndexMap};

pub mod tokens;
    use tokens::*;

pub fn get_keywords() -> IndexMap<&'static str, LexingToken> { return indexmap! {
    "stt" => LexingToken::VariableInstruction(VariableInstruction::STT),
    "set" => LexingToken::VariableInstruction(VariableInstruction::SET),
    "lod" => LexingToken::VariableInstruction(VariableInstruction::LOD),
    "ret" => LexingToken::VariableInstruction(VariableInstruction::RET),
    "end" => LexingToken::VariableInstruction(VariableInstruction::END),

    "add" => LexingToken::ArithmeticInstruction(ArithmeticInstruction::ADD),
    "sub" => LexingToken::ArithmeticInstruction(ArithmeticInstruction::SUB),
    "mul" => LexingToken::ArithmeticInstruction(ArithmeticInstruction::MUL),
    "div" => LexingToken::ArithmeticInstruction(ArithmeticInstruction::DIV),

    "," => LexingToken::ArgSeperator,

    "\n" => LexingToken::EndOfInstruction,

    "!" => LexingToken::OperatingSizeDenotator(OperationSizeDenotation::Direct),
    "f" => LexingToken::OperatingSizeDenotator(OperationSizeDenotation::Float),
    "*" => LexingToken::OperatingSizeDenotator(OperationSizeDenotation::Pointer),
}}


pub fn generate_lexing_token_stream(file_content: Arc<String>) -> Vec<LexingToken> {
    let mut constructed_lexing_token_stream: Vec<LexingToken> = Vec::new();

    let mut current_char_index: usize = 0;

    while current_char_index < file_content.clone().len() {
        // Check if this character is a number
        if let Ok(_) = file_content.get(current_char_index..=current_char_index).unwrap().parse::<u8>() {
            // find the last digit
            let mut last_digit_index = current_char_index;
            while let Ok(_) = file_content.get(last_digit_index+1..=last_digit_index+1).unwrap().parse::<u8>() {
                last_digit_index += 1;
            }

            // construct the number token
            let full_number = String::from(file_content.get(current_char_index..=last_digit_index).unwrap());
            constructed_lexing_token_stream.push(LexingToken::Number(full_number));

            // Move the last digit to the end of the number
            current_char_index = last_digit_index;
        }

        // Search for the keyword
        for (keyword, keyword_type) in get_keywords().iter() {
            if let Some(slice) = file_content.get(current_char_index..current_char_index+keyword.len()) {
                if slice == *keyword {
                    constructed_lexing_token_stream.push(keyword_type.clone());

                }
            }
        }

        current_char_index += 1;
    }

    return constructed_lexing_token_stream;
}
