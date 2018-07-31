use cpu::Cpu;

#[derive(Debug)]
pub struct Rustemu86 {
  // Must have cpu, memory, peripherals
  cpu: Cpu,
}

impl Rustemu86 {
  pub fn new() -> Rustemu86 {
    Rustemu86 {
      cpu: Cpu::new(),
    }
  }
}

pub trait DebugMode {
  fn do_cycle_end_action(&self, _cpu: &Cpu) {}
}

pub struct NoneDebug {}
impl DebugMode for NoneDebug {
  fn do_cycle_end_action(&self, _cpu: &Cpu) {}
}

pub struct PerCycleDump {}
impl DebugMode for PerCycleDump {
  fn do_cycle_end_action(&self, cpu: &Cpu) {
    println!("{}", &cpu);
  }
}

use std::io;
pub struct Interactive {}
impl DebugMode for Interactive {
  fn do_cycle_end_action(&self, cpu: &Cpu) {
    println!("{}", &cpu);
    println!("Press Enter key to continue.");
    let mut key = String::new();
    io::stdin().read_line(&mut key).unwrap();
  }
}
