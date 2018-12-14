use crate::cpu::model::CpuModel;
use peripherals::interconnect::Interconnect;
use crate::rustemu86::DebugMode;

use std::result;
pub type Result<T> = result::Result<T, InternalError>;

pub struct InternalError(String);

#[allow(dead_code)]
pub struct Riscv {
    mmio: Interconnect,
    debug: Box<dyn DebugMode>,
}

impl CpuModel for Riscv {
    type Error = InternalError;

    fn new(mmio: Interconnect, debug: Box<dyn DebugMode>) -> Riscv {
        Riscv {
            mmio: mmio,
            debug: debug,
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
    use peripherals::interconnect::Interconnect;
    use peripherals::memory_access::{MemoryAccess, MemoryAccessError};
    use peripherals::uart16550::{self, Target};
    use crate::rustemu86::DebugDesabled;

    struct FakeDisplay();
    impl MemoryAccess for FakeDisplay {
        fn read_u8(&self, addr: usize) -> result::Result<u8, MemoryAccessError> {
            unimplemented!()
        }

        fn write_u8(&mut self, addr: usize, data: u8) -> result::Result<(), MemoryAccessError> {
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
        let mut riscv = Riscv::new(mmio, Box::new(DebugDesabled {}));

        let result = riscv.run();
        assert!(result.is_ok());
    }
}
