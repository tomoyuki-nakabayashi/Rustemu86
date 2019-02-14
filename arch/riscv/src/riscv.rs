use crate::csr::Csr;
use crate::debug::DebugInterface;
use crate::decode::decode;
use crate::execute::execute;
use crate::fetch::fetch;
use crate::gpr::Gpr;
use crate::lsu::load_store;
use cpu::model::CpuModel;
use debug::DebugMode;
use peripherals::interconnect::Interconnect;
use peripherals::memory_access::MemoryAccess;

use crate::isa::exceptions::InternalExceptions;
use std::result;
pub type Result<T> = result::Result<T, InternalExceptions>;

use std::fmt;

/// RISC-V CPU model.
#[allow(dead_code)]
pub struct Riscv<BUS: MemoryAccess> {
    pc: u32,
    mmio: BUS,
    debug: DebugMode,
    gpr: Gpr,
    csr: Csr,
    halted: bool,
}

impl<BUS: MemoryAccess> Riscv<BUS> {
    /// Temporary `new`.
    /// TODO: This must be a new. It requires to modify CpuModel interface.
    pub fn fabricate(mmio: BUS, debug: DebugMode) -> Self {
        Riscv {
            pc: 0,
            mmio,
            debug,
            gpr: Gpr::new(),
            csr: Csr::new(),
            halted: true,
        }
    }
}

impl<BUS: MemoryAccess> CpuModel for Riscv<BUS> {
    type Error = InternalExceptions;

    fn new(_mmio: Interconnect, _debug: DebugMode) -> Self {
        unimplemented!()
    }

    /// Initialize CPU state for run.
    fn init(&mut self) {
        self.halted = false;
    }

    /// Executes instructions until WFI.
    fn run(&mut self) -> Result<()> {
        while !self.halted {
            let (instr, next_pc) = fetch(&self.mmio, self.pc)?;
            let instr = decode(instr, &self.gpr, self.pc, next_pc)?;
            let (wb, next_pc) = execute(instr)?;

            // Change CPU state only here.
            // First, update program counter.
            // This will be updated again in case of priviledged instruction.
            self.pc = next_pc;

            // Next, write to general purpose register and control, status register,
            // and memory.
            use crate::execute::WriteBackData::*;
            use crate::isa::opcode::PrivOp;
            match wb {
                Gpr { target, value } => self.gpr.write_u32(target, value),
                Lsu(ref op) => {
                    let wb = load_store(&mut self.mmio, op)?;
                    if let Gpr { target, value } = wb {
                        self.gpr.write_u32(target, value);
                    };
                }
                Csr(instr) => {
                    use crate::isa::opcode::CsrOp::*;
                    match instr.op {
                        WRITE => {
                            let old = self.csr.read_u32(instr.csr_addr);
                            self.csr.write_u32(instr.csr_addr, instr.src);
                            self.gpr.write_u32(instr.dest, old);
                        }
                        SET => {
                            let old = self.csr.read_u32(instr.csr_addr);
                            self.csr.write_u32(instr.csr_addr, instr.src | old);
                            self.gpr.write_u32(instr.dest, old);
                        }
                        CLEAR => {
                            let old = self.csr.read_u32(instr.csr_addr);
                            self.csr.write_u32(instr.csr_addr, (!instr.src) & old);
                            self.gpr.write_u32(instr.dest, old);
                        }
                    }
                }
                Priv(op) => match op {
                    PrivOp::ECALL => {
                        use crate::isa::csr_map::mcause;
                        self.pc = 0x8000_0004; // trap vector for riscv-tests
                        self.csr.write_u32(mcause, 11);
                    }
                    PrivOp::WFI => self.halted = true,
                    PrivOp::MRET => {
                        use crate::isa::csr_map::mepc;
                        self.pc = self.csr.read_u32(mepc);
                    }
                },
            }
        }
        Ok(())
    }
}

impl<BUS: MemoryAccess> DebugInterface for Riscv<BUS> {
    fn set_gpr(&mut self, index: u32, value: u32) {
        self.gpr.write_u32(index, value);
    }

    fn get_gpr(&self, index: u32) -> u32 {
        self.gpr.read_u32(index)
    }

    fn set_pc(&mut self, pc: u32) {
        self.pc = pc;
    }

    fn get_pc(&self) -> u32 {
        self.pc
    }

    fn get_csr(&self, index: u32) -> u32 {
        self.csr.read_u32(index)
    }
}

impl<BUS: MemoryAccess> fmt::Display for Riscv<BUS> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            r"
pc: {:08x}
general purpose register:{}
",
            self.pc, self.gpr
        )
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use peripherals::memory::Memory;
    use peripherals::mmio::Mmio;

    #[test]
    fn stop_at_wfi() {
        let program = vec![0x73, 0x00, 0x50, 0x10];
        let dram = Memory::new_with_filled_ram(&program, program.len());
        let mut mmio = Mmio::empty();
        mmio.add((0, program.len()), Box::new(dram)).unwrap();
        let mut riscv = Riscv::fabricate(mmio, DebugMode::Disabled);
        riscv.init();

        let result = riscv.run();
        assert!(result.is_ok());
    }
}
