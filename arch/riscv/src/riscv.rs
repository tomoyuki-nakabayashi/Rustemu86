use crate::decode::decode;
use crate::execute::execute;
use crate::fetch::fetch;
use crate::gpr::Gpr;
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
    gpr: Gpr,
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
            gpr: Gpr::new(),
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
            let wb = execute(instr, &self.gpr)?;

            // Change CPU state only here.
            use crate::execute::WriteBackData::*;;
            match wb {
                Gpr { target, value } => {
                    self.gpr.write_u32(target, value);
                }
                Halt => {
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
    use crate::isa::abi_name::*;
    use peripherals::memory::Memory;

    #[test]
    fn stop_at_wfi() {
        let program = vec![0x73, 0x00, 0x50, 0x10];
        let dram = Memory::new_with_filled_ram(&program, program.len());
        let mut mmio = Mmio::empty();
        mmio.add((0, program.len()), Box::new(dram)).unwrap();
        let mut riscv = Riscv::fabricate(mmio, DebugMode::Disabled);
        riscv.init();

        let result = riscv.run();
        assert!(result.is_ok());
    }

    #[test]
    fn add_imm() {
        let program = vec![
            0x93, 0x80, 0x10, 0x00, // addi ra, zero, 1
            0x13, 0x01, 0xf1, 0xff, // addi sp, sp -1
            0x73, 0x00, 0x50, 0x10, // wfi
        ];

        let dram = Memory::new_with_filled_ram(&program, program.len());
        let mut mmio = Mmio::empty();
        mmio.add((0, program.len()), Box::new(dram)).unwrap();
        let mut riscv = Riscv::fabricate(mmio, DebugMode::Disabled);
        riscv.init();

        let result = riscv.run();
        assert!(result.is_ok());
        assert_eq!(riscv.gpr.read_u32(ra), 1);
        assert_eq!(riscv.gpr.read_u32(sp) as i32, -1);
    }

    #[test]
    fn or_imm() {
        let program = vec![
            0x93, 0xe0, 0x20, 0x00, // ori ra, zero, 2
            0x13, 0x61, 0xf1, 0xff, // ori sp, sp -1
            0x73, 0x00, 0x50, 0x10, // wfi
        ];

        let dram = Memory::new_with_filled_ram(&program, program.len());
        let mut mmio = Mmio::empty();
        mmio.add((0, program.len()), Box::new(dram)).unwrap();
        let mut riscv = Riscv::fabricate(mmio, DebugMode::Disabled);
        riscv.init();

        let result = riscv.run();
        assert!(result.is_ok());
        assert_eq!(riscv.gpr.read_u32(ra), 2);
        assert_eq!(riscv.gpr.read_u32(sp), 0xffff_ffff);
    }
}
