//! Fetch unit.
//! Returns an instruction to the next stage.
//!
//! An instruction is either u32 or u16. `u16` is for compressed instruction extension.
//!
//! According to RISC-V mailing list, Instruction fetch misaligned exceptions are not
//! possible on machines the support compressed instruction set extension.

use peripherals::error::MemoryAccessError;
use peripherals::memory_access::MemoryAccess;

/// Exceptions occur in fetch stage.
#[derive(Debug, Fail, PartialEq)]
pub enum FetchError {
    #[fail(display = "{}", error)]
    InvalidMemoryAccess { error: MemoryAccessError },

    #[fail(display = "instruction fetch misaligned at {}", pc)]
    MisalingedFetch { pc: u32 },
}

impl From<MemoryAccessError> for FetchError {
    fn from(error: MemoryAccessError) -> FetchError {
        FetchError::InvalidMemoryAccess { error }
    }
}

/// Fetches an instruction from the `instr_mem` of the `pc`.
pub fn fetch(instr_mem: &dyn MemoryAccess, pc: usize) -> Result<u32, FetchError> {
    alignment_check(pc)?;

    let instr = instr_mem.read_u32(pc)?;
    Ok(instr)
}

#[inline(always)]
fn alignment_check(pc: usize) -> Result<(), FetchError> {
    // TODO: If compressed extension is supported, this check should be disabled.
    if pc % 4 != 0 {
        return Err(FetchError::MisalingedFetch { pc: pc as u32 });
    }
    Ok(())
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

        // Well alignmented but out of range.
        let instr = fetch(&dram, 8);
        match instr {
            Err(FetchError::InvalidMemoryAccess { .. }) => (),
            _ => panic!(),
        };
    }

    #[test]
    fn fetch_istr_misaligned() {
        let program = vec![0x00, 0x00, 0x00, 0x00]; // Don't care
        let dram = Memory::new_with_filled_ram(&program, program.len());

        let instr = fetch(&dram, 1);
        assert_eq!(Err(FetchError::MisalingedFetch { pc: 1 }), instr);
    }
}
