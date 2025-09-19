use crate::syntax_tree::{self, tokens::{GivenValueType, OperatorConfig}, types::VarFrameType};


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
        syntax_tree::tokens::ArithmeticInstruction::SUB(_, _) => return 0x0e,
        syntax_tree::tokens::ArithmeticInstruction::MUL(_, _) => return 0x0f,
        syntax_tree::tokens::ArithmeticInstruction::DIV(_, _) => return 0x10,
    }}
}}

pub trait ToBinaryRepresentation {
    fn to_binary_representation(&self) -> Vec<u8>;
}

const IS_POINTER_MASK: u8 = 0b1000_0000;
const IS_FLOAT_MASK:   u8 = 0b0010_0000;
const IS_DIRECT_MASK:  u8 = 0b0001_0000;
impl ToBinaryRepresentation for syntax_tree::tokens::OperatorConfig {
    fn to_binary_representation(&self) -> Vec<u8> {
        let mut constructed_value: u8 = self.size as u8;

        if self.is_pointer { constructed_value |= IS_POINTER_MASK }
        if self.is_float   { constructed_value |= IS_FLOAT_MASK   }
        if self.is_direct  { constructed_value |= IS_DIRECT_MASK  }

        println!("operator config binary: {:08b}", constructed_value);

        return Vec::from([constructed_value]);
    }
}

impl ToBinaryRepresentation for VarFrameType {
    fn to_binary_representation(&self) -> Vec<u8> {
        return self.to_le_bytes().to_vec();
    }
}

pub trait GetInnerOfArithOrBitwse {
    fn get_inner(&self) -> (OperatorConfig, GivenValueType);
}

impl GetInnerOfArithOrBitwse for syntax_tree::tokens::ArithmeticInstruction {
    fn get_inner(&self) -> (OperatorConfig, GivenValueType) { match self {
        syntax_tree::tokens::ArithmeticInstruction::ADD(operator_config, given_value) => return (operator_config.clone(), given_value.clone()),
        syntax_tree::tokens::ArithmeticInstruction::SUB(operator_config, given_value) => return (operator_config.clone(), given_value.clone()),
        syntax_tree::tokens::ArithmeticInstruction::MUL(operator_config, given_value) => return (operator_config.clone(), given_value.clone()),
        syntax_tree::tokens::ArithmeticInstruction::DIV(operator_config, given_value) => return (operator_config.clone(), given_value.clone()),
    }}
}
