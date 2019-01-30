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

        let pheaders = ProgramHeader::extract_pheaders(&mapped_file, &header);

        Ok(ElfLoader {
            mapped_file,
            header,
            pheaders,
        })
    }

    /// Returns whole memory image including both data and meta information
    /// for each program segment.
    /// This function may be expensive because it copies binary data.
    /// We intend not to use mmaped file directly.
    pub fn memory_image(&self) -> Vec<MemoryLayout> {
        let mut memory_image = Vec::new();
        for pheader in &self.pheaders {
            let (begin, end) = pheader.segment_offset_range();
            let layout = MemoryLayout {
                binary: self.mapped_file[begin..end].to_vec(),
                pheader: pheader.clone(),
            };
            memory_image.push(layout);
        }

        memory_image
    }
}

pub struct MemoryLayout {
    binary: Vec<u8>,
    pheader: ProgramHeader,
}

#[cfg(test)]
mod test {
    use super::*;
    use byteorder::{LittleEndian, ReadBytesExt};

    #[test]
    fn load_elf() {
        let loader = ElfLoader::try_new("tests/data/elf/rv32ui-p-simple");
        assert!(loader.is_ok(), "target file is not elf binary");
    }

    #[test]
    fn get_memory_image() {
        let loader = ElfLoader::try_new("tests/data/elf/rv32ui-p-simple").unwrap();
        let memory_iamge = loader.memory_image();

        assert_eq!(memory_iamge.len(), 2);
        let init_segment = &memory_iamge[0];
        assert_eq!(init_segment.pheader.segment_offset_range(), (0x1000, 0x1000 + 0x144));
        let first_inst = (&init_segment.binary[0..4]).read_u32::<LittleEndian>().unwrap();
        assert_eq!(first_inst, 0x04c0_006f);
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
