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

pub mod options;
pub mod display;
pub mod loader;
pub mod peripherals;
pub mod rustemu86;
mod cpu;
mod targets;

use options::EmulationMode;
use targets::x86_64::{self, X86_64};
use cpu::model::CpuModel;
use cpu::model::cpu_factory;
use display::GtkVgaTextBuffer;
use peripherals::interconnect::Interconnect;
use rustemu86::{DebugMode, Interactive, DebugDesabled, PerCycleDump};

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
        },
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use display::GtkVgaTextBuffer;

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
