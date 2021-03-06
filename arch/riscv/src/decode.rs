//! Decode stage.
mod operand_fetch;

use self::operand_fetch::OperandFetch;
use crate::gpr::Gpr;
use crate::isa::instr_format::*;
use crate::isa::opcode::{AluOp, BranchType, CsrOp, LoadStoreType, Opcode, PrivOp};
use bit_field::BitField;
use num::FromPrimitive;

use std::result;
type Result<T> = result::Result<T, DecodeError>;

/// Exceptions occur in decode stage.
#[derive(Debug, Fail, PartialEq)]
pub enum DecodeError {
    #[fail(display = "undefined opcode: 0b{:07b}", opcode)]
    UndefinedInstr { opcode: u32 },

    #[fail(display = "undefined funct3: 0b{:03b}", funct3)]
    UndefinedFunct3 { funct3: u32 },
}

#[derive(Debug, PartialEq)]
pub enum DecodedInstr {
    System { op: PrivOp, npc: u32 },
    Csr(CsrInstr),
    Alu(AluInstr),
    Br(BrInstr),
    Lsu(LsuInstr),
}

/// Decoded format for instructions executed in ALU.
#[derive(Debug, PartialEq)]
pub struct CsrInstr {
    pub op: CsrOp,
    pub dest: u32,
    pub src: u32,
    pub csr_addr: u32,
    pub next_pc: u32,
}

impl CsrInstr {
    // Create CsrInstr from InstrFormat.
    fn from(op: CsrOp, rs1_as_imm: bool, instr: &ITypeInstr, gpr: &Gpr, npc: u32) -> CsrInstr {
        CsrInstr {
            op,
            dest: instr.rd(),
            src: if rs1_as_imm {
                instr.rs1()
            } else {
                gpr.read_u32(instr.rs1())
            },
            csr_addr: instr.imm12() as u32,
            next_pc: npc,
        }
    }
}

/// Decoded format for instructions executed in ALU.
#[derive(Debug, PartialEq)]
pub struct AluInstr {
    pub alu_opcode: AluOp,
    pub dest: u32,
    pub src1: u32,
    pub src2: u32,
    pub next_pc: u32,
}

impl AluInstr {
    // Create AluInstr from InstrFormat.
    fn from<T: OperandFetch>(op: AluOp, use_imm: bool, instr: &T, gpr: &Gpr, npc: u32) -> AluInstr {
        AluInstr {
            alu_opcode: op,
            dest: instr.rd(),
            src1: instr.rs1(&gpr),
            src2: if use_imm {
                instr.imm()
            } else {
                instr.rs2(&gpr)
            },
            next_pc: npc,
        }
    }
}

struct AluInstrBuilder<'a, T: OperandFetch> {
    use_imm: bool,
    instr: &'a T,
    gpr: &'a Gpr,
    npc: u32,
}

impl<'a, T: OperandFetch> AluInstrBuilder<'a, T> {
    fn new(use_imm: bool, instr: &'a T, gpr: &'a Gpr, npc: u32) -> Self {
        AluInstrBuilder {
            use_imm,
            instr,
            gpr,
            npc,
        }
    }

    fn build_instr(&self, op: AluOp) -> AluInstr {
        AluInstr::from(op, self.use_imm, self.instr, &self.gpr, self.npc)
    }
}

/// Decoded format for instructions executed in Branch unit.
#[derive(Debug, PartialEq)]
pub struct BrInstr {
    pub op: BranchType,
    pub dest: u32,
    pub src1: u32,
    pub src2: u32,
    pub base: u32,
    pub offset: u32,
    pub next_pc: u32,
}

impl BrInstr {
    fn from<T: OperandFetch>(op: BranchType, instr: &T, gpr: &Gpr, pc: u32, npc: u32) -> BrInstr {
        BrInstr {
            op,
            dest: instr.rd(),
            src1: instr.rs1(&gpr),
            src2: instr.rs2(&gpr),
            base: pc,
            offset: instr.imm(),
            next_pc: npc,
        }
    }
}

/// Decoded format for instructions going to LSU.
/// load/store instructions generate their address in ALU.
/// So that, it also has `AluInstr`.
#[derive(Debug, PartialEq)]
pub struct LsuInstr {
    pub op: LoadStoreType,
    pub dest: u32,
    pub base: u32,
    pub src: u32,
    pub offset: u32,
    pub next_pc: u32,
}

impl LsuInstr {
    pub fn from<T: OperandFetch>(op: LoadStoreType, instr: &T, gpr: &Gpr, npc: u32) -> LsuInstr {
        LsuInstr {
            op,
            dest: instr.rd(),
            base: instr.rs1(&gpr),
            src: instr.rs2(&gpr),
            offset: instr.imm(),
            next_pc: npc,
        }
    }
}

