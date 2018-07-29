use std::{env, process};
use getopts::Options;

#[derive(Debug)]
pub struct Args {
  pub file_path: String,
}

fn print_usage(program: &str, opts: &Options) {
  let brief = format!("Usage: {} BINARY [options]", program);
  print!("{}", opts.usage(&brief));
  process::exit(0);
}

pub fn parse_args() -> Args {
  let args: Vec<String> = env::args().collect();
  let program = args[0].clone();

  let mut opts = Options::new();
  opts.optflag("h", "help", "Print this help menu");

  let matches = match opts.parse(&args[1..]) {
    Ok(m) => { m }
    Err(f) => { panic!(f.to_string()) }
  };

  if matches.opt_present("h") {
    print_usage(&program, &opts);
  }

  if matches.free.is_empty() {
    print_usage(&program, &opts)
  }
  
  Args {
    file_path: matches.free[0].clone(),
  }
}