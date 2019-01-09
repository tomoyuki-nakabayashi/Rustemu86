//! ELF format.
#![allow(dead_code)]
use crate::error::{LoaderError, Result};
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

const PT_LOAD: u32 = 1;
// flags in program header indicate access permission for the segment.
const PF_X: u32 = 0x1;
const PF_W: u32 = 0x2;
const PF_R: u32 = 0x4;

/// ELF header parsing first 52/64 bytes for 32/64 bits binary.
/// Currently, support only 32-bit binary.
#[derive(Debug)]
pub struct ElfHeader {
    identification: ElfIdentification,
    elf_type: u16,
    elf_machine: u16,
    elf_version: u32,
    elf_entry: usize,
    elf_pheader_offset: usize,
    elf_section_header_offset: usize,
    elf_flag: u32,
    elf_header_size: u16,
    elf_pheader_entry_size: u16,
    elf_pheader_num: u16,
    elf_section_header_entry_size: u16,
    elf_section_header_num: u16,
    elf_section_header_table_index: u16,
}

impl ElfHeader {
    /// Create an object from byte array.
    pub fn try_new(binary: &[u8]) -> Result<ElfHeader> {
        if binary.len() < SIZE_ELF32_HEADER {
            return Err(LoaderError::TooShortBinary {});
        }
        // unwrap the result of read_u* because the binary has enough length to read
        // and the operations never fail.
        Ok(ElfHeader {
            identification: ElfIdentification::new(&binary[0..=15]),
            elf_type: read_u16(&binary[16..=17]).unwrap(),
            elf_machine: read_u16(&binary[18..=19]).unwrap(),
            elf_version: read_u32(&binary[20..=23]).unwrap(),
            elf_entry: read_u32(&binary[24..=27]).unwrap() as usize,
            elf_pheader_offset: read_u32(&binary[28..=31]).unwrap() as usize,
            elf_section_header_offset: read_u32(&binary[32..=35]).unwrap() as usize,
            elf_flag: read_u32(&binary[36..=39]).unwrap(),
            elf_header_size: read_u16(&binary[40..=41]).unwrap(),
            elf_pheader_entry_size: read_u16(&binary[42..=43]).unwrap(),
            elf_pheader_num: read_u16(&binary[44..=45]).unwrap(),
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
#[derive(Debug)]
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

#[derive(Debug)]
pub(crate) struct ProgramHeader {
    program_type: u32,
    pub offset: usize, // 4-byte
    pub vaddr: usize,  // 4-byte
    paddr: usize,  // 4-byte
    file_size: u32,
    pub mem_size: u32,
    flags: u32,
    pub align: u32,
}

impl ProgramHeader {
    // create an instance from byte array which starts at the first byte of
    // the header will be read.
    fn new(start: &[u8]) -> ProgramHeader {
        ProgramHeader {
            program_type: read_u32(&start[0..=3]).unwrap(),
            offset: read_u32(&start[4..=7]).unwrap() as usize,
            vaddr: read_u32(&start[8..=11]).unwrap() as usize,
            paddr: read_u32(&start[12..=15]).unwrap() as usize,
            file_size: read_u32(&start[16..=19]).unwrap(),
            mem_size: read_u32(&start[20..=23]).unwrap(),
            flags: read_u32(&start[24..=27]).unwrap(),
            align: read_u32(&start[28..=31]).unwrap(),
        }
    }

    // TODO: Check the binary size is large enough to extract headers.
    pub fn extract_headers(binary: &[u8], header: &ElfHeader) -> Vec<ProgramHeader> {
        let mut pheaders: Vec<ProgramHeader> = Vec::new();

        for i in 0..header.elf_pheader_num {
            let start_offset = header.elf_pheader_offset
                + (i * header.elf_pheader_entry_size) as usize;
            pheaders.push(ProgramHeader::new(&(binary[start_offset..])));
        }

        pheaders
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

        assert_eq!(identification.class, ELF_CLASS_32);
        assert_eq!(identification.endianess, ELF_DATA_LSB);
        assert_eq!(identification.version, EIV_CURRENT);
        assert_eq!(identification.os_abi, ELF_OS_ABI_NONE);
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

        assert_eq!(header.elf_type, ET_EXEC);
        assert_eq!(header.elf_machine, EM_RISCV);
        assert_eq!(header.elf_version, EV_CURRENT);
        assert_eq!(header.elf_entry, 0x8000_0000);
        assert_eq!(header.elf_pheader_offset, 52);
        assert_eq!(header.elf_section_header_offset, 8692);
        assert_eq!(header.elf_flag, 0);
        assert_eq!(header.elf_header_size, 52);
        assert_eq!(header.elf_pheader_entry_size, 32);
        assert_eq!(header.elf_pheader_num, 2);
        assert_eq!(header.elf_section_header_entry_size, 40);
        assert_eq!(header.elf_section_header_num, 6);
        assert_eq!(header.elf_section_header_table_index, 5);
    }

    // Entry point 0x80000000
    // There are 2 program headers, starting at offset 52

    // Program Headers:
    //   Type           Offset   VirtAddr   PhysAddr   FileSiz MemSiz  Flg Align
    //   LOAD           0x001000 0x80000000 0x80000000 0x00144 0x00144 R E 0x1000
    //   LOAD           0x002000 0x80001000 0x80001000 0x00048 0x00048 RW  0x1000

    //  Section to Segment mapping:
    //   Segment Sections...
    //    00     .text.init
    //    01     .tohost
    #[test]
    fn program_header() {
        let file = File::open("tests/data/elf/rv32ui-p-simple").unwrap();
        // safe accessing only from the main thread, and treating the contents as immutable.
        let mapped_file = unsafe { Mmap::map(&file).unwrap() };

        let text_init = ProgramHeader::new(&mapped_file[52..]);

        assert_eq!(text_init.program_type, PT_LOAD);
        assert_eq!(text_init.offset, 0x1000);
        assert_eq!(text_init.vaddr, 0x8000_0000);
        assert_eq!(text_init.paddr, 0x8000_0000);
        assert_eq!(text_init.file_size, 0x144);
        assert_eq!(text_init.mem_size, 0x144);
        assert_eq!(text_init.flags, (PF_R | PF_X));
        assert_eq!(text_init.align, 0x1000);
    }

    #[test]
    fn all_program_headers() {
        let file = File::open("tests/data/elf/rv32ui-p-simple").unwrap();
        // safe accessing only from the main thread, and treating the contents as immutable.
        let mapped_file = unsafe { Mmap::map(&file).unwrap() };
        let header = ElfHeader::try_new(&mapped_file).unwrap();

        let pheaders = ProgramHeader::extract_headers(&mapped_file, &header);

        assert_eq!(pheaders.len(), 2);
    }
}
