//! Execute stage.
//! Returns write back data.

use crate::decode::DecodedInstr;

pub enum WriteBackData {
    Halt,
}

/// Exceptions occur in execute stage.
#[derive(Debug, Fail, PartialEq)]
pub enum ExecuteError {
    #[fail(display = "overflow occurs")]
    Overflow,
}

/// Executes an instruction.
pub fn execute(_instr: &DecodedInstr) -> Result<WriteBackData, ExecuteError> {
    Ok(WriteBackData::Halt)
}