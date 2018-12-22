//! Fetch unit.
//! Returns an instruction to the next stage.
//!
//! An instruction is either u32 or u16. `u16` is for compressed instruction extension.

use peripherals::error::MemoryAccessError;
use peripherals::memory_access::MemoryAccess;
use bit_field::BitField;

/// Errors occur in fetch stage.
#[derive(Debug, Fail, PartialEq)]
pub enum FetchError {
    #[fail(display = "{}", error)]
    InvalidMemoryAccess { error: MemoryAccessError },

    #[fail(display = "undefined opcode: 0b{:7b}", opcode)]
    UndefinedInstruction { opcode: u32 },
}

impl From<MemoryAccessError> for FetchError {
    fn from(error: MemoryAccessError) -> FetchError {
        FetchError::InvalidMemoryAccess { error }
    }
}

/// Fetches an instruction from the `instr_mem` of the `pc`.
pub fn fetch(instr_mem: &dyn MemoryAccess, pc: usize) -> Result<u32, FetchError> {
    let instr = instr_mem.read_u32(pc)?;
    let opcode = instr.get_bits(0..8);
    if opcode != 0x73 {
        Err(FetchError::UndefinedInstruction { opcode: opcode })
    } else {
        Ok(instr)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use peripherals::memory::Memory;

    #[test]
    fn fetch_an_instruction() {
        let program = vec![0x73, 0x00, 0x50, 0x10];
        let dram = Memory::new_with_filled_ram(&program, program.len());

        let instr = fetch(&dram, 0).expect("fail to fetch instruction from DRAM");
        assert_eq!(0x1050_0073, instr, "endianess is not converted!");
    }

    #[test]
    fn fetch_invalid_address() {
        let program = vec![0x73, 0x00, 0x50, 0x10];
        let dram = Memory::new_with_filled_ram(&program, program.len());

        let instr = fetch(&dram, 5);
        match instr {
            Err(FetchError::InvalidMemoryAccess { .. }) => (),
            _ => panic!(),
        };
    }

    #[test]
    fn fetch_undefined_opcode() {
        let program = vec![0x07, 0x00, 0x00, 0x00]; // FLW won't implement for the present.
        let dram = Memory::new_with_filled_ram(&program, program.len());

        let instr = fetch(&dram, 0);
        assert_eq!(Err(FetchError::UndefinedInstruction { opcode: 0b0000111 }), instr);
    }
}
