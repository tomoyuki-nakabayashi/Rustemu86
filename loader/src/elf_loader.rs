//! EFL loader.

use crate::elf;
use std::fs::File;
use memmap::Mmap;

pub struct ElfLoader {
    mapped_file: Mmap,
}

impl ElfLoader {
    pub fn try_new(file_path: &str) -> std::io::Result<ElfLoader> {
        let file = File::open(&file_path)?;
        Ok(ElfLoader {
            mapped_file: unsafe { Mmap::map(&file)? },
        })
    }

    fn is_elf(&self) -> bool {
        self.mapped_file[0..4] == elf::HEADER_MAGIC
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