#[macro_use]
extern crate failure;
#[macro_use]
extern crate enum_primitive;

mod fetch;
mod decode;
mod execute;
mod gpr;
mod isa;
pub mod riscv;
