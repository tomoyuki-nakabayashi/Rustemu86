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
extern crate serde_derive;
extern crate serde;
extern crate serde_json;

pub mod args;
pub mod cpu;
pub mod display;
pub mod loader;
pub mod peripherals;
pub mod rustemu86;

use args::EmulationMode;
use cpu::Cpu;
use display::GtkVgaTextBuffer;
use peripherals::interconnect::Interconnect;
use rustemu86::{Interactive, NoneDebug, PerCycleDump};

pub struct CpuError {}

pub fn start_emulation(
    program: Vec<u8>,
    mode_option: EmulationMode,
    vga_text_buffer: GtkVgaTextBuffer,
) -> Result<(), CpuError> {
    let mut interconnect = Interconnect::new(mode_option.clone(), vga_text_buffer);
    interconnect.init_memory(program);
    let mut cpu = Cpu::new(interconnect);

    let result = match mode_option {
        EmulationMode::Normal | EmulationMode::Test(_) => cpu.run(&NoneDebug {}),
        EmulationMode::PerCycleDump => cpu.run(&PerCycleDump {}),
        EmulationMode::InteractiveMode => cpu.run(&Interactive {}),
    };

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
