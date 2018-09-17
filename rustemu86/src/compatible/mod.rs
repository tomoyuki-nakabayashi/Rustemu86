extern crate qemu_from;

use peripherals::interconnect::Interconnect;

pub struct CompatibleMode {
    bus: Interconnect,
}

impl CompatibleMode {
    pub fn new(peripheral_bus: Interconnect) -> CompatibleMode {
        CompatibleMode {
            bus: peripheral_bus,
        }
    }

    pub fn run(&mut self) -> Result<(), CompatibleException> {
        let inst_candidate = self.bus.fetch_inst_candidate(0);
        if inst_candidate[0] == 0xf4 {
            Ok(())
        } else {
            Err(CompatibleException("Invalid instruction.".to_string()))
        }
    }
}

#[derive(Debug)]
pub struct CompatibleException(String);

#[cfg(test)]
mod test {
    use super::*;
    use args::EmulationMode;
    use display::GtkVgaTextBuffer;

    #[test]
    fn stop_at_hlt() {
        let program = vec![0xf4];
        let mut bus = Interconnect::new(EmulationMode::Normal, GtkVgaTextBuffer::new());
        bus.init_memory(program);

        let mut cpu = CompatibleMode::new(bus);
        let result = cpu.run();

        assert!(result.is_ok(), "{:?}", result.err());
    }
}
