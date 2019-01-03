use riscv::Riscv;
use riscv::abi_name::*;
use riscv::DebugInterface;
use cpu::model::CpuModel;
use debug::DebugMode;
use peripherals::{memory::Memory, mmio::Mmio};

use std::fs::File;
use std::io::BufReader;
use std::io::Read;

#[test]
fn riscv_tests_simple() {
    riscv_tests(
        "./tests/riscv_tests/rv32ui-p-simple.bin",
    );
}

fn riscv_tests(filename: &str) {
    let program = load(filename);
    let dram = Memory::new_with_filled_ram(&program, 0x1000);
    let mut mmio = Mmio::empty();
    mmio.add((0x8000_0000, program.len()), Box::new(dram))
        .unwrap();

    // create object and run.
    let mut riscv = Riscv::fabricate(mmio, DebugMode::Disabled);
    riscv.set_pc(0x8000_0000);
    riscv.init();

    let result = riscv.run();

    // Confirm exit because of memory access error.
    assert!(result.is_err());

    // Check success or not.
    assert_eq!(riscv.get_gpr(gp), 1);
}

fn load(filename: &str) -> Vec<u8> {
    let mut reader = BufReader::new(File::open(&filename).unwrap());
    let mut program = Vec::new();
    reader.read_to_end(&mut program).unwrap();

    program
}