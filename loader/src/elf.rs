//! ELF format.

/// 0x7f 'E' 'L' 'F'
pub const HEADER_MAGIC: [u8; 4] = [0x7f, 0x45, 0x4c, 0x46];

/// File identification in elf header.
pub struct ElfIdentification {
    pub magic: [u8; 4],
    class: u8,
    endianess: u8,
    version: u8,
    os_abi: u8,
    os_abi_version: u8,
    reserved: [u8; 7],
}

impl ElfIdentification {
    pub fn new(binary: &[u8]) -> ElfIdentification {
        let mut magic: [u8; 4] = [0; 4];
        for(i, b) in binary[0..4].iter().enumerate() {
            magic[i] = *b;
        }
        ElfIdentification {
            magic,
            class: binary[5],
            endianess: binary[6],
            version: binary[7],
            os_abi: binary[8],
            os_abi_version: binary[9],
            reserved: [0; 7],
        }
    }
}