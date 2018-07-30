extern crate rustemu86;

fn main() {
  let args = rustemu86::args::parse_args();
  let mut reader = rustemu86::loader::load(&args.file_path).unwrap();
  let result = rustemu86::start_emulation(
      &mut reader, args.emulation_mode);
  assert!(result.is_ok());
}
