#[macro_use]
extern crate failure;
#[macro_use]
extern crate enum_primitive;
#[macro_use]
extern crate bitflags;

pub mod options;
mod targets;

use cpu::model::cpu_factory;
use cpu::model::CpuModel;
use crate::options::EmulationMode;
use debug::DebugMode;
use crate::targets::x86_64::{self, X86_64};

use peripherals::interconnect::Interconnect;
use peripherals::memory_access::MemoryAccess;

pub struct CpuError {}

/* Pseudo code for switching target isa.
fn run(target: TargetArch, interconnect: Interconnect, mode: EmulationMode) {
    match target {
        X86 => { let x86_64: X86 = cpu_factory(interconnect); x86_64.run(&mode); }
    }
}
*/

pub fn start_emulation(
    program: Vec<u8>,
    mode_option: EmulationMode,
    serial: Box<dyn MemoryAccess>,
    display: Box<dyn MemoryAccess>,
) -> Result<(), CpuError> {
    let mut interconnect = Interconnect::new(serial, display);
    // Need to initialize according to elf.
    interconnect.init_memory(&program, 0);
    let debug: DebugMode = match mode_option {
        EmulationMode::Normal | EmulationMode::Test(_) => DebugMode::Disabled,
        EmulationMode::PerCycleDump => DebugMode::PerCycleDump,
        EmulationMode::InteractiveMode => DebugMode::Interactive,
    };

    let mut cpu = cpu_factory::<X86_64>(interconnect, debug);
    let result = cpu.run();

    match result {
        Ok(_) => Ok(()),
        Err(err) => {
            println!("Emulation stopped at error: {:?}", err);
            Err(CpuError {})
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use gui::display::GtkVgaTextBuffer;
    use peripherals::uart16550::{uart_factory, Target};
    use loader;

    #[test]
    fn success_emulation() {
        let mut reader = loader::load("./tests/asms/simple_add").unwrap();
        let program = loader::map_to_memory(&mut reader).unwrap();
        let serial = uart_factory(Target::Buffer);
        let display = Box::new(GtkVgaTextBuffer::new());

        let result = start_emulation(program, EmulationMode::Normal, serial, display);
        assert!(result.is_ok());
    }

    #[test]
    fn success_emulation_with_per_cycle_dump() {
        let mut reader = loader::load("./tests/asms/simple_add").unwrap();
        let program = loader::map_to_memory(&mut reader).unwrap();
        let serial = uart_factory(Target::Buffer);
        let display = Box::new(GtkVgaTextBuffer::new());

        let result = start_emulation(program, EmulationMode::PerCycleDump, serial, display);
        assert!(result.is_ok());
    }
}
