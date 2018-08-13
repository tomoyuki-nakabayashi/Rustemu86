use std::io;
use std::fmt;
use std::fs;
use std::fmt::Write;

pub struct Uart16550<T> {
  tx_writer: T,
}

impl<T: UartWrite<T>> Uart16550<T> {
  fn new() -> Uart16550<T> {
    Uart16550 {
      tx_writer: T::new(),
    }
  }

  fn write(&mut self, c: char) {
    write!(self.tx_writer, "{}", c).expect("Printing to serial failed")
  }
}

pub trait UartWrite<T>: Write {
  fn new() -> T;
}
struct DefaultWriter {}

impl UartWrite<DefaultWriter> for DefaultWriter {
  fn new() -> DefaultWriter {
    DefaultWriter {}
  }
}

impl Write for DefaultWriter {
  fn write_str(&mut self, s: &str) -> fmt::Result {
    use std::io::Write;
    write!(io::stdout(), "{}", s).unwrap();
    Ok(())
  }
}

struct FileWriter {
  file: fs::File,
}

impl UartWrite<FileWriter> for FileWriter {
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
    let mut uart16550 = Uart16550::<DefaultWriter>::new();
    uart16550.write('a');
  }

  #[test]
  fn file_write() {
    let mut uart16550 = Uart16550::<FileWriter>::new();
    uart16550.write('a');

    let created_file = File::open("test");
    assert!(created_file.is_ok());
    let mut contents = String::new();
    created_file.unwrap().read_to_string(&mut contents).unwrap();
    assert_eq!(contents, "a");
  }
}