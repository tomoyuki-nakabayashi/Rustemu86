//! EFL loader.
#![allow(dead_code)]
use crate::elf::ElfHeader;
use std::fs::File;
use memmap::Mmap;

/// ELF Loader
pub struct ElfLoader {
    mapped_file: Mmap,
    header: ElfHeader
}

impl ElfLoader {
    /// This method returns error when
    /// - failed to open file
    /// - failed to mmap
    pub fn try_new(file_path: &str) -> std::io::Result<ElfLoader> {
        let file = File::open(&file_path)?;
        // to be safe, lock is required.
        let mapped_file = unsafe { Mmap::map(&file)? };
        let header = ElfHeader::new(&mapped_file);
        Ok(ElfLoader {
            mapped_file,
            header,
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn load_elf() {
        let loader = ElfLoader::try_new("tests/data/elf/rv32ui-p-simple");
        assert!(loader.is_ok(), "target file is not elf binary");
    }
}