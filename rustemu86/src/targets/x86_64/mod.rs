extern crate bit_field;

mod decoder;
mod ex_stage;
mod exceptions;
mod fetcher;
mod isa;
mod register_file;

use self::decoder::ExecuteInstType;
use self::ex_stage::{WriteBack, WriteBackData};
use self::exceptions::InternalException;
use self::fetcher::{FetchUnit, FetchedInst};
use self::register_file::RegisterFile;
use crate::cpu::model::{CpuModel, Pipeline};
use peripherals::interconnect::Interconnect;
use peripherals::memory_access::MemoryAccess;
use crate::rustemu86::DebugMode;
use std::fmt;
use std::result;

pub type Result<T> = result::Result<T, InternalException>;
pub struct X86_64 {
    rf: RegisterFile,
    fetch_unit: FetchUnit,
    executed_insts: u64,
    mmio: Interconnect,
    state: CpuState,
    debug: Box<dyn DebugMode>,
}

impl CpuModel for X86_64 {
    type Error = InternalException;

    fn new(mmio: Interconnect, debug: Box<dyn DebugMode>) -> X86_64 {
        X86_64 {
            rf: RegisterFile::new(),
            fetch_unit: FetchUnit::new(),
            executed_insts: 0,
            mmio: mmio,
            state: CpuState::Running,
            debug: debug,
        }
    }

    fn init(&mut self) {
        unimplemented!()
    }

    fn run(&mut self) -> Result<()> {
        while self.state == CpuState::Running {
            let inst_candidate = self.mmio.fetch_inst_candidate(self.fetch_unit.get_rip());
            let inst = self.fetch_unit.fetch(&inst_candidate)?;
            let uops = self.decode(&inst)?;
            let wbs = self.execute(&uops)?;
            self.write_back(&wbs)?;
            self.executed_insts += 1;
            self.debug.do_cycle_end_action(&self);
        }
        println!(
            "Finish emulation. {} instructions executed.",
            self.executed_insts
        );
        Ok(())
    }
}

impl Pipeline for X86_64 {
    type Error = InternalException;
    type Fetched = FetchedInst;
    type Decoded = Vec<ExecuteInstType>;
    type Executed = Vec<WriteBack>;

    fn fetch(&self, _program: &[u8]) -> Result<Self::Fetched> {
        //self.fetch_unit.fetch(&program)
        unimplemented!()
    }

    fn decode(&self, inst: &Self::Fetched) -> Result<Self::Decoded> {
        decoder::decode(&self.rf, &inst)
    }

    fn execute(&self, insts: &Self::Decoded) -> Result<Self::Executed> {
        let results: Self::Executed = (&insts)
            .into_iter()
            .map(|inst| ex_stage::execute(&inst).unwrap())
            .collect();
        Ok(results)
    }

    fn write_back(&mut self, inst: &Self::Executed) -> Result<()> {
        for wb in inst {
            match wb {
                WriteBack::GeneralRegister(dest, value) => self.rf.write64(*dest, *value),
                WriteBack::Rip(next_rip) => self.fetch_unit.set_rip(*next_rip),
                WriteBack::Load(dest, addr) => self
                    .rf
                    .write64(*dest, self.mmio.read_u64(*addr as usize).unwrap()),
                WriteBack::Store(addr, data) => {
                    let addr = *addr as usize;
                    use self::WriteBackData::*;
                    match data {
                        Byte(data) => self.mmio.write_u8(addr, *data).unwrap(),
                        Word(data) => self.mmio.write_u16(addr, *data).unwrap(),
                        DWord(data) => self.mmio.write_u32(addr, *data).unwrap(),
                        QWord(data) => self.mmio.write_u64(addr, *data).unwrap(),
                    }
                }
                WriteBack::CpuState(next_state) => self.state = *next_state,
                WriteBack::Return(addr) => self
                    .fetch_unit
                    .set_rip(self.mmio.read_u64(*addr as usize).unwrap()),
            };
        }
        Ok(())
    }
}

impl fmt::Debug for X86_64 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "=== CPU status ({} instructions executed.)===\nRIP: {}\nRegisters:\n{}",
            self.executed_insts,
            self.fetch_unit.get_rip(),
            self.rf
        )
    }
}

impl fmt::Display for X86_64 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "=== CPU status ({} instructions executed.)===\nRIP: 0x{:>08X}\nRegisters:\n{}",
            self.executed_insts,
            self.fetch_unit.get_rip(),
            self.rf
        )
    }
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum CpuState {
    Running,
    Halt,
}

#[cfg(test)]
mod test {
    use super::*;
    use peripherals::interconnect::Interconnect;
    use peripherals::memory_access::MemoryAccessError;
    use peripherals::uart16550::{self, Target};
    use crate::rustemu86::DebugDesabled;
    use crate::x86_64::isa::registers::Reg64Id::{Rax, Rbx, Rcx, Rsp};

    struct FakeDisplay();
    impl MemoryAccess for FakeDisplay {
        fn read_u8(&self, addr: usize) -> result::Result<u8, MemoryAccessError> {
            unimplemented!()
        }

        fn write_u8(&mut self, addr: usize, data: u8) -> result::Result<(), MemoryAccessError> {
            unimplemented!()
        }
    }

