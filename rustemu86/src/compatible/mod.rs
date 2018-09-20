extern crate qemu_from;

mod gpr;
mod fetcher;
mod isa;

use self::gpr::RegisterFile;
use self::isa::opcode::OpcodeCompat;
use peripherals::interconnect::Interconnect;
use num::FromPrimitive;
use std::result;

pub type Result<T> = result::Result<T, CompatibleException>;

pub struct CompatibleMode {
    ip: u64,
    bus: Interconnect,
    rf: RegisterFile,
}

impl CompatibleMode {
    pub fn new(peripheral_bus: Interconnect) -> CompatibleMode {
        CompatibleMode {
            ip: 0,
            bus: peripheral_bus,
            rf: RegisterFile::new(),
        }
    }

    pub fn run(&mut self) -> Result<()> {
        let inst_candidate = self.bus.fetch_inst_candidate(self.ip);
        let fetched_inst = fetcher::fetch(&inst_candidate);
        self.ip = fetched_inst.unwrap().increment_ip(self.ip);
        match OpcodeCompat::from_u8(inst_candidate[0]).unwrap() {
            OpcodeCompat::Hlt => Ok(()),
            OpcodeCompat::Xor => { self.rf.write_u64(0, 0); Ok(()) },
            _ => Err(CompatibleException("Invalid instruction.".to_string())),
        }
    }
}

#[derive(Debug)]
pub struct CompatibleException(String);

#[cfg(test)]
mod test {
    use super::*;
    use args::EmulationMode;
    use display::GtkVgaTextBuffer;

    fn execute_program(program: Vec<u8>) -> CompatibleMode {
        let mut interconnect = Interconnect::new(EmulationMode::Normal, GtkVgaTextBuffer::new());
        interconnect.init_memory(program);
        let mut cpu = CompatibleMode::new(interconnect);
        let result = cpu.run();

        assert!(result.is_ok(), "{:?}", result.err());
        cpu
    }

    #[test]
    fn stop_at_hlt() {
        let program = vec![0xf4];
        let _cpu = execute_program(program);
    }

    #[test]
    fn clear_register_by_xor() {
        let program = vec![0x31, 0xc0, 0xf4];
        let cpu = execute_program(program);

        assert_eq!(cpu.rf.read_u64(0), 0);
    }
}
