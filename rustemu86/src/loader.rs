use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

#[derive(Debug)]
pub struct BinaryReader {
  file: BufReader<File>,
}

impl BinaryReader {

}

fn load(filename: &str) -> ::std::io::Result<BinaryReader> {
  Ok(BinaryReader{ file: BufReader::new(File::open(&filename)?) })
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn load_mov_binary() {
    let load_result = load("/home/tomoyuki/work/02.x86/Rustemu86/workspace/asms_for_test/mov");
    assert!(load_result.is_ok());

    let mut buffer = [0; 4];
    let mut binary_file = load_result.unwrap();
    binary_file.file.read(&mut buffer);

    assert_eq!(0xb8, buffer[0]);
    assert_eq!(0x00, buffer[1]);
    assert_eq!(0x00, buffer[2]);
    assert_eq!(0x00, buffer[3]);
  }

  fn load_failed() {
    let non_exist_file_open = load("./not_exist");
    assert!(non_exist_file_open.is_err());
  }
}