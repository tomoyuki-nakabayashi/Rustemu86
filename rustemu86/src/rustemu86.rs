use x86_64::Cpu;

pub trait DebugMode {
    fn do_cycle_end_action(&self, _cpu: &Cpu) {}
}

pub struct NoneDebug {}
impl DebugMode for NoneDebug {
    fn do_cycle_end_action(&self, _cpu: &Cpu) {}
}

pub struct PerCycleDump {}
impl DebugMode for PerCycleDump {
    fn do_cycle_end_action(&self, x86_64: &Cpu) {
        println!("{}", &x86_64);
    }
}

use std::io;
pub struct Interactive {}
impl DebugMode for Interactive {
    fn do_cycle_end_action(&self, x86_64: &Cpu) {
        println!("{}", &x86_64);
        println!("Press Enter key to continue.");
        let mut key = String::new();
        io::stdin().read_line(&mut key).unwrap();
    }
}
