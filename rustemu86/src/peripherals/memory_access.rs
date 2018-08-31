use byteorder::{LittleEndian, ReadBytesExt};

#[derive(Debug)]
pub struct MemoryAccessError;

pub trait MemoryAccess {
  fn read_u8(&self, addr: usize) -> Result<u8, MemoryAccessError>;

  fn read_u16(&self, addr: usize) -> Result<u16, MemoryAccessError> {
    let mut bytes: &[u8] = &[self.read_u8(addr)?, self.read_u8(addr+1)?];
    bytes.read_u16::<LittleEndian>().or(Err(MemoryAccessError{ }))
  }

  fn read_u32(&self, addr: usize) -> Result<u32, MemoryAccessError> {
    let mut bytes: &[u8] = &[
      self.read_u8(addr)?, self.read_u8(addr+1)?,
      self.read_u8(addr+2)?, self.read_u8(addr+3)?];
    bytes.read_u32::<LittleEndian>().or(Err(MemoryAccessError{ }))
  }

  fn read_u64(&self, addr: usize) -> Result<u64, MemoryAccessError> {
    match (0..8).map(|x| self.read_u8(addr+x) ).collect::<Result<Vec<u8>, _>>() {
      Ok(bytes) => (&bytes[..]).read_u64::<LittleEndian>().or(Err(MemoryAccessError{ })),
      Err(err) => Err(err,)
    }
  }
/* 
  fn write_u8(&self, addr: usize, data: u8) -> Result<(), MemoryAccessError>;
  fn write_u16(&self, addr: usize, data: u16) -> Result<(), MemoryAccessError>;
  fn write_u32(&self, addr: usize, data: u32) -> Result<(), MemoryAccessError>;
  fn write_u64(&self, addr: usize, data: u64) -> Result<(), MemoryAccessError>;
 */
}

#[cfg(test)]
mod test {
  use super::*;

  struct TestMemory(Vec<u8>);
  impl MemoryAccess for TestMemory {
    fn read_u8(&self, addr: usize) -> Result<u8, MemoryAccessError> {
      Ok(self.0[addr])
    }
  }

  #[test]
  fn test_read() {
    let buffer = vec![0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07];
    let memory = TestMemory(buffer);
    assert_eq!(memory.read_u8(0).unwrap(), 0x00);
    assert_eq!(memory.read_u8(1).unwrap(), 0x01);

    assert_eq!(memory.read_u16(0).unwrap(), 0x0100);
    assert_eq!(memory.read_u32(0).unwrap(), 0x03020100);
    assert_eq!(memory.read_u64(0).unwrap(), 0x0706050403020100);
  }
}