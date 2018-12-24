use crate::decode::decode;
use crate::execute::execute;
use crate::fetch::fetch;
use cpu::model::CpuModel;
use debug::DebugMode;
use peripherals::interconnect::Interconnect;
use peripherals::mmio::Mmio;

use crate::isa::exceptions::InternalExceptions;
use std::result;
pub type Result<T> = result::Result<T, InternalExceptions>;

/// RISC-V CPU model.
#[allow(dead_code)]
pub struct Riscv {
    pc: u32,
    mmio: Mmio,
    debug: DebugMode,
    halted: bool,
}

impl Riscv {
    /// Temporary `new`.
    /// TODO: This must be a new. It requires to modify CpuModel interface.
    pub fn fabricate(mmio: Mmio, debug: DebugMode) -> Riscv {
        Riscv {
            pc: 0,
            mmio,
            debug,
            halted: true,
        }
    }
}

impl CpuModel for Riscv {
    type Error = InternalExceptions;

    fn new(_mmio: Interconnect, _debug: DebugMode) -> Riscv {
        unimplemented!()
    }

    /// Initialize CPU state for run.
    fn init(&mut self) {
        self.halted = false;
    }

    /// Executes instructions until WFI.
    fn run(&mut self) -> Result<()> {
        while !self.halted {
            let instr = fetch(&self.mmio, self.pc as usize)?;
            let instr = decode(instr)?;
            let wb = execute(&instr)?;

            // Change CPU state only here.
            use crate::execute::WriteBackData;
            match wb {
                WriteBackData::Halt => {
                    self.halted = true;
                }
            }
            self.pc += 4;
        }
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use peripherals::memory::Memory;

    #[test]
    fn stop_at_wfi() {
        let program = vec![0x73, 0x00, 0x50, 0x10];
        let dram = Memory::new_with_filled_ram(&program, program.len());
        let mut mmio = Mmio::empty();
        mmio.add((0, 4), Box::new(dram)).unwrap();
        let mut riscv = Riscv::fabricate(mmio, DebugMode::Disabled);

        let result = riscv.run();
        assert!(result.is_ok());
    }

    #[test]
    fn parse_wfi() {
        use bit_field::BitField;
        use byteorder::{LittleEndian, ReadBytesExt};

        let program = vec![0x73, 0x00, 0x50, 0x10];
        let mut instr = &program[0..4];
        let instr = instr.read_u32::<LittleEndian>().unwrap();
        let opcode = instr.get_bits(0..7);

        assert_eq!(0b1110011, opcode);
    }
}
