use std::sync::Arc;

use indexmap::{indexmap, IndexMap};
use serde::Deserialize;

pub fn get_keywords() -> IndexMap<&'static str, LexingToken> { return indexmap! {
    "stt" => LexingToken::VariableInstruction(VariableInstruction::STT),
    "set" => LexingToken::VariableInstruction(VariableInstruction::SET),
    "lod" => LexingToken::VariableInstruction(VariableInstruction::LOD),
    "ret" => LexingToken::VariableInstruction(VariableInstruction::RET),
    "end" => LexingToken::VariableInstruction(VariableInstruction::END),

    "add" => LexingToken::ArithmeticInstruction(ArithmeticInstruction::ADD),

    "," => LexingToken::ArgSeperator,

    "\n" => LexingToken::EndOfInstruction,

    "!" => LexingToken::OperatingSizeDenotator(OperationSizeDenotation::Direct),
    "f" => LexingToken::OperatingSizeDenotator(OperationSizeDenotation::Float),
    "*" => LexingToken::OperatingSizeDenotator(OperationSizeDenotation::Pointer),
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
    VariableInstruction(VariableInstruction),
    ArithmeticInstruction(ArithmeticInstruction),
    Number(String),
    ArgSeperator,
    EndOfInstruction,
    OperatingSizeDenotator(OperationSizeDenotation)
} impl LexingToken {
    pub fn to_number(&self) -> Option<String> {
        if let Self::Number(number) = self {
            return Some(number.clone())
        } else {
            return None
        }
    }

    pub fn to_eoi(self) -> Option<()> {
        if let Self::EndOfInstruction = self {
            return Some(())
        } else {
            return None
        }
    }
}

#[derive(PartialEq, Debug, Clone)]
pub enum OperationSizeDenotation {
    Direct,
    Float,
    Pointer,
}

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum VariableInstruction {
    STT,
    SET,
    LOD,
    RET,
    END,
}

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum ArithmeticInstruction {
    ADD,
}
