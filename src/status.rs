use crate::Num;

#[derive(Debug, Clone, PartialEq)]
pub enum ErrorStatus {
    UnterminatedProgram,
    UnrecognizedOp(Num),
    IllegalMemoryAccess,
    OutOfBounds,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ExecutionStatus {
    Blocking,
    Halted,
}
