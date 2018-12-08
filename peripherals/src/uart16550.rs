use crate::memory_access::{MemoryAccess, MemoryAccessError, Result};
use std::cell::RefCell;
use std::collections::VecDeque;
use std::fmt::{self, Write};
use std::io;

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

pub enum Target {
    Stdout,
    Buffer,
}

pub fn uart_factory(target: Target) -> Box<dyn MemoryAccess> {
    match target {
        Target::Stdout => Box::new(Uart16550 {
            tx_writer: Box::new(StdoutWriter::new()),
        }),
        Target::Buffer => Box::new(UartLoopback {
            buffer: RefCell::new(VecDeque::new()),
        }),
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
