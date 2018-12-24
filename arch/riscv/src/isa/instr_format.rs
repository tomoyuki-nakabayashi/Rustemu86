//! Instruction format for rv32ic.

use bitfield::bitfield;

/// Instruction format is either `Base` (32-bit) or `Compressed` (16-bit).
pub enum InstrFormat {
    Base(BaseInstrFormat),
    Compressed(CompressedInstrFormat),
}

/// `Base` (32-bit) format for rv32i.
#[allow(dead_code, non_camel_case_types)]
pub enum BaseInstrFormat {
    R_FORMAT(RTypeInstrFormat),
    I_FORMAT,
    S_FORMAT,
    B_FORMAT,
    U_FORMAT,
    J_FORMAT,
}

/// `Compressed` (16-bit) format for rv32ic.
#[allow(dead_code, non_camel_case_types)]
pub enum CompressedInstrFormat {
    CR_FORMAT,
    CI_FORMAT,
    CSS_FORMAT,
    CIW_FORMAT,
    CL_FORMAT,
    CS_FORMAT,
    CB_FORMAT,
    CJ_FORMAT,
}

/// R type format. dst = rs1 op rs2
bitfield! {
    #[derive(Clone, Copy, Debug)]
    pub struct RTypeInstrFormat(u32);
    funct7, _: 25, 31;
    rs2, _: 20, 24;
    rs1, _: 15, 19;
    funct3, _: 12, 14;
    rd, _: 7, 11;
    opcode, _: 6, 0;
}

/// I type format. dst = rs1 op imm[11:0]
/// ADDI / SLTI[U] / ANDI / ORI / XORI
bitfield! {
    #[derive(Clone, Copy, Debug)]
    pub struct ITypeInstrFormat(u32);
    imm12, _: 20, 31;
    rs1, _: 15, 19;
    funct3, _: 12, 14;
    rd, _: 7, 11;
    opcode, _: 6, 0;
}
