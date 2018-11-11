extern crate rustemu86;
use rustemu86::display::GtkVgaTextBuffer;
use rustemu86::options::EmulationMode;
use std::fs::File;
use std::io::prelude::*;

#[test]
fn test_hello_from_rust() {
    let mut reader = rustemu86::loader::load("./tests/bins/hello-x86_64.bin").unwrap();
    let program = rustemu86::loader::map_to_memory(&mut reader).unwrap();
    let result = rustemu86::start_emulation(
        program,
        EmulationMode::Test("test_hello_from_rust".to_string()),
        GtkVgaTextBuffer::new(),
    );
    assert!(result.is_ok());

    let created_file = File::open("test_hello_from_rust");
    assert!(created_file.is_ok());
    let mut contents = String::new();
    created_file.unwrap().read_to_string(&mut contents).unwrap();
    assert_eq!(contents, "Hello from Rust!\n");
    std::fs::remove_file("test_hello_from_rust").unwrap();
}
