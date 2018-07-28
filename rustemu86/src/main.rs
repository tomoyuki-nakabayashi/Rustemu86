#[macro_use]
extern crate rustemu86;

fn main() {
  let args = rustemu86::args::parse_args();
  println!("{:?}", args);
}
