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
use rustemu86::DebugMode;
use std::result;

pub type Result<T> = result::Result<T, CompatibleException>;

/// x86 32-bit mode.
pub struct X86 {
    ip: u64,
    mmio: Interconnect,
    rf: RegisterFile,
    state: CpuState,
}

impl X86 {
    /// Creates instance just after booting bios.
    /// IP starts with 0x7c00.
    pub fn boot_bios(&mut self) {
        self.rf.write_u64(Eax, 0xaa55u64);
        self.rf.write_u64(Esp, 0x6f2cu64);
        self.ip = 0x7c00u64;
    }
}

impl CpuModel for X86 {
    type Error = CompatibleException;

    fn new(mmio: Interconnect, _debug: Box<dyn DebugMode>) -> X86 {
        X86 {
            ip: 0,
            mmio: mmio,
            rf: RegisterFile::new(),
            state: CpuState::Running,
        }
    }

    fn init(&mut self) {
        unimplemented!()
    }

    fn run(&mut self) -> Result<()> {
        while self.state == CpuState::Running {
            let inst_candidate = self.mmio.fetch_inst_candidate(self.ip);
            self.execute_an_instruction(&inst_candidate)?;
        }
        Ok(())
    }
}

impl Pipeline for X86 {
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
    use rustemu86::DebugDesabled;

    fn execute_program(program: Vec<u8>, start_addr: usize) -> X86 {
        let mut interconnect = Interconnect::new(
            EmulationMode::Normal, GtkVgaTextBuffer::new());
        interconnect.init_memory(program, start_addr);
        let mut x86: X86 = cpu_factory(interconnect, Box::new(DebugDesabled{}));
        let result = x86.run();

        assert!(result.is_ok(), "{:?}", result.err());
        x86
    }

    #[test]
    fn skip_bios() {
        let program = vec![0xf4];
        let mut interconnect = Interconnect::new(
            EmulationMode::Normal, GtkVgaTextBuffer::new());
        interconnect.init_memory(program, 0x7c00);

        let mut x86: X86 = cpu_factory(interconnect, Box::new(DebugDesabled{}));
        x86.boot_bios();

        assert_eq!(x86.rf.read_u64(Eax), 0xaa55u64);
        assert_eq!(x86.rf.read_u64(Esp), 0x6f2cu64);
        assert_eq!(x86.ip, 0x7c00u64);

        let result = x86.run();
        assert!(result.is_ok(), "{:?}", result.err());
        assert_eq!(x86.ip, 0x7c01u64);
        assert_eq!(x86.state, CpuState::Halted);
    }

    #[test]
    fn stop_at_hlt() {
        let program = vec![0xf4];
        let x86 = execute_program(program, 0);

        assert_eq!(x86.state, CpuState::Halted);
    }

    #[test]
    fn clear_register_by_xor() {
        let program = vec![0x31, 0xc0, 0xf4];
        let x86 = execute_program(program, 0);

        assert_eq!(x86.rf.read_u64(Eax), 0);
    }
}
