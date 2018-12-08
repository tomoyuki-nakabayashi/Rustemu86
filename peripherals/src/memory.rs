use crate::memory_access::{MemoryAccess, MemoryAccessError, Result};

pub struct Memory {
    ram: Vec<u8>,
    size: usize,
}

impl Memory {
    pub fn new(size: usize) -> Memory {
        Memory {
            ram: vec![0; size],
            size,
        }
    }

    pub fn fill_ram(&mut self, data: &[u8], start: usize) {
        for (pos, b) in data.iter().enumerate() {
            self.ram[start + pos] = *b;
        }
    }
}

impl MemoryAccess for Memory {
    fn read_u8(&self, addr: usize) -> Result<u8> {
        Ok(self.ram[addr])
    }

    fn write_u8(&mut self, addr: usize, data: u8) -> Result<()> {
        if addr >= self.size {
            return Err(MemoryAccessError {});
        }

        self.ram[addr] = data;
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::memory_access::MemoryAccess;

    #[test]
    fn read() {
        let memory = Memory::new(1024);

        assert_eq!(memory.read_u64(0).unwrap(), 0);
        assert_eq!(memory.read_u64(24).unwrap(), 0);
    }

    #[test]
    fn read_after_write() {
        let mut memory = Memory::new(1024);

        assert!(memory.write_u64(0, 1).is_ok());
        assert_eq!(memory.read_u64(0).unwrap(), 1);
    }
}
