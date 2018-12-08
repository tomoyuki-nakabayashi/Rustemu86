//! Memory mapped system bus.
//! Currently memory map assumes AT&T compatible machine.
use crate::memory::Memory;
use crate::memory_access::{MemoryAccess, MemoryAccessError, Result};

// From x86_64 specification.
const MAX_INSTRUCTION_LENGTH: usize = 15;
// This is temporary.
const MEMORY_SIZE: usize = 0x10000;

pub struct Interconnect {
    memory: Memory,
    serial: Box<dyn MemoryAccess>,
    display: Box<dyn MemoryAccess>,
}

impl Interconnect {
    pub fn new(serial: Box<dyn MemoryAccess>, display: Box<dyn MemoryAccess>) -> Interconnect {
        Interconnect {
            memory: Memory::new(MEMORY_SIZE),
            serial: serial,
            display: display,
        }
    }

    pub fn init_memory(&mut self, program: Vec<u8>, start: usize) {
        self.memory.fill_ram(program, start);
    }

    pub fn fetch_inst_candidate(&self, rip: u64) -> Vec<u8> {
        (0..MAX_INSTRUCTION_LENGTH)
            .map(|x| self.read_u8(rip as usize + x))
            .collect::<Result<Vec<u8>>>()
            .unwrap()
    }
}

impl MemoryAccess for Interconnect {
    fn read_u8(&self, addr: usize) -> Result<u8> {
        match addr {
            0x0...MEMORY_SIZE => self.memory.read_u8(addr as usize),
            0x10000000 => self.serial.read_u8(0),
            _ => Err(MemoryAccessError {}),
        }
    }

    fn write_u8(&mut self, addr: usize, data: u8) -> Result<()> {
        match addr {
            0x0...MEMORY_SIZE => self.memory.write_u8(addr as usize, data),
            0xb8000...0xb8FA0 => self.display.write_u8((addr & 0xfff) as usize, data),
            0x10000000 => self.serial.write_u8(0, data),
            _ => Err(MemoryAccessError {}),
        }
    }

    fn write_u64(&mut self, addr: usize, data: u64) -> Result<()> {
        match addr {
            0x0...MEMORY_SIZE => self.memory.write_u64(addr as usize, data),
            0xb8000...0xb8FA0 => self.display.write_u16((addr & 0xfff) as usize, data as u16),
            0x10000000 => self.serial.write_u8(0, data as u8),
            _ => Err(MemoryAccessError {}),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::uart16550::{self, Target};

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
    fn uart_write() {
        let buffer = vec![0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07];
        let display: Box<dyn MemoryAccess> = Box::new(TestMemory(buffer));
        let serial = uart16550::uart_factory(Target::Buffer);

        let mut interconnect = Interconnect::new(serial, display);
        assert!(interconnect.write_u8(0x10000000, b'h').is_ok());
        assert!(interconnect.write_u8(0x10000000, b'e').is_ok());
        assert!(interconnect.write_u8(0x10000000, b'l').is_ok());
        assert!(interconnect.write_u8(0x10000000, b'l').is_ok());
        assert!(interconnect.write_u8(0x10000000, b'o').is_ok());

        assert_eq!(interconnect.read_u8(0x10000000).unwrap(), b'h');
        assert_eq!(interconnect.read_u8(0x10000000).unwrap(), b'e');
        assert_eq!(interconnect.read_u8(0x10000000).unwrap(), b'l');
        assert_eq!(interconnect.read_u8(0x10000000).unwrap(), b'l');
        assert_eq!(interconnect.read_u8(0x10000000).unwrap(), b'o');
    }

    #[test]
    fn test_init_memory() {
        let buffer = vec![0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07];
        let display: Box<dyn MemoryAccess> = Box::new(TestMemory(buffer));
        let serial = uart16550::uart_factory(Target::Buffer);
        let mut interconnect = Interconnect::new(serial, display);

        let program = vec![0x48, 0xff, 0xc0];
        interconnect.init_memory(program, 0);

        assert_eq!(interconnect.read_u8(0x0).unwrap(), 0x48);
        assert_eq!(interconnect.read_u8(0x1).unwrap(), 0xff);
        assert_eq!(interconnect.read_u8(0x2).unwrap(), 0xc0);
    }
}
