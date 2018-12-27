//! ABI Name alias of RISC-V Calling convention.

#![allow(dead_code, non_upper_case_globals)]
/// hardwired zero
pub const zero: usize = 0;
/// return address
pub const ra: usize = 1;
/// stack pointer
pub const sp: usize = 2;
/// global pointer
pub const gp: usize = 3;
/// thread pointer
pub const tp: usize = 4;
/// temporary registers
pub const t0: usize = 5;
pub const t1: usize = 6;
pub const t2: usize = 7;
/// saved register / frame pointer
pub const s0: usize = 8;
pub const fp: usize = 8;
/// saved register
pub const s1: usize = 9;
/// function arguments / return values
pub const a0: usize = 10;
pub const a1: usize = 11;
/// function arguments
pub const a2: usize = 12;
pub const a3: usize = 13;
pub const a4: usize = 14;
pub const a5: usize = 15;
pub const a6: usize = 16;
pub const a7: usize = 17;
/// saved registers
pub const s2: usize = 18;
pub const s3: usize = 19;
pub const s4: usize = 20;
pub const s5: usize = 21;
pub const s6: usize = 22;
pub const s7: usize = 23;
pub const s8: usize = 24;
pub const s9: usize = 25;
pub const s10: usize = 26;
pub const s11: usize = 27;
///temporary registers
pub const t3: usize = 28;
pub const t4: usize = 29;
pub const t5: usize = 30;
pub const t6: usize = 31;