pub const ADD: u8 = 0x01;
pub const MOV_RAX: u8 = 0xb8;
pub const MOV_DI: u8 = 0xbf;
pub const JMP_REL8: u8 = 0xeb;
pub const INC: u8 = 0xff;

pub const REX_W: u8 = 0x48;

enum_from_primitive! {
  #[derive(Debug, Clone, Copy, PartialEq)]
  pub enum Opcode {
    Add = 0x01,
    MovImm32 = 0xb8,
    JmpRel8 = 0xeb,
    Inc = 0xff,
  }
}
