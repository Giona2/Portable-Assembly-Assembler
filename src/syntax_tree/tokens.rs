use super::types::*;


#[derive(Debug, Clone)]
pub enum SyntaxTreeToken {
    VariableInstruction(VariableInstruction),
    ArithmeticInstruction(ArithmeticInstruction),
}

#[derive(Debug, Clone)]
pub enum VariableInstruction {
    // Stack size
    STT(i16),
    SET(OperatorConfig, VarFrameType, GivenValueType),
    LOD(OperatorConfig, GivenValueType),
    RET(OperatorConfig, VarFrameType),
    END,
}

#[derive(Debug, Clone)]
pub enum ArithmeticInstruction {
    ADD(OperatorConfig, GivenValueType),
    SUB(OperatorConfig, GivenValueType),
    MUL(OperatorConfig, GivenValueType),
    DIV(OperatorConfig, GivenValueType),
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
    VariableIndex(VarFrameType),
    DirectValue(Vec<u8>)
}
