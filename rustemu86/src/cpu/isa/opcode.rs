pub const REX: u8 = 0x40;
pub const REX_WRXB: u8 = 0x4F;

enum_from_primitive! {
  #[derive(Debug, Clone, Copy, PartialEq)]
  pub enum Opcode {
    Add       = 0x01,
    CallRel32 = 0xe8,
    Halt      = 0xf4,
    Inc       = 0xff,
    Invalid   = 0x06,
    JmpRel8   = 0xeb,
    MovToRm   = 0x89,
    MovToReg  = 0x8b,
    MovImm32  = 0xb8,
    MovRmImm32 = 0xc7,
    PushR     = 0x50,
    PopR      = 0x58,
    Ret       = 0xc3,
  }
}
