use bit_field::BitField;
use targets::x86::{Result, CompatibleException};
use targets::x86::isa::opcode::{self, OpcodeCompat};
use targets::x86::isa::modrm::ModRm;
use num::FromPrimitive;

pub struct FetchedInst {
    opcode: OpcodeCompat,
    modrm: Option<ModRm>,
    inst_bytes: u64,
}

impl FetchedInst {
    pub(crate) fn get_opcode(&self) -> OpcodeCompat {
        self.opcode
    }

    pub(super) fn increment_ip(&self, ip: u64) -> u64 {
        ip + self.inst_bytes
    }

    pub(crate) fn get_modrm(&self) -> ModRm {
        let modrm = self.modrm.expect("Mod RM filed was not fetched.");
        modrm
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
        let mut rd: u8 = 0;
        {
            let extract_rd = |opcode| {
                rd = candidate.get_bits(0..3);
                Some(opcode)
            };
            let parse_opcode_plus_rd = || OpcodeCompat::from_u8(candidate & 0xf8)
                .and_then(extract_rd);
            self.opcode = OpcodeCompat::from_u8(candidate)
                .or_else(parse_opcode_plus_rd)
                .ok_or(CompatibleException(
                    format!("Encounters undefined opcode: '0x{:x}' in fetch stage.", candidate)))?
        }
        self.current_offset += 1;
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
            modrm: self.modrm,
            inst_bytes: self.current_offset as u64,
        }
    }
}