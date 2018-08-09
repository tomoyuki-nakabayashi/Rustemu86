use cpu::isa::opcode::Opcode;

#[derive(Debug, Fail)]
pub enum InternalException {
  #[fail(display = "fetch error")]
  FetchError{},
  #[fail(display = "undefined instruction: {:?}", opcode)]
  UndefinedInstruction {
    opcode: Opcode,
  },
}