    fn execute_program(program: Vec<u8>) -> X86_64 {
        let display: Box<dyn MemoryAccess> = Box::new(FakeDisplay());
        let serial = uart16550::uart_factory(Target::Buffer);
        let mut mmio = Interconnect::new(serial, display);
        mmio.init_memory(&program, 0);
        let mut x86_64 = X86_64::new(mmio, Box::new(DebugDesabled {}));
        let result = x86_64.run();

        assert!(result.is_ok(), "{:?}", result.err());
        x86_64
    }

    fn execute_program_after_init(program: Vec<u8>, initializer: &Fn(&mut X86_64)) -> X86_64 {
        let display: Box<dyn MemoryAccess> = Box::new(FakeDisplay());
        let serial = uart16550::uart_factory(Target::Buffer);
        let mut mmio = Interconnect::new(serial, display);
        mmio.init_memory(&program, 0);
        let mut x86_64 = X86_64::new(mmio, Box::new(DebugDesabled {}));
        initializer(&mut x86_64);
        let result = x86_64.run();

        assert!(result.is_ok(), "{:?}", result.err());
        x86_64
    }

    #[test]
    fn execute_two_instructions() {
        let program = vec![
            0xb8, 0x00, 0x00, 0x00, 0x00, // mov rax, 0
            0x48, 0xff, 0xc0, // inc rax
            0xf4,
        ]; // hlt
        let x86_64 = execute_program(program);
        assert_eq!(x86_64.fetch_unit.get_rip(), 9);
    }

    #[test]
    fn execute_mov32() {
        let program = vec![0xb8, 0x01, 0x00, 0x00, 0x00, 0xf4];
        let x86_64 = execute_program(program);
        assert_eq!(x86_64.rf.read64(Rax), 1);
    }

    #[test]
    fn execute_mov_rm_imm32() {
        let program = vec![0x48, 0xc7, 0xc0, 0x00, 0x00, 0x00, 0x10, 0xf4];
        let x86_64 = execute_program(program);
        assert_eq!(x86_64.rf.read64(Rax), 0x10000000);
    }

    #[test]
    fn execute_inc() {
        let program = vec![0x48, 0xff, 0xc0, 0xf4];
        let initializer = |x86_64: &mut X86_64| x86_64.rf.write64(Rax, 0);
        let x86_64 = execute_program_after_init(program, &initializer);
        assert_eq!(x86_64.rf.read64(Rax), 1);
    }

    #[test]
    fn execute_add() {
        let program = vec![0x48, 0x01, 0xc8, 0xf4];
        let initializer = |x86_64: &mut X86_64| {
            x86_64.rf.write64(Rax, 1);
            x86_64.rf.write64(Rcx, 2);
        };
        let x86_64 = execute_program_after_init(program, &initializer);
        assert_eq!(x86_64.rf.read64(Rax), 3);
    }

    #[test]
    fn execute_jmp_short() {
        let program = vec![0xeb, 0x05, 0x00, 0x00, 0x00, 0x00, 0x00, 0xf4];
        let x86_64 = execute_program(program);
        assert_eq!(x86_64.fetch_unit.get_rip(), 8);
    }

    #[test]
    fn execute_load_store() {
        let program = vec![0x48, 0x89, 0x18, 0x48, 0x8b, 0x08, 0xf4];
        let initializer = |x86_64: &mut X86_64| {
            x86_64.rf.write64(Rax, 100);
            x86_64.rf.write64(Rbx, 1);
        };
        let x86_64 = execute_program_after_init(program, &initializer);
        assert_eq!(x86_64.mmio.read_u64(100).unwrap(), 1);
        assert_eq!(x86_64.rf.read64(Rcx), 1);
    }

    #[test]
    fn execute_push_pop() {
        let program = vec![0x50, 0x5b, 0xf4];
        let initializer = |x86_64: &mut X86_64| {
            x86_64.rf.write64(Rsp, 0x0100);
            x86_64.rf.write64(Rax, 1);
        };
        let x86_64 = execute_program_after_init(program, &initializer);
        assert_eq!(x86_64.mmio.read_u64(0x100 - 8).unwrap(), 1);
        assert_eq!(x86_64.rf.read64(Rbx), 1);
    }

    #[test]
    fn execute_call_ret() {
        let program = vec![0xe8, 0x03, 0x00, 0x00, 0x00, 0xf4, 0x00, 0x00, 0xc3];
        let initializer = |x86_64: &mut X86_64| {
            x86_64.rf.write64(Rsp, 0x0100);
        };
        let x86_64 = execute_program_after_init(program, &initializer);
        assert_eq!(x86_64.mmio.read_u64(0x100 - 8).unwrap(), 5);
        assert_eq!(x86_64.executed_insts, 3);
    }

    #[test]
    fn execute_mov_rm8_imm8() {
        let program = vec![0xC6, 0x04, 0x25, 0x00, 0x01, 0x00, 0x00, 0x48, 0xf4];
        let x86_64 = execute_program(program);
        assert_eq!(x86_64.mmio.read_u64(0x100).unwrap(), 0x48);
    }

    #[test]
    fn execute_mov_rm16_imm16() {
        let program = vec![
            0x66, 0xC7, 0x04, 0x25, 0x00, 0x01, 0x00, 0x00, 0x48, 0x0e, 0xf4,
        ];
        let x86_64 = execute_program(program);
        assert_eq!(x86_64.mmio.read_u64(0x100).unwrap(), 0x0e48);
    }

}
