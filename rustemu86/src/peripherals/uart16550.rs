use std::io::Write;
use std::str;
use std::collections::VecDeque;

struct Uart16550 {
  tx_buffer: VecDeque<char>,
}

impl Uart16550 {
  fn new() -> Uart16550 {
    Uart16550 {
      tx_buffer: VecDeque::<char>::new(),
    }
  }

  fn write(&mut self, c: char) {
    self.tx_buffer.push_back(c)
  }
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn read_string() {
    let mut uart16550 = Uart16550::new();
    uart16550.write('a');

    assert_eq!(uart16550.tx_buffer.pop_front().unwrap(), 'a');
  }
}