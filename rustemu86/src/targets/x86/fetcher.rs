use bit_field::BitField;
use byteorder::{LittleEndian, ReadBytesExt};
use targets::x86::{Result, CompatibleException};
use targets::x86::isa::opcode::{self, OpcodeCompat};
use targets::x86::isa::modrm::ModRm;
use targets::x86::gpr::Reg32;
use num::FromPrimitive;

pub struct FetchedInst {
    opcode: OpcodeCompat,
    modrm: Option<ModRm>,
    rd: u8,
    imm: Option<u64>,
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

    pub(crate) fn get_rd(&self) -> Reg32 {
        Reg32::from_u8(self.rd).expect("rd field was not fetched.")
    }

    pub(crate) fn get_imm(&self) -> u64 {
        let imm = self.imm.expect("Immediate filed was not fetched.");
        imm
    }
}

pub(super) fn fetch(program: &[u8]) -> Result<FetchedInst> {
    let inst = FetchedInstBuilder::new(program)
        .parse_opcode()?
        .parse_modrm()
        .parse_imm()
        .build();
    Ok(inst)
}

struct FetchedInstBuilder<'a> {
    opcode: OpcodeCompat,
    modrm: Option<ModRm>,
    rd: u8,
    imm: Option<u64>,
    program: &'a [u8],
    current_offset: usize,
}

impl<'a> FetchedInstBuilder<'a> {
    fn new(program: &[u8]) -> FetchedInstBuilder {
        FetchedInstBuilder {
            opcode: OpcodeCompat::Hlt,
            modrm: None,
            rd: 0,
            imm: None,
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
        self.rd = rd;
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

    fn parse_imm(&mut self) -> &mut FetchedInstBuilder<'a> {
        match self.opcode {
            OpcodeCompat::MovOi => {
                let mut imm = &self.program[self.current_offset..self.current_offset + 2];
                self.imm = Some(imm.read_u16::<LittleEndian>().unwrap().into());
                self.current_offset += 2;
            }
            _ => {}
        }
        self
    }

    fn build(&self) -> FetchedInst {
        FetchedInst {
            opcode: self.opcode,
            modrm: self.modrm,
            rd: self.rd,
            imm: self.imm,
            inst_bytes: self.current_offset as u64,
        }
    }
}