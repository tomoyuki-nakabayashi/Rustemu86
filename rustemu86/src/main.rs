extern crate getopts;
extern crate byteorder;
extern crate num;

mod args;
mod loader;
mod rustemu86;

use args::Args;

fn main() {
  let args = args::parse_args();
  println!("{:?}", args);
}
