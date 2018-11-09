use num::FromPrimitive;

pub const OPERAND_SIZE_OVERRIDE_PREFIX: u8 = 0x66;
pub const ADDRESS_SIZE_OVERRIDE_PREFIX: u8 = 0x67;

enum_from_primitive! {
    #[derive(Debug, Clone, Copy, PartialEq)]
    pub enum OpcodeCompat {
        Cld = 0xfc,
        Lea = 0x8d,
        MovRmSreg = 0x8e,
        MovOi = 0xb8,
        Xor = 0x31,
        Hlt = 0xf4,
    }
}

pub fn inst_use_modrm(opcode: OpcodeCompat) -> bool {
    use self::OpcodeCompat::*;
    opcode == Xor || opcode == MovRmSreg || opcode == Lea
}

trait OpcodeFeature {
    fn opcode(&self) -> OpcodeCompat;
    fn use_modrm(&self) -> bool { false }
}

struct ArithRm {
    opcode: OpcodeCompat,
}

impl OpcodeFeature for ArithRm {
    fn opcode(&self) -> OpcodeCompat {
        return self.opcode
    }

    fn use_modrm(&self) -> bool {
        true
    }
}

#[derive(Debug)]
pub enum DataType {
    UByte,
    UWord,
    UDWord,
    UQWord,
    SByte,
    SWord,
    SDWord,
    SQWord,
}

#[derive(Debug)]
pub struct MetaInst {
    opcode: OpcodeCompat,
    modrm: bool,
    imm_type: Option<DataType>,
    disp_type: Option<DataType>,
}

impl MetaInst {
    pub fn from_u8(candidate: u8) -> Option<MetaInst> {
        let opcode = OpcodeCompat::from_u8(candidate)?;
        use self::OpcodeCompat::*;
        use self::DataType::*;
        Some(match opcode {
            Cld => MetaInst { opcode: opcode, modrm: false, imm_type: None, disp_type: None, },
            Lea => MetaInst { opcode: opcode, modrm: true, imm_type: None, disp_type: Some(UQWord), },
            MovRmSreg => MetaInst { opcode: opcode, modrm: true, imm_type: None, disp_type: None, },
            MovOi => MetaInst { opcode: opcode, modrm: false, imm_type: Some(UDWord), disp_type: None, },
            Xor => MetaInst { opcode: opcode, modrm: true, imm_type: None, disp_type: None, },
            Hlt => MetaInst { opcode: opcode, modrm: false, imm_type: None, disp_type: None, },
        })
    }

    pub fn use_modrm(&self) -> bool {
        self.modrm
    }
}

/*
(op, modrm, imm_u16, disp_u16)
=>
enum_from_primitive! {
    pub enum Opcode {
        ...
        ...
    }
}

impl MetaInst {
    fn from_u8() -> Option<MetaInst> {
        match opcode {
            op => Some(op, modrm, imm_u16, disp_u16),
        }
    }
}
*/