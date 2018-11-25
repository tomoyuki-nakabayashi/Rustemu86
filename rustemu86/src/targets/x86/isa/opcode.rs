use crate::targets::x86::CompatibleException;
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
        Nop = 0x90,
        Xor = 0x31,
        Hlt = 0xf4,
    }
}

impl Default for Opcode {
    fn default() -> Opcode {
        Opcode::Nop
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

/// Generate MetaInst table.
#[macro_use]
macro_rules! meta_inst_table {
    ( $target: ident, $( ( $op: ident, $( $key: ident : $value: expr ),* ) ),+ ) => ({
        match $target {
            $(
                $op => {
                    let mut inst = MetaInst { opcode: $op, ..MetaInst::default() };
                    $( inst.$key = $value; )*
                    Some(inst)
                }
            )+
            _ => None,
        }
    });

    // For trailing comma.
    ( $target: ident, $( ( $op: ident, $( $key: ident : $value: expr ),* ) ),+, ) => ({
        meta_inst_table!($target, $( ( $op, $( $key : $value ),* ) ),+ )
    });
}

/// MetaInst represents meta infomation for an opcode in the opcode field.
///
/// You can obtain a MetaInst from an `u8` which can be translated to a valid opcode.
#[derive(Debug, Default)]
pub struct MetaInst {
    opcode: Opcode,
    modrm: bool,
    r: bool,
    imm_type: Option<DataType>,
    disp_type: Option<DataType>,
}

impl MetaInst {
    /// Instanciates MetaInsts from byte array.
    /// Currently, just from a byte, this will be fixed in the future.
    pub fn from(candidate: u8) -> Result<MetaInst, CompatibleException> {
        Self::from_u8(candidate)
            .or_else(|| Self::plus_r_from_u8(candidate))
            .ok_or(CompatibleException(format!(
                "Encounters undefined opcode: '0x{:x}' in fetch stage.",
                candidate
            )))
    }

    // Instantiate from `u8` except for `plus r` opcode.
    fn from_u8(candidate: u8) -> Option<MetaInst> {
        let opcode = Opcode::from_u8(candidate)?;
        use self::DataType::*;
        use self::Opcode::*;

        meta_inst_table!(
            opcode,
            (Cld, ),
            (Lea, modrm: true, disp_type: Some(UDWord)),
            (MovRmSreg, modrm: true),
            (Xor, modrm: true),
            (Hlt, ),
        )
    }

    // `plus r` is a special case of u8 opcode.
    // The u8 contains the destination address in their lower significant 3-bits.
    fn plus_r_from_u8(candidate: u8) -> Option<MetaInst> {
        let opcode = Opcode::from_u8(candidate & 0xf8)?;
        use self::DataType::*;
        use self::Opcode::*;

        meta_inst_table!(
            opcode,
            (MovOi, r: true, imm_type: Some(UDWord)),
        )
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
