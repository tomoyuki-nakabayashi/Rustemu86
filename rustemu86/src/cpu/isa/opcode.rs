pub const REX: u8 = 0x40;
pub const REX_WRXB: u8 = 0x4F;

enum_from_primitive! {
  #[derive(Debug, Clone, Copy, PartialEq)]
  pub enum Opcode {
    Add = 0x01,
    Invalid = 0x06,
    MovToRm = 0x89,
    MovToReg = 0x8b,
    MovImm32 = 0xb8,
    JmpRel8 = 0xeb,
    Inc = 0xff,
  }
}
