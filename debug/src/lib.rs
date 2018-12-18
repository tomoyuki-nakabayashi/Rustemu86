use std::fmt;
use std::io;

pub enum DebugMode {
    Disabled,
    PerCycleDump,
    Interactive,
}

impl DebugMode {
    pub fn do_cycle_end_action<T>(&self, cpu: &T)
        where T: fmt::Display
    {
        match self {
            DebugMode::Disabled => (),
            DebugMode::PerCycleDump => {
                println!("{}", &cpu);
            },
            DebugMode::Interactive => {
                println!("{}", &cpu);
                println!("Press Enter key to continue.");
                let mut key = String::new();
                io::stdin().read_line(&mut key).unwrap();
            }
        }
    }
}
