use x86_64::isa::modrm::ModRm;

pub const REX: u8 = 0x40;
pub const REX_WRXB: u8 = 0x4F;
pub const OVERRIDE_OP_SIZE: u8 = 0x66;

enum_from_primitive! {
  #[derive(Debug, Clone, Copy, PartialEq)]
  pub enum Opcode {
    Add       = 0x01,
    CallRel32 = 0xe8,
    Halt      = 0xf4,
    Inc       = 0xff,
    Invalid   = 0x06,
    JmpRel8   = 0xeb,
    // Operand encoding: MR
    MovToRm   = 0x89,
    // Operand encoding: RM
    MovToReg  = 0x8b,
    // Operand encoding: OI
    MovImm    = 0xb8,
    // Operand encoding: MI
    MovRmImm8 = 0xc6,
    MovRmImm  = 0xc7,
    PushR     = 0x50,
    PopR      = 0x58,
    Ret       = 0xc3,
  }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum OperandSize {
    Byte,
    Word,
    DoubleWord,
    QuadWord,
}

impl Opcode {
    pub fn modrm_if_required(&self, candidate: u8) -> Option<ModRm> {
        match self {
            Opcode::Add
            | Opcode::Inc
            | Opcode::MovToRm
            | Opcode::MovToReg
            | Opcode::MovRmImm
            | Opcode::MovRmImm8 => return Some(ModRm::new(candidate)),
            _ => return None,
        }
    }
}
