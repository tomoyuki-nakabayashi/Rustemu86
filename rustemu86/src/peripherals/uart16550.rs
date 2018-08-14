use std::io;
use std::fmt;
use std::fs;
use std::fmt::Write;

pub struct Uart16550 {
  tx_writer: Box<Write>,
}

impl Uart16550 {
  pub fn write(&mut self, c: u8) {
    write!(self.tx_writer, "{}", c as char).expect("Printing to serial failed")
  }
}

pub enum Target {
  Stdout,
  File,
}

pub fn uart_factory(target: Target) -> Uart16550 {
  match target {
    Target::Stdout => Uart16550 { tx_writer: Box::new(StdoutWriter::new()) },
    Target::File => Uart16550 { tx_writer: Box::new(FileWriter::new()) },
  }
}

struct StdoutWriter;
impl StdoutWriter{
  fn new() -> StdoutWriter {
    StdoutWriter{}
  }
}

impl Write for StdoutWriter {
  fn write_str(&mut self, s: &str) -> fmt::Result {
    use std::io::Write;
    write!(io::stdout(), "{}", s).unwrap();
    Ok(())
  }
}

struct FileWriter {
  file: fs::File,
}

impl FileWriter {
  fn new() -> FileWriter {
    let file = fs::File::create("test").expect("Fail to create file.");
    FileWriter {
      file: file,
    }
  }
}

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
    let mut uart = uart_factory(Target::Stdout);
    uart.write(b'a');
  }

  #[test]
  fn file_write() {
    let mut uart = uart_factory(Target::File);
    uart.write(b'a');

    let created_file = File::open("test");
    assert!(created_file.is_ok());
    let mut contents = String::new();
    created_file.unwrap().read_to_string(&mut contents).unwrap();
    assert_eq!(contents, "a");
  }
}