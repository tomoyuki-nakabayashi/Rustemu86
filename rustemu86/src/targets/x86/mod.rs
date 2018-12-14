mod decoder;
mod executor;
mod fetcher;
mod gpr;
mod isa;
mod status_regs;

use self::decoder::ExecuteInst;
use self::executor::WriteBackType;
use self::fetcher::FetchedInst;
use self::gpr::{RegisterFile, SegmentRegister};
use self::isa::eflags::EFlags;
use self::status_regs::CpuState;
use crate::cpu::model::{CpuModel, Pipeline};
use peripherals::{interconnect::Interconnect, memory_access::MemoryAccess};
use crate::rustemu86::DebugMode;
use std::result;

pub type Result<T> = result::Result<T, CompatibleException>;

/// x86 32-bit mode.
/// Note that this does not cover all of x86 instructions.
/// But will cover enough instructions to boot BlosOS.
///
/// ip: instruction pointer.
/// mmio: memory mapped io.
/// rf: general purpose register.
/// state: CPU status either Run or Halted.
pub struct X86 {
    ip: u64,
    mmio: Interconnect,
    rf: RegisterFile,
    segment: SegmentRegister,
    eflags: EFlags,
    state: CpuState,
}

impl X86 {
    /// Creates instance just after booting bios.
    /// IP starts with 0x7c00.
    pub fn boot_bios(&mut self) {
        use self::gpr::Reg32::*;
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
            segment: SegmentRegister::new(),
            eflags: EFlags::empty(),
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
                println!("{:?} <= {}", inst.index, inst.value);
                self.rf.write_u64(inst.index, inst.value);
            }
            WriteBackType::Store(inst) => {
                self.mmio
                    .write_u64(inst.index, inst.value)
                    .map_err(|_| CompatibleException("Invalid memory operation.".to_string()))?;
            }
            WriteBackType::Segment(inst) => {
                println!("{:?} <= {}", inst.index, inst.value);
                self.segment.write_u64(inst.index, inst.value);
            }
            WriteBackType::EFlags(inst) => {
                self.eflags.set(inst.target, inst.value);
            }
            WriteBackType::Status(inst) => {
                self.state = inst.state;
            }
        }
        Ok(())
    }
}

/// Represents errors of the emulator implementation.
/// Not CPU exception such as interrupts.
#[derive(Debug)]
pub struct CompatibleException(String);

#[cfg(test)]
mod test {
    use self::gpr::Reg32::*;
    use self::gpr::SegReg::*;
    use super::*;
    use crate::cpu::model::cpu_factory;
    use peripherals::interconnect::Interconnect;
    use peripherals::memory_access::MemoryAccessError;
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

    fn execute_program(program: Vec<u8>, start_addr: usize) -> X86 {
        let display: Box<dyn MemoryAccess> = Box::new(FakeDisplay());
        let serial = uart16550::uart_factory(Target::Buffer);
        let mut mmio = Interconnect::new(serial, display);
        mmio.init_memory(&program, start_addr);
        let mut x86: X86 = cpu_factory(mmio, Box::new(DebugDesabled {}));
        let result = x86.run();

        assert!(result.is_ok(), "{:?}", result.err());
        x86
    }

    fn execute_program_after(program: Vec<u8>, initializer: fn(&mut X86)) -> X86 {
        let display: Box<dyn MemoryAccess> = Box::new(FakeDisplay());
        let serial = uart16550::uart_factory(Target::Buffer);
        let mut mmio = Interconnect::new(serial, display);
        mmio.init_memory(&program, 0);
        let mut x86 = X86::new(mmio, Box::new(DebugDesabled {}));
        initializer(&mut x86);
        let result = x86.run();

        assert!(result.is_ok(), "{:?}", result.err());
        x86
    }

    #[test]
    fn skip_bios() {
        let program = vec![0xf4];
        let display: Box<dyn MemoryAccess> = Box::new(FakeDisplay());
        let serial = uart16550::uart_factory(Target::Buffer);
        let mut mmio = Interconnect::new(serial, display);
        mmio.init_memory(&program, 0x7c00);

        let mut x86: X86 = cpu_factory(mmio, Box::new(DebugDesabled {}));
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
        let program = vec![0xf4]; // hlt
        let x86 = execute_program(program, 0);

        assert_eq!(x86.state, CpuState::Halted);
    }

    #[test]
    fn clear_register_by_xor() {
        let program = vec![
            0x31, 0xc0, // xor    ax,ax
            0xf4,
        ];
        let x86 = execute_program(program, 0);

        assert_eq!(x86.rf.read_u64(Eax), 0);
    }

    #[test]
    fn mov_rm_to_sreg() {
        let program = vec![
            0x8e, 0xd8, // mov    ds,ax
            0x8e, 0xc0, // mov    es,ax
            0x8e, 0xd0, // mov    ss,ax
            0x8e, 0xe0, // mov    fs,ax
            0x8e, 0xe8, // mov    gs,ax
            0xf4,
        ];
        let x86 = execute_program_after(program, |cpu: &mut X86| {
            cpu.rf.write_u64(Eax, 0xaa55u64);
        });

        assert_eq!(x86.segment.read_u64(Ds), x86.rf.read_u64(Eax));
        assert_eq!(x86.segment.read_u64(Es), x86.rf.read_u64(Eax));
        assert_eq!(x86.segment.read_u64(Ss), x86.rf.read_u64(Eax));
        assert_eq!(x86.segment.read_u64(Fs), x86.rf.read_u64(Eax));
        assert_eq!(x86.segment.read_u64(Gs), x86.rf.read_u64(Eax));
    }

    #[test]
    fn mov_imm() {
        let program = vec![
            0xbc, 0x00, 0x7c, // mov    sp,0x7c00
            0xf4,
        ];
        let x86 = execute_program(program, 0);

        assert_eq!(x86.rf.read_u64(Esp), 0x7c00u64);
    }

    #[test]
    fn cld() {
        let program = vec![
            0xfc, // cld
            0xf4,
        ];
        let x86 = execute_program_after(program, |cpu: &mut X86| {
            cpu.eflags.set(EFlags::DIRECTION_FLAG, true);
        });

        assert!((x86.eflags & EFlags::DIRECTION_FLAG).is_empty());
    }

    #[test]
    fn lea() {
        let program = vec![
            0x67, 0x8d, 0x35, 0x16, 0x7d, 0x00, 0x00, // addr32 lea si,ds:0x7d16
            0xf4,
        ];
        let x86 = execute_program(program, 0);

        assert_eq!(x86.rf.read_u64(Esi), 0x00007d16);
    }

    #[test]
    fn load_mr_store_rm() {
        let program = vec![
            0x67, 0x89, 0x18, // addr32 mov [eax], ebx
            //0x67, 0x8b, 0x08,  // addr32 mov ecx, [eax]
            0xf4,
        ];
        let initializer = |cpu: &mut X86| {
            cpu.rf.write_u64(Eax, 100);
            cpu.rf.write_u64(Ebx, 1);
        };
        let x86_64 = execute_program_after(program, initializer);
        assert_eq!(x86_64.mmio.read_u64(100).unwrap(), 1);
        //assert_eq!(x86_64.rf.read_u64(Ecx), 1);
    }
}
