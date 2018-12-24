//! Execute stage.
//! Returns write back data.

use crate::decode::DecodedInstr;
use crate::isa::opcode::Opcode;
use num::FromPrimitive;

/// Packet to modify CPU state finally.
pub enum WriteBackData {
    Gpr{ target: usize, value: u32 },
    Halt,
}

/// Exceptions occur in execute stage.
#[derive(Debug, Fail, PartialEq)]
pub enum ExecuteError {
    #[fail(display = "overflow occurs")]
    Overflow,
}

/// Executes an instruction.
pub fn execute(instr: DecodedInstr) -> Result<WriteBackData, ExecuteError> {
    let DecodedInstr(instr) = instr;

    let opcode = Opcode::from_u32(instr.opcode()).unwrap();
    match opcode {
        Opcode::OpWfi =>  Ok(WriteBackData::Halt),
        _ => Ok(WriteBackData::Gpr { target: 1, value: 1 }),
    }
   
}
