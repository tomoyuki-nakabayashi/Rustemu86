pub const REX: u8 = 0x40;
pub const REX_WRXB: u8 = 0x4F;

enum_from_primitive! {
  #[derive(Debug, Clone, Copy, PartialEq)]
  pub enum Opcode {
    Add      = 0x01,
    Invalid  = 0x06,
    PushR    = 0x50,
    MovToRm  = 0x89,
    MovToReg = 0x8b,
    MovImm32 = 0xb8,
    JmpRel8  = 0xeb,
    Halt     = 0xf4,
    Inc      = 0xff,
  }
}
