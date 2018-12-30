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
    R_FORMAT(RTypeInstr),
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

/// R type format:
/// funct7 | rs2 | rs1 | funct3 | rd | opcode
/// OP /
bitfield! {
    #[derive(Clone, Copy, Debug)]
    pub struct RTypeInstr(u32);
    u32;
    pub funct7, _: 31, 25;
    pub rs2, _: 24, 20;
    pub rs1, _: 19, 15;
    pub funct3, _: 14, 12;
    pub rd, _: 11, 7;
    pub opcode, _: 6, 0;
}

/// I type format:
/// imm[11:0] | rs1 | funct3 | rd | opcode
/// OP-IMM /
bitfield! {
    #[derive(Clone, Copy, Debug, PartialEq)]
    pub struct ITypeInstr(u32);
    i32;
    pub imm12, _: 31, 20;
    u32;
    pub rs1, _: 19, 15;
    pub funct3, _: 14, 12;
    pub rd, _: 11, 7;
    pub opcode, _: 6, 0;
}

/// B type format:
/// imm[12] | imm[10:5] | rs2 | rs1 | funct3 | imm[4:1] | imm[11] | opcode
/// BRANCH
bitfield! {
    #[derive(Clone, Copy, Debug, PartialEq)]
    pub struct BTypeInstr(u32);
    u32;
    imm12, _: 31;
    imm10_5, _: 30, 25;
    pub rs2, _: 24, 20;
    pub rs1, _: 19, 15;
    pub funct3, _: 14, 12;
    imm4_1, _: 11, 8;
    imm11, _: 7;
    u32;
    pub opcode, _: 6, 0;
}

impl BTypeInstr {
    pub fn offset_12_1(self) -> i32 {
        let imm12 = (self.imm12() as u32) << (12 - 1);
        let imm11 = (self.imm11() as u32) << (11 - 1);
        let imm10_5 = self.imm10_5() << (5 - 1);
        let imm4_1 = self.imm4_1() << 1;
        let imm12_0 = imm12 | imm11 | imm10_5 | imm4_1;

        sign_extend_at(imm12_0, 12) as i32
    }
}

/// J type format:
/// imm[20] | imm[10:1] | imm[11] | imm[19:12] | rd | opcode
/// JAL
bitfield! {
    #[derive(Clone, Copy, Debug, PartialEq)]
    pub struct JTypeInstr(u32);
    u32;
    imm20, _: 31;
    imm10_1, _: 30, 21;
    imm11, _: 20;
    imm19_12, _: 19, 12;
    u32;
    pub rd, _: 11, 7;
    pub opcode, _: 6, 0;
}

impl JTypeInstr {
    pub fn offset_20_1(self) -> i32 {
        let imm20 = (self.imm20() as u32) << (20 - 1);
        let imm19_12 = self.imm19_12() << (12 - 1);
        let imm11 = (self.imm11() as u32) << (11 - 1);
        let imm10_1 = self.imm10_1() << 1;
        let imm20_0 = imm20 | imm19_12 | imm11 | imm10_1;

        sign_extend_at(imm20_0, 20) as i32
    }
}

// helper function for sign extension
// Assumption: bits in `n` above position `sign_bit_pos` are already zero.
#[inline(always)]
fn sign_extend_at(n: u32, sign_bit_pos: u32) -> u32 {
    let sign_bit_mask = 1u32 << sign_bit_pos - 1;
    (n ^ sign_bit_mask).wrapping_sub(sign_bit_mask)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_bitfield() {
        let instr = ITypeInstr(0x0010_8093);
        assert_eq!(1, instr.rd());
    }

    #[test]
    fn test_offset_20_1() {
        // positive offset.
        let instr = JTypeInstr(0x008000ef);
        let offset = instr.offset_20_1();
        assert_eq!(8, offset);

        // negative offset.
        let instr = JTypeInstr(0xffdff0ef);
        let offset = instr.offset_20_1();
        assert_eq!(-4, offset);
    }
}
