use std::io;
use std::fmt;
use std::fs;
use std::fmt::Write;

pub struct Uart16550 {
  tx_writer: Box<UartWrite>,
}

impl Uart16550 {
  pub fn new<F>(create_writer: F) -> Uart16550
    where F: FnOnce() -> Box<UartWrite>
  {
    Uart16550 {
      tx_writer: create_writer(),
    }
  }

  pub fn write(&mut self, c: u8) {
    write!(self.tx_writer, "{}", c as char).expect("Printing to serial failed")
  }
}

pub trait UartWrite: Write {}

pub struct UartFactory;
impl UartFactory {
  pub fn create(&self) -> Uart16550
  {
    Uart16550::new(|| Box::new(FileWriter::new()))
  }
}

pub struct DefaultWriter;

impl DefaultWriter{
  pub fn new() -> DefaultWriter {
    DefaultWriter{}
  }
}

impl UartWrite for DefaultWriter {}

impl Write for DefaultWriter {
  fn write_str(&mut self, s: &str) -> fmt::Result {
    use std::io::Write;
    write!(io::stdout(), "{}", s).unwrap();
    Ok(())
  }
}

pub struct FileWriter {
  file: fs::File,
}

impl FileWriter {
  pub fn new() -> FileWriter {
    let file = fs::File::create("test").expect("Fail to create file.");
    FileWriter {
      file: file,
    }
  }
}

impl UartWrite for FileWriter {}

impl Write for FileWriter {
  fn write_str(&mut self, s: &str) -> fmt::Result {
    use std::io::Write;
    write!(self.file, "{}", s).unwrap();
    Ok(())
  }
}

#[cfg(test)]
mod test {
  use super::*;
  use std::fs::File;
  use std::io::prelude::*;

  #[test]
  fn stdout_write() {
    let factory = UartFactory;
    let mut uart16550 = factory.create();;
    uart16550.write(b'a');
  }

  #[test]
  fn file_write() {
    let factory = UartFactory;
    let mut uart16550 = factory.create();;
    uart16550.write(b'a');

    let created_file = File::open("test");
    assert!(created_file.is_ok());
    let mut contents = String::new();
    created_file.unwrap().read_to_string(&mut contents).unwrap();
    assert_eq!(contents, "a");
  }
}