/// Decode an instruction.
/// There are two sub-stage in the decode.
///   - Decode an instruction according to opcode.
///   - Prepare operand either reading GPR or zero/sign extending the immediate.
pub fn decode(instr: u32, gpr: &Gpr, pc: u32, npc: u32) -> Result<DecodedInstr> {
    let opcode = get_opcode(instr)?;
    use self::DecodedInstr::*;
    use self::Opcode::*;
    match opcode {
        Load => Ok(Lsu(decode_load(ITypeInstr(instr), &gpr, npc)?)),
        Store => Ok(Lsu(decode_store(STypeInstr(instr), &gpr, npc)?)),
        MiscMem => Ok(Alu(decode_as_nop(npc).unwrap())),
        OpImm => Ok(Alu(decode_op_imm(ITypeInstr(instr), &gpr, npc)?)),
        Auipc => Ok(Alu(decode_auipc(UTypeInstr(instr), pc, npc)?)),
        Op => Ok(Alu(decode_op(RTypeInstr(instr), &gpr, npc)?)),
        Lui => Ok(Alu(decode_lui(UTypeInstr(instr), &gpr, npc)?)),
        Jalr => Ok(Br(decode_jalr(ITypeInstr(instr), &gpr, pc, npc)?)),
        Jal => Ok(Br(decode_jal(JTypeInstr(instr), &gpr, pc, npc)?)),
        Branch => Ok(Br(decode_branch(BTypeInstr(instr), &gpr, pc, npc)?)),
        OpSystem => decode_system(ITypeInstr(instr), &gpr, npc),
    }
}

// get opcode
fn get_opcode(instr: u32) -> Result<Opcode> {
    let opcode = instr.get_bits(0..7);
    Opcode::from_u32(opcode).ok_or(DecodeError::UndefinedInstr { opcode })
}

// decode OP-IMM
fn decode_op_imm(instr: ITypeInstr, gpr: &Gpr, npc: u32) -> Result<AluInstr> {
    use crate::isa::funct::Rv32iOpImmFunct3::{self, *};
    let funct3 =
        Rv32iOpImmFunct3::from_u32(instr.funct3()).ok_or(DecodeError::UndefinedFunct3 {
            funct3: instr.funct3(),
        })?;
    let builder = AluInstrBuilder::new(true, &instr, &gpr, npc);
    let decoded = match funct3 {
        ADDI => builder.build_instr(AluOp::ADD),
        SLLI => builder.build_instr(AluOp::SLL),
        ORI => builder.build_instr(AluOp::OR),
        SLTI => builder.build_instr(AluOp::SLT),
        SLTIU => builder.build_instr(AluOp::SLTU),
        ANDI => builder.build_instr(AluOp::AND),
        XORI => builder.build_instr(AluOp::XOR),
        SRxI => {
            if instr.funct7() == 0b010_0000 {
                builder.build_instr(AluOp::SRA)
            } else {
                builder.build_instr(AluOp::SRL)
            }
        }
    };
    Ok(decoded)
}

// decode OP
fn decode_op(instr: RTypeInstr, gpr: &Gpr, npc: u32) -> Result<AluInstr> {
    use crate::isa::funct::Rv32iOpFunct3::{self, *};
    let funct3 = Rv32iOpFunct3::from_u32(instr.funct3()).ok_or(DecodeError::UndefinedFunct3 {
        funct3: instr.funct3(),
    })?;
    let builder = AluInstrBuilder::new(false, &instr, &gpr, npc);
    let decoded = match funct3 {
        ADD => {
            if instr.funct7() == 0b010_0000 {
                builder.build_instr(AluOp::SUB)
            } else {
                builder.build_instr(AluOp::ADD)
            }
        }
        SLT => builder.build_instr(AluOp::SLT),
        SLTU => builder.build_instr(AluOp::SLTU),
        AND => builder.build_instr(AluOp::AND),
        OR => builder.build_instr(AluOp::OR),
        XOR => builder.build_instr(AluOp::XOR),
        SLL => builder.build_instr(AluOp::SLL),
        SRx => {
            if instr.funct7() == 0b010_0000 {
                builder.build_instr(AluOp::SRA)
            } else {
                builder.build_instr(AluOp::SRL)
            }
        }
    };
    Ok(decoded)
}

// decode LUI
fn decode_lui(instr: UTypeInstr, gpr: &Gpr, npc: u32) -> Result<AluInstr> {
    Ok(AluInstr::from(AluOp::LUI, true, &instr, &gpr, npc))
}

// decode AUIPC
fn decode_auipc(instr: UTypeInstr, pc: u32, npc: u32) -> Result<AluInstr> {
    Ok(AluInstr {
        alu_opcode: AluOp::AUIPC,
        dest: instr.rd(),
        src1: pc,
        src2: instr.imm() as u32,
        next_pc: npc,
    })
}

// decode JALR
fn decode_jalr(instr: ITypeInstr, gpr: &Gpr, pc: u32, npc: u32) -> Result<BrInstr> {
    Ok(BrInstr::from(BranchType::JALR, &instr, &gpr, pc, npc))
}

