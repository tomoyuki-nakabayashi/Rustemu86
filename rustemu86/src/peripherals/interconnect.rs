use peripherals::memory::Memory;
use peripherals::uart16550::Uart16550;

pub struct Interconnect {
  memory_map: Vec<MemoryMapEntry>,
  memory: Memory,
  serial: Uart16550<DefaultWriter>,
}

impl Interconnect {
  pub fn new() {
    let mut memory_map: Vec<MemoryMapEntry>;
    memory_map.push(MemoryMapEntry{ address: 0, size: 1024 });
    memory_map.push(MemoryMapEntry{ address: 0x10000000, size: 0x10} );
    Interconnect {
      memory_map: memory_map,
      memory: Memory::new(1024),
      serial: Uart16550::new()
    }
  }
}

struct MemoryMapEntry {
  address: u64,
  size: usize
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn how_to_use_interconnect() {
    
  }
}
