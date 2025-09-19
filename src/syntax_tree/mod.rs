use crate::lexer;

pub mod helpers;
    use helpers::*;
pub mod tokens;
    use tokens::*;
pub mod types;
    use types::*;


pub fn generate_syntax_tree(lexing_token_stream: &[lexer::tokens::LexingToken]) -> Vec<SyntaxTreeToken> {
    let mut constructed_syntax_tree: Vec<SyntaxTreeToken> = Vec::new();

    let mut current_token_index: usize = 0;
    while current_token_index < lexing_token_stream.len() { match &lexing_token_stream[current_token_index] {
        lexer::tokens::LexingToken::VariableInstruction(instruction) => { match instruction {
            lexer::tokens::VariableInstruction::STT => {
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

            lexer::tokens::VariableInstruction::SET => {
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
                let variable_index: VarFrameType = lexing_token_stream[current_token_index].to_number().unwrap()
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

            lexer::tokens::VariableInstruction::LOD => {
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

            lexer::tokens::VariableInstruction::RET => {
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
                let variable_index: VarFrameType = (&lexing_token_stream[current_token_index]).to_number().unwrap()
                    .parse().unwrap();

                // Look for the EOI
                current_token_index += 1;
                look_for_eoi(lexing_token_stream, &current_token_index);

                // Construct the token
                let constructed_token = VariableInstruction::RET(operator_config, variable_index);
                constructed_syntax_tree.push(SyntaxTreeToken::VariableInstruction(constructed_token));
            }

            lexer::tokens::VariableInstruction::END => {
                // Get EOI
                current_token_index += 1;
                look_for_eoi(lexing_token_stream, &current_token_index);

                // Construct the token
                let constructed_token = VariableInstruction::END;
                constructed_syntax_tree.push(SyntaxTreeToken::VariableInstruction(constructed_token));
            }
        }}

        lexer::tokens::LexingToken::ArithmeticInstruction(instruction) => {
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
            let constructed_token = match instruction {
                lexer::tokens::ArithmeticInstruction::ADD => ArithmeticInstruction::ADD(operator_config, used_bytes_value),
                lexer::tokens::ArithmeticInstruction::SUB => ArithmeticInstruction::SUB(operator_config, used_bytes_value),
                lexer::tokens::ArithmeticInstruction::MUL => ArithmeticInstruction::MUL(operator_config, used_bytes_value),
                lexer::tokens::ArithmeticInstruction::DIV => ArithmeticInstruction::DIV(operator_config, used_bytes_value),
            };
            constructed_syntax_tree.push(SyntaxTreeToken::ArithmeticInstruction(constructed_token));
        }

        _ => {}

    } current_token_index += 1; }

    return constructed_syntax_tree;
}
