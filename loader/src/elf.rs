//! ELF format.
#![allow(dead_code)]
/// 0x7f 'E' 'L' 'F'
const HEADER_MAGIC: [u8; 4] = [0x7f, 0x45, 0x4c, 0x46];
const SIZE_ELF_IDENT: usize = 16;
const SIZE_ELF32_HEADER: usize = 52;
const SIZE_ELF64_HEADER: usize = 64;

pub struct ElfHeader {
    identification: ElfIdentification,
}

impl ElfHeader {
    pub fn new(binary: &[u8]) -> ElfHeader {
        if binary.len() < SIZE_ELF_IDENT {
            panic!("too short binary!");
        }
        ElfHeader {
            identification: ElfIdentification::new(binary),
        }
    }

    pub fn is_elf(&self) -> bool {
        self.identification.magic == HEADER_MAGIC
    }
}

/// File identification in elf header.
pub struct ElfIdentification {
    magic: [u8; 4],
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

#[cfg(test)]
mod test {
    use super::*;
    use std::fs::File;
    use memmap::Mmap;

    #[test]
    fn is_elf() {
        let file = File::open("tests/data/elf/rv32ui-p-simple").unwrap();
        // safe accessing only from the main thread, and treating the contents as immutable.
        let mapped_file = unsafe { Mmap::map(&file).unwrap() };
        let header = ElfHeader::new(&mapped_file[0..16]);

        assert!(header.is_elf());
    }
}