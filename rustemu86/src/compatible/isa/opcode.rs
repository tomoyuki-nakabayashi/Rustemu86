enum_from_primitive! {
    #[derive(Debug, Clone, Copy, PartialEq)]
    pub enum OpcodeCompat {
        Xor = 0x31,
        Hlt = 0xf4,
    }
}

pub fn inst_use_modrm(opcode: OpcodeCompat) -> bool {
    opcode == OpcodeCompat::Xor
}