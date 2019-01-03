#[macro_use]
extern crate failure;
#[macro_use]
extern crate enum_primitive;

mod csr;
mod decode;
mod execute;
mod fetch;
mod gpr;
mod isa;
mod lsu;
mod debug;
pub mod riscv;
pub use self::riscv::Riscv as Riscv;
pub use self::debug::DebugInterface;
pub use self::isa::abi_name as abi_name;

#[cfg(test)]
mod instruction_level_tests;
