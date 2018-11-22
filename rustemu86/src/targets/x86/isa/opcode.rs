use num::FromPrimitive;

pub const OPERAND_SIZE_OVERRIDE_PREFIX: u8 = 0x66;
pub const ADDRESS_SIZE_OVERRIDE_PREFIX: u8 = 0x67;

/// Opcode represents x86 opcode.
/// Currently, assume that the opcode is u8, but there are multi-byte opcodes.
///
/// Alphabetically ordered.
enum_from_primitive! {
    #[derive(Debug, Clone, Copy, PartialEq)]
    pub enum Opcode {
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

/// Just a wrapper macro to create MetaInst.
#[macro_use]
macro_rules! meta_inst {
    ( $opcode: expr, $modrm: expr, $r: expr, $imm: expr, $disp: expr) => ({
        Some(MetaInst {
            opcode: $opcode,
            modrm: $modrm,
            r: $r,
            imm_type: $imm,
            disp_type: $disp,
        })
    })
}

/// MetaInst represents meta infomation for an opcode in the opcode field.
///
/// You can obtain a MetaInst from an `u8` which can be translated to a valid opcode.
#[derive(Debug)]
pub struct MetaInst {
    opcode: Opcode,
    modrm: bool,
    r: bool,
    imm_type: Option<DataType>,
    disp_type: Option<DataType>,
}

impl MetaInst {
    /// Instantiate from `u8` except for `plus r` opcode.
    pub fn from_u8(candidate: u8) -> Option<MetaInst> {
        let opcode = Opcode::from_u8(candidate)?;
        use self::DataType::*;
        use self::Opcode::*;
        match opcode {
            Cld => meta_inst!(opcode, false, false, None, None),
            Lea => meta_inst!(opcode, true, false, None, Some(UDWord)),
            MovRmSreg => meta_inst!(opcode, true, false, None, None),
            Xor => meta_inst!(opcode, true, false, None, None),
            Hlt => meta_inst!(opcode, false, false, None, None),
            _ => None,
        }
    }

    /// `plus r` is a special case of u8 opcode.
    /// The u8 contains the destination address in their lower significant 3-bits.
    pub fn plus_r_from_u8(candidate: u8) -> Option<MetaInst> {
        let opcode = Opcode::from_u8(candidate & 0xf8)?;
        use self::DataType::*;
        use self::Opcode::*;
        match opcode {
            MovOi => meta_inst!(opcode, false, true, Some(UDWord), None),
            _ => None,
        }
    }

    pub fn get_opcode(&self) -> Opcode {
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

    pub fn get_disp_type(&self) -> Option<DataType> {
        self.disp_type
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
