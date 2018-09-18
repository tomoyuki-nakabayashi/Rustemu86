/// General Purpose Register File.
/// 

pub(crate) struct RegisterFile{
    rams: Vec<u64>,
}

impl RegisterFile {
    pub fn new() -> RegisterFile {
        RegisterFile {
            rams: vec![0xffff_ffff_ffff_ffff; 8],
        }
    }

    pub fn read_u64(self, index: usize) -> u64 {
        self.rams[index]
    }
}