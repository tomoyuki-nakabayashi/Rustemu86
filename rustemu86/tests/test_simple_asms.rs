extern crate rustemu86;
use rustemu86::args::EmulationMode;
use rustemu86::display::GtkVgaTextBuffer;

#[test]
fn test_simple_add() {
  let mut reader = rustemu86::loader::load("./tests/asms/simple_add").unwrap();
  let program = rustemu86::loader::map_to_memory(&mut reader).unwrap();
  let result = rustemu86::start_emulation(program, EmulationMode::Normal, GtkVgaTextBuffer::new());
  assert!(result.is_ok());
}

#[test]
fn test_jump() {
  let mut reader = rustemu86::loader::load("./tests/asms/jump").unwrap();
  let program = rustemu86::loader::map_to_memory(&mut reader).unwrap();
  let result = rustemu86::start_emulation(program, EmulationMode::Normal, GtkVgaTextBuffer::new());
  assert!(result.is_ok());
}