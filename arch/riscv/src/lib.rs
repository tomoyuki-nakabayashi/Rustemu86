#[macro_use]
extern crate failure;
#[macro_use]
extern crate enum_primitive;

mod decode;
mod execute;
mod fetch;
mod gpr;
mod isa;
pub mod riscv;

#[cfg(test)]
mod instruction_level_tests;
