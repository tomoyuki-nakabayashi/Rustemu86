extern crate getopts;
extern crate byteorder;

mod args;
mod loader;
mod rustemu86;
mod register_file;
mod instructions;

use args::Args;

fn main() {
  let args = args::parse_args();
  println!("{:?}", args);
}
