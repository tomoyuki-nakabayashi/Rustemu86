use std::collections::VecDeque;
use std::io;
use std::fmt;
use std::fmt::Write;

struct Uart16550<'a, T: 'a> {
  tx_buffer: &'a mut T,
}

impl<'a, T: 'a + Write> Uart16550<'a, T> {
  fn new(buffer: &'a mut T) -> Uart16550<T> {
    Uart16550 {
      tx_buffer: buffer,
    }
  }

  fn write(&mut self, c: char) {
    write!(self.tx_buffer, "{}", c).unwrap()
  }
}

struct DebugWriter {
  buffer: VecDeque<u8>,
}

impl DebugWriter {
  fn new() -> DebugWriter {
    DebugWriter {
      buffer: VecDeque::<u8>::new(),
    }
  }

  fn write_byte(&mut self, byte: u8) {
    self.buffer.push_back(byte);
  }

  fn write_string(&mut self, s: &str) {
    for byte in s.bytes() {
      match byte {
        0x20...0x7e | b'\n' => self.write_byte(byte),
        _ => self.write_byte(0xfe),
      }
    }
  }
}

impl Write for DebugWriter {
  fn write_str(&mut self, s: &str) -> fmt::Result {
    self.write_string(s);
    Ok(())
  }
}

struct DefaultWriter {}
impl Write for DefaultWriter {
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
  fn debug_write() {
    let mut buffer = DebugWriter::new();
    let mut uart16550 = Uart16550::new(&mut buffer);
    uart16550.write('a');

    assert_eq!(uart16550.tx_buffer.buffer.pop_front().unwrap(), 'a' as u8);
  }

  #[test]
  fn stdout_write() {
    let mut writer = DefaultWriter{};
    let mut uart16550 = Uart16550::new(&mut writer);
    uart16550.write('a');
  }
}