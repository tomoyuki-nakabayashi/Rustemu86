use compatible::{Result, CompatibleException};
use compatible::isa::opcode::{self, OpcodeCompat};
use compatible::isa::modrm::ModRm;
use num::FromPrimitive;

pub(crate) struct FetchedInst {
    opcode: OpcodeCompat,
    inst_bytes: u64,
}

impl FetchedInst {
    pub(crate) fn get_opcode(&self) -> OpcodeCompat {
        self.opcode
    }

    pub(super) fn increment_ip(&self, ip: u64) -> u64 {
        ip + self.inst_bytes
    }
}

pub(super) fn fetch(program: &[u8]) -> Result<FetchedInst> {
    let inst = FetchedInstBuilder::new(program)
        .parse_opcode()?
        .parse_modrm()
        .build();
    Ok(inst)
}

struct FetchedInstBuilder<'a> {
    opcode: OpcodeCompat,
    modrm: Option<ModRm>,
    program: &'a [u8],
    current_offset: usize,
}

impl<'a> FetchedInstBuilder<'a> {
    fn new(program: &[u8]) -> FetchedInstBuilder {
        FetchedInstBuilder {
            opcode: OpcodeCompat::Hlt,
            modrm: None,
            program: program,
            current_offset: 0,
        }
    }

    fn parse_opcode(&mut self) -> Result<&mut FetchedInstBuilder<'a>> {
        let candidate = self.program[self.current_offset];
        if let Some(opcode) = OpcodeCompat::from_u8(candidate) {
            self.opcode = opcode;
            self.current_offset += 1;
        } else {
            return Err(CompatibleException(format!("Undefined opcode '{}'.", candidate)))
        }
        Ok(self)
    }

    fn parse_modrm(&mut self) -> &mut FetchedInstBuilder<'a> {
        let candidate = self.program[self.current_offset];
        if opcode::inst_use_modrm(self.opcode) {
            self.modrm = Some(ModRm::new(candidate));
            self.current_offset += 1;
        }
        self
    }

    fn build(&self) -> FetchedInst {
        FetchedInst {
            opcode: self.opcode,
            inst_bytes: self.current_offset as u64,
        }
    }
}