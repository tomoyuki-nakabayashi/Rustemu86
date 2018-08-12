use std::fs::File;
use std::io::Result;
use std::io::BufReader;
use std::io::Read;

#[derive(Debug)]
pub struct BinaryReader {
  pub reader: BufReader<File>,
}

impl BinaryReader {}

pub fn load(filename: &str) -> Result<BinaryReader> {
  Ok(BinaryReader {
    reader: BufReader::new(File::open(&filename)?),
  })
}

pub fn map_to_memory(binary: &mut BinaryReader) -> Result<Vec<u8>> {
  let mut program = Vec::new();
  binary.reader.read_to_end(&mut program)?;
  println!("Program load... {} bytes.", program.len());

  Ok(program)
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn load_mov_binary() {
    let load_result = load("./tests/asms/simple_add");
    assert!(load_result.is_ok());

    let mut binary_file = load_result.unwrap();
    let mut buffer = [0; 5];
    let len = binary_file.reader.read(&mut buffer);
    assert!(len.is_ok());

    let mov_rax: &[u8] = &[0xb8, 0x01, 0x00, 0x00, 0x00];
    assert_eq!(mov_rax, buffer);
  }

  #[test]
  fn load_failed() {
    let non_exist_file_open = load("./not_exist");
    assert!(non_exist_file_open.is_err());
  }
}
