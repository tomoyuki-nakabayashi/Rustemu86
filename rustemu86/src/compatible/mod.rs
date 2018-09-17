extern crate qemu_from;

pub struct CompatibleMode {}

impl CompatibleMode {
    pub fn new() -> CompatibleMode {
        CompatibleMode{}
    }

    pub fn run(&mut self) -> Result<(), ()> {
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn stop_at_hlt() {
        let program = vec![0xf4];

        let mut cpu = CompatibleMode::new();
        let result = cpu.run();

        assert!(result.is_ok());
    }
}