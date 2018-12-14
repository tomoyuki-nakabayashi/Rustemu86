use crate::memory_access::{MemoryAccess, MemoryAccessError, Result};
use std::cell::RefCell;
use std::collections::VecDeque;
use std::fmt::{self, Write};
use std::fs;

/// UART 16550 but it's not compatible yet.
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

/// Loopback write data to read buffer. This is mainly for integration test.
struct UartLoopback {
    buffer: RefCell<VecDeque<u8>>,
}

impl MemoryAccess for UartLoopback {
    // Read written data from FIFO buffer.
    fn read_u8(&self, _addr: usize) -> Result<u8> {
        Ok(self.buffer.borrow_mut().pop_front().unwrap())
    }

    // Write to FIFO buffer.
    fn write_u8(&mut self, _addr: usize, data: u8) -> Result<()> {
        self.buffer.get_mut().push_back(data);
        Ok(())
    }
}

/// UART which writes the tx data to a file.
struct UartFile {
    file: fs::File,
}

impl UartFile {
    fn new(path: &str) -> UartFile {
        let file = fs::File::create(&path).expect("Fail to create file.");
        UartFile { file }
    }
}

impl MemoryAccess for UartFile {
    /// Not implemented yet.
    fn read_u8(&self, _addr: usize) -> Result<u8> {
        Err(MemoryAccessError {})
    }

    /// TODO: Allow write only to tx buffer.
    fn write_u8(&mut self, _addr: usize, data: u8) -> Result<()> {
        use std::io::Write;
        write!(self.file, "{}", data as char).map_err(|_| MemoryAccessError{} )?;
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub enum Target {
    Stdout,
    Buffer,
    File(String)
}

pub fn uart_factory(target: Target) -> Box<dyn MemoryAccess> {
    use self::Target::*;
    match target {
        Stdout => Box::new(Uart16550 {
            tx_writer: Box::new(StdoutWriter::new()),
        }),
        Buffer => Box::new(UartLoopback {
            buffer: RefCell::new(VecDeque::new()),
        }),
        File(path) => Box::new( UartFile::new(&path) ),
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
        print!("{}", s);
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn stdout_write() {
        let mut uart = uart_factory(Target::Stdout);
        assert!(uart.write_u8(0, b'a').is_ok());
    }

    #[test]
    fn buffer_write() {
        let mut loopback_uart = uart_factory(Target::Buffer);
        assert!(loopback_uart.write_u8(0, b'o').is_ok());
        assert!(loopback_uart.write_u8(0, b'k').is_ok());

        assert_eq!(loopback_uart.read_u8(0).unwrap(), b'o');
        assert_eq!(loopback_uart.read_u8(0).unwrap(), b'k');
    }
}