// decode JAL
fn decode_jal(instr: JTypeInstr, gpr: &Gpr, pc: u32, npc: u32) -> Result<BrInstr> {
    Ok(BrInstr::from(BranchType::JAL, &instr, &gpr, pc, npc))
}

// decode BRANCH
fn decode_branch(instr: BTypeInstr, gpr: &Gpr, pc: u32, npc: u32) -> Result<BrInstr> {
    use self::BranchType::*;
    use crate::isa::funct::Rv32iBranchFunct3::{self, *};
    let funct3 =
        Rv32iBranchFunct3::from_u32(instr.funct3()).ok_or(DecodeError::UndefinedFunct3 {
            funct3: instr.funct3(),
        })?;

    let decoded = match funct3 {
        BEQ => BrInstr::from(COND_EQ, &instr, &gpr, pc, npc),
        BNE => BrInstr::from(COND_NE, &instr, &gpr, pc, npc),
        BLT => BrInstr::from(COND_LT, &instr, &gpr, pc, npc),
        BLTU => BrInstr::from(COND_LTU, &instr, &gpr, pc, npc),
        BGE => BrInstr::from(COND_GE, &instr, &gpr, pc, npc),
        BGEU => BrInstr::from(COND_GEU, &instr, &gpr, pc, npc),
    };
    Ok(decoded)
}

// decode LOAD
fn decode_load(instr: ITypeInstr, gpr: &Gpr, npc: u32) -> Result<LsuInstr> {
    use crate::isa::funct::Rv32iLoadFunct3::{self, *};
    let funct3 = Rv32iLoadFunct3::from_u32(instr.funct3()).ok_or(DecodeError::UndefinedFunct3 {
        funct3: instr.funct3(),
    })?;
    let decoded = match funct3 {
        LW => LsuInstr::from(LoadStoreType::LW, &instr, &gpr, npc),
        LH => LsuInstr::from(LoadStoreType::LH, &instr, &gpr, npc),
        LHU => LsuInstr::from(LoadStoreType::LHU, &instr, &gpr, npc),
        LB => LsuInstr::from(LoadStoreType::LB, &instr, &gpr, npc),
        LBU => LsuInstr::from(LoadStoreType::LBU, &instr, &gpr, npc),
    };
    Ok(decoded)
}

// decode STORE
fn decode_store(instr: STypeInstr, gpr: &Gpr, npc: u32) -> Result<LsuInstr> {
    use crate::isa::funct::Rv32iStoreFunct3::{self, *};
    let funct3 =
        Rv32iStoreFunct3::from_u32(instr.funct3()).ok_or(DecodeError::UndefinedFunct3 {
            funct3: instr.funct3(),
        })?;
    let decoded = match funct3 {
        SW => LsuInstr::from(LoadStoreType::SW, &instr, &gpr, npc),
        SH => LsuInstr::from(LoadStoreType::SH, &instr, &gpr, npc),
        SB => LsuInstr::from(LoadStoreType::SB, &instr, &gpr, npc),
    };
    Ok(decoded)
}

// decode SYSTEM
fn decode_system(instr: ITypeInstr, gpr: &Gpr, npc: u32) -> Result<DecodedInstr> {
    use crate::isa::funct::Rv32iSystemFunct3::{self, *};
    let funct3 =
        Rv32iSystemFunct3::from_u32(instr.funct3()).ok_or(DecodeError::UndefinedFunct3 {
            funct3: instr.funct3(),
        })?;

    let decoded = match funct3 {
        PRIV => match instr.imm12() {
            0b0000_0000_0000 => DecodedInstr::System {
                op: PrivOp::ECALL,
                npc,
            },
            0b0011_0000_0010 => DecodedInstr::System {
                op: PrivOp::MRET,
                npc,
            },
            0b0001_0000_0101 => DecodedInstr::System {
                op: PrivOp::WFI,
                npc,
            },
            _ => unimplemented!(),
        },
        CSRRW => DecodedInstr::Csr(CsrInstr::from(CsrOp::WRITE, false, &instr, gpr, npc)),
        CSRRS => DecodedInstr::Csr(CsrInstr::from(CsrOp::SET, false, &instr, gpr, npc)),
        CSRRC => DecodedInstr::Csr(CsrInstr::from(CsrOp::CLEAR, false, &instr, gpr, npc)),
        CSRRWI => DecodedInstr::Csr(CsrInstr::from(CsrOp::WRITE, true, &instr, gpr, npc)),
    };
    Ok(decoded)
}

// decode as NOP for fence.i
fn decode_as_nop(npc: u32) -> Result<AluInstr> {
    Ok(AluInstr {
        alu_opcode: AluOp::ADD,
        dest: 0,
        src1: 0,
        src2: 0,
        next_pc: npc,
    })
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn decode_undefined_opcode() {
        let gpr = Gpr::new();
        let instr = 0x0000_0007u32; // FLW won't implement for the present.
        let result = decode(instr, &gpr, 0, 4);

        assert_eq!(
            Err(DecodeError::UndefinedInstr { opcode: 0b000_0111 }),
            result
        );
    }
}
