//! tests using [riscv-tests](https://github.com/riscv/riscv-tests/tree/master/isa/rv32ui)
//! This file executes rv32ui test cases.

use self::riscv_tests_memory::Memory;
use cpu::model::CpuModel;
use debug::DebugMode;
use riscv::abi_name::*;
use riscv::DebugInterface;
use riscv::Riscv;

const TEST_BASE: &str = "./tests/riscv_tests/rv32ui-p-";

macro_rules! riscv_test {
    ($name: ident, $test_case: expr) => {
        #[test]
        fn $name() {
            let file = TEST_BASE.to_owned() + $test_case;
            riscv_test_elf(&file);
        }
    };
}

riscv_test!(simple, "simple");
riscv_test!(add, "add");
riscv_test!(addi, "addi");
riscv_test!(and, "and");
riscv_test!(andi, "andi");
riscv_test!(auipc, "auipc");
riscv_test!(beq, "beq");
riscv_test!(bge, "bge");
riscv_test!(bgeu, "bgeu");
riscv_test!(blt, "blt");
riscv_test!(bltu, "bltu");
riscv_test!(bne, "bne");
//riscv_test!(fence_i, "fence_i");
riscv_test!(jal, "jal");
riscv_test!(jalr, "jalr");
//riscv_test!(lb, "lb");
//riscv_test!(lbu, "lbu");
//riscv_test!(lh, "lh");
//riscv_test!(lhu, "lhu");
riscv_test!(lui, "lui");
//riscv_test!(lw, "lw");
riscv_test!(or, "or");
riscv_test!(ori, "ori");
//riscv_test!(sb, "sb");
//riscv_test!(sh, "sh");
riscv_test!(sll, "sll");
riscv_test!(slli, "slli");
riscv_test!(slt, "slt");
riscv_test!(slti, "slti");
riscv_test!(sltiu, "sltiu");
riscv_test!(sltu, "sltu");
riscv_test!(sra, "sra");
riscv_test!(srai, "srai");
riscv_test!(srl, "srl");
riscv_test!(srli, "srli");
riscv_test!(sub, "sub");
//riscv_test!(sw, "sw");
riscv_test!(xor, "xor");
riscv_test!(xori, "xori");

fn riscv_test_elf(filename: &str) {
    let bus = Memory::new(&filename);

    // create object and run.
    let mut riscv = Riscv::fabricate(bus, DebugMode::Disabled);
    riscv.set_pc(0x8000_0000);
    riscv.init();

    let result = riscv.run();

    // Confirm exit because of memory access error.
    assert!(result.is_err());

    // Check success or not.
    assert_eq!(riscv.get_gpr(gp), 1);
}

// A memory which hooks `tohost` store to 0x8000_1000 which indicates the finish of test.
mod riscv_tests_memory {
    use loader::elf_loader::ElfLoader;
    use peripherals::error::MemoryAccessError;
    use peripherals::memory;
    use peripherals::memory_access::{self, MemoryAccess};

    pub struct Memory {
        memory: memory::Memory,
    }

    impl Memory {
        pub fn new(filename: &str) -> Memory {
            let loader = ElfLoader::try_new(filename).unwrap();
            let layouts = loader.memory_image();
            let text = &layouts[0];

            Memory {
                memory: memory::Memory::new_with_filled_ram(text.binary_as_ref(), text.size()),
            }
        }
    }

    impl MemoryAccess for Memory {
        fn read_u8(&self, addr: usize) -> memory_access::Result<u8> {
            if addr == 0x8000_1000 {
                return Err(MemoryAccessError::DeviceNotMapped { addr });
            } else {
                self.memory.read_u8(addr - 0x8000_0000)
            }
        }

        fn write_u8(&mut self, addr: usize, data: u8) -> memory_access::Result<()> {
            if addr == 0x8000_1000 {
                return Err(MemoryAccessError::DeviceNotMapped { addr });
            } else {
                self.memory.write_u8(addr - 0x8000_0000, data)
            }
        }
    }
}
