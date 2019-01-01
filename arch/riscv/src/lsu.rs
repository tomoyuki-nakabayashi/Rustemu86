//! Load store unit.

use crate::execute::{LsuOp, WriteBackData};
use crate::isa::opcode::LoadStoreType;
use peripherals::memory_access::MemoryAccess;

/// Exceptions occur in load/store stage.
#[derive(Debug, Fail, PartialEq)]
pub enum LsuError {
    #[fail(display = "misaligned memory access to {:08x}", addr)]
    Misalignment { addr: u32 },

    #[fail(display = "memory access error to {:08x}", addr)]
    MemoryAccessError { addr: u32 },
}

pub fn load_store(
    data_mem: &mut dyn MemoryAccess,
    instr: &LsuOp,
) -> Result<WriteBackData, LsuError> {
    use self::LoadStoreType::*;
    match instr.op {
        LW => {
            let data = data_mem
                .read_u32(instr.addr as usize)
                .map_err(|_| LsuError::MemoryAccessError { addr: instr.addr })?;
            Ok(WriteBackData::Gpr {
                target: instr.dest,
                value: data,
            })
        }
        LH => {
            let data = data_mem
                .read_u16(instr.addr as usize)
                .map_err(|_| LsuError::MemoryAccessError { addr: instr.addr })?;
            Ok(WriteBackData::Gpr {
                target: instr.dest,
                value: sign_extend_from_u16(data),
            })
        }
        LHU => {
            let data = data_mem
                .read_u16(instr.addr as usize)
                .map_err(|_| LsuError::MemoryAccessError { addr: instr.addr })?;
            Ok(WriteBackData::Gpr {
                target: instr.dest,
                value: data.into(),
            })
        }
        LB => {
            let data = data_mem
                .read_u8(instr.addr as usize)
                .map_err(|_| LsuError::MemoryAccessError { addr: instr.addr })?;
            Ok(WriteBackData::Gpr {
                target: instr.dest,
                value: sign_extend_from_u8(data),
            })
        }
        LBU => {
            let data = data_mem
                .read_u8(instr.addr as usize)
                .map_err(|_| LsuError::MemoryAccessError { addr: instr.addr })?;
            Ok(WriteBackData::Gpr {
                target: instr.dest,
                value: data.into(),
            })
        }
        SW => {
            data_mem
                .write_u32(instr.addr as usize, instr.value)
                .map_err(|_| LsuError::MemoryAccessError { addr: instr.addr })?;
            Ok(WriteBackData::Gpr {
                target: instr.dest,
                value: 0,
            })
        }
        SH => {
            data_mem
                .write_u16(instr.addr as usize, instr.value as u16)
                .map_err(|_| LsuError::MemoryAccessError { addr: instr.addr })?;
            Ok(WriteBackData::Gpr {
                target: instr.dest,
                value: 0,
            })
        }
        SB => {
            data_mem
                .write_u8(instr.addr as usize, instr.value as u8)
                .map_err(|_| LsuError::MemoryAccessError { addr: instr.addr })?;
            Ok(WriteBackData::Gpr {
                target: instr.dest,
                value: 0,
            })
        }
    }
}

// helper for sign extend
fn sign_extend_from_u16(data: u16) -> u32 {
    i32::from(data as i16) as u32
}

// helper for sign extend
fn sign_extend_from_u8(data: u8) -> u32 {
    i32::from(data as i8) as u32
}

#[cfg(test)]
mod test {
    #[test]
    fn sign_extend() {
        let half_word = 0xffffu16; // `-1` in singed integer
        let signed_word = i32::from(half_word as i16);

        assert_eq!(signed_word, -1); // 0xffff_ffff
    }
}
