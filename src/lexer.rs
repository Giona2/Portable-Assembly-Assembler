
use std::sync::Arc;


pub fn generate_lexing_token_stream(file_content: Arc<String>) -> Vec<LexingToken> {
    let mut constructed_lexing_token_stream: Vec<LexingToken> = Vec::new();

    let mut current_char_index: usize = 0;

    while current_char_index < file_content.clone().len() { let current_char = file_content.get(current_char_index..=current_char_index).unwrap();
        if      LexingTokenInstruction::STT.kwrd_is_found_here(file_content.clone(), current_char_index) {
            constructed_lexing_token_stream.push(LexingToken::Instruction(LexingTokenInstruction::STT));
            current_char_index += LexingTokenInstruction::STT.get_kwrd().len()-1;
        }
        else if LexingTokenInstruction::NEW.kwrd_is_found_here(file_content.clone(), current_char_index) {
            constructed_lexing_token_stream.push(LexingToken::Instruction(LexingTokenInstruction::NEW));
            current_char_index += LexingTokenInstruction::NEW.get_kwrd().len()-1;
        }
        else if LexingTokenInstruction::SET.kwrd_is_found_here(file_content.clone(), current_char_index) {
            constructed_lexing_token_stream.push(LexingToken::Instruction(LexingTokenInstruction::SET));
            current_char_index += LexingTokenInstruction::SET.get_kwrd().len()-1;
        }
        else if LexingTokenInstruction::DRP.kwrd_is_found_here(file_content.clone(), current_char_index) {
            constructed_lexing_token_stream.push(LexingToken::Instruction(LexingTokenInstruction::DRP));
            current_char_index += LexingTokenInstruction::DRP.get_kwrd().len()-1;
        }
        else if LexingTokenInstruction::LOD.kwrd_is_found_here(file_content.clone(), current_char_index) {
            constructed_lexing_token_stream.push(LexingToken::Instruction(LexingTokenInstruction::LOD));
            current_char_index += LexingTokenInstruction::LOD.get_kwrd().len()-1;
        }
        else if LexingTokenInstruction::RET.kwrd_is_found_here(file_content.clone(), current_char_index) {
            constructed_lexing_token_stream.push(LexingToken::Instruction(LexingTokenInstruction::RET));
            current_char_index += LexingTokenInstruction::RET.get_kwrd().len()-1;
        }
        else if LexingTokenInstruction::END.kwrd_is_found_here(file_content.clone(), current_char_index) {
            constructed_lexing_token_stream.push(LexingToken::Instruction(LexingTokenInstruction::END));
            current_char_index += LexingTokenInstruction::END.get_kwrd().len()-1;
        }
        else if let Ok(_) = current_char.parse::<usize>() {
            let mut end_of_num_index = current_char_index;

            while let Ok(_) = file_content.get(current_char_index..=end_of_num_index).unwrap().parse::<usize>() {
                end_of_num_index += 1;
            }

            let constructed_number: &str = file_content.get(current_char_index..end_of_num_index).unwrap();

            constructed_lexing_token_stream.push(
                LexingToken::Number(constructed_number.parse().unwrap())
            );

            current_char_index = end_of_num_index-1;
        }
        else if current_char == "," {
            constructed_lexing_token_stream.push(LexingToken::ArgSeperator);
        }
        else if current_char == "\n" {
            constructed_lexing_token_stream.push(LexingToken::EndOfInstruction);
        }

        current_char_index += 1;
    }

    return constructed_lexing_token_stream;
}

#[derive(Debug)]
pub enum LexingToken {
    Instruction(LexingTokenInstruction),
    Number(usize),
    ArgSeperator,
    EndOfInstruction,
}

#[derive(Debug)]
pub enum LexingTokenInstruction {
    STT,
    NEW,
    SET,
    DRP,
    LOD,
    RET,
    END,
} impl LexingTokenInstruction {
    pub fn get_kwrd(&self) -> String { match self {
        LexingTokenInstruction::STT => String::from("stt"),
        LexingTokenInstruction::NEW => String::from("new"),
        LexingTokenInstruction::SET => String::from("set"),
        LexingTokenInstruction::DRP => String::from("drp"),
        LexingTokenInstruction::LOD => String::from("lod"),
        LexingTokenInstruction::RET => String::from("ret"),
        LexingTokenInstruction::END => String::from("end"),
    }}

    pub fn kwrd_is_found_here(&self, file_content: Arc<String>, current_char_index: usize) -> bool {
        let target_kwrd = self.get_kwrd();

        if let Some(selected_text) = file_content.get(current_char_index..(current_char_index+target_kwrd.len())) {
            if selected_text == target_kwrd { return true }
        }

        return false
    }
}
