#[derive(Debug)]
pub enum SyntaxTreeToken {
    Instruction
}

#[derive(Debug)]
pub enum Instruction {
    STT,
    NEW(i8),
    SET(i8, usize),
    DRP(i8),
    LOD(i8),
    RET,
    END,
}
