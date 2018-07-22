extern crate getopts;

mod args;
mod loader;

use args::Args;

fn main() {
  let args = args::parse_args();
  println!("{:?}", args);
}
