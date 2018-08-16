use peripherals::memory::Memory;
use peripherals::uart16550;
use peripherals::uart16550::Uart16550;
use ::args::EmulationMode;

const MAX_INSTRUCTION_LENGTH: usize = 15;

pub struct Interconnect {
  memory_map: Vec<MemoryMapEntry>,
  memory: Memory,
  serial: Uart16550,
}

impl Interconnect {
  pub fn new(mode: EmulationMode) -> Interconnect {
    let mut memory_map = Vec::<MemoryMapEntry>::new();
    memory_map.push(MemoryMapEntry{ address: 0, size: 1024 });
    memory_map.push(MemoryMapEntry{ address: 0x10000000, size: 0x10} );
    Interconnect {
      memory_map: memory_map,
      memory: Memory::new(1024),
      serial: match mode {
        EmulationMode::Normal => uart16550::uart_factory(uart16550::Target::Stdout),
        _ => uart16550::uart_factory(uart16550::Target::File),
      }
    }
  }

  pub fn init_memory(&mut self, program: Vec<u8>) {
    self.memory.fill_ram(program, 0);
  }

  pub fn fetch_inst_candidate(&self, rip: u64) -> Vec<u8> {
    let mut inst_candidate = Vec::with_capacity(MAX_INSTRUCTION_LENGTH);
    for i in 0..MAX_INSTRUCTION_LENGTH {
      inst_candidate.push(self.read8(rip + i as u64));
    }

    inst_candidate
  }

  pub fn read8(&self, addr: u64) -> u8 {
    match addr {
      0x0...0x200 => self.memory.read8(addr as usize),
      _ => 0,
    }
  }

  pub fn write64(&mut self, addr: u64, data: u64) {
    match addr {
      0x0...0x200 => self.memory.write64(addr as usize, data),
      0x10000000 => self.serial.write(data as u8),
      _ => (),
    }
  }

  pub fn read64(&self, addr: u64) -> u64 {
    match addr {
      0x0...0x200 => self.memory.read64(addr as usize),
      _ => 0,
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
  use std::fs::File;
  use std::io::prelude::*;
  use ::args::EmulationMode;

  #[test]
  fn uart_write() {
    let mut interconnect = Interconnect::new(EmulationMode::IntegrationTest);
    interconnect.write64(0x10000000, 'h' as u64);
    interconnect.write64(0x10000000, 'e' as u64);
    interconnect.write64(0x10000000, 'l' as u64);
    interconnect.write64(0x10000000, 'l' as u64);
    interconnect.write64(0x10000000, 'o' as u64);

    let created_file = File::open("test");
    assert!(created_file.is_ok());
    let mut contents = String::new();
    created_file.unwrap().read_to_string(&mut contents).unwrap();
    assert_eq!(contents, "hello");
  }

  #[test]
  fn test_init_memory() {
    let program = vec![0x48, 0xff, 0xc0];
    let mut interconnect = Interconnect::new(EmulationMode::IntegrationTest);
    interconnect.init_memory(program);

    assert_eq!(interconnect.read8(0x0), 0x48);
    assert_eq!(interconnect.read8(0x1), 0xff);
    assert_eq!(interconnect.read8(0x2), 0xc0);
  }
}
