use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

#[derive(Debug)]
pub struct BinaryReader {
  pub reader: BufReader<File>,
}

impl BinaryReader {

}

pub fn load(filename: &str) -> ::std::io::Result<BinaryReader> {
  Ok(BinaryReader{ reader: BufReader::new(File::open(&filename)?) })
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn load_mov_binary() {
    let load_result = load("../workspace/asms_for_test/mov");
    assert!(load_result.is_ok());

    let mut binary_file = load_result.unwrap();
    let mut buffer = [0; 6];
    let len = binary_file.reader.read(&mut buffer);
    assert!(len.is_ok());

    let mov_rax: &[u8] = &[0xb8, 0x00, 0x00, 0x00, 0x00, 0x00];
    assert_eq!(mov_rax, buffer);
  }

  #[test]
  fn load_failed() {
    let non_exist_file_open = load("./not_exist");
    assert!(non_exist_file_open.is_err());
  }
}