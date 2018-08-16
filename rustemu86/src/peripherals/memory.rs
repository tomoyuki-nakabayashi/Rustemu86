use byteorder::{LittleEndian, ReadBytesExt};
use std::mem;

pub struct Memory {
  ram: Vec<u8>,
}

impl Memory {
  pub fn new(size: usize) -> Memory {
    Memory {
      ram: vec![0; size],
    }
  }

  pub fn fill_ram(&mut self, data: Vec<u8>, start: usize) {
    for (pos, b) in data.iter().enumerate() {
      self.ram[start + pos] = *b;
    }
  }

  pub fn read8(&self, addr: usize) -> u8 {
    self.ram[addr]
  }

  pub fn read64(&self, addr: usize) -> u64 {
    let mut start = &self.ram[addr..addr + mem::size_of::<u64>()];
    start.read_u64::<LittleEndian>().unwrap()
  }

  pub fn write64(&mut self, addr: usize, data: u64) {
    let bytes: [u8; mem::size_of::<u64>()] = unsafe{ mem::transmute(data) };
    for (pos, byte) in bytes.iter().enumerate() {
      self.ram[addr + pos] = *byte;
    }
  }
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn read() {
    let memory = Memory::new(1024);

    assert_eq!(memory.read64(0), 0);
    assert_eq!(memory.read64(24), 0);
  }

  #[test]
  fn read_after_write() {
    let mut memory = Memory::new(1024);

    memory.write64(0, 1);
    assert_eq!(memory.read64(0), 1);
  }
}