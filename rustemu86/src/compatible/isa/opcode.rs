enum_from_primitive! {
    #[derive(Debug, Clone, Copy, PartialEq)]
    pub enum OpcodeCompat {
        Xor = 0x31,
        Hlt = 0xf4,
    }
}
