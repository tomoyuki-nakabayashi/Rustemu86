//! Instruction format translator.

use crate::gpr::Gpr;
use crate::isa::instr_format::*;

pub trait OperandFetch {
    fn rd(&self) -> u32;
    fn rs1(&self, gpr: &Gpr) -> u32;
    fn rs2(&self, gpr: &Gpr) -> u32;
    fn imm(&self) -> u32;
}

impl OperandFetch for RTypeInstr {
    fn rd(&self) -> u32 {
        self.rd()
    }
    fn rs1(&self, gpr: &Gpr) -> u32 {
        gpr.read_u32(self.rs1())
    }
    fn rs2(&self, gpr: &Gpr) -> u32 {
        gpr.read_u32(self.rs2())
    }
    fn imm(&self) -> u32 {
        panic!("Never call");
    }
}

impl OperandFetch for ITypeInstr {
    fn rd(&self) -> u32 {
        self.rd()
    }
    fn rs1(&self, gpr: &Gpr) -> u32 {
        gpr.read_u32(self.rs1())
    }
    fn rs2(&self, _gpr: &Gpr) -> u32 {
        panic!("Never call");
    }
    fn imm(&self) -> u32 {
        self.imm12() as u32
    }
}

impl OperandFetch for STypeInstr {
    fn rd(&self) -> u32 {
        // ignore
        0
    }
    fn rs1(&self, gpr: &Gpr) -> u32 {
        gpr.read_u32(self.rs1())
    }
    fn rs2(&self, gpr: &Gpr) -> u32 {
        gpr.read_u32(self.rs2())
    }
    fn imm(&self) -> u32 {
        self.offset_11_0() as u32
    }
}

impl OperandFetch for BTypeInstr {
    fn rd(&self) -> u32 {
        // ignore
        0
    }
    fn rs1(&self, gpr: &Gpr) -> u32 {
        gpr.read_u32(self.rs1())
    }
    fn rs2(&self, gpr: &Gpr) -> u32 {
        gpr.read_u32(self.rs2())
    }
    fn imm(&self) -> u32 {
        self.offset_12_1() as u32
    }
}

impl OperandFetch for JTypeInstr {
    fn rd(&self) -> u32 {
        self.rd()
    }
    fn rs1(&self, _gpr: &Gpr) -> u32 {
        panic!("Never call");
    }
    fn rs2(&self, _gpr: &Gpr) -> u32 {
        panic!("Never call");
    }
    fn imm(&self) -> u32 {
        self.offset_20_1() as u32
    }
}
