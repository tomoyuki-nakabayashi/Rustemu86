#[macro_use]
extern crate failure;

pub mod riscv;
mod fetch;
mod decode;
mod execute;
mod isa;