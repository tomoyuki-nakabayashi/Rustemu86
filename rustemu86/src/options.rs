use getopts::Options;
use std::{env, process};

#[derive(Debug, Clone)]
pub enum EmulationMode {
    Normal,
    Test(String),
    PerCycleDump,
    InteractiveMode,
}

#[derive(Debug)]
pub struct RustemuOptions {
    pub file_path: String,
    pub emulation_mode: EmulationMode,
}

fn print_usage(program: &str, opts: &Options) {
    let brief = format!("Usage: {} BINARY [options]", program);
    print!("{}", opts.usage(&brief));
    process::exit(0);
}

pub fn parse_args() -> RustemuOptions {
    let options: Vec<String> = env::args().collect();
    let program = options[0].clone();

    let mut opts = Options::new();
    opts.optflag("h", "help", "Print this help menu");
    opts.optflag(
        "v",
        "verbose",
        "Print verbose log messages during emulation",
    );
    opts.optflag("i", "interactive", "Run emulation with interactive shell.");

    let matches = match opts.parse(&options[1..]) {
        Ok(m) => m,
        Err(f) => panic!(f.to_string()),
    };

    if matches.opt_present("h") {
        print_usage(&program, &opts);
    }

    let mode = if matches.opt_present("v") {
        EmulationMode::PerCycleDump
    } else if matches.opt_present("i") {
        EmulationMode::InteractiveMode
    } else {
        EmulationMode::Normal
    };

    if matches.free.is_empty() {
        print_usage(&program, &opts)
    }

    RustemuOptions {
        file_path: matches.free[0].clone(),
        emulation_mode: mode,
    }
}
