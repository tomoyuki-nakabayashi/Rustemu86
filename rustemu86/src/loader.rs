use std::fs::File;
use std::io::prelude::*;

fn load() -> ::std::io::Result<()> {
  let mut binary_file = File::open("/home/tomoyuki/work/02.x86/Rustemu86/workspace/asms_for_test/mov")?;
  Ok(())
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn success_load() {
    assert!(load().is_ok());
  }
}