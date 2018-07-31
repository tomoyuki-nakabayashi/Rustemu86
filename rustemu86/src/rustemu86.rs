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
  fn do_cycle_end_action(&self, _cpu: &Cpu) {}
}

pub struct NormalEmulation {}
impl EmulationStrategy for NormalEmulation {
  fn do_cycle_end_action(&self, _cpu: &Cpu) {}
}

pub struct PerCycleDumpEmulation {}
impl EmulationStrategy for PerCycleDumpEmulation {
  fn do_cycle_end_action(&self, cpu: &Cpu) {
    println!("*** Instructions Executed. ***");
    println!("{}", &cpu);
  }
}

#[cfg(test)]
mod test {
  use super::*;

  fn execute_strategy(strategy: &EmulationStrategy) {
    strategy.do_cycle_end_action(&Cpu::new());
  }

  #[test]
  fn emulation_strategy() {
    let dump = PerCycleDumpEmulation{};
    execute_strategy(&dump);
  }
}