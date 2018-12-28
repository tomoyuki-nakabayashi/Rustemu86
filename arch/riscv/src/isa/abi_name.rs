//! ABI Name alias of RISC-V Calling convention.

#![allow(dead_code, non_upper_case_globals)]
/// hardwired zero
pub const zero: u32 = 0;
/// return address
pub const ra: u32 = 1;
/// stack pointer
pub const sp: u32 = 2;
/// global pointer
pub const gp: u32 = 3;
/// thread pointer
pub const tp: u32 = 4;
/// temporary registers
pub const t0: u32 = 5;
pub const t1: u32 = 6;
pub const t2: u32 = 7;
/// saved register / frame pointer
pub const s0: u32 = 8;
pub const fp: u32 = 8;
/// saved register
pub const s1: u32 = 9;
/// function arguments / return values
pub const a0: u32 = 10;
pub const a1: u32 = 11;
/// function arguments
pub const a2: u32 = 12;
pub const a3: u32 = 13;
pub const a4: u32 = 14;
pub const a5: u32 = 15;
pub const a6: u32 = 16;
pub const a7: u32 = 17;
/// saved registers
pub const s2: u32 = 18;
pub const s3: u32 = 19;
pub const s4: u32 = 20;
pub const s5: u32 = 21;
pub const s6: u32 = 22;
pub const s7: u32 = 23;
pub const s8: u32 = 24;
pub const s9: u32 = 25;
pub const s10: u32 = 26;
pub const s11: u32 = 27;
///temporary registers
pub const t3: u32 = 28;
pub const t4: u32 = 29;
pub const t5: u32 = 30;
pub const t6: u32 = 31;
