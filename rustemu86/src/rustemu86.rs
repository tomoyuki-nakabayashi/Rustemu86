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

pub trait EmulationStrategy {
  fn do_cycle_end_action(cpu: &Cpu) {}
}

pub struct NormalEmulation {}
impl EmulationStrategy for NormalEmulation {
  fn do_cycle_end_action(cpu: &Cpu) {}
}

pub struct PerCycleDumpEmulation {}
impl EmulationStrategy for PerCycleDumpEmulation {
  fn do_cycle_end_action() {
    println!("*** Instructions Executed. ***");
  }
}
