use rustemu86::DebugMode;
use peripherals::interconnect::Interconnect;
use cpu::model::{CpuModel, Pipeline};

use std::result;
pub type Result<T> = result::Result<T, InternalError>;

pub struct InternalError (String);

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
    use rustemu86::DebugDesabled;
    use args::EmulationMode;
    use display::GtkVgaTextBuffer;

    #[test]
    fn stop_at_wfi() {
        let program = vec![0x73, 0x00, 0x50, 0x10];
        let mut mmio = Interconnect::new(EmulationMode::Normal,
            GtkVgaTextBuffer::new());
        mmio.init_memory(program);
        let mut riscv = Riscv::new(mmio, Box::new(DebugDesabled {}));

        let result = riscv.run();
        assert!(result.is_ok());
    }
}