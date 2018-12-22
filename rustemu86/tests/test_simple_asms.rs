use peripherals::error::MemoryAccessError;
use peripherals::memory_access::MemoryAccess;
use peripherals::uart16550::{self, Target};
use loader;
use rustemu86::options::EmulationMode;

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
fn test_simple_add() {
    let mut reader = loader::load("./tests/asms/simple_add").unwrap();
    let program = loader::map_to_memory(&mut reader).unwrap();
    let display: Box<dyn MemoryAccess> = Box::new(FakeDisplay());
    let serial = uart16550::uart_factory(Target::Buffer);

    let result = rustemu86::start_emulation(program, EmulationMode::Normal, serial, display);
    assert!(result.is_ok());
}

#[test]
fn test_jump() {
    let mut reader = loader::load("./tests/asms/jump").unwrap();
    let program = loader::map_to_memory(&mut reader).unwrap();
    let display: Box<dyn MemoryAccess> = Box::new(FakeDisplay());
    let serial = uart16550::uart_factory(Target::Buffer);

    let result = rustemu86::start_emulation(program, EmulationMode::Normal, serial, display);
    assert!(result.is_ok());
}
