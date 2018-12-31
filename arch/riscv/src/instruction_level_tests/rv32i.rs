use crate::isa::abi_name::*;
use crate::riscv::Riscv;
use cpu::model::CpuModel;
use debug::DebugMode;
use peripherals::{memory::Memory, mmio::Mmio};

// # Integer Regiser-Immediate Instructions

#[test]
fn add_imm() {
    let program = vec![
        0x93, 0x80, 0x10, 0x00, // addi ra, zero, 1
        0x13, 0x01, 0xf1, 0xff, // addi sp, sp -1
        0x73, 0x00, 0x50, 0x10, // wfi
    ];

    let riscv = execute_program(program);

    assert_eq!(riscv.get_gpr(ra), 1);
    assert_eq!(riscv.get_gpr(sp), 0xffff_ffff);
}

#[test]
fn slti_imm() {
    let program = vec![
        0x13, 0xa1, 0x10, 0x00, // slti sp, ra, 1
        0x93, 0xa1, 0xf0, 0xff, // slti gp, ra -1
        0x73, 0x00, 0x50, 0x10, // wfi
    ];

    let riscv = execute_program(program);

    assert_eq!(riscv.get_gpr(sp), 1);
    assert_eq!(riscv.get_gpr(gp), 0);
}

#[test]
fn sltiu_imm() {
    let program = vec![
        0x13, 0xb1, 0x10, 0x00, // sltiu sp, ra, 1
        0x93, 0xb1, 0xf0, 0xff, // sltiu gp, ra -1 indicates less than MAX of u32
        0x73, 0x00, 0x50, 0x10, // wfi
    ];

    let riscv = execute_program(program);

    assert_eq!(riscv.get_gpr(sp), 1);
    assert_eq!(riscv.get_gpr(gp), 1);
}

