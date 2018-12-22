use peripherals::memory_access::MemoryAccess;
use peripherals::error::MemoryAccessError;

/// Error in fetch stage.
#[derive(Debug, Fail, PartialEq)]
pub enum FetchError {
     #[fail(display = "{}", error)]
    InvalidMemoryAccess{ error: MemoryAccessError },

     #[fail(display = "undefined opcode: 0b{:b}", opcode)]
    UndefinedInstruction{ opcode: u8 },
}

impl From<MemoryAccessError> for FetchError {
    fn from(error: MemoryAccessError) -> FetchError {
        FetchError::InvalidMemoryAccess {
            error,
        }
    }
}

pub fn fetch(instr_mem: &dyn MemoryAccess, pc: usize)
    -> Result<u32, FetchError>
{
    instr_mem.read_u32(pc).map_err(Into::into)
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
}