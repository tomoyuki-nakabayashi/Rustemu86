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