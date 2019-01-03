#[macro_use]
extern crate failure;
#[macro_use]
extern crate enum_primitive;

mod csr;
mod debug;
mod decode;
mod execute;
mod fetch;
mod gpr;
mod isa;
mod lsu;
pub mod riscv;
pub use self::debug::DebugInterface;
pub use self::isa::abi_name;
pub use self::riscv::Riscv;

#[cfg(test)]
mod instruction_level_tests;
