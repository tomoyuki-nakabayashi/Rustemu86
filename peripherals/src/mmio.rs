//! Memory mapped IOs.
//! Users create their own memory mapping.
use crate::memory_access::{self, MemoryAccess, MemoryAccessError};
use std::collections::HashMap;

/// start adderess, size of the memory map can be accessed.
pub type MemoryRange = (usize, usize);

// Memory map that contains pairs of memory range and memory mapped device.
type MemoryMap = HashMap<MemoryRange, Box<dyn MemoryAccess>>;

pub struct Mmio {
    memory_map: MemoryMap,
}

impl Mmio {
    /// Create empty memory mapped IO.
    pub fn empty() -> Mmio {
        Mmio { memory_map: MemoryMap::new() }
    }

    /// Add a memory mapped device.
    /// 
    /// # Examples
    /// ```
    /// use peripherals::{mmio::Mmio, memory::Memory};
    /// let dram = Box::new(Memory::new(64));
    /// let mut mmio = Mmio::empty();
    /// let result = mmio.add((0, 64), dram);
    /// assert!(true, result.is_ok());
    /// ```
    /// 
    /// TODO:
    /// - Fail if MemoryRange overlapping existing memory mapped device.
    /// - Validate the `range`. MemoryAccess must know the range can be accessed.
    pub fn add(&mut self, range: MemoryRange, device: Box<dyn MemoryAccess>) -> Result<(), ()> {
        self.memory_map.insert(range, device);
        Ok(())
    }
}

impl MemoryAccess for Mmio {
    fn read_u8(&self, addr: usize) -> memory_access::Result<u8> {
        for (range, device) in &self.memory_map {
            if range.0 <= addr && addr <= range.1 {
                return device.read_u8(addr)
            }
        }
        Err(MemoryAccessError{})
    }

    fn write_u8(&mut self, addr: usize, data: u8) -> memory_access::Result<()> {
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::memory::Memory;

    #[test]
    fn simple_mmio_use() {
        let dram = Box::new(Memory::new(64));

        let mut mmio = Mmio::empty();
        let result = mmio.add((0, 64), dram);
        assert!(true, result.is_ok());

        assert_eq!(0, mmio.read_u8(0).unwrap());
    }
}