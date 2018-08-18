extern crate rustemu86;

fn main() {
  let args = rustemu86::args::parse_args();
  let mut reader = rustemu86::loader::load(&args.file_path).unwrap();
  let program = rustemu86::loader::map_to_memory(&mut reader).unwrap();
  let result = rustemu86::start_emulation(program, args.emulation_mode);
  assert!(result.is_ok());
}
