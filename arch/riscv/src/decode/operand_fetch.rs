//! Instruction format translator.

use crate::gpr::Gpr;
use crate::isa::instr_format::{ITypeInstr, RTypeInstr, UTypeInstr};

pub trait OperandFetch {
    fn dest(&self) -> u32;
    fn operand1(&self, gpr: &Gpr) -> u32;
    fn operand2(&self, gpr: &Gpr) -> u32;
}

impl OperandFetch for RTypeInstr {
    fn dest(&self) -> u32 {
        self.rd()
    }
    fn operand1(&self, gpr: &Gpr) -> u32 {
        gpr.read_u32(self.rs1())
    }
    fn operand2(&self, gpr: &Gpr) -> u32 {
        gpr.read_u32(self.rs2())
    }
}

impl OperandFetch for ITypeInstr {
    fn dest(&self) -> u32 {
        self.rd()
    }
    fn operand1(&self, gpr: &Gpr) -> u32 {
        gpr.read_u32(self.rs1())
    }
    fn operand2(&self, _gpr: &Gpr) -> u32 {
        self.imm12() as u32
    }
}

impl OperandFetch for UTypeInstr {
    fn dest(&self) -> u32 {
        self.rd()
    }
    fn operand1(&self, _gpr: &Gpr) -> u32 {
        unimplemented!()
    }
    // Something wrong
    fn operand2(&self, _gpr: &Gpr) -> u32 {
        self.offset_20_1() as u32
    }
}
