use cpu::model::CpuModel;
use debug::DebugMode;
use peripherals::interconnect::Interconnect;

use std::result;
pub type Result<T> = result::Result<T, InternalError>;

pub struct InternalError(String);

#[allow(dead_code)]
pub struct Riscv {
    pc: u32,
    mmio: Interconnect,
    debug: DebugMode,
}

impl CpuModel for Riscv {
    type Error = InternalError;

    fn new(mmio: Interconnect, debug: DebugMode) -> Riscv {
        Riscv {
            pc: 0u32,
            mmio,
            debug,
        }
    }

    fn init(&mut self) {
        unimplemented!()
    }

    fn run(&mut self) -> Result<()> {
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use peripherals::memory_access::{MemoryAccess, MemoryAccessError};
    use peripherals::uart16550::{self, Target};

    struct FakeDisplay();
    impl MemoryAccess for FakeDisplay {
        fn read_u8(&self, _addr: usize) -> result::Result<u8, MemoryAccessError> {
            unimplemented!()
        }

        fn write_u8(&mut self, _addr: usize, _data: u8) -> result::Result<(), MemoryAccessError> {
            unimplemented!()
        }
    }

    #[test]
    fn stop_at_wfi() {
        let program = vec![0x73, 0x00, 0x50, 0x10];
        let display: Box<dyn MemoryAccess> = Box::new(FakeDisplay());
        let serial = uart16550::uart_factory(Target::Buffer);
        let mut mmio = Interconnect::new(serial, display);
        mmio.init_memory(&program, 0);
        let mut riscv = Riscv::new(mmio, DebugMode::Disabled);

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
