use crate::peripherals::memory_access::{MemoryAccess, MemoryAccessError, Result};
use std::fmt;
use std::fmt::Write;
use std::fs;
use std::io;

pub struct Uart16550 {
    tx_writer: Box<Write>,
}

impl MemoryAccess for Uart16550 {
    /// Not implemented yet.
    fn read_u8(&self, _addr: usize) -> Result<u8> {
        Err(MemoryAccessError {})
    }

    /// TODO: Allow write only to tx buffer.
    fn write_u8(&mut self, _addr: usize, data: u8) -> Result<()> {
        write!(self.tx_writer, "{}", data as char).expect("Printing to serial failed");
        Ok(())
    }
}

pub enum Target {
    Stdout,
    File(String),
}

pub fn uart_factory(target: Target) -> Uart16550 {
    match target {
        Target::Stdout => Uart16550 {
            tx_writer: Box::new(StdoutWriter::new()),
        },
        Target::File(path) => Uart16550 {
            tx_writer: Box::new(FileWriter::new(&path)),
        },
    }
}

struct StdoutWriter;
impl StdoutWriter {
    fn new() -> StdoutWriter {
        StdoutWriter {}
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
    fn new(path: &str) -> FileWriter {
        let file = fs::File::create(&path).expect("Fail to create file.");
        FileWriter { file: file }
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
    use crate::peripherals::memory_access::MemoryAccess;
    use std::fs::File;
    use std::io::prelude::*;

    #[test]
    fn stdout_write() {
        let mut uart = uart_factory(Target::Stdout);
        assert!(uart.write_u8(0, b'a').is_ok());
    }

    #[test]
    fn file_write() {
        let mut uart = uart_factory(Target::File("test_file_write.txt".to_string()));
        assert!(uart.write_u8(0, b'a').is_ok());

        let created_file = File::open("test_file_write.txt");
        assert!(created_file.is_ok());
        let mut contents = String::new();
        created_file.unwrap().read_to_string(&mut contents).unwrap();
        assert_eq!(contents, "a");
        fs::remove_file("test_file_write.txt").unwrap();
    }
}
