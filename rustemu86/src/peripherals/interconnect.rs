pub struct Interconnect {
  memory_map: Vec<MemoryMapEntry>,
}

impl Interconnect {
  pub fn map_memory(&mut self, addr: u64, size: usize) {}
}

struct MemoryMapEntry {
  address: u64,
  size: usize
}
