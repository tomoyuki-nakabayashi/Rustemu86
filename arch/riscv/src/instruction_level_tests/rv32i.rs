use crate::isa::abi_name::*;
use crate::riscv::Riscv;
use cpu::model::CpuModel;
use debug::DebugMode;
use peripherals::{memory::Memory, mmio::Mmio};

// Helper for test.
// Simply execute the program with memory.
fn execute_program(program: Vec<u8>) -> Riscv {
    // prepare minimum peripherals.
    let dram = Memory::new_with_filled_ram(&program, program.len());
    let mut mmio = Mmio::empty();
    mmio.add((0, program.len()), Box::new(dram)).unwrap();

    // create object and run.
    let mut riscv = Riscv::fabricate(mmio, DebugMode::Disabled);
    riscv.init();
    let result = riscv.run();

    // check the execution successfully finished.
    assert!(result.is_ok(), "fail to execute program.");

    // return the cpu state.
    riscv
}

#[test]
fn add_imm() {
    let program = vec![
        0x93, 0x80, 0x10, 0x00, // addi ra, zero, 1
        0x13, 0x01, 0xf1, 0xff, // addi sp, sp -1
        0x73, 0x00, 0x50, 0x10, // wfi
    ];

    let riscv = execute_program(program);

    assert_eq!(riscv.get_gpr(ra), 1);
    assert_eq!(riscv.get_gpr(sp) as i32, -1);
}

#[test]
fn or_imm() {
    let program = vec![
        0x93, 0xe0, 0x20, 0x00, // ori ra, zero, 2
        0x13, 0x61, 0xf1, 0xff, // ori sp, sp -1
        0x73, 0x00, 0x50, 0x10, // wfi
    ];

    let riscv = execute_program(program);

    assert_eq!(riscv.get_gpr(ra), 2);
    assert_eq!(riscv.get_gpr(sp), 0xffff_ffff);
}
