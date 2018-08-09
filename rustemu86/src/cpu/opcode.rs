pub const REX: u8 = 0x40;
pub const REX_WRXB: u8 = 0x4F;

enum_from_primitive! {
  #[derive(Debug, Clone, Copy, PartialEq)]
  pub enum Opcode {
    Add = 0x01,
    Invalid = 0x06,
    MovImm32 = 0xb8,
    JmpRel8 = 0xeb,
    Inc = 0xff,
  }
}
