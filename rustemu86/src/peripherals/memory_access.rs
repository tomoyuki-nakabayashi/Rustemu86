use byteorder::{LittleEndian, ReadBytesExt};
use std::mem;
use std::result;

pub type Result<T> = result::Result<T, MemoryAccessError>;

#[derive(Debug)]
pub struct MemoryAccessError;

pub trait MemoryAccess {
  fn read_u8(&self, addr: usize) -> Result<u8>;

  fn read_u16(&self, addr: usize) -> Result<u16> {
    let mut bytes: &[u8] = &[self.read_u8(addr)?, self.read_u8(addr + 1)?];
    bytes
      .read_u16::<LittleEndian>()
      .or(Err(MemoryAccessError {}))
  }

  fn read_u32(&self, addr: usize) -> Result<u32> {
    match (0..mem::size_of::<u32>())
      .map(|x| self.read_u8(addr + x))
      .collect::<Result<Vec<u8>>>()
    {
      Ok(bytes) => (&bytes[..])
        .read_u32::<LittleEndian>()
        .or(Err(MemoryAccessError {})),
      Err(err) => Err(err),
    }
  }

  fn read_u64(&self, addr: usize) -> Result<u64> {
    match (0..mem::size_of::<u64>())
      .map(|x| self.read_u8(addr + x))
      .collect::<Result<Vec<u8>>>()
    {
      Ok(bytes) => (&bytes[..])
        .read_u64::<LittleEndian>()
        .or(Err(MemoryAccessError {})),
      Err(err) => Err(err),
    }
  }

  fn write_u8(&mut self, addr: usize, data: u8) -> Result<()>;

  fn write_u16(&mut self, addr: usize, data: u16) -> Result<()> {
    let bytes: [u8; mem::size_of::<u16>()] = unsafe { mem::transmute(data) };
    (0..mem::size_of::<u16>())
      .map(|x| self.write_u8(addr + x, bytes[x]))
      .collect::<Result<()>>()
  }

  fn write_u32(&mut self, addr: usize, data: u32) -> Result<()> {
    let bytes: [u8; mem::size_of::<u32>()] = unsafe { mem::transmute(data) };
    (0..mem::size_of::<u32>())
      .map(|x| self.write_u8(addr + x, bytes[x]))
      .collect::<Result<()>>()
  }

  fn write_u64(&mut self, addr: usize, data: u64) -> Result<()> {
    let bytes: [u8; mem::size_of::<u64>()] = unsafe { mem::transmute(data) };
    (0..mem::size_of::<u64>())
      .map(|x| self.write_u8(addr + x, bytes[x]))
      .collect::<Result<()>>()
  }
}

#[cfg(test)]
mod test {
  use super::*;

  struct TestMemory(Vec<u8>);
  impl MemoryAccess for TestMemory {
    fn read_u8(&self, addr: usize) -> Result<u8> {
      match addr {
        0...7 => Ok(self.0[addr]),
        _ => Err(MemoryAccessError {}),
      }
    }

    fn write_u8(&mut self, addr: usize, data: u8) -> Result<()> {
      match addr {
        0...7 => {
          self.0[addr] = data;
          Ok(())
        }
        _ => Err(MemoryAccessError {}),
      }
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

  #[test]
  fn test_read_error() {
    let buffer = vec![0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07];
    let memory = TestMemory(buffer);
    assert!(memory.read_u8(8).is_err());
    assert!(memory.read_u16(8).is_err());
    assert!(memory.read_u32(8).is_err());
    assert!(memory.read_u64(8).is_err());
  }

  #[test]
  fn test_write() {
    let buffer = vec![0x00; 8];
    let mut memory = TestMemory(buffer);

    assert!(memory.write_u8(1, 0x01).is_ok());
    assert!(memory.write_u16(2, 0x0302).is_ok());
    assert!(memory.write_u32(4, 0x07060504).is_ok());

    assert_eq!(memory.read_u64(0).unwrap(), 0x0706050403020100);
  }
}
