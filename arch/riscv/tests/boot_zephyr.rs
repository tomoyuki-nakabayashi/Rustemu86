
//! Test to boot Zephyr hello sample.

use self::riscv_tests_memory::Memory;
use cpu::model::CpuModel;
use debug::DebugMode;
use riscv::DebugInterface;
use riscv::Riscv;

const TEST_BINARY: &str = "./tests/zephyr/zephyr.elf";
// these are for targeting qemu_riscv32.
const START_PC: u32 = 0x2040_0000;
const TRAP_VECTOR: u32 = 0x2040_0010;
const ROM_ORIGIN: u32 = 0x2040_0000;
const ROM_LENGTH: u32 = 0xC00000;
const RAM_ORIGIN: u32 = 0x80000000;
const RAM_LENGTH: u32 = 0x4000;

#[test]
fn test_boot_zephyr() {
    let bus = Memory::new(&TEST_BINARY);

    // create object and run.
    let mut riscv = Riscv::fabricate(bus, DebugMode::Disabled);
    riscv.set_pc(START_PC);
    riscv.set_trap_vector(TRAP_VECTOR);
    riscv.init();

    let result = riscv.run();

    assert!(result.is_ok(), "{}", result.unwrap_err());
}

// A memory which hooks `tohost` store to 0x8000_1000 which indicates the finish of test.
mod riscv_tests_memory {
    use super::*;
    use loader::elf_loader::ElfLoader;
    use peripherals::error::MemoryAccessError;
    use peripherals::memory;
    use peripherals::sifive_uart::SifiveUart;
    use peripherals::memory_access::{self, MemoryAccess};

    // riscv-tests has only two segment of memory.
    pub struct Memory {
        rom: memory::Memory,
        ram: memory::Memory,
        clint: memory::Memory,
        plic0 : memory::Memory,
        gpio0: memory::Memory,
        uart0: SifiveUart,
    }

    impl Memory {
        pub fn new(filename: &str) -> Memory {
            let loader = ElfLoader::try_new(filename).unwrap();
            // There are three sections in hello sample in Zephyr.
            let layouts = loader.memory_image();
            let vector = &layouts[0];
            let data_rom = &layouts[1];
            let bss = &layouts[2];

            let mut rom = memory::Memory::new(ROM_LENGTH as usize);
            let mut ram = memory::Memory::new(RAM_LENGTH as usize);

            rom.fill_ram(vector.binary_as_ref(), vector.start_addr() - ROM_ORIGIN as usize);
            rom.fill_ram(data_rom.binary_as_ref(), data_rom.start_addr() - ROM_ORIGIN as usize);
            ram.fill_ram(bss.binary_as_ref(), bss.start_addr() - RAM_ORIGIN as usize);

            Memory {
                rom,
                ram,
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
                0x0200_0000...0x0200_ffff => self.clint.read_u8(addr - 0x0200_0000),
                0x0c00_0000...0x0c30_0000 => self.plic0.read_u8(addr - 0x0c00_0000),
                0x1001_2000...0x1001_2fff => self.gpio0.read_u8(addr - 0x1001_2000),
                0x1001_3000...0x1001_3fff => self.uart0.read_u8(addr - 0x1001_3000),
                0x2040_0000...0x20ff_ffff => self.rom.read_u8(addr - 0x2040_0000),
                0x8000_0000...0x8000_3fff => self.ram.read_u8(addr - 0x8000_0000),
                _ => Err(MemoryAccessError::DeviceNotMapped { addr }),
            }
        }

        fn read_u32(&self, addr: usize) -> memory_access::Result<u32> {
            match addr {
                0x0200_0000...0x0200_ffff => self.clint.read_u32(addr - 0x0200_0000),
                0x0c00_0000...0x0c30_0000 => self.plic0.read_u32(addr - 0x0c00_0000),
                0x1001_2000...0x1001_2fff => self.gpio0.read_u32(addr - 0x1001_2000),
                0x1001_3000...0x1001_3fff => self.uart0.read_u32(addr - 0x1001_3000),
                0x2040_0000...0x20ff_ffff => self.rom.read_u32(addr - 0x2040_0000),
                0x8000_0000...0x8000_3fff => self.ram.read_u32(addr - 0x8000_0000),
                _ => Err(MemoryAccessError::DeviceNotMapped { addr }),
            }
        }

        fn write_u8(&mut self, addr: usize, data: u8) -> memory_access::Result<()> {
            match addr {
                0x0200_0000...0x0200_ffff => self.clint.write_u8(addr - 0x0200_0000, data),
                0x0c00_0000...0x0c30_0000 => self.plic0.write_u8(addr - 0x0c00_0000, data),
                0x1001_2000...0x1001_2fff => self.gpio0.write_u8(addr - 0x1001_2000, data),
                0x1001_3000...0x1001_3fff => self.uart0.write_u8(addr - 0x1001_3000, data),
                0x8000_0000...0x8000_3fff => self.ram.write_u8(addr - 0x8000_0000, data),
                _ => Err(MemoryAccessError::DeviceNotMapped { addr }),
            }
        }

        fn write_u32(&mut self, addr: usize, data: u32) -> memory_access::Result<()> {
            match addr {
                0x0200_0000...0x0200_ffff => self.clint.write_u32(addr - 0x0200_0000, data),
                0x0c00_0000...0x0c30_0000 => self.plic0.write_u32(addr - 0x0c00_0000, data),
                0x1001_2000...0x1001_2fff => self.gpio0.write_u32(addr - 0x1001_2000, data),
                0x1001_3000...0x1001_3fff => self.uart0.write_u32(addr - 0x1001_3000, data),
                0x8000_0000...0x8000_3fff => self.ram.write_u32(addr - 0x8000_0000, data),
                _ => Err(MemoryAccessError::DeviceNotMapped { addr }),
            }
        }
    }
}