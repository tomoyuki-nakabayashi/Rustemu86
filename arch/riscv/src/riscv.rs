use cpu::model::CpuModel;
use debug::DebugMode;
use peripherals::interconnect::Interconnect;
use peripherals::mmio::Mmio;
use crate::fetch::fetch;

use std::result;
pub type Result<T> = result::Result<T, InternalError>;

pub struct InternalError(String);

#[allow(dead_code)]
pub struct Riscv {
    pc: u32,
    mmio: Mmio,
    debug: DebugMode,
}

impl Riscv {
    pub fn fabricate(mmio: Mmio, debug: DebugMode) -> Riscv {
        Riscv {
            pc: 0,
            mmio,
            debug,
        }
    }
}

impl CpuModel for Riscv {
    type Error = InternalError;

    fn new(_mmio: Interconnect, _debug: DebugMode) -> Riscv {
        unimplemented!()
    }

    fn init(&mut self) {
        unimplemented!()
    }

    fn run(&mut self) -> Result<()> {
        let instr = fetch(&self.mmio, self.pc as usize).unwrap();
        if instr == 0x1050_0073 {
            Ok(())
        } else {
            Err(InternalError("Error".to_string()))
        }
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
        use byteorder::{LittleEndian, ReadBytesExt};
        use bit_field::BitField;

        let program = vec![0x73, 0x00, 0x50, 0x10];
        let mut instr = &program[0..4];
        let instr = instr.read_u32::<LittleEndian>().unwrap();
        let opcode = instr.get_bits(0..7);

        assert_eq!(0b1110011, opcode);
    }
}
