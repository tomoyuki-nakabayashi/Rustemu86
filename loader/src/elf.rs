//! ELF format.
#![allow(dead_code)]
use crate::error::{Result, LoaderError};
use byteorder::{LittleEndian, ReadBytesExt};

/// 0x7f 'E' 'L' 'F'
const HEADER_MAGIC: [u8; 4] = [0x7f, 0x45, 0x4c, 0x46];
const SIZE_ELF_IDENT: usize = 16;
const SIZE_ELF32_HEADER: usize = 52;
const SIZE_ELF64_HEADER: usize = 64;

const ELF_CLASS_32: u8 = 1;
const ELF_DATA_LSB: u8 = 1;
const EIV_CURRENT: u8 = 1;
const ELF_OS_ABI_NONE: u8 = 0;

const ET_EXEC: u16 = 2;
const EM_RISCV: u16 = 243;
const EV_CURRENT: u32 = 1;

/// ELF header parsing first 52/64 bytes for 32/64 bits binary.
/// Currently, support only 32-bit binary.
pub struct ElfHeader {
    identification: ElfIdentification,
    elf_type: u16,
    elf_machine: u16,
    elf_version: u32,
    elf_entry: usize,
    elf_program_header_offset: usize,
    elf_section_header_offset: usize,
    elf_flag: u32,
    elf_header_size: u16,
    elf_program_header_entry_size: u16,
    elf_program_header_num: u16,
    elf_section_header_entry_size: u16,
    elf_section_header_num: u16,
    elf_section_header_table_index: u16,
}

impl ElfHeader {
    /// Create an object from byte array.
    // TODO: return Result.
    pub fn try_new(binary: &[u8]) -> Result<ElfHeader> {
        if binary.len() < SIZE_ELF32_HEADER {
            return Err(LoaderError::TooShortBinary{})
        }
        // unwrap the result of read_u* because the binary has enough length to read
        // and the operations never fail.
        Ok(ElfHeader {
            identification: ElfIdentification::new(&binary[0..=15]),
            elf_type: read_u16(&binary[16..=17]).unwrap(),
            elf_machine: read_u16(&binary[18..=19]).unwrap(),
            elf_version: read_u32(&binary[20..=23]).unwrap(),
            elf_entry: read_u32(&binary[24..=27]).unwrap() as usize,
            elf_program_header_offset: read_u32(&binary[28..=31]).unwrap() as usize,
            elf_section_header_offset: read_u32(&binary[32..=35]).unwrap() as usize,
            elf_flag: read_u32(&binary[36..=39]).unwrap(),
            elf_header_size: read_u16(&binary[40..=41]).unwrap(),
            elf_program_header_entry_size: read_u16(&binary[42..=43]).unwrap(),
            elf_program_header_num: read_u16(&binary[44..=45]).unwrap(),
            elf_section_header_entry_size: read_u16(&binary[46..=47]).unwrap(),
            elf_section_header_num: read_u16(&binary[48..=49]).unwrap(),
            elf_section_header_table_index: read_u16(&binary[50..=51]).unwrap(),
        })
    }

    /// Check the elf magic.
    pub fn is_elf(&self) -> bool {
        self.identification.magic == HEADER_MAGIC
    }
}

// Reads u32 from the head of given byte array.
#[inline(always)]
fn read_u32(binary: &[u8]) -> std::io::Result<u32> {
    (&binary[0..=3]).read_u32::<LittleEndian>()
}

// Reads u16 from the head of given byte array.
#[inline(always)]
fn read_u16(binary: &[u8]) -> std::io::Result<u16> {
    (&binary[0..=1]).read_u16::<LittleEndian>()
}

// File identification in elf header.
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
    // assumption: `binary` has enough length to read elf identification.
    fn new(binary: &[u8]) -> ElfIdentification {
        let mut magic: [u8; 4] = [0; 4];
        for (i, b) in binary[0..4].iter().enumerate() {
            magic[i] = *b;
        }
        ElfIdentification {
            magic,
            class: binary[4],
            endianess: binary[5],
            version: binary[6],
            os_abi: binary[7],
            os_abi_version: binary[8],
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

    // Check the ELF identification is as bellow:
    //   Magic:   7f 45 4c 46 01 01 01 00 00 00 00 00 00 00 00 00
    //   Class:                             ELF32
    //   Data:                              2's complement, little endian
    //   Version:                           1 (current)
    //   OS/ABI:                            UNIX - System V
    #[test]
    fn elf_identification() {
        let file = File::open("tests/data/elf/rv32ui-p-simple").unwrap();
        let mapped_file = unsafe { Mmap::map(&file).unwrap() };
        let identification = ElfIdentification::new(&mapped_file);

        assert_eq!(ELF_CLASS_32, identification.class);
        assert_eq!(ELF_DATA_LSB, identification.endianess);
        assert_eq!(EIV_CURRENT, identification.version);
        assert_eq!(ELF_OS_ABI_NONE, identification.os_abi);
    }

    // Check the ELF header is as bellow:
    //   Type:                              EXEC (Executable file)
    //   Machine:                           RISC-V
    //   Version:                           0x1
    //   Entry point address:               0x80000000
    //   Start of program headers:          52 (bytes into file)
    //   Start of section headers:          8692 (bytes into file)
    //   Flags:                             0x0
    //   Size of this header:               52 (bytes)
    //   Size of program headers:           32 (bytes)
    //   Number of program headers:         2
    //   Size of section headers:           40 (bytes)
    //   Number of section headers:         6
    //   Section header string table index: 5
    #[test]
    fn elf_header() {
        let file = File::open("tests/data/elf/rv32ui-p-simple").unwrap();
        // safe accessing only from the main thread, and treating the contents as immutable.
        let mapped_file = unsafe { Mmap::map(&file).unwrap() };
        let header = ElfHeader::try_new(&mapped_file).unwrap();

        assert_eq!(ET_EXEC, header.elf_type);
        assert_eq!(EM_RISCV, header.elf_machine);
        assert_eq!(EV_CURRENT, header.elf_version);
        assert_eq!(0x8000_0000, header.elf_entry);
        assert_eq!(52, header.elf_program_header_offset);
        assert_eq!(8692, header.elf_section_header_offset);
        assert_eq!(0, header.elf_flag);
        assert_eq!(52, header.elf_header_size);
        assert_eq!(32, header.elf_program_header_entry_size);
        assert_eq!(2, header.elf_program_header_num);
        assert_eq!(40, header.elf_section_header_entry_size);
        assert_eq!(6, header.elf_section_header_num);
        assert_eq!(5, header.elf_section_header_table_index);
    }
}
