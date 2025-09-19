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

    /*
    pub fn to_eoi(self) -> Option<()> {
        if let Self::EndOfInstruction = self {
            return Some(())
        } else {
            return None
        }
    }
    */
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
    SUB,
    MUL,
    DIV,
}
