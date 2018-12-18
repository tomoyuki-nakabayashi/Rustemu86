//! General Purpose Register File.

const REGISTER_NUM: usize = 8;

pub(crate) struct RegisterFile {
    rams: Vec<u64>,
}

impl RegisterFile {
    pub fn new() -> RegisterFile {
        RegisterFile {
            rams: vec![0xffff_ffff_ffff_ffff; REGISTER_NUM],
        }
    }

    pub fn read_u64(&self, index: Reg32) -> u64 {
        self.rams[index as usize]
    }

    pub fn write_u64(&mut self, index: Reg32, value: u64) {
        self.rams[index as usize] = value;
    }
}

enum_from_primitive! {
    /// Register index for x86 32bit mode.
    #[derive(Debug, Clone, Copy, PartialEq)]
    pub enum Reg32 {
        Eax = 0x00,
        Ecx = 0x01,
        Edx = 0x02,
        Ebx = 0x03,
        Esp = 0x04,
        Ebp = 0x05,
        Esi = 0x06,
        Edi = 0x07,
    }
}

const NUM_SEGMENT_REGSITERS: usize = 6;

/// Segment register.
pub(crate) struct SegmentRegister {
    rams: Vec<u64>,
}

impl SegmentRegister {
    pub fn new() -> SegmentRegister {
        SegmentRegister {
            rams: vec![0xffff_ffff_ffff_ffff; NUM_SEGMENT_REGSITERS],
        }
    }

    pub fn read_u64(&self, index: SegReg) -> u64 {
        self.rams[index as usize]
    }

    pub fn write_u64(&mut self, index: SegReg, value: u64) {
        self.rams[index as usize] = value;
    }
}

enum_from_primitive! {
    /// Segment register index for x86 32bit mode.
    #[derive(Debug, Clone, Copy, PartialEq)]
    pub enum SegReg {
        Es = 0x00,
        Cs = 0x01,
        Ss = 0x02,
        Ds = 0x03,
        Fs = 0x04,
        Gs = 0x05,
    }
}
