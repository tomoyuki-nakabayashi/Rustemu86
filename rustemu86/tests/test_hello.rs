extern crate rustemu86;
use rustemu86::args::EmulationMode;
use std::fs::File;
use std::io::prelude::*;

#[test]
fn test_hello() {
  let mut reader = rustemu86::loader::load("./tests/asms/hello").unwrap();
  let mut program = rustemu86::loader::map_to_memory(&mut reader).unwrap();
  let result = rustemu86::start_emulation(&mut program, EmulationMode::IntegrationTest);
  assert!(result.is_ok());

  let created_file = File::open("test");
  assert!(created_file.is_ok());
  let mut contents = String::new();
  created_file.unwrap().read_to_string(&mut contents).unwrap();
  assert_eq!(contents, "Hello\n");
}


#[test]
fn test_push_pop() {
  let mut reader = rustemu86::loader::load("./tests/asms/push_pop").unwrap();
  let mut program = rustemu86::loader::map_to_memory(&mut reader).unwrap();
  let result = rustemu86::start_emulation(&mut program, EmulationMode::IntegrationTest);
  assert!(result.is_ok());

  let created_file = File::open("test");
  assert!(created_file.is_ok());
  let mut contents = String::new();
  created_file.unwrap().read_to_string(&mut contents).unwrap();
  assert_eq!(contents, "CBA");
}
