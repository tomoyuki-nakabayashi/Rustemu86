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
  fn binary_load() {
    let binary_reader = load("/home/tomoyuki/work/02.x86/Rustemu86/workspace/asms_for_test/mov");
    assert!(binary_reader.is_ok());

    let non_exist_file_open = load("./not_exist");
    assert!(non_exist_file_open.is_err());
  }
}