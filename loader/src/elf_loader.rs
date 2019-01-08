//! EFL loader.
#![allow(dead_code)]
use crate::elf::{ElfHeader, ProgramHeader};
use crate::error::{LoaderError, Result};
use memmap::Mmap;
use std::fs::File;

/// ELF Loader
pub struct ElfLoader {
    mapped_file: Mmap,
    header: ElfHeader,
    pheaders: Vec<ProgramHeader>,
}

impl ElfLoader {
    /// This method returns error when
    /// - failed to open file
    /// - failed to mmap
    pub fn try_new(file_path: &str) -> Result<ElfLoader> {
        let file = File::open(&file_path)?;
        // to be safe, lock is required.
        let mapped_file = unsafe { Mmap::map(&file)? };
        let header = ElfHeader::try_new(&mapped_file)?;

        if !header.is_elf() {
            return Err(LoaderError::InvalidElfFormat {});
        }

        let pheaders = ProgramHeader::extract_headers(&mapped_file, &header);

        Ok(ElfLoader {
            mapped_file,
            header,
            pheaders,
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
        assert_eq!(loader.unwrap().pheaders.len(), 2);
    }

    #[test]
    fn load_non_elf_binary() {
        let loader = ElfLoader::try_new("tests/data/non-elf-binary");
        assert!(
            loader.is_err(),
            "target file is incorrectly recognized as ELF file"
        );
    }

    #[test]
    fn load_too_short_binary() {
        let loader = ElfLoader::try_new("tests/data/simple_add");
        assert!(
            loader.is_err(),
            "target file is incorrectly recognized as ELF file"
        );
    }
}
