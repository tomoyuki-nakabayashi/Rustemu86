use byteorder::{LittleEndian, ReadBytesExt};

pub struct MemoryAccessError;

pub trait MemoryAccess {
  fn read_u8(&self, addr: usize) -> Result<u8, MemoryAccessError>;
  fn read_u16(&self, addr: usize) -> Result<u16, MemoryAccessError> {
    let mut bytes = Vec::with_capacity(2);
    bytes.push(self.read_u8(addr)?);
    bytes.push(self.read_u8(addr+1)?);
    let mut bytes = &bytes[..];
    Ok(bytes.read_u16::<LittleEndian>().unwrap())
  }
  fn read_u32(&self, addr: usize) -> Result<u32, MemoryAccessError>;
  fn read_u64(&self, addr: usize) -> Result<u64, MemoryAccessError>;
}