use std::sync::Arc;

use indexmap::{indexmap, IndexMap};
use serde::Deserialize;

pub fn get_keywords() -> IndexMap<&'static str, LexingToken> { return indexmap! {
    "stt" => LexingToken::VariableInstruction(LexingTokenVariableInstruction::STT),
    "set" => LexingToken::VariableInstruction(LexingTokenVariableInstruction::SET),
    "lod" => LexingToken::VariableInstruction(LexingTokenVariableInstruction::LOD),
    "ret" => LexingToken::VariableInstruction(LexingTokenVariableInstruction::RET),
    "end" => LexingToken::VariableInstruction(LexingTokenVariableInstruction::END),

    "," => LexingToken::ArgSeperator,

    "\n" => LexingToken::EndOfInstruction,

    "!" => LexingToken::OperatingSizeDenotator(OperatingSizeDenotator::Direct),
    "f" => LexingToken::OperatingSizeDenotator(OperatingSizeDenotator::Float),
    "*" => LexingToken::OperatingSizeDenotator(OperatingSizeDenotator::Pointer),
}}


#[derive(Debug, Deserialize, Clone)]
struct KeywordConfig {
    enum_type: String,
    branch: i8,
}


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

#[derive(PartialEq, Debug, Clone)]
pub enum LexingToken {
    VariableInstruction(LexingTokenVariableInstruction),
    Number(String),
    ArgSeperator,
    EndOfInstruction,
    OperatingSizeDenotator(OperatingSizeDenotator)
}

#[derive(PartialEq, Debug, Clone)]
pub enum OperatingSizeDenotator {
    Direct,
    Float,
    Pointer,
}

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum LexingTokenVariableInstruction {
    STT = 0,
    SET = 1,
    LOD = 2,
    RET = 3,
    END = 4,
}
