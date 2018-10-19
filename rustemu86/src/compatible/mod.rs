mod gpr;
mod fetcher;
mod decoder;
mod executor;
mod status_regs;
mod isa;

use self::gpr::RegisterFile;
use self::gpr::Reg32::*;
use self::status_regs::CpuState;
use self::fetcher::FetchedInst;
use self::decoder::ExecuteInst;
use self::executor::WriteBackType;
use peripherals::interconnect::Interconnect;
use cpu::model::{CpuModel, Pipeline};
use std::result;

pub type Result<T> = result::Result<T, CompatibleException>;

/// x86 32-bit mode.
pub struct CompatibleMode {
    ip: u64,
    bus: Interconnect,
    rf: RegisterFile,
    state: CpuState,
}

impl CompatibleMode {
    /// Creates instance just after booting bios.
    /// IP starts with 0x7c00.
    pub fn boot_bios(peripheral_bus: Interconnect) -> CompatibleMode {
        let mut rf = RegisterFile::new();
        rf.write_u64(Eax as usize, 0xaa55u64);
        rf.write_u64(Esp as usize, 0x6f2cu64);
        CompatibleMode {
            ip: 0x7c00u64,
            bus: peripheral_bus,
            rf: rf,
            state: CpuState::Running,
        }
    }
}

impl CpuModel for CompatibleMode {
    type Error = CompatibleException;

    fn new(peripheral_bus: Interconnect) -> CompatibleMode {
        CompatibleMode {
            ip: 0,
            bus: peripheral_bus,
            rf: RegisterFile::new(),
            state: CpuState::Running,
        }
    }

    fn init(&mut self) {
        unimplemented!()
    }

    fn run(&mut self) -> Result<()> {
        while self.state == CpuState::Running {
            let inst_candidate = self.bus.fetch_inst_candidate(self.ip);
            self.execute_an_instruction(&inst_candidate)?;
        }
        Ok(())
    }
}

impl Pipeline for CompatibleMode {
    type Error = CompatibleException;
    type Fetched = FetchedInst;
    type Decoded = ExecuteInst;
    type Executed = WriteBackType;

    fn execute_an_instruction(&mut self, program: &[u8]) -> Result<()> {
        let fetched_inst = fetcher::fetch(&program)?;
        self.ip = fetched_inst.increment_ip(self.ip);
        let decoded_inst = decoder::decode(&fetched_inst, &self.rf)?;
        let write_back_packet = executor::execute(&decoded_inst)?;

        self.write_back(&write_back_packet)
    }

    fn fetch(&self, program: &[u8]) -> Result<Self::Fetched> {
        fetcher::fetch(program)
    }

    fn decode(&self, inst: &Self::Fetched) -> Result<Self::Decoded> {
        decoder::decode(&inst, &self.rf)
    }

    fn execute(&self, inst: &Self::Decoded) -> Result<Self::Executed> {
        executor::execute(&inst)
    }

    fn write_back(&mut self, inst: &Self::Executed) -> Result<()> {
        match inst {
            WriteBackType::Gpr(inst) => {
                self.rf.write_u64(inst.index, inst.value);
            }
            WriteBackType::Status(inst) => {
                self.state = inst.state;
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
    use cpu::model::cpu_factory;

    fn execute_program(program: Vec<u8>) -> CompatibleMode {
        let mut interconnect = Interconnect::new(
            EmulationMode::Normal, GtkVgaTextBuffer::new());
        interconnect.init_memory(program);
        let mut cpu: CompatibleMode = cpu_factory(interconnect);
        let result = cpu.run();

        assert!(result.is_ok(), "{:?}", result.err());
        cpu
    }

    #[test]
    fn skip_bios() {
        let mut interconnect = Interconnect::new(
            EmulationMode::Normal, GtkVgaTextBuffer::new());
        let cpu = CompatibleMode::boot_bios(interconnect);

        assert_eq!(cpu.rf.read_u64(Eax as usize), 0xaa55u64);
        assert_eq!(cpu.rf.read_u64(Esp as usize), 0x6f2cu64);
        assert_eq!(cpu.ip, 0x7c00u64);
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
