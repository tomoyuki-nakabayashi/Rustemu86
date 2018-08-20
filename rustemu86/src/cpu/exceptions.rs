use cpu::isa::opcode::Opcode;

#[derive(Debug, Fail)]
pub enum InternalException {
  #[fail(display = "Fetch error, unknown opcode {}", opcode)]
  FetchError{
    opcode: u8,
  },
  #[fail(display = "undefined instruction: {:?}", opcode)]
  UndefinedInstruction {
    opcode: Opcode,
  },
}