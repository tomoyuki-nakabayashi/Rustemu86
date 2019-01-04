//! EFL loader.

use crate::elf::{self, ElfIdentification};
use std::fs::File;
use memmap::Mmap;

/// ELF Loader
pub struct ElfLoader {
    mapped_file: Mmap,
    identification: ElfIdentification
}

impl ElfLoader {
    /// This method returns error when
    /// - failed to open file
    /// - failed to mmap
    pub fn try_new(file_path: &str) -> std::io::Result<ElfLoader> {
        let file = File::open(&file_path)?;
        // safe accessing only from the main thread, and treating the contents as immutable.
        let mapped_file = unsafe { Mmap::map(&file)? };
        let identification = ElfIdentification::new(&mapped_file[0..16]);
        Ok(ElfLoader {
            mapped_file,
            identification,
        })
    }

    fn is_elf(&self) -> bool {
        self.identification.magic == elf::HEADER_MAGIC
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn is_elf() {
        let loader = ElfLoader::try_new("tests/data/elf/rv32ui-p-simple").unwrap();
        assert!(loader.is_elf(), "target file is not elf binary");
    }
}