#[test]
fn and_imm() {
    let program = vec![
        0x93, 0x80, 0xf0, 0xff, // addi ra, zero, -1
        0x13, 0xf1, 0xa0, 0x0a, // andi sp, ra 0x0aa
        0x93, 0xf1, 0x50, 0xf5, // andi gp, ra 0xf55
        0x73, 0x00, 0x50, 0x10, // wfi
    ];

    let riscv = execute_program(program);

    assert_eq!(riscv.get_gpr(sp), 0xaa);
    assert_eq!(riscv.get_gpr(gp), 0xffff_ff55);
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

#[test]
fn xor_imm() {
    let program = vec![
        0x93, 0x00, 0xf0, 0xff, // xori ra, zero -1
        0x13, 0xc1, 0xf0, 0xff, // xori sp, ra, -1
        0x73, 0x00, 0x50, 0x10, // wfi
    ];

    let riscv = execute_program(program);

    assert_eq!(riscv.get_gpr(ra), 0xffff_ffff);
    assert_eq!(riscv.get_gpr(sp), 0x0000_0000);
}

#[test]
fn sll_imm() {
    let program = vec![
        0x93, 0x80, 0x10, 0x00, // addi ra, zero, 1
        0x93, 0x90, 0x50, 0x00, // slli ra, ra 5
        0x73, 0x00, 0x50, 0x10, // wfi
    ];

    let riscv = execute_program(program);

    assert_eq!(riscv.get_gpr(ra), 1u32 << 5);
}

#[test]
fn srl_imm() {
    let program = vec![
        0x93, 0x80, 0x00, 0x10, // addi ra, zero, 0x100
        0x13, 0xd1, 0x50, 0x00, // srli sp, ra 5
        0x93, 0x00, 0xf0, 0xff, // addi ra, zero, -1
        0x93, 0xd1, 0x40, 0x00, // srli gp, ra 4
        0x73, 0x00, 0x50, 0x10, // wfi
    ];

    let riscv = execute_program(program);

    assert_eq!(riscv.get_gpr(sp), 0x100u32 >> 5);
    assert_eq!(riscv.get_gpr(gp), 0x0fff_ffff);
}

#[test]
fn sra_imm() {
    let program = vec![
        0x93, 0x00, 0x00, 0xf0, // addi ra, zero, 0xf00
        0x13, 0xd1, 0x50, 0x40, // srai sp, ra 5
        0x73, 0x00, 0x50, 0x10, // wfi
    ];

    let riscv = execute_program(program);

    assert_eq!(riscv.get_gpr(sp), 0xffff_fff8);
}

#[test]
fn lui() {
    let program = vec![
        0xb7, 0x50, 0x34, 0x12, // lui ra, 0x12345
        0x73, 0x00, 0x50, 0x10, // wfi
    ];

    let riscv = execute_program(program);

    assert_eq!(riscv.get_gpr(ra), 0x1234_5000);
}

#[test]
fn auipc() {
    let program = vec![
        0x13, 0x00, 0x00, 0x00, // nop
        0x97, 0x50, 0x34, 0x12, // auipc ra, 0x12345
        0x73, 0x00, 0x50, 0x10, // wfi
    ];

    let riscv = execute_program(program);

    assert_eq!(riscv.get_gpr(ra), 0x1234_5004);
}

// # Integer Regiser-Register Instructions

#[test]
fn add() {
    let program = vec![
        0xb3, 0x80, 0x10, 0x00, // add ra, ra, ra
        0x33, 0x01, 0x11, 0x00, // add sp, sp, ra
        0x73, 0x00, 0x50, 0x10, // wfi
    ];

    let initializer = |riscv: &mut Riscv| {
        riscv.set_gpr(ra, 2);
        riscv.set_gpr(sp, 0x7fff_ffff);
    };
    let riscv = execute_program_init_by(program, initializer);

    assert_eq!(riscv.get_gpr(ra), 4);
    // ignore overflow
    assert_eq!(riscv.get_gpr(sp), 0x8000_0003);
}

#[test]
fn sub() {
    let program = vec![
        0x33, 0x01, 0x11, 0x40, // sub sp, sp, ra
        0x33, 0x01, 0x11, 0x40, // sub sp, sp, ra
        0x73, 0x00, 0x50, 0x10, // wfi
    ];

    let initializer = |riscv: &mut Riscv| {
        riscv.set_gpr(ra, 2);
        riscv.set_gpr(sp, 3);
    };
    let riscv = execute_program_init_by(program, initializer);

    // ignore overflow
    assert_eq!(riscv.get_gpr(sp), 0xffff_ffff);
}

#[test]
fn slt() {
    let program = vec![
        0x33, 0x21, 0x10, 0x00, // slt sp, zero, ra
        0xb3, 0xa1, 0x00, 0x00, // slt gp, ra, zero
        0x73, 0x00, 0x50, 0x10, // wfi
    ];

    let initializer = |riscv: &mut Riscv| {
        riscv.set_gpr(ra, 0xffff_ffff);
    };
    let riscv = execute_program_init_by(program, initializer);

    assert_eq!(riscv.get_gpr(sp), 0);
    assert_eq!(riscv.get_gpr(gp), 1);
}

#[test]
fn sltu() {
    let program = vec![
        0x33, 0x31, 0x10, 0x00, // sltu sp, zero, ra
        0xb3, 0xb1, 0x00, 0x00, // sltu gp, ra, zero
        0x73, 0x00, 0x50, 0x10, // wfi
    ];

    let initializer = |riscv: &mut Riscv| {
        riscv.set_gpr(ra, 0xffff_ffff);
    };
    let riscv = execute_program_init_by(program, initializer);

    assert_eq!(riscv.get_gpr(sp), 1);
    assert_eq!(riscv.get_gpr(gp), 0);
}

#[test]
fn and() {
    let program = vec![
        0xb3, 0xf1, 0x20, 0x00, // and gp, ra, sp
        0x73, 0x00, 0x50, 0x10, // wfi
    ];

    let initializer = |riscv: &mut Riscv| {
        riscv.set_gpr(ra, 0xff00_ff00);
        riscv.set_gpr(sp, 0xaaaa_aaaa);
    };
    let riscv = execute_program_init_by(program, initializer);

    assert_eq!(riscv.get_gpr(gp), 0xaa00_aa00);
}

#[test]
fn or() {
    let program = vec![
        0xb3, 0xe1, 0x20, 0x00, // or gp, ra, sp
        0x73, 0x00, 0x50, 0x10, // wfi
    ];

    let initializer = |riscv: &mut Riscv| {
        riscv.set_gpr(ra, 0xff00_ff00);
        riscv.set_gpr(sp, 0xaaaa_aaaa);
    };
    let riscv = execute_program_init_by(program, initializer);

    assert_eq!(riscv.get_gpr(gp), 0xffaa_ffaa);
}

#[test]
fn xor() {
    let program = vec![
        0xb3, 0xc1, 0x20, 0x00, // xor gp, ra, sp
        0x73, 0x00, 0x50, 0x10, // wfi
    ];

    let initializer = |riscv: &mut Riscv| {
        riscv.set_gpr(ra, 0xff00_ff00);
        riscv.set_gpr(sp, 0xaaaa_aaaa);
    };
    let riscv = execute_program_init_by(program, initializer);

    assert_eq!(riscv.get_gpr(gp), 0x55aa_55aa);
}

#[test]
fn sll() {
    let program = vec![
        0xb3, 0x91, 0x20, 0x00, // sll gp, ra, sp
        0x73, 0x00, 0x50, 0x10, // wfi
    ];

    let initializer = |riscv: &mut Riscv| {
        riscv.set_gpr(ra, 0xff00_ff00);
        riscv.set_gpr(sp, 5);
    };
    let riscv = execute_program_init_by(program, initializer);

    assert_eq!(riscv.get_gpr(gp), 0xe01f_e000);
}

#[test]
fn srl() {
    let program = vec![
        0xb3, 0xd1, 0x20, 0x00, // srl gp, ra, sp
        0x73, 0x00, 0x50, 0x10, // wfi
    ];

    let initializer = |riscv: &mut Riscv| {
        riscv.set_gpr(ra, 0xff00_ff00);
        riscv.set_gpr(sp, 5);
    };
    let riscv = execute_program_init_by(program, initializer);

    assert_eq!(riscv.get_gpr(gp), 0x07f8_07f8);
}

#[test]
fn sra() {
    let program = vec![
        0xb3, 0xd1, 0x20, 0x40, // sra gp, ra, sp
        0x73, 0x00, 0x50, 0x10, // wfi
    ];

    let initializer = |riscv: &mut Riscv| {
        riscv.set_gpr(ra, 0xff00_ff00);
        riscv.set_gpr(sp, 5);
    };
    let riscv = execute_program_init_by(program, initializer);

    assert_eq!(riscv.get_gpr(gp), 0xfff8_07f8);
}

#[test]
fn jal() {
    let program = vec![
        0xef, 0x00, 0x80, 0x00, // jal ra, 0x8
        0x93, 0xe0, 0x00, 0x00, // ori ra, zero, 0
        0x73, 0x00, 0x50, 0x10, // wfi
    ];

    let riscv = execute_program(program);

    assert_eq!(riscv.get_gpr(ra), 0x4);
    assert_eq!(riscv.get_pc(), 0xc);
}

#[test]
fn beq() {
    let program = vec![
        0x63, 0x84, 0x20, 0x00, // beq ra, sp, 0x4
        0x93, 0xe0, 0x00, 0x00, // ori ra, zero, 0
        0x73, 0x00, 0x50, 0x10, // wfi
    ];

    let initializer = |riscv: &mut Riscv| {
        riscv.set_gpr(ra, 1);
        riscv.set_gpr(sp, 1);
    };
    let riscv = execute_program_init_by(program, initializer);

    assert_eq!(riscv.get_pc(), 0xc);
}

// Load second instruction in program, i.e., wfi.
#[test]
fn load() {
    let program = vec![
        0x83, 0x20, 0x41, 0x00, // lw ra, 4(sp)
        0x73, 0x00, 0x50, 0x10, // wfi
    ];

    let riscv = execute_program(program);

    assert_eq!(riscv.get_gpr(ra), 0x1050_0073);
}

// Store to third instruction position.
#[test]
fn store() {
    let program = vec![
        0x23, 0x26, 0x11, 0x00, // sw ra, 0xc(sp)
        0x83, 0x20, 0xc1, 0x00, // lw ra, 0xc(sp)
        0x73, 0x00, 0x50, 0x10, // wfi
        0xff, 0xff, 0xff, 0xff, // dummy initial data at address 0xc.
    ];

    let riscv = execute_program(program);

    assert_eq!(riscv.get_gpr(ra), 0);
}

// Test for dummy implementation. This instruction does no-effect in this emulator.
// This is required for pipelined processor.
#[test]
fn fence_i() {
    let program = vec![
        0x0f, 0x10, 0x00, 0x00, // fence.i
        0x73, 0x00, 0x50, 0x10, // wfi
    ];

    // just check whether it successfully finishes execution.
    execute_program(program);
}

// Helper for test.
// Simply execute the program with memory.
fn execute_program(program: Vec<u8>) -> Riscv {
    let mut riscv = create_riscv_cpu(program);
    let result = riscv.run();

    // check the execution successfully finished.
    assert!(result.is_ok(), "{}", result.unwrap_err());

    // return the cpu state.
    riscv
}

// Helper for test.
fn execute_program_init_by(program: Vec<u8>, initializer: fn(&mut Riscv)) -> Riscv {
    let mut riscv = create_riscv_cpu(program);
    initializer(&mut riscv);
    let result = riscv.run();

    // check the execution successfully finished.
    assert!(result.is_ok(), "{}", result.unwrap_err());

    // return the cpu state.
    riscv
}

// helper for test.
fn create_riscv_cpu(program: Vec<u8>) -> Riscv {
    // prepare minimum peripherals.
    let dram = Memory::new_with_filled_ram(&program, program.len());
    let mut mmio = Mmio::empty();
    mmio.add((0, program.len()), Box::new(dram)).unwrap();

    // create object and run.
    let mut riscv = Riscv::fabricate(mmio, DebugMode::Disabled);
    riscv.init();

    riscv
}
