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

#[derive(Debug, Clone, Copy)]
pub enum DataType {
//    UByte,
//    UWord,
    UDWord,
//    UQWord,
//    SByte,
//    SWord,
//    SDWord,
//    SQWord,
}

#[derive(Debug)]
pub struct MetaInst {
    opcode: OpcodeCompat,
    modrm: bool,
    r: bool,
    imm_type: Option<DataType>,
    disp_type: Option<DataType>,
}

impl MetaInst {
    pub fn from_u8(candidate: u8) -> Option<MetaInst> {
        let opcode = OpcodeCompat::from_u8(candidate)?;
        use self::OpcodeCompat::*;
        use self::DataType::*;
        match opcode {
            Cld => Some(MetaInst { opcode: opcode, modrm: false, r: false, imm_type: None, disp_type: None, }),
            // TODO: imm_type should be none.
            Lea => Some(MetaInst { opcode: opcode, modrm: true, r: false, imm_type: Some(UDWord), disp_type: Some(UDWord), }),
            MovRmSreg => Some(MetaInst { opcode: opcode, modrm: true, r: false, imm_type: None, disp_type: None, }),
            Xor => Some(MetaInst { opcode: opcode, modrm: true, r: false, imm_type: None, disp_type: None, }),
            Hlt => Some(MetaInst { opcode: opcode, modrm: false, r: false, imm_type: None, disp_type: None, }),
            MovOi => None,
        }
    }

    pub fn plus_r_from_u8(candidate: u8) -> Option<MetaInst> {
        let opcode = OpcodeCompat::from_u8(candidate & 0xf8)?;
        use self::OpcodeCompat::*;
        use self::DataType::*;
        match opcode {
            MovOi => Some(MetaInst { opcode: opcode, modrm: false, r: true, imm_type: Some(UDWord), disp_type: None, }),
            _ => None,
        }
    }

    pub fn get_opcode(&self) -> OpcodeCompat {
        self.opcode
    }

    pub fn use_modrm(&self) -> bool {
        self.modrm
    }

    pub fn use_r(&self) -> bool {
        self.r
    }

    pub fn get_imm_type(&self) -> Option<DataType> {
        self.imm_type
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