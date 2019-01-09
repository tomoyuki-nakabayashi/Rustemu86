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

    /// Returns an iterator to get segments.
    pub fn get_segments(&self) -> Vec<Segment> {
        let mut segments = Vec::new();
        for ph in self.pheaders.iter() {
            let bytes = self.mapped_file[ph.offset..(ph.offset + ph.mem_size as usize)].to_vec();
            let segment = Segment {
                vaddr: ph.vaddr,
                size: ph.mem_size as usize,
                bytes
            };
            segments.push(segment);
        }

        segments
    }
}

pub struct Segment {
    pub vaddr: usize,
    pub size: usize,
    pub bytes: Vec<u8>,
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
    fn get_segments() {
        let loader = ElfLoader::try_new("tests/data/elf/rv32ui-p-simple").unwrap();
        let segments = loader.get_segments();

        assert_eq!(segments.len(), 2);
        assert_eq!(segments[0].vaddr, 0x8000_0000);
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
