//! Fetch unit for x86 real mode.

use crate::gpr::Reg32;
use crate::isa::modrm::ModRm;
use crate::isa::opcode::{self, DataType, MetaInst, Opcode};
use crate::Result;
use bit_field::BitField;
use byteorder::{LittleEndian, ReadBytesExt};
use num::FromPrimitive;

pub struct FetchedInst {
    opcode: Opcode,
    modrm: Option<ModRm>,
    rd: u8,
    imm: Option<u64>,
    disp: Option<u64>,
    inst_bytes: u64,
}

impl FetchedInst {
    pub(crate) fn get_opcode(&self) -> Opcode {
        self.opcode
    }

    pub(super) fn increment_ip(&self, ip: u64) -> u64 {
        ip + self.inst_bytes
    }

    pub(crate) fn get_modrm(&self) -> ModRm {
        self.modrm.expect("Mod RM filed was not fetched.")
    }

    pub(crate) fn get_rd(&self) -> Reg32 {
        Reg32::from_u8(self.rd).expect("rd field was not fetched.")
    }

    pub(crate) fn get_imm(&self) -> u64 {
        self.imm.expect("Immediate filed was not fetched.")
    }

    pub(crate) fn get_disp(&self) -> u64 {
        self.disp.expect("Displacement filed was not fetched.")
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
        .parse_disp()
        .build();
    Ok(inst)
}

// Builder pattern to build an instruction.
struct FetchedInstBuilder<'a> {
    addr_size_override: bool,
    opcode: Opcode,
    modrm: Option<ModRm>,
    rd: u8,
    imm: Option<u64>,
    disp: Option<u64>,
    program: &'a [u8],
    current_offset: usize,
    meta_inst: MetaInst,
}

impl<'a> FetchedInstBuilder<'a> {
    fn new(program: &[u8]) -> FetchedInstBuilder {
        FetchedInstBuilder {
            addr_size_override: false,
            opcode: Opcode::default(),
            modrm: None,
            rd: 0,
            imm: None,
            disp: None,
            program,
            current_offset: 0,
            meta_inst: MetaInst::default(),
        }
    }

    fn parse_legacy_prefix(&mut self) -> &mut FetchedInstBuilder<'a> {
        let candidate = self.peek_u8();
        match candidate {
            opcode::ADDRESS_SIZE_OVERRIDE_PREFIX => {
                self.addr_size_override = true;
                self.consume_bytes(1);
            }
            _ => (),
        };

        self
    }

    fn parse_opcode(&mut self) -> Result<&mut FetchedInstBuilder<'a>> {
        let candidate = self.peek_u8();
        self.meta_inst = MetaInst::from(candidate)?;

        self.opcode = self.meta_inst.get_opcode();
        if self.meta_inst.use_r() {
            self.rd = candidate.get_bits(0..3);
        }
        self.consume_bytes(1);
        Ok(self)
    }

    fn parse_modrm(&mut self) -> &mut FetchedInstBuilder<'a> {
        let candidate = self.peek_u8();
        if self.meta_inst.use_modrm() {
            self.modrm = Some(ModRm::new(candidate));
            self.consume_bytes(1);
        }
        self
    }

    fn parse_imm(&mut self) -> &mut FetchedInstBuilder<'a> {
        match self.meta_inst.get_imm_type() {
            None => (),
            Some(DataType::UWord) => {
                if self.addr_size_override {
                    self.read_imm_u32();
                } else {
                    self.read_imm_u16();
                }
            }
        }
        self
    }

    fn parse_disp(&mut self) -> &mut FetchedInstBuilder<'a> {
        match self.meta_inst.get_disp_type() {
            None => (),
            Some(DataType::UWord) => {
                if self.addr_size_override {
                    self.read_disp_u32();
                } else {
                    self.read_disp_u16();
                }
            }
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
            disp: self.disp,
            inst_bytes: self.current_offset as u64,
        }
    }

    // Helper function to peek next one byte.
    fn peek_u8(&self) -> u8 {
        self.program[self.current_offset]
    }

    // Helper function to consume `num` of bytes from `self.program`.
    fn consume_bytes(&mut self, num: usize) {
        self.current_offset += num;
    }

    // Helper function to read u16 to immediate.
    fn read_imm_u16(&mut self) {
        let mut imm = &self.program[self.current_offset..self.current_offset + 2];
        self.imm = Some(imm.read_u16::<LittleEndian>().unwrap().into());
        self.consume_bytes(2);
    }

    // Helper function to read u32 to immediate.
    fn read_imm_u32(&mut self) {
        let mut imm = &self.program[self.current_offset..self.current_offset + 4];
        self.imm = Some(imm.read_u32::<LittleEndian>().unwrap().into());
        self.consume_bytes(4);
    }

    // Helper function to read u16 to displacement.
    fn read_disp_u16(&mut self) {
        let mut disp = &self.program[self.current_offset..self.current_offset + 2];
        self.disp = Some(disp.read_u16::<LittleEndian>().unwrap().into());
        self.consume_bytes(2);
    }

    // Helper function to read u32 to displacement.
    fn read_disp_u32(&mut self) {
        let mut disp = &self.program[self.current_offset..self.current_offset + 4];
        self.disp = Some(disp.read_u32::<LittleEndian>().unwrap().into());
        self.consume_bytes(4);
    }
}
