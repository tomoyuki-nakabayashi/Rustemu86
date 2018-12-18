enum_from_primitive! {
  #[derive(Debug, Clone, Copy, PartialEq)]
  pub enum Reg64Id {
    Rax = 0x00,
    Rcx = 0x01,
    Rdx = 0x02,
    Rbx = 0x03,
    Rsp = 0x04,
    Rbp = 0x05,
    Rsi = 0x06,
    Rdi = 0x07,
    Unknown = 0xff,
  }
}
