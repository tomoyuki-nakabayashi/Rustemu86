//! Instruction format translator.

use crate::gpr::Gpr;
use crate::isa::instr_format::*;

// TODO: rename like
// dest -> rd
// operand1 -> rs1
// operand2 -> rs2
// operand3 -> imm

pub trait OperandFetch {
    fn dest(&self) -> u32;
    fn operand1(&self, gpr: &Gpr) -> u32;
    fn operand2(&self, gpr: &Gpr) -> u32;
    fn operand3(&self) -> u32;
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
    fn operand3(&self) -> u32 {
        panic!("Never call");
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
    fn operand3(&self) -> u32 {
        panic!("Never call");
    }
}

impl OperandFetch for STypeInstr {
    fn dest(&self) -> u32 {
        // ignore
        0
    }
    fn operand1(&self, gpr: &Gpr) -> u32 {
        gpr.read_u32(self.rs1())
    }
    fn operand2(&self, gpr: &Gpr) -> u32 {
        gpr.read_u32(self.rs2())
    }
    fn operand3(&self) -> u32 {
        self.offset_11_0() as u32
    }
}

impl OperandFetch for BTypeInstr {
    fn dest(&self) -> u32 {
        // ignore
        0
    }
    fn operand1(&self, gpr: &Gpr) -> u32 {
        gpr.read_u32(self.rs1())
    }
    fn operand2(&self, gpr: &Gpr) -> u32 {
        gpr.read_u32(self.rs2())
    }
    fn operand3(&self) -> u32 {
        self.offset_12_1() as u32
    }
}

impl OperandFetch for JTypeInstr {
    fn dest(&self) -> u32 {
        self.rd()
    }
    fn operand1(&self, _gpr: &Gpr) -> u32 {
        self.offset_20_1() as u32
    }
    fn operand2(&self, _gpr: &Gpr) -> u32 {
        panic!("Never call");
    }
    fn operand3(&self) -> u32 {
        panic!("Never call");
    }
}

