use args::EmulationMode;
use display::GtkVgaTextBuffer;
use peripherals::memory_access::{MemoryAccess, MemoryAccessError, Result};
use peripherals::memory::Memory;
use peripherals::uart16550;
use peripherals::uart16550::Target;
use peripherals::uart16550::Uart16550;

const MAX_INSTRUCTION_LENGTH: usize = 15;
const MEMORY_SIZE: usize = 0x10000;

pub struct Interconnect {
    memory: Memory,
    serial: Uart16550,
    vga_text_buffer: GtkVgaTextBuffer,
}

impl Interconnect {
    pub fn new(mode: EmulationMode, vga_text_buffer: GtkVgaTextBuffer) -> Interconnect {
        Interconnect {
            memory: Memory::new(MEMORY_SIZE),
            serial: match mode {
                EmulationMode::Test(path) => uart16550::uart_factory(Target::File(path)),
                _ => uart16550::uart_factory(uart16550::Target::Stdout),
            },
            vga_text_buffer: vga_text_buffer,
        }
    }

    pub fn init_memory(&mut self, program: Vec<u8>, start: usize) {
        self.memory.fill_ram(program, start);
    }

    pub fn fetch_inst_candidate(&self, rip: u64) -> Vec<u8> {
        (0..MAX_INSTRUCTION_LENGTH)
            .map(|x| self.read_u8(rip as usize + x))
            .collect::<Result<Vec<u8>>>().unwrap()
    }
}

impl MemoryAccess for Interconnect {
    fn read_u8(&self, addr: usize) -> Result<u8> {
        match addr {
            0x0...MEMORY_SIZE => self.memory.read_u8(addr as usize),
            _ => Err(MemoryAccessError {}),
        }
    }

    fn write_u8(&mut self, addr: usize, data: u8) -> Result<()> {
        match addr {
            0x0...MEMORY_SIZE => self.memory.write_u8(addr as usize, data),
            0xb8000...0xb8FA0 => self
                .vga_text_buffer
                .write_u8((addr & 0xfff) as usize, data),
            0x10000000 => self.serial.write_u8(0, data),
            _ => Err(MemoryAccessError {}),
        }
    }

    fn write_u64(&mut self, addr: usize, data: u64) -> Result<()> {
        match addr {
            0x0...MEMORY_SIZE => self.memory.write_u64(addr as usize, data),
            0xb8000...0xb8FA0 => self
                .vga_text_buffer
                .write_u16((addr & 0xfff) as usize, data as u16),
            0x10000000 => self.serial.write_u8(0, data as u8),
            _ => Err(MemoryAccessError {}),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use args::EmulationMode;
    use display::GtkVgaTextBuffer;
    use std::fs::File;
    use std::io::prelude::*;

    #[test]
    fn uart_write() {
        let mut interconnect = Interconnect::new(
            EmulationMode::Test("test".to_string()),
            GtkVgaTextBuffer::new(),
        );
        assert!(interconnect.write_u8(0x10000000, 'h' as u8).is_ok());
        assert!(interconnect.write_u8(0x10000000, 'e' as u8).is_ok());
        assert!(interconnect.write_u8(0x10000000, 'l' as u8).is_ok());
        assert!(interconnect.write_u8(0x10000000, 'l' as u8).is_ok());
        assert!(interconnect.write_u8(0x10000000, 'o' as u8).is_ok());

        let created_file = File::open("test");
        assert!(created_file.is_ok());
        let mut contents = String::new();
        created_file.unwrap().read_to_string(&mut contents).unwrap();
        assert_eq!(contents, "hello");
    }

    #[test]
    fn test_init_memory() {
        let program = vec![0x48, 0xff, 0xc0];
        let mut interconnect = Interconnect::new(
            EmulationMode::Test("test".to_string()),
            GtkVgaTextBuffer::new(),
        );
        interconnect.init_memory(program, 0);

        assert_eq!(interconnect.read_u8(0x0).unwrap(), 0x48);
        assert_eq!(interconnect.read_u8(0x1).unwrap(), 0xff);
        assert_eq!(interconnect.read_u8(0x2).unwrap(), 0xc0);
    }
}
