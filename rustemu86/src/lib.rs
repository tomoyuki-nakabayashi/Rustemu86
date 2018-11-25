extern crate bit_field;
extern crate byteorder;
extern crate getopts;
extern crate gio;
extern crate gtk;
extern crate num;
#[macro_use]
extern crate failure;
#[macro_use]
extern crate enum_primitive;
#[macro_use]
extern crate bitflags;

mod cpu;
pub mod display;
pub mod loader;
pub mod options;
pub mod peripherals;
pub mod rustemu86;
mod targets;

use crate::cpu::model::cpu_factory;
use crate::cpu::model::CpuModel;
use crate::display::GtkVgaTextBuffer;
use crate::options::EmulationMode;
use crate::peripherals::interconnect::Interconnect;
use crate::rustemu86::{DebugDesabled, DebugMode, Interactive, PerCycleDump};
use crate::targets::x86_64::{self, X86_64};

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
    vga_text_buffer: GtkVgaTextBuffer,
) -> Result<(), CpuError> {
    let mut interconnect = Interconnect::new(mode_option.clone(), vga_text_buffer);
    // Need to initialize according to elf.
    interconnect.init_memory(program, 0);
    let debug: Box<dyn DebugMode> = match mode_option {
        EmulationMode::Normal | EmulationMode::Test(_) => Box::new(DebugDesabled {}),
        EmulationMode::PerCycleDump => Box::new(PerCycleDump {}),
        EmulationMode::InteractiveMode => Box::new(Interactive {}),
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
    use crate::display::GtkVgaTextBuffer;

    #[test]
    fn success_emulation() {
        let mut reader = loader::load("./tests/asms/simple_add").unwrap();
        let program = loader::map_to_memory(&mut reader).unwrap();
        let result = start_emulation(program, EmulationMode::Normal, GtkVgaTextBuffer::new());
        assert!(result.is_ok());
    }

    #[test]
    fn success_emulation_with_per_cycle_dump() {
        let mut reader = loader::load("./tests/asms/simple_add").unwrap();
        let program = loader::map_to_memory(&mut reader).unwrap();
        let result = start_emulation(
            program,
            EmulationMode::PerCycleDump,
            GtkVgaTextBuffer::new(),
        );
        assert!(result.is_ok());
    }
}
