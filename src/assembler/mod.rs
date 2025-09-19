use crate::syntax_tree;

mod helpers;
    use helpers::*;


pub fn generate_binary(syntax_tree: &[syntax_tree::tokens::SyntaxTreeToken]) -> Vec<u8> {
    let mut constructed_binary: Vec<u8> = Vec::new();

    for token in syntax_tree {
        // Start the instruction
        constructed_binary.push(translate_instruction(token));

        // Construct the rest of the instruction
        match token {
            syntax_tree::tokens::SyntaxTreeToken::VariableInstruction(instruction) => { match instruction {
                syntax_tree::tokens::VariableInstruction::STT(variable_frame_size) => {
                    // Add the variable frame size
                    constructed_binary.append(&mut variable_frame_size.to_binary_representation());
                }

                syntax_tree::tokens::VariableInstruction::SET(operator_config, variable_index, given_value) => {
                    // Add the operator configuration
                    constructed_binary.append(&mut operator_config.to_binary_representation());

                    // Add the variable index
                    constructed_binary.append(&mut variable_index.to_binary_representation());

                    // Add the given value
                    match given_value {
                        syntax_tree::tokens::GivenValueType::VariableIndex(variable_index) => {
                            constructed_binary.append(&mut variable_index.to_binary_representation());
                        }

                        syntax_tree::tokens::GivenValueType::DirectValue(given_value) => {
                            constructed_binary.append(&mut given_value.clone());
                        }
                    }
                }

                syntax_tree::tokens::VariableInstruction::LOD(operator_config, given_value) => {
                    // Add the operator configuration
                    constructed_binary.append(&mut operator_config.to_binary_representation());

                    // Add the given value
                    match given_value {
                        syntax_tree::tokens::GivenValueType::VariableIndex(variable_index) => {
                            constructed_binary.append(&mut variable_index.to_binary_representation());
                        }

                        syntax_tree::tokens::GivenValueType::DirectValue(given_value) => {
                            constructed_binary.append(&mut given_value.clone());
                        }
                    }
                }

                syntax_tree::tokens::VariableInstruction::RET(operator_config, variable_index) => {
                    // Add the operator configuration
                    constructed_binary.append(&mut operator_config.to_binary_representation());

                    // Add the variable index
                    constructed_binary.append(&mut variable_index.to_binary_representation());
                }

                syntax_tree::tokens::VariableInstruction::END => {}
            }}

            syntax_tree::tokens::SyntaxTreeToken::ArithmeticInstruction(instruction) => {
                let (operator_config, given_value) = instruction.get_inner();

                // Add the operator configuration
                constructed_binary.append(&mut operator_config.to_binary_representation());

                // Add the given value
                match given_value {
                    syntax_tree::tokens::GivenValueType::VariableIndex(variable_index) => {
                        constructed_binary.append(&mut variable_index.to_binary_representation());
                    }

                    syntax_tree::tokens::GivenValueType::DirectValue(given_value) => {
                        constructed_binary.append(&mut given_value.clone());
                    }
                }
            }
        }
    }

    return constructed_binary;
}
