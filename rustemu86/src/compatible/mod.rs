mod gpr;
mod fetcher;
mod decoder;
mod executor;
mod status_regs;
mod isa;

use self::gpr::RegisterFile;
use self::executor::WriteBackType;
use self::isa::opcode::OpcodeCompat;
use self::status_regs::CpuState;
use peripherals::interconnect::Interconnect;
use std::result;

pub type Result<T> = result::Result<T, CompatibleException>;

pub struct CompatibleMode {
    ip: u64,
    bus: Interconnect,
    rf: RegisterFile,
    state: CpuState,
}

impl CompatibleMode {
    pub fn new(peripheral_bus: Interconnect) -> CompatibleMode {
        CompatibleMode {
            ip: 0,
            bus: peripheral_bus,
            rf: RegisterFile::new(),
            state: CpuState::Running,
        }
    }

    pub fn run(&mut self) -> Result<()> {
 
        while self.state == CpuState::Running {
            let inst_candidate = self.bus.fetch_inst_candidate(self.ip);
            let fetched_inst = fetcher::fetch(&inst_candidate)?;
            self.ip = fetched_inst.increment_ip(self.ip);
            let decoded_inst = decoder::decode(fetched_inst)?;
            let write_back_packet = executor::execute(decoded_inst)?;
            self.write_back(write_back_packet)?;
        }
        Ok(())
    }

    fn write_back(&mut self, packet: WriteBackType) -> Result<()> {
        match packet {
            WriteBackType::Gpr(packet) => {
                self.rf.write_u64(packet.index, packet.value);
            }
            WriteBackType::Status(packet) => {
                self.state = packet.state;
            }
        }
        Ok(())
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
        let cpu = execute_program(program);

        assert_eq!(cpu.state, CpuState::Halted)
    }

    #[test]
    fn clear_register_by_xor() {
        let program = vec![0x31, 0xc0, 0xf4];
        let cpu = execute_program(program);

        assert_eq!(cpu.rf.read_u64(0), 0);
    }
}
