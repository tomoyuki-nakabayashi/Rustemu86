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
}