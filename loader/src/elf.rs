//! ELF format.

pub const HEADER_MAGIC: [u8; 4] = [0x7f, 0x45, 0x4c, 0x46]; // 0x7f 'E' 'L' 'F'