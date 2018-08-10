#![feature(int_to_from_bytes)]
use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};

pub struct Memory {
  ram: Vec<u8>,
}

impl Memory {
  pub fn new(size: usize) -> Memory {
    Memory {
      ram: vec![0; size],
    }
  }

  pub fn read64(&self, addr: usize) -> u64 {
    let mut start = &self.ram[addr..addr+8];
    start.read_u64::<LittleEndian>().unwrap()
  }

  pub fn write64(&mut self, addr: usize, data: u64) {
    let bytes = data.to_le().to_bytes();
    for pos in 0..8 {
      self.ram[addr + pos] = bytes[pos];
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