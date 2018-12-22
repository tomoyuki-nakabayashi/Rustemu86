extern crate rustemu86;
use peripherals::error::MemoryAccessError;
use peripherals::memory_access::MemoryAccess;
use peripherals::uart16550::{self, Target};
use loader;
use rustemu86::options::EmulationMode;
use std::fs::File;
use std::io::prelude::*;

struct FakeDisplay();
impl MemoryAccess for FakeDisplay {
    fn read_u8(&self, addr: usize) -> Result<u8, MemoryAccessError> {
        unimplemented!()
    }

    fn write_u8(&mut self, addr: usize, data: u8) -> Result<(), MemoryAccessError> {
        unimplemented!()
    }
}

#[test]
fn test_hello_from_rust() {
    let mut reader = loader::load("./tests/bins/hello-x86_64.bin").unwrap();
    let program = loader::map_to_memory(&mut reader).unwrap();
    let display: Box<dyn MemoryAccess> = Box::new(FakeDisplay());
    let serial = uart16550::uart_factory(Target::File("test_hello_from_rust".to_string()));

    let result = rustemu86::start_emulation(
        program,
        EmulationMode::Test("test_hello_from_rust".to_string()),
        serial,
        display,
    );
    assert!(result.is_ok());

    let created_file = File::open("test_hello_from_rust");
    assert!(created_file.is_ok());
    let mut contents = String::new();
    created_file.unwrap().read_to_string(&mut contents).unwrap();
    assert_eq!(contents, "Hello from Rust!\n");
    std::fs::remove_file("test_hello_from_rust").unwrap();
}
