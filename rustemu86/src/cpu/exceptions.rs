use cpu::isa::opcode::Opcode;

#[derive(Debug, Fail)]
pub enum InternalException {
    #[fail(display = "fetcher: Fetch error, unknown opcode {}", opcode)]
    FetchError { opcode: u8 },
    #[fail(display = "decoder: Undefined instruction: {:?}", opcode)]
    UndefinedInstruction { opcode: Opcode },
    #[fail(display = "decoder: ModRM is required but not fetched for {:?}.", opcode)]
    ModRmRequired { opcode: Opcode },
}
