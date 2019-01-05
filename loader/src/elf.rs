//! ELF format.
#![allow(dead_code)]
use crate::error::{Result, LoaderError};

/// 0x7f 'E' 'L' 'F'
const HEADER_MAGIC: [u8; 4] = [0x7f, 0x45, 0x4c, 0x46];
const SIZE_ELF_IDENT: usize = 16;
const SIZE_ELF32_HEADER: usize = 52;
const SIZE_ELF64_HEADER: usize = 64;

/// ELF header parsing first 52/64 bytes for 32/64 bits binary.
/// Currently, support only 32-bit binary.
pub struct ElfHeader {
    identification: ElfIdentification,
}

impl ElfHeader {
    /// Create an object from byte array.
    // TODO: return Result.
    pub fn try_new(binary: &[u8]) -> Result<ElfHeader> {
        if binary.len() < SIZE_ELF32_HEADER {
            return Err(LoaderError::TooShortBinary{})
        }
        Ok(ElfHeader {
            identification: ElfIdentification::new(binary),
        })
    }

    /// Check the elf magic.
    pub fn is_elf(&self) -> bool {
        self.identification.magic == HEADER_MAGIC
    }
}

/// File identification in elf header.
struct ElfIdentification {
    magic: [u8; 4],
    class: u8,
    endianess: u8,
    version: u8,
    os_abi: u8,
    os_abi_version: u8,
    reserved: [u8; 7], // zero filled.
}

impl ElfIdentification {
    pub fn new(binary: &[u8]) -> ElfIdentification {
        let mut magic: [u8; 4] = [0; 4];
        for (i, b) in binary[0..4].iter().enumerate() {
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
    use memmap::Mmap;
    use std::fs::File;

    #[test]
    fn is_elf() {
        let file = File::open("tests/data/elf/rv32ui-p-simple").unwrap();
        // safe accessing only from the main thread, and treating the contents as immutable.
        let mapped_file = unsafe { Mmap::map(&file).unwrap() };
        let header = ElfHeader::try_new(&mapped_file).unwrap();

        assert!(header.is_elf());
    }
}
