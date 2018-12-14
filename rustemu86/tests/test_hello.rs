extern crate rustemu86;
use rustemu86::options::EmulationMode;
use peripherals::interconnect::Interconnect;
use peripherals::memory_access::{MemoryAccess, MemoryAccessError};
use peripherals::uart16550::{self, Target};
use std::fs::File;
use std::io::prelude::*;

struct FakeDisplay();
impl MemoryAccess for FakeDisplay {
    fn read_u8(&self, _addr: usize) -> Result<u8, MemoryAccessError> {
        unimplemented!()
    }

    fn write_u8(&mut self, _addr: usize, _data: u8) -> Result<(), MemoryAccessError> {
        unimplemented!()
    }
}

#[test]
fn test_hello() {
    let mut reader = rustemu86::loader::load("./tests/asms/hello").unwrap();
    let program = rustemu86::loader::map_to_memory(&mut reader).unwrap();
    let display: Box<dyn MemoryAccess> = Box::new(FakeDisplay());
    let serial = uart16550::uart_factory(Target::File("test_hello".to_string()));

    let result = rustemu86::start_emulation(
        program,
        EmulationMode::Test("test_hello".to_string()),
        serial,
        display
    );
    assert!(result.is_ok());

    let created_file = File::open("test_hello");
    assert!(created_file.is_ok());
    let mut contents = String::new();
    created_file.unwrap().read_to_string(&mut contents).unwrap();
    assert_eq!(contents, "Hello\n");
    std::fs::remove_file("test_hello").unwrap();
}

#[test]
fn test_push_pop() {
    let mut reader = rustemu86::loader::load("./tests/asms/push_pop").unwrap();
    let program = rustemu86::loader::map_to_memory(&mut reader).unwrap();
    let display: Box<dyn MemoryAccess> = Box::new(FakeDisplay());
    let serial = uart16550::uart_factory(Target::File("test_push_pop".to_string()));

    let result = rustemu86::start_emulation(
        program,
        EmulationMode::Test("test_push_pop".to_string()),
        serial,
        display
    );
    assert!(result.is_ok());

    let created_file = File::open("test_push_pop");
    assert!(created_file.is_ok());
    let mut contents = String::new();
    created_file.unwrap().read_to_string(&mut contents).unwrap();
    assert_eq!(contents, "CBA");
    std::fs::remove_file("test_push_pop").unwrap();
}
