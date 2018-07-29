extern crate rustemu86;

fn main() {
  let args = rustemu86::args::parse_args();
  let reader = rustemu86::loader::load(&args.file_path);
  assert!(reader.is_ok());
  println!("{:?}", args);
}
