
//! Test to boot Zephyr hello sample.

use self::riscv_tests_memory::Memory;
use cpu::model::CpuModel;
use debug::DebugMode;
use riscv::DebugInterface;
use riscv::Riscv;

const TEST_BINARY: &str = "./tests/zephyr/zephyr.elf";

#[test]
fn test_boot_zephyr() {
    let bus = Memory::new(&TEST_BINARY);

    // create object and run.
    let mut riscv = Riscv::fabricate(bus, DebugMode::Disabled);
    riscv.set_pc(0x2040_0000);
    riscv.init();

    let result = riscv.run();

    assert!(result.is_ok(), "{}", result.unwrap_err());
}

// A memory which hooks `tohost` store to 0x8000_1000 which indicates the finish of test.
mod riscv_tests_memory {
    use loader::elf_loader::ElfLoader;
    use peripherals::error::MemoryAccessError;
    use peripherals::memory;
    use peripherals::sifive_uart::SifiveUart;
    use peripherals::memory_access::{self, MemoryAccess};

    // riscv-tests has only two segment of memory.
    pub struct Memory {
        vector: memory::Memory,
        data_rom: memory::Memory,
        data: memory::Memory,
        bss: memory::Memory,
        clint: memory::Memory,
        plic0 : memory::Memory,
        gpio0: memory::Memory,
        uart0: SifiveUart,
    }

    impl Memory {
        pub fn new(filename: &str) -> Memory {
            let loader = ElfLoader::try_new(filename).unwrap();
            let layouts = loader.memory_image();
            let vector = &layouts[0];
            let data_rom = &layouts[1];
            let bss = &layouts[2];

            Memory {
                vector: memory::Memory::new_with_filled_ram(vector.binary_as_ref(), vector.size()),
                data_rom: memory::Memory::new_with_filled_ram(data_rom.binary_as_ref(), data_rom.size()),
                data: memory::Memory::new(data_rom.size()),
                bss: memory::Memory::new_with_filled_ram(bss.binary_as_ref(), bss.size()),
                clint: memory::Memory::new(0x10000),
                plic0: memory::Memory::new(0x300000),
                gpio0: memory::Memory::new(0x1000),
                uart0: SifiveUart::new(),
            }
        }
    }

    // riscv-tests let us know finishing the test case by storing something into 0x8000_1000.
    impl MemoryAccess for Memory {
        fn read_u8(&self, addr: usize) -> memory_access::Result<u8> {
            match addr {
                0x1001_2000...0x1001_2fff => self.gpio0.read_u8(addr - 0x1001_2000),
                0x1001_3000...0x1001_3fff => self.uart0.read_u8(addr - 0x1001_3000),
                0x0200_0000...0x0200_ffff => self.clint.read_u8(addr - 0x0200_0000),
                0x0c00_0000...0x0c30_0000 => self.plic0.read_u8(addr - 0x0c00_0000),
                0x2040_0000...0x2040_36c7 => self.vector.read_u8(addr - 0x2040_0000),
                0x2040_36c8...0x2040_3747 => self.data_rom.read_u8(addr - 0x2040_36c8),
                0x8000_0000...0x8000_007f => self.data.read_u8(addr - 0x8000_0000),
                0x8000_0080...0x8000_0fcf => self.bss.read_u8(addr - 0x8000_0080),
                _ => Err(MemoryAccessError::DeviceNotMapped { addr }),
            }
        }

        fn read_u32(&self, addr: usize) -> memory_access::Result<u32> {
            match addr {
                0x1001_2000...0x1001_2fff => self.gpio0.read_u32(addr - 0x1001_2000),
                0x1001_3000...0x1001_3fff => self.uart0.read_u32(addr - 0x1001_3000),
                0x0200_0000...0x0200_ffff => self.clint.read_u32(addr - 0x0200_0000),
                0x0c00_0000...0x0c30_0000 => self.plic0.read_u32(addr - 0x0c00_0000),
                0x2040_0000...0x2040_36c7 => self.vector.read_u32(addr - 0x2040_0000),
                0x2040_36c8...0x2040_3747 => self.data_rom.read_u32(addr - 0x2040_36c8),
                0x8000_0000...0x8000_007f => self.data.read_u32(addr - 0x8000_0000),
                0x8000_0080...0x8000_0fcf => self.bss.read_u32(addr - 0x8000_0080),
                _ => Err(MemoryAccessError::DeviceNotMapped { addr }),
            }
        }

        fn write_u8(&mut self, addr: usize, data: u8) -> memory_access::Result<()> {
            match addr {
                0x1001_2000...0x1001_2fff => self.gpio0.write_u8(addr - 0x1001_2000, data),
                0x1001_3000...0x1001_3fff => self.uart0.write_u8(addr - 0x1001_3000, data),
                0x0c00_0000...0x0c30_0000 => self.plic0.write_u8(addr - 0x0c00_0000, data),
                0x0200_0000...0x0200_ffff => self.clint.write_u8(addr - 0x0200_0000, data),
                0x8000_0000...0x8000_007f => self.data.write_u8(addr - 0x8000_0000, data),
                0x8000_0080...0x8000_0fcf => self.bss.write_u8(addr - 0x8000_0080, data),
                _ => Err(MemoryAccessError::DeviceNotMapped { addr }),
            }
        }

        fn write_u32(&mut self, addr: usize, data: u32) -> memory_access::Result<()> {
            match addr {
                0x1001_2000...0x1001_2fff => self.gpio0.write_u32(addr - 0x1001_2000, data),
                0x1001_3000...0x1001_3fff => self.uart0.write_u32(addr - 0x1001_3000, data),
                0x0c00_0000...0x0c30_0000 => self.plic0.write_u32(addr - 0x0c00_0000, data),
                0x0200_0000...0x0200_ffff => self.clint.write_u32(addr - 0x0200_0000, data),
                0x8000_0000...0x8000_007f => self.data.write_u32(addr - 0x8000_0000, data),
                0x8000_0080...0x8000_0fcf => self.bss.write_u32(addr - 0x8000_0080, data),
                _ => Err(MemoryAccessError::DeviceNotMapped { addr }),
            }
        }
    }
}