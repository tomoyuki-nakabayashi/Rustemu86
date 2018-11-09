//! Fetch unit for x86 real mode.

use bit_field::BitField;
use byteorder::{LittleEndian, ReadBytesExt};
use targets::x86::{Result, CompatibleException};
use targets::x86::isa::opcode::{self, OpcodeCompat, MetaInst};
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

/// Fetch an instruction.
/// program must be long enough to parse an x86 instruction.
pub(super) fn fetch(program: &[u8]) -> Result<FetchedInst> {
    let inst = FetchedInstBuilder::new(program)
        .parse_legacy_prefix()
        .parse_opcode()?
        .parse_modrm()
        .parse_imm()
        .build();
    Ok(inst)
}

// Builder pattern to build an instruction.
struct FetchedInstBuilder<'a> {
    addr_size_override: bool,
    opcode: OpcodeCompat,
    modrm: Option<ModRm>,
    rd: u8,
    imm: Option<u64>,
    program: &'a [u8],
    current_offset: usize,
    meta_inst: Option<MetaInst>,
}

impl<'a> FetchedInstBuilder<'a> {
    fn new(program: &[u8]) -> FetchedInstBuilder {
        FetchedInstBuilder {
            addr_size_override: false,
            opcode: OpcodeCompat::Hlt,
            modrm: None,
            rd: 0,
            imm: None,
            program: program,
            current_offset: 0,
            meta_inst: None,
        }
    }

    fn parse_legacy_prefix(&mut self) -> &mut FetchedInstBuilder<'a> {
        let candidate = self.peek_u8();
        match candidate {
            opcode::ADDRESS_SIZE_OVERRIDE_PREFIX => {
                self.addr_size_override = true;
                self.current_offset += 1;
            }
            _ => ()
        };

        self
    }

    fn parse_opcode(&mut self) -> Result<&mut FetchedInstBuilder<'a>> {
        let candidate = self.peek_u8();
        self.meta_inst = MetaInst::from_u8(candidate)
            .or_else(|| MetaInst::from_u8(candidate & 0xf8));

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
        let candidate = self.peek_u8();
        // TODO: Remove the unwrap!
        if self.meta_inst.as_ref().unwrap().use_modrm() {
            self.modrm = Some(ModRm::new(candidate));
            self.current_offset += 1;
        }
        self
    }

    fn parse_imm(&mut self) -> &mut FetchedInstBuilder<'a> {
        match self.opcode {
            OpcodeCompat::MovOi => {
                self.read_imm_u16();    
            }
            OpcodeCompat::Lea => {
                if self.addr_size_override {
                    self.read_imm_u32();
                } else {
                    self.read_imm_u16();    
                }
            }
            _ => {}
        }
        self
    }

    // Build the result of the builder.
    fn build(&self) -> FetchedInst {
        FetchedInst {
            opcode: self.opcode,
            modrm: self.modrm,
            rd: self.rd,
            imm: self.imm,
            inst_bytes: self.current_offset as u64,
        }
    }

    // Helper function to peek next one byte.
    fn peek_u8(&self) -> u8 {
        self.program[self.current_offset]
    }

    // Helper function to read u16 to immediate.
    fn read_imm_u16(&mut self) {
        let mut imm = &self.program[self.current_offset..self.current_offset + 2];
        self.imm = Some(imm.read_u16::<LittleEndian>().unwrap().into());
        self.current_offset += 2;
    }

    // Helper function to read u32 to immediate.
    fn read_imm_u32(&mut self) {
        let mut imm = &self.program[self.current_offset..self.current_offset + 4];
        self.imm = Some(imm.read_u32::<LittleEndian>().unwrap().into());
        self.current_offset += 4;
    }
}