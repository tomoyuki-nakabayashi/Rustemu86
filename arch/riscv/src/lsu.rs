//! Load store unit.

use crate::execute::{LsuOp, WriteBackData};
use crate::isa::opcode::LoadStoreType;
use peripherals::memory_access::MemoryAccess;

/// Exceptions occur in load/store stage.
#[derive(Debug, Fail, PartialEq)]
pub enum LsuError {
    #[fail(display = "misaligned memory access to {:08x}", addr)]
    Misalignment { addr: u32 },
}

pub fn load_store(
    data_mem: &mut dyn MemoryAccess,
    instr: &LsuOp,
) -> Result<WriteBackData, LsuError> {
    match instr.op {
        LoadStoreType::LW => {
            let data = data_mem.read_u32(instr.addr as usize).unwrap();
            Ok(WriteBackData::Gpr {
                target: instr.dest,
                value: data,
            })
        }
        LoadStoreType::SW => {
            data_mem
                .write_u32(instr.addr as usize, instr.value)
                .unwrap();
            Ok(WriteBackData::Gpr {
                target: instr.dest,
                value: 0,
            })
        }
    }